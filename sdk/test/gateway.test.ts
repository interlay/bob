import { assert, describe, expect, it } from 'vitest';
import { GatewaySDK } from '../src/gateway';
import { MAINNET_GATEWAY_BASE_URL } from '../src/gateway/client';
import { SYMBOL_LOOKUP } from '../src/gateway/tokens';
import { Chain, ChainId } from '../src/gateway/types';
import { ZeroAddress } from 'ethers';
import nock from 'nock';
import * as bitcoin from 'bitcoinjs-lib';

const TBTC = SYMBOL_LOOKUP[ChainId.BOB]['tbtc'];
const TBTC_ADDRESS = TBTC.address;
const SOLVBTC = SYMBOL_LOOKUP[ChainId.BOB]['solvbtc'];
const SOLVBTC_ADDRESS = SOLVBTC.address;

describe('Gateway Tests', () => {
    it('should get chains', async () => {
        const gatewaySDK = new GatewaySDK('bob');
        assert.deepEqual(gatewaySDK.getChains(), Object.values(Chain));
    });

    it('should reject invalid chain', async () => {
        expect(() => {
            new GatewaySDK('bob-testnet');
        }).toThrowError('Invalid chain');
    });

    it('should get quote', async () => {
        const gatewaySDK = new GatewaySDK('mainnet');

        const mockQuote = {
            gatewayAddress: ZeroAddress,
            baseTokenAddress: TBTC_ADDRESS,
            dustThreshold: 1000,
            satoshis: 1000,
            fee: 10,
            bitcoinAddress: '',
            txProofDifficultyFactor: 3,
            strategyAddress: ZeroAddress,
            baseToken: TBTC,
            outputToken: TBTC,
        };

        nock(`${MAINNET_GATEWAY_BASE_URL}`).get(`/quote/${TBTC_ADDRESS}?satoshis=1000`).times(5).reply(200, mockQuote);

        assert.deepEqual(
            await gatewaySDK.getQuote({
                toChain: 'BOB',
                toToken: 'tBTC',
                toUserAddress: ZeroAddress,
                amount: 1000,
            }),
            mockQuote
        );
        assert.deepEqual(
            await gatewaySDK.getQuote({
                toChain: 'bob',
                toToken: 'tbtc',
                toUserAddress: ZeroAddress,
                amount: 1000,
            }),
            mockQuote
        );
        assert.deepEqual(
            await gatewaySDK.getQuote({
                toChain: 60808,
                toToken: 'tbtc',
                toUserAddress: ZeroAddress,
                amount: 1000,
            }),
            mockQuote
        );
        assert.deepEqual(
            await gatewaySDK.getQuote({
                toChain: 'BOB',
                toToken: TBTC_ADDRESS,
                toUserAddress: ZeroAddress,
                amount: 1000,
            }),
            mockQuote
        );
        assert.deepEqual(
            await gatewaySDK.getQuote({
                toChain: 'BOB',
                toToken: 'tBTC',
                toUserAddress: ZeroAddress,
                amount: 1000,
                gasRefill: 5,
            }),
            { ...mockQuote, fee: 15 }
        );

        // get the total available without amount
        nock(`${MAINNET_GATEWAY_BASE_URL}`).get(`/quote/${TBTC_ADDRESS}`).reply(200, mockQuote);
        assert.deepEqual(
            await gatewaySDK.getQuote({
                toChain: 'BOB',
                toToken: TBTC_ADDRESS,
                toUserAddress: ZeroAddress,
            }),
            mockQuote
        );
    });

    it('should throw error on invalid token', async () => {
        const gatewaySDK = new GatewaySDK('mainnet');
        await expect(async () => {
            await gatewaySDK.getQuote({
                toChain: 'BOB',
                toToken: 'unknownToken',
                toUserAddress: ZeroAddress,
                amount: 1000,
            });
        }).rejects.toThrowError('Unknown output token');
    });

    it('should reject invalid network', async () => {
        const gatewaySDK = new GatewaySDK('testnet');
        await expect(async () => {
            await gatewaySDK.getQuote({
                toChain: 'BOB',
                toToken: 'tbtc',
                toUserAddress: ZeroAddress,
                amount: 1000,
            });
        }).rejects.toThrowError('Invalid output chain');

        await expect(async () => {
            await gatewaySDK.getQuote({
                toChain: 'unknownChain',
                toToken: 'tbtc',
                toUserAddress: ZeroAddress,
                amount: 1000,
            });
        }).rejects.toThrowError('Invalid output chain');
    });

    it('should start order', async () => {
        const gatewaySDK = new GatewaySDK('bob');

        const mockQuote = {
            gatewayAddress: ZeroAddress,
            baseTokenAddress: TBTC_ADDRESS,
            dustThreshold: 1000,
            satoshis: 10000,
            fee: 10,
            bitcoinAddress: 'bc1qafk4yhqvj4wep57m62dgrmutldusqde8adh20d',
            txProofDifficultyFactor: 3,
            strategyAddress: ZeroAddress,
        };

        nock(`${MAINNET_GATEWAY_BASE_URL}`).post(`/order`).reply(201, {
            uuid: '00000000-0000-0000-0000-000000000000',
            opReturnHash: '0x8d0fd89210149d4219c87fa814a4bcde0c6a36b8fe2dff52b1d3eaa9e7cf0a9a',
        });

        await expect(async () => {
            await gatewaySDK.startOrder(mockQuote, {
                toChain: 'BOB',
                toToken: 'tBTC',
                toUserAddress: '2N8DbeaBdjkktkRzaKL1tHj9FQELV7jA8Re',
                amount: 10000,
                fromChain: 'Bitcoin',
                fromToken: 'BTC',
                fromUserAddress: 'bc1qar0srrr7xfkvy5l643lydnw9re59gtzzwf5mdq',
            });
        }).rejects.toThrowError('Invalid user address');

        const result = await gatewaySDK.startOrder(mockQuote, {
            toChain: 'BOB',
            toToken: 'tBTC',
            toUserAddress: ZeroAddress,
            amount: 10000,
            fromChain: 'Bitcoin',
            fromToken: 'BTC',
            fromUserAddress: 'bc1qar0srrr7xfkvy5l643lydnw9re59gtzzwf5mdq',
            feeRecipient: '3DFVKuT9Ft4rWpysAZ1bHpg55EBy1HVPcr',
            fee: 600,
        });

        assert.isDefined(result.psbtBase64);
        const psbt = bitcoin.Psbt.fromBase64(result.psbtBase64!);
        assert.deepEqual(
            psbt.txOutputs[0].script,
            bitcoin.script.compile([bitcoin.opcodes.OP_RETURN, Buffer.from(result.opReturnHash.slice(2), 'hex')])
        );
        assert.deepEqual(psbt.txOutputs[1].address, '3DFVKuT9Ft4rWpysAZ1bHpg55EBy1HVPcr');
        assert.deepEqual(psbt.txOutputs[1].value, 600);
    });

    it('should get strategies', async () => {
        nock(`${MAINNET_GATEWAY_BASE_URL}`)
            .get(`/strategies`)
            .reply(200, [
                {
                    strategyAddress: ZeroAddress,
                    inputTokenAddress: TBTC_ADDRESS,
                    strategyName: 'Pell Network (tBTC)',
                    strategyType: 'staking',
                },
            ]);
        nock(`${MAINNET_GATEWAY_BASE_URL}`)
            .get(`/quote/${TBTC_ADDRESS}?satoshis=1000&strategy=${ZeroAddress}`)
            .times(4)
            .reply(200, {
                gatewayAddress: ZeroAddress,
                dustThreshold: 1000,
                satoshis: 1000,
                fee: 10,
                bitcoinAddress: '',
                txProofDifficultyFactor: 3,
                strategyAddress: ZeroAddress,
            });

        const gatewaySDK = new GatewaySDK('bob');
        const strategies = await gatewaySDK.getStrategies();

        assert.lengthOf(strategies, 1);
        assert.isDefined(strategies[0].inputToken);
        assert.isNull(strategies[0].outputToken);
        assert.equal(strategies[0].integration.slug, 'pell-network-tbtc');

        const strategy = strategies[0];
        await gatewaySDK.getQuote({
            toUserAddress: ZeroAddress,
            amount: 1000,

            toChain: strategy.chain.chainId,
            toToken: strategy.inputToken.symbol,
            strategyAddress: strategy.address,
        });
    });

    it('should get tokens', async () => {
        nock(`${MAINNET_GATEWAY_BASE_URL}`).get(`/tokens?includeStrategies=false`).reply(200, [ZeroAddress]);

        const gatewaySDK = new GatewaySDK('bob');
        assert.deepEqual(await gatewaySDK.getTokenAddresses(false), [ZeroAddress]);
    });

    it('should get orders', async () => {
        const mockOrder = {
            gatewayAddress: ZeroAddress,
            baseTokenAddress: TBTC_ADDRESS,
            txid: '',
            status: false,
            timestamp: 0,
            tokens: '0',
            satoshis: 0,
            fee: 0,
            txProofDifficultyFactor: 0,
            satsToConvertToEth: 0,
        };
        nock(`${MAINNET_GATEWAY_BASE_URL}`)
            .get(`/orders/${ZeroAddress}`)
            .reply(200, [
                // staking - success
                {
                    ...mockOrder,
                    satoshis: 1000,
                    fee: 0,
                    status: true,
                    strategyAddress: ZeroAddress,
                    outputTokenAmount: '2000',
                    outputTokenAddress: SOLVBTC_ADDRESS,
                },
                // staking - pending
                {
                    ...mockOrder,
                    satoshis: 1000,
                    fee: 0,
                    strategyAddress: ZeroAddress,
                },
                // staking - failed
                {
                    ...mockOrder,
                    satoshis: 1000,
                    fee: 0,
                    status: true,
                    strategyAddress: ZeroAddress,
                },
                // swapping - pending
                {
                    ...mockOrder,
                    satoshis: 1000,
                    fee: 0,
                },
                // swapping - success
                {
                    ...mockOrder,
                    satoshis: 1000,
                    fee: 0,
                    status: true,
                },
                // staking - failed (wBTC)
                {
                    ...mockOrder,
                    baseTokenAddress: SYMBOL_LOOKUP[ChainId.BOB]['wbtc'],
                    satoshis: 1000,
                    fee: 0,
                    status: true,
                    strategyAddress: ZeroAddress,
                },
                // swapping - success (wBTC)
                {
                    ...mockOrder,
                    baseTokenAddress: SYMBOL_LOOKUP[ChainId.BOB]['wbtc'],
                    satoshis: 1000,
                    fee: 0,
                    status: true,
                },
            ]);

        const gatewaySDK = new GatewaySDK('bob');
        const orders = await gatewaySDK.getOrders(ZeroAddress);
        assert.lengthOf(orders, 7);

        assert.strictEqual(orders[0].getTokenAmount(), '2000'); // success (staking)
        assert.strictEqual(orders[1].getTokenAmount(), undefined); // pending (staking)
        assert.strictEqual(orders[2].getTokenAmount(), 10000000000000); // failed (staking)
        assert.strictEqual(orders[3].getTokenAmount(), 10000000000000); // pending (swapping)
        assert.strictEqual(orders[4].getTokenAmount(), 10000000000000); // success (swapping)
        assert.strictEqual(orders[5].getTokenAmount(), 1000); // failed (staking)
        assert.strictEqual(orders[6].getTokenAmount(), 1000); // success (swapping)

        assert.strictEqual(orders[0].getToken()!.address, SOLVBTC_ADDRESS);
        assert.strictEqual(orders[1].getToken(), undefined);
    });
});
