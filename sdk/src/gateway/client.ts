import { ethers, AbiCoder } from 'ethers';
import {
    GatewayQuoteParams,
    GatewayQuote,
    Chain,
    ChainId,
    Token,
    GatewayStrategyContract,
    GatewayCreateOrderRequest,
    GatewayCreateOrderResponse,
    GatewayOrder,
    GatewayOrderResponse,
    GatewayStartOrder,
    GatewayStrategy,
    EvmAddress,
    GatewayTokensInfo,
    OrderStatus,
} from './types';
import { SYMBOL_LOOKUP, ADDRESS_LOOKUP } from './tokens';
import { createBitcoinPsbt } from '../wallet';
import { Network } from 'bitcoin-address-validation';
import { EsploraClient } from '../esplora';
import { stripHexPrefix } from '../utils';

type Optional<T, K extends keyof T> = Omit<T, K> & Partial<T>;

/**
 * Base url for the mainnet Gateway API.
 * @default "https://gateway-api-mainnet.gobob.xyz"
 */
export const MAINNET_GATEWAY_BASE_URL = 'https://gateway-api-mainnet.gobob.xyz';

/**
 * Base url for the testnet Gateway API.
 * @default "https://gateway-api-testnet.gobob.xyz"
 */
export const TESTNET_GATEWAY_BASE_URL = 'https://gateway-api-testnet.gobob.xyz';

/**
 * Gateway REST HTTP API client
 */
export class GatewayApiClient {
    private chain: Chain.BOB | Chain.BOB_SEPOLIA;
    private baseUrl: string;

    /**
     * @constructor
     * @param chainName The chain name.
     */
    constructor(chainName: string) {
        switch (chainName) {
            case 'mainnet':
            case Chain.BOB:
                this.chain = Chain.BOB;
                this.baseUrl = MAINNET_GATEWAY_BASE_URL;
                break;
            case 'testnet':
            case Chain.BOB_SEPOLIA:
                this.chain = Chain.BOB_SEPOLIA;
                this.baseUrl = TESTNET_GATEWAY_BASE_URL;
                break;
            default:
                throw new Error('Invalid chain');
        }
    }

    private get chainId(): ChainId {
        return this.chain === Chain.BOB ? ChainId.BOB : ChainId.BOB_SEPOLIA;
    }

    /**
     * Returns all chains supported by the SDK.
     *
     * @returns {string[]} The array of chain names.
     */
    getChains(): string[] {
        return Object.values(Chain);
    }

    /**
     * Get a quote from the Gateway API for swapping or staking BTC.
     *
     * @param params The parameters for the quote.
     */
    async getQuote(
        params: Optional<GatewayQuoteParams, 'amount' | 'fromChain' | 'fromToken' | 'fromUserAddress' | 'toUserAddress'>
    ): Promise<GatewayQuote & GatewayTokensInfo> {
        const isMainnet =
            params.toChain === ChainId.BOB ||
            (typeof params.toChain === 'string' && params.toChain.toLowerCase() === Chain.BOB);
        const isTestnet =
            params.toChain === ChainId.BOB_SEPOLIA ||
            (typeof params.toChain === 'string' && params.toChain.toLowerCase() === Chain.BOB_SEPOLIA);

        const isInvalidNetwork = !isMainnet && !isTestnet;
        const isMismatchMainnet = isMainnet && this.chain !== Chain.BOB;
        const isMismatchTestnet = isTestnet && this.chain !== Chain.BOB_SEPOLIA;

        // TODO: switch URL if `toChain` is different chain?
        if (isInvalidNetwork || isMismatchMainnet || isMismatchTestnet) {
            throw new Error('Invalid output chain');
        }

        let outputTokenAddress = '';
        let strategyAddress: string | undefined;

        const toToken = params.toToken.toLowerCase();
        if (params.strategyAddress?.startsWith('0x')) {
            strategyAddress = params.strategyAddress;
        }

        const chainId = this.chainId;
        if (toToken.startsWith('0x')) {
            outputTokenAddress = toToken;
        } else if (isMainnet && SYMBOL_LOOKUP[chainId][toToken]) {
            outputTokenAddress = SYMBOL_LOOKUP[chainId][toToken].address;
        } else if (isTestnet && SYMBOL_LOOKUP[chainId][toToken]) {
            outputTokenAddress = SYMBOL_LOOKUP[chainId][toToken].address;
        } else {
            throw new Error('Unknown output token');
        }

        const url = new URL(`${this.baseUrl}/quote/${outputTokenAddress}`);
        if (strategyAddress) {
            url.searchParams.append('strategy', `${strategyAddress}`);
        }
        const atomicAmount = params.amount;
        if (atomicAmount) {
            url.searchParams.append('satoshis', `${atomicAmount}`);
        }

        const response = await fetch(url, {
            headers: {
                'Content-Type': 'application/json',
                Accept: 'application/json',
            },
        });

        const quote: GatewayQuote = await response.json();
        return {
            ...quote,
            fee: quote.fee + (params.gasRefill || 0),
            baseToken: ADDRESS_LOOKUP[chainId][quote.baseTokenAddress],
            outputToken: quote.strategyAddress ? ADDRESS_LOOKUP[chainId][outputTokenAddress] : undefined,
        };
    }

