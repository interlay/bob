import { OrdinalsClient, InscriptionJson, OutputJson, InscriptionId, SatPoint } from '../src/ordinal-api';
import { assert, describe, it } from 'vitest';

describe('Ordinal API Tests', () => {
    it('should get inscription from id', async () => {
        const client = new OrdinalsClient('mainnet');
        // Deploy ORDI - BRC20
        const inscriptionJson = await client.getInscriptionFromId({
            txid: 'b61b0172d95e266c18aea0c624db987e971a5d6d4ebc2aaed85da4642d635735',
            index: 0,
        });

        const expectedInscriptionJson: InscriptionJson<InscriptionId, SatPoint> = {
            address: 'bc1pxaneaf3w4d27hl2y93fuft2xk6m4u3wc4rafevc6slgd7f5tq2dqyfgy06',
            charms: [],
            children: [],
            content_length: 94,
            content_type: 'text/plain;charset=utf-8',
            effective_content_type: 'text/plain;charset=utf-8',
            fee: 4830,
            height: 779832,
            id: InscriptionId.fromString('b61b0172d95e266c18aea0c624db987e971a5d6d4ebc2aaed85da4642d635735i0'),
            number: 348020,
            next: InscriptionId.fromString('693bd98380ad6e58f83de6068c236c6eb9d629c825cc3342c2d93f24c6762c6di0'),
            parent: null,
            parents: [],
            previous: InscriptionId.fromString('4f0ff6259efa9d56b16664e6c5c9755c148818dc6bbca98f7f9166b277e4b7c0i0'),
            rune: null,
            sat: null,
            satpoint: SatPoint.fromString('b61b0172d95e266c18aea0c624db987e971a5d6d4ebc2aaed85da4642d635735:0:0'),
            timestamp: 1678248991,
            value: 10000,
        };

        assert.deepStrictEqual(inscriptionJson, expectedInscriptionJson);
    });

    it('should get inscriptions', { timeout: 10000 }, async () => {
        const client = new OrdinalsClient('mainnet');
        const inscriptionsJson = await client.getInscriptions();
        // assert that inscriptionsJson is not null, undefined or empty
        assert.isNotNull(inscriptionsJson);
        assert.isNotEmpty(inscriptionsJson);
    });

    it('should get inscriptions from block', { timeout: 10000 }, async () => {
        const client = new OrdinalsClient('mainnet');
        const block: number = 804691;
        const inscriptionsJson = await client.getInscriptionsFromBlock(block);
        const expectedInscriptionsJson = {
            ids: [
                { txid: 'a1d4de525915eb5bff293f21e266ef760e274f8454add9731c9d298d031b024c', index: 0 },
                { txid: 'dfe942a58b7e29a3952d8d1ed6608086c66475d20bc7bdbc3d784d616f9a6a7a', index: 0 },
                { txid: '3b161db6bcde6c6a5da98bfdbca465d23c80ffd71097c2b74da46c824c441fab', index: 0 },
                { txid: 'f9cb2752c20957df62d3a743e6242758c9770f3870ecea159fb76852f55ce71f', index: 0 },
                { txid: '46ff2e2a6dd22cbe7eb38016ce71559e406f8bda35de421ae902e7472ca940f5', index: 0 },
                { txid: 'e2ae28b5c589c3799de31e83fe95351a81591c6eb231602ed112b8b4c928e820', index: 0 },
                { txid: '7dd18fc67ea45d96d2b3e5151f4461bdb029bb8488fad29e2c191470dda3f929', index: 0 },
            ],
            more: false,
            page_index: 0,
        };
        assert.deepStrictEqual(inscriptionsJson, expectedInscriptionsJson);
    });

    it('should get inscriptions from UTXO', { timeout: 10000 }, async () => {
        const client = new OrdinalsClient('mainnet');
        const outputJson = await client.getInscriptionsFromOutPoint({
            txid: 'dfe942a58b7e29a3952d8d1ed6608086c66475d20bc7bdbc3d784d616f9a6a7a',
            vout: 0,
        });
        const expectedOutputJson: OutputJson = {
            value: 10737,
            script_pubkey: '5120e18a5367c5d11ee31d10bf4c53e743a7479c70e3336e70dbdea1fd927305c022',
            address: 'bc1pux99xe796y0wx8gshax98e6r5arecu8rxdh8pk77587eyuc9cq3q2e3nng',
            transaction: 'dfe942a58b7e29a3952d8d1ed6608086c66475d20bc7bdbc3d784d616f9a6a7a',
            sat_ranges: null,
            inscriptions: ['dfe942a58b7e29a3952d8d1ed6608086c66475d20bc7bdbc3d784d616f9a6a7ai0'],
            runes: {},
            indexed: true,
            spent: false,
        };
        assert.deepStrictEqual(outputJson, expectedOutputJson);
    });

    it('should get inscriptions from Sat', { timeout: 10000 }, async () => {
        const client = new OrdinalsClient('mainnet');
        const sat: number = 100;
        const satJson = await client.getInscriptionsFromSat(sat);
        const expectedSatJson = {
            block: 0,
            charms: [],
            cycle: 0,
            decimal: '0.100',
            degree: '0°0′0″100‴',
            epoch: 0,
            inscriptions: [],
            name: 'nvtdijuwxht',
            number: 100,
            offset: 100,
            percentile: '0.0000000000047619047671428594%',
            period: 0,
            rarity: 'common',
            satpoint: null,
            timestamp: 1231006505,
        };
        assert.deepStrictEqual(satJson, expectedSatJson);
    });

    it('should get inscriptions from start block', { timeout: 10000 }, async () => {
        const client = new OrdinalsClient('mainnet');
        const startBlock: number = 2537138;
        const inscriptions = await client.getInscriptionsFromStartBlock(startBlock);
        // assert that inscriptions is not null or undefined
        assert.isNotNull(inscriptions);
    });
});
