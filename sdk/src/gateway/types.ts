import type { EsploraClient } from '../esplora';

type ChainSlug = string | number;
type TokenSymbol = string;

export type EvmAddress = string;

export enum Chain {
    // NOTE: we also support Bitcoin testnet
    BITCOIN = 'bitcoin',
    BOB = 'bob',
    BOB_SEPOLIA = 'bob-sepolia',
}

export enum ChainId {
    BOB = 60808,
    BOB_SEPOLIA = 808813,
}

/**
 * Designed to be compatible with the Superchain token list.
 * https://github.com/ethereum-optimism/ethereum-optimism.github.io
 */
export interface Token {
    chainId: number;
    address: string;
    name: string;
    symbol: string;
    decimals: number;
    logoURI: string;
}

/**
 * Designed to be compatible with the Swing SDK.
 * https://developers.swing.xyz/reference/sdk/get-a-quote
 */
export interface GatewayQuoteParams {
    /** @description Source chain slug or ID */
    fromChain: ChainSlug;
    /** @description Destination chain slug or ID */
    toChain: ChainSlug;
    /** @description Token symbol or address on source chain */
    fromToken: TokenSymbol;
    /** @description Token symbol or address on destination chain */
    toToken: TokenSymbol;
    /** @description Wallet address on source chain */
    fromUserAddress: string;
    /** @description Wallet address on destination chain */
    toUserAddress: string;
    /** @description Amount of tokens to send from the source chain */
    amount: number | string; // NOTE: modified from Swing

    /** @description Maximum slippage percentage between 0.01 and 0.03 (Default: 0.03) */
    maxSlippage?: number;

    /** @description Unique affiliate ID for tracking */
    affiliateId?: string;
    /** @description Optionally filter the type of routes returned */
    type?: 'swap' | 'deposit' | 'withdraw' | 'claim';
    /** @description The percentage of fee charged by partners in Basis Points (BPS) units. This will override the default fee rate configured via platform. 1 BPS = 0.01%. The maximum value is 1000 (which equals 10%). The minimum value is 1 (which equals 0.01%). */
    fee?: number;

    // NOTE: the following are new fields added by us
    /** @description Amount of satoshis to swap for ETH */
    gasRefill?: number;
    /** @description Wallet public key on source chain */
    fromUserPublicKey?: string;
    /** @description Strategy address */
    strategyAddress?: string;
    /** @description Partner address */
    feeRecipient?: string;
}

/**
 * IntegrationType
 * @enum {string}
 */
type GatewayIntegrationType = 'bridge' | 'dex' | 'staking' | 'lending';

interface GatewayIntegration {
    type: GatewayIntegrationType;
    /** @example pell-network-wbtc */
    slug: string;
    /** @example Pell Network (wBTC) */
    name: string;
    /** Format: uri */
    logo: string;
    monetization: boolean;
}

type GatewayStrategyType = 'deposit' | 'withdraw' | 'claim' | 'router' | 'bridge';

interface GatewayToken {
    /** @example ETH */
    symbol: string;
    /** @example 0x000000000000000 */
    address: string;
    /** @example https://raw.githubusercontent.com/bob-collective/assets/master/blockchains/ethereum/assets/0x0000000000000000000000000000000000000000/logo.png */
    logo: string;
    /** @example 18 */
    decimals: number;
    /** @example ethereum */
    chain: string;
}

type GatewayChainType = 'evm' | 'ibc' | 'solana' | 'multiversx' | 'bitcoin' | 'ton' | 'tron';

interface GatewayChain {
    id: string;
    chainId: number;
    /** @example ethereum */
    slug: string;
    /** @example Ethereum */
    name: string;
    /** @example https://raw.githubusercontent.com/bob-collective/assets/master/blockchains/ethereum/info/logo.png */
    logo: string;
    type: GatewayChainType;
    /** @description Single chain swapping is supported for this chain. */
    singleChainSwap: boolean;
    /** @description Single chain staking is supported for this chain. */
    singleChainStaking: boolean;
    nativeToken?: GatewayToken;
    /**
     * @description URL template to transaction details.
     * @example https://etherscan.io/tx/{txHash}
     */
    txExplorer?: string;
    /**
     * @description URL template to token details.
     * @example https://etherscan.io/tokens/{address}
     */
    tokenExplorer?: string;
    /**
     * @description URL template to RPC endpoint.
     * @example https://eth-mainnet.g.alchemy.com/v2/xxx
     */
    rpcUrl?: string;
}

/**
 * Designed to be compatible with the Swing SDK.
 * https://developers.swing.xyz/reference/sdk/staking/contracts
 */