    // TODO: add error handling
    /**
     * Start an order via the Gateway API to reserve liquidity. This is step 1 of 2, see the {@link finalizeOrder} method.
     *
     * @param gatewayQuote The quote given by the {@link getQuote} method.
     * @param params The parameters for the quote, same as before.
     * @returns {Promise<GatewayStartOrder>} The success object.
     */
    async startOrder(
        gatewayQuote: GatewayQuote,
        params: Optional<GatewayQuoteParams, 'toToken' | 'amount'>
    ): Promise<GatewayStartOrder> {
        if (!params.toUserAddress.startsWith('0x') || !ethers.isAddress(params.toUserAddress)) {
            throw new Error('Invalid user address');
        }

        const abiCoder = new AbiCoder();
        const request: GatewayCreateOrderRequest = {
            gatewayAddress: gatewayQuote.gatewayAddress,
            strategyAddress: gatewayQuote.strategyAddress,
            satsToConvertToEth: params.gasRefill || 0,
            userAddress: params.toUserAddress,
            gatewayExtraData: undefined,
            // TODO: update strategy data
            strategyExtraData: abiCoder.encode(['uint256'], [0]),
            satoshis: gatewayQuote.satoshis,
        };

        const response = await fetch(`${this.baseUrl}/order`, {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json',
                Accept: 'application/json',
            },
            body: JSON.stringify(request),
        });

        if (!response.ok) {
            throw new Error('Failed to create order');
        }

        const data: GatewayCreateOrderResponse = await response.json();
        // NOTE: could remove this check but good for sanity
        if (data.opReturnHash != calculateOpReturnHash(request)) {
            throw new Error('Invalid OP_RETURN hash');
        }

        let psbtBase64: string;
        if (
            params.fromUserAddress &&
            typeof params.fromChain === 'string' &&
            params.fromChain.toLowerCase() === Chain.BITCOIN
        ) {
            psbtBase64 = await createBitcoinPsbt({
                fromAddress: params.fromUserAddress,
                toAddress: gatewayQuote.bitcoinAddress,
                amount: gatewayQuote.satoshis,
                publicKey: params.fromUserPublicKey,
                opReturnData: data.opReturnHash,
                confirmationTarget: gatewayQuote.txProofDifficultyFactor,
                feeRecipient: params.feeRecipient,
                feeAmount: calculateFeeAmount(gatewayQuote.satoshis, params.fee),
            });
        }