export interface GatewayStrategyContract {
    id: string;
    type: GatewayStrategyType;
    /**
     * @description Contract address
     * @example 0x...
     */
    address: string;
    /** @example deposit */
    method: string;
    /** @example bob */
    chain: GatewayChain;
    /** @example segment */
    integration: GatewayIntegration;

    inputToken: GatewayToken;
    /** @example seWBTC */
    outputToken: GatewayToken | null;
}

export type GatewayQuote = {
    /** @description The gateway address */
    gatewayAddress: EvmAddress;
    /** @description The base token address (e.g. wBTC or tBTC) */
    baseTokenAddress: EvmAddress;
    /** @description The minimum amount of Bitcoin to send */
    dustThreshold: number;
    /** @description The satoshi output amount */
    satoshis: number;
    /** @description The fee paid in satoshis (includes gas refill) */
    fee: number;
    /** @description The Bitcoin address to send BTC */
    bitcoinAddress: string;
    /** @description The number of confirmations required to confirm the Bitcoin tx */
    txProofDifficultyFactor: number;
    /** @description The optional strategy address */
    strategyAddress?: EvmAddress;
};

/** @dev Internal */
export type GatewayCreateOrderRequest = {
    gatewayAddress: EvmAddress;
    strategyAddress?: EvmAddress;
    satsToConvertToEth: number;
    userAddress: EvmAddress;
    gatewayExtraData?: string;
    strategyExtraData?: string;
    satoshis: number;
};

export interface GatewayOrderResponse {
    /** @description The gateway address */
    gatewayAddress: EvmAddress;
    /** @description The base token address (e.g. wBTC or tBTC) */
    baseTokenAddress: EvmAddress;
    /** @description The Bitcoin txid */
    txid: string;
    /** @description True when the order was executed on BOB */
    status: boolean;
    /** @description When the order was created */
    timestamp: number;
    // TODO: return converted fee
    /** @description The converted satoshi amount */
    tokens: string;
    /** @description The satoshi output amount */
    satoshis: number;
    /** @description The fee paid in satoshis (includes gas refill) */
    fee: number;
    /** @description The number of confirmations required to confirm the Bitcoin tx */
    txProofDifficultyFactor: number;
    /** @description The optional strategy address */
    strategyAddress?: EvmAddress;
    /** @description The gas refill in satoshis */
    satsToConvertToEth: number;
    /** @description The amount of ETH received */
    outputEthAmount?: string;
    /** @description The output token (from strategies) */
    outputTokenAddress?: EvmAddress;
    /** @description The output amount (from strategies) */
    outputTokenAmount?: string;
    /** @description The tx hash on the EVM chain */
    txHash?: string;
}

export type OrderStatusData = {
    confirmations: number;
};

export type OrderStatus =
    | {
          confirmed: false;
          pending?: never;
          success?: never;
          data: OrderStatusData;
      }
    | {
          confirmed?: never;
          pending: true;
          success?: never;
          data: OrderStatusData;
      }
    | {
          confirmed?: never;
          pending?: never;
          success: boolean;
          data: OrderStatusData;
      };

/** Order given by the Gateway API once the bitcoin tx is submitted */
export type GatewayOrder = Omit<
    GatewayOrderResponse & {
        /** @description The gas refill in satoshis */
        gasRefill: number;
    },
    'satsToConvertToEth'
> & {
    /** @description Get the actual token address received */
    getTokenAddress(): string | undefined;
    /** @description Get the actual token received */
    getToken(): Token | undefined;
    /** @description Get the actual amount received of the token */
    getTokenAmount(): string | number | undefined;
    /** @description Get the number of confirmations */
    getConfirmations(esploraClient: EsploraClient, latestHeight?: number): Promise<number>;
    /** @description Get the actual order status */
    getStatus(esploraClient: EsploraClient, latestHeight?: number): Promise<OrderStatus>;
};

export type GatewayTokensInfo = {
    /** @description The base token (e.g. wBTC or tBTC) */
    baseToken: Token;
    /** @description The output token (e.g. uniBTC or SolvBTC.BBN) */
    outputToken?: Token;
};

/** @dev Internal */
export type GatewayCreateOrderResponse = {
    uuid: string;
    opReturnHash: string;
};

/** @dev The success type on create order */
export type GatewayStartOrder = GatewayCreateOrderResponse & {
    bitcoinAddress: string;
    satoshis: number;
    psbtBase64?: string;
};

/** @dev Internal */
export interface GatewayStrategy {
    strategyAddress: string;
    strategyName: string;
    strategyType: 'staking' | 'lending';
    projectName: string;
    projectLogo?: string;
    inputTokenAddress: string;
    outputTokenAddress?: string;
}