        return {
            uuid: data.uuid,
            opReturnHash: data.opReturnHash,
            bitcoinAddress: gatewayQuote.bitcoinAddress,
            satoshis: gatewayQuote.satoshis,
            psbtBase64,
        };
    }

    /**
     * Finalize an order via the Gateway API by providing the Bitcoin transaction. The tx will
     * be validated for correctness and forwarded to the mempool so there is no need to separately
     * broadcast the transaction. This is step 2 of 2, see the {@link startOrder} method.
     *
     * @param uuid The id given by the {@link startOrder} method.
     * @param bitcoinTxOrId The hex encoded Bitcoin transaction or txid.
     * @returns {Promise<string>} The Bitcoin txid.
     */
    async finalizeOrder(uuid: string, bitcoinTxOrId: string): Promise<string> {
        bitcoinTxOrId = stripHexPrefix(bitcoinTxOrId);

        let bitcoinTxHex: string;
        if (bitcoinTxOrId.length === 64) {
            const esploraClient = new EsploraClient(this.chain === Chain.BOB ? Network.mainnet : Network.testnet);
            bitcoinTxHex = await esploraClient.getTransactionHex(bitcoinTxOrId);
        } else {
            bitcoinTxHex = bitcoinTxOrId;
        }

        const response = await fetch(`${this.baseUrl}/order/${uuid}`, {
            method: 'PATCH',
            headers: {
                'Content-Type': 'application/json',
                Accept: 'application/json',
            },
            body: JSON.stringify({ bitcoinTx: bitcoinTxHex }),
        });

        if (!response.ok) {
            throw new Error('Failed to update order');
        }

        return await response.json();
    }

    /**
     * Returns all pending and completed orders for this account.
     *
     * @param userAddress The user's EVM address.
     * @returns {Promise<GatewayOrder[]>} The array of account orders.
     */
    async getOrders(userAddress: EvmAddress): Promise<(GatewayOrder & GatewayTokensInfo)[]> {
        const chainId = this.chainId;
        const response = await this.fetchGet(`${this.baseUrl}/orders/${userAddress}`);
        const orders: GatewayOrderResponse[] = await response.json();
        return orders.map((order) => {
            function getFinal<L, R>(base?: L, output?: R) {
                return order.status
                    ? order.strategyAddress
                        ? output
                            ? output // success
                            : base // failed
                        : base // success
                    : order.strategyAddress // pending
                      ? output
                      : base;
            }
            const getTokenAddress = (): string | undefined => {
                return getFinal(order.baseTokenAddress, order.outputTokenAddress);
            };
            const getToken = (): Token | undefined => {
                return ADDRESS_LOOKUP[chainId][getTokenAddress()];
            };
            const getConfirmations = async (esploraClient: EsploraClient, latestHeight?: number) => {
                const txStatus = await esploraClient.getTransactionStatus(order.txid);
                if (!latestHeight) {
                    latestHeight = await esploraClient.getLatestHeight();
                }
                return txStatus.confirmed ? latestHeight - txStatus.block_height! + 1 : 0;
            };
            return {
                gasRefill: order.satsToConvertToEth,
                ...order,
                baseToken: ADDRESS_LOOKUP[chainId][order.baseTokenAddress],
                outputToken: ADDRESS_LOOKUP[chainId][order.outputTokenAddress],
                getTokenAddress,
                getToken,
                getTokenAmount() {
                    let amount = order.satoshis - order.fee;
                    const token = getToken();
                    if (token && !order.outputTokenAmount) {
                        amount *= Math.pow(10, token.decimals - 8);
                    }
                    return getFinal(amount, order.outputTokenAmount);
                },
                getConfirmations,
                async getStatus(esploraClient: EsploraClient, latestHeight?: number): Promise<OrderStatus> {
                    const confirmations = await getConfirmations(esploraClient, latestHeight);
                    const hasEnoughConfirmations = confirmations >= order.txProofDifficultyFactor;
                    const data = { confirmations };
                    return !hasEnoughConfirmations
                        ? { confirmed: false, data }
                        : order.status
                          ? order.strategyAddress
                              ? order.outputTokenAddress
                                  ? { success: true, data }
                                  : { success: false, data }
                              : { success: true, data }
                          : { pending: true, data };
                },
            };
        });
    }

    /**
     * Returns all strategies supported by the Gateway API.
     *
     * @returns {Promise<GatewayStrategyContract[]>} The array of strategies.
     */
    async getStrategies(): Promise<GatewayStrategyContract[]> {
        const response = await this.fetchGet(`${this.baseUrl}/strategies`);

        const chainName = this.chain.toString();
        const chainId = this.chainId;

        const strategies: GatewayStrategy[] = await response.json();
        return strategies.map((strategy) => {
            const strategySlug = slugify(strategy.strategyName);
            const inputToken = ADDRESS_LOOKUP[chainId][strategy.inputTokenAddress];
            const outputToken = strategy.outputTokenAddress
                ? ADDRESS_LOOKUP[chainId][strategy.outputTokenAddress]
                : undefined;
            return {
                id: strategySlug,
                type: 'deposit',
                address: strategy.strategyAddress,
                method: '',
                chain: {
                    id: '', // TODO
                    chainId: chainId,
                    slug: chainName,
                    name: chainName,
                    logo: '', // TODO
                    type: 'evm',
                    singleChainSwap: true,
                    singleChainStaking: true,
                },
                integration: {
                    type: strategy.strategyType,
                    slug: strategySlug,
                    name: strategy.strategyName,
                    logo: strategy.projectLogo || outputToken?.logoURI || '',
                    monetization: false,
                },
                inputToken: {
                    symbol: inputToken.symbol,
                    address: inputToken.address,
                    logo: inputToken.logoURI,
                    decimals: inputToken.decimals,
                    chain: chainName,
                },
                outputToken: outputToken
                    ? {
                          symbol: outputToken.symbol,
                          address: outputToken.address,
                          logo: outputToken.logoURI,
                          decimals: outputToken.decimals,
                          chain: chainName,
                      }
                    : null,
            };
        });
    }

    /**
     * Returns all tokens supported by the Gateway API.
     *
     * @param includeStrategies Also include output tokens via strategies (e.g. staking or lending).
     * @returns {Promise<EvmAddress[]>} The array of token addresses.
     */
    async getTokenAddresses(includeStrategies: boolean = true): Promise<EvmAddress[]> {
        const response = await this.fetchGet(`${this.baseUrl}/tokens?includeStrategies=${includeStrategies}`);
        return response.json();
    }

    /**
     * Same as {@link getTokenAddresses} but with additional info.
     *
     * @param includeStrategies Also include output tokens via strategies (e.g. staking or lending).
     * @returns {Promise<Token[]>} The array of tokens.
     */
    async getTokens(includeStrategies: boolean = true): Promise<Token[]> {
        // https://github.com/ethereum-optimism/ecosystem/blob/c6faa01455f9e846f31c0343a0be4c03cbeb2a6d/packages/op-app/src/hooks/useOPTokens.ts#L10
        const tokens = await this.getTokenAddresses(includeStrategies);
        return tokens.map((token) => ADDRESS_LOOKUP[this.chainId][token]).filter((token) => token !== undefined);
    }

    private async fetchGet(url: string) {
        return await fetch(url, {
            method: 'GET',
            headers: {
                'Content-Type': 'application/json',
                Accept: 'application/json',
            },
        });
    }
}

/**
 * Should compute the same OP_RETURN hash as the Gateway API and smart contracts.
 * This is used for data integrity checking.
 */
function calculateOpReturnHash(req: GatewayCreateOrderRequest) {
    const abiCoder = new AbiCoder();
    return ethers.keccak256(
        abiCoder.encode(
            ['address', 'address', 'uint256', 'address', 'bytes', 'bytes'],
            [
                req.gatewayAddress,
                req.strategyAddress || ethers.ZeroAddress,
                req.satsToConvertToEth,
                req.userAddress,
                req.gatewayExtraData || '0x',
                req.strategyExtraData || '0x',
            ]
        )
    );
}

function slugify(str: string): string {
    return str
        .toLowerCase()
        .replace(/ /g, '-')
        .replace(/[^\w-]+/g, '');
}

function calculateFeeAmount(amount: number, fee?: number) {
    if (typeof fee === 'undefined' || fee < 1 || fee > 1000) {
        fee = 0;
    }

    const feeDecimal = fee / 10000;
    const feeAmount = amount * feeDecimal;
    return feeAmount;
}
