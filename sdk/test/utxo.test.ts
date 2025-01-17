import { vi, describe, it, assert, Mock, expect, beforeEach } from 'vitest';
import { AddressType, getAddressInfo, Network } from 'bitcoin-address-validation';
import { Address, NETWORK, OutScript, Script, Transaction, p2sh, p2wpkh, selectUTXO } from '@scure/btc-signer';
import { hex, base64 } from '@scure/base';
import {
    createBitcoinPsbt,
    getInputFromUtxoAndTx,
    estimateTxFee,
    Input,
    getBalance,
    findSafeUtxos,
} from '../src/wallet/utxo';
import { TransactionOutput } from '@scure/btc-signer/psbt';
import { OrdinalsClient, OutPoint } from '../src/ordinal-api';
import { EsploraClient, UTXO } from '../src/esplora';

vi.mock(import('@scure/btc-signer'), async (importOriginal) => {
    const actual = await importOriginal();

    return {
        ...actual,
        selectUTXO: vi.fn(actual.selectUTXO),
    };
});

// TODO: Add more tests using https://github.com/paulmillr/scure-btc-signer/tree/5ead71ea9a873d8ba1882a9cd6aa561ad410d0d1/test/bitcoinjs-test/fixtures/bitcoinjs
// TODO: Ensure that the paymentAddresses have sufficient funds to create the transaction
describe('UTXO Tests', () => {
    beforeEach(() => {
        vi.restoreAllMocks();
    });

    it('should spend from address to create a transaction with an OP return output', { timeout: 50000 }, async () => {
        // Addresses where randomly picked from blockstream.info
        const paymentAddresses = [
            // P2WPKH: https://blockstream.info/address/bc1qar0srrr7xfkvy5l643lydnw9re59gtzzwf5mdq
            'bc1qar0srrr7xfkvy5l643lydnw9re59gtzzwf5mdq',
            // P2SH-P2WPKH: https://blockstream.info/address/3DFVKuT9Ft4rWpysAZ1bHpg55EBy1HVPcr
            // TODO: Use a real P2SH-P2WPKH address
            // TODO: Add the pubkey to allow spending from the outputs
            // '3DFVKuT9Ft4rWpysAZ1bHpg55EBy1HVPcr',
            // P2PKH: https://blockstream.info/address/1Kr6QSydW9bFQG1mXiPNNu6WpJGmUa9i1g
            '1Kr6QSydW9bFQG1mXiPNNu6WpJGmUa9i1g',
            // P2TR https://blockstream.info/address/bc1peqr5a5kfufvsl66444jm9y8qq0s87ph0zv4lfkcs7h40ew02uvsqkhjav0
            'bc1peqr5a5kfufvsl66444jm9y8qq0s87ph0zv4lfkcs7h40ew02uvsqkhjav0',
        ];

        const toAddresses = [
            // P2SH
            '35iMHbUZeTssxBodiHwEEkb32jpBfVueEL',
            // P2WSH
            'bc1q6rgl33d3s9dugudw7n68yrryajkr3ha9q8q24j20zs62se4q9tsqdy0t2q',
            // P2WPKH
            'bc1qafk4yhqvj4wep57m62dgrmutldusqde8adh20d',
            // P2PKH
            '1Pr4Y216BpyGxj1Qa9GUzLQU6uUuzE61YS',
            // P2TR
            'bc1peqr5a5kfufvsl66444jm9y8qq0s87ph0zv4lfkcs7h40ew02uvsqkhjav0',
        ];
        const amount = 1000;

        // EVM address for OP return
        let opReturn = '0xC02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2';

        // Refactor to execute in parallel
        await Promise.all(
            toAddresses.map(async (toAddress) => {
                await Promise.all(
                    paymentAddresses.map(async (paymentAddress) => {
                        const paymentAddressType = getAddressInfo(paymentAddress).type;

                        let pubkey: string | undefined;

                        if (
                            paymentAddressType === AddressType.p2sh ||
                            paymentAddressType === AddressType.p2wsh ||
                            paymentAddressType === AddressType.p2tr
                        ) {
                            // Use a random public key for P2SH-P2WPKH
                            pubkey = '03b366c69e8237d9be7c4f1ac2a7abc6a79932fbf3de4e2f6c04797d7ef27abfe1';
                        }
                        // Note: it is possible that the above addresses have spent all of their funds
                        // and the transaction will fail.
                        const psbtBase64 = await createBitcoinPsbt(paymentAddress, toAddress, amount, pubkey, opReturn);
                        const transaction = Transaction.fromPSBT(base64.decode(psbtBase64));

                        assert(transaction);

                        // Check that output conditions are correct
                        const addressType = getAddressInfo(toAddress).type;

                        // Get all outputs and add them to array
                        const outputs: TransactionOutput[] = [];

                        for (let i = 0; i < transaction.outputsLength; i++) {
                            const output = transaction.getOutput(i);

                            outputs.push(output);
                        }

                        for (const output of outputs) {
                            // All outputs should have an amount and a script
                            assert.exists(output.amount);
                            assert.exists(output.script);
                            // Check OP_RETURN
                            if (opReturn.startsWith('0x')) {
                                opReturn = opReturn.slice(2);
                            }
                            if (output.amount! === BigInt(0)) {
                                const parsedScript = Script.decode(output.script!);

                                assert.equal(parsedScript.length, 2);
                                assert.equal(parsedScript[0], 'RETURN');
                                assert.deepEqual(parsedScript[1], hex.decode(opReturn));

                                // Check the transfer script to the toAddress
                            } else if (output.amount === BigInt(amount)) {
                                // eslint-disable-next-line @typescript-eslint/no-explicit-any
                                const scriptDecoded = OutScript.decode(output.script!) as any;

                                // Remove "p2" from the address type as it's exluced in the OutScript type
                                assert.equal(scriptDecoded.type, addressType.slice(2));

                                // eslint-disable-next-line @typescript-eslint/no-explicit-any
                                const address = Address(NETWORK).decode(toAddress) as any;

                                assert.deepEqual(scriptDecoded.hash, address.hash);

                                // Check the possible change output
                            } else {
                                // eslint-disable-next-line @typescript-eslint/no-explicit-any
                                const scriptDecoded = OutScript.decode(output.script!) as any;

                                // Remove "p2" from the address type as it's exluced in the OutScript type
                                assert.equal(scriptDecoded.type, paymentAddressType.slice(2));
                            }
                        }
                    })
                );
            })
        );
    });

    it('should get input from an UTXO and its transaction', async () => {
        const testset = [
            // - P2WPKH (Unisat)
            {
                utxo: {
                    // https://blockstream.info/tx/47ba2a950608cc1cba218b98d18ee5bc74e4f80023f2ecd1d81e87a88557eec4
                    txid: '47ba2a950608cc1cba218b98d18ee5bc74e4f80023f2ecd1d81e87a88557eec4',
                    vout: 2,
                    value: 100000,
                    confirmed: true,
                    height: 821504,
                },
                transaction: Transaction.fromRaw(
                    Buffer.from(
                        '0100000000010192488f51132fc73b22bb4d1fd81b77537fe7cf51136a8440c271f84959d5cb590a00000000ffffffff0b37050a0000000000160014e8df018c7e326cc253faac7e46cdc51e68542c423703f9000000000016001476b266d98da6faba506dabc3b00677c64f188a5840420f0000000000160014d756a6f5d39388f233152e46cc6a158b4b6b09aa0ae70300000000001600141c68092871e29a51e89cb47e9c4cb9094e8601340d242b0000000000160014de9ddc9f2dac708171028a51191b07797b1e2232f56d1200000000001600140a92a0fff2025bd29a7850887539652882509c4c389c1c00000000001600147ab5220da0a05704f05877e7cff9bfb6c251205b93ef0300000000001600142c73866d315405c14943ba3da0b187aa3d7cd9441cd305000000000016001486b28a10dd335f089b9e6f1700f0375f0aebf80e7802e50000000000160014b3b58eafe9cfbbf56657cc89dec1d6c334829fe95124470d000000001600147ce3e33d3d4a123ca358399f5c0925a85dbe905d02473044022024d21268f3b3e4150962e2826612b2302db21e8e12bbe58f21f7460035205ef7022028375980ea8cebd8127565ef04ce2aa844a64be5ccc84789839a87beb866795f0121030c49ba250eaf17bc57bc07d07665647c72b81b9c420c8f65c1f5df2a0fbffad000000000',
                        'hex'
                    )
                ),
                addressType: AddressType.p2wpkh,
                publicKey: undefined,
            },
            // - P2SH-P2WPKH (Xverse, Unisat)
            {
                utxo: {
                    // https://blockstream.info/tx/10f746b824fcd844f4615daf5faa10105ef0a3ad24d583c00e350f6e981515dc
                    txid: '10f746b824fcd844f4615daf5faa10105ef0a3ad24d583c00e350f6e981515dc',
                    vout: 4,
                    value: 6000,
                    confirmed: true,
                    height: 841314,
                },
                transaction: Transaction.fromRaw(
                    Buffer.from(
                        '02000000000104f6ffa6acd416ee27c6381598f9d1ace8f57bd5f997205fe9c10ca717a81208e20100000017160014d32ae97372f0b2549d26fa40e33f0699cfe86d55fffffffff6ffa6acd416ee27c6381598f9d1ace8f57bd5f997205fe9c10ca717a81208e20400000017160014d32ae97372f0b2549d26fa40e33f0699cfe86d55ffffffff500f4b6891bd86538937ca96508b6b34fd1eb8869e61a63448c1c7a5307c32fdbf00000000ffffffff248a0badd48bdc94ab7438cd8df43915af9e3ce334e4f3217d18564343ccbf510500000017160014d32ae97372f0b2549d26fa40e33f0699cfe86d55ffffffff07b00400000000000017a9147ecd91afdcadf6f1b9e8e026a312e4cce61e63ea872202000000000000225120173c179ec304311da634604e70958ac92f39325915ca5c0778061155c7735eb59a9d000000000000225120dca54ace6e977c3071e81ce4ee359c1c6ee003586434a29fca3632b346006e6ce80300000000000017a914ea6b832a05c6ca578baa3836f3f25553d41068a587580200000000000017a9147ecd91afdcadf6f1b9e8e026a312e4cce61e63ea87580200000000000017a9147ecd91afdcadf6f1b9e8e026a312e4cce61e63ea87a88f00000000000017a9147ecd91afdcadf6f1b9e8e026a312e4cce61e63ea870247304402200534ee640f73a3d2c03cdd67b3e55a8e9af73132bbfef97cdb6e240011e51892022075f2de763e423465659c741f9c73baccb158ae1fec5782075664383c87a79b4e012103b366c69e8237d9be7c4f1ac2a7abc6a79932fbf3de4e2f6c04797d7ef27abfe102473044022066cbcc462d3d30faebb927496ae21945d9a9cca3d26a9908809ff90bab1ad89802201877bad7327af6d2c8ebd6afe94990215285fe5dbf057604bb4a4cdabdfe1af7012103b366c69e8237d9be7c4f1ac2a7abc6a79932fbf3de4e2f6c04797d7ef27abfe10141a942ec1a6d845dbe9d0fa117cebcee4968458f400ccc0ef2f9a872aae043c068a7158637c3e372d5312f2bcaab2405438b48ceb56cf16ada3bf5d087d1d3dbf08302483045022100e5538a760f9cc8dd5d369f4dc48a4b4c858962da55c6f70b2c10fae90f052fef02204c3f5105f4c0b5c2bfb01d41a465d840e8e8863fff3eb37999c8ee2997efb30e012103b366c69e8237d9be7c4f1ac2a7abc6a79932fbf3de4e2f6c04797d7ef27abfe100000000',
                        'hex'
                    )
                ),
                addressType: AddressType.p2sh,
                publicKey: '030000000000000000000000000000000000000000000000000000000000000001', // use a random public key
            },
            // - P2PKH (Unisat)
            // https://github.com/paulmillr/scure-btc-signer/blob/5ead71ea9a873d8ba1882a9cd6aa561ad410d0d1/test/basic.test.js#L48
            {
                utxo: {
                    // https://blockstream.info/tx/c061c23190ed3370ad5206769651eaf6fac6d87d85b5db34e30a74e0c4a6da3e?expand
                    txid: '1c261c03d93e03d72a65606719a5eaf6fac6d87d55b5db34e30a74e0c4a6da3e',
                    vout: 0,
                    value: 550,
                    confirmed: true,
                    height: 609815,
                },
                transaction: Transaction.fromRaw(
                    Buffer.from(
                        '010000000160a941de6f9989628da24ede7730bb68990eac96da30379cec4da3cd3f1097cd010000008b4830450221009ee3ca47753ae5e4991358787ae660508222979493bd7d9b7191deff85d5e96102206a2d4b42a9d4afb3817d0e7a5b37c4d639eb16a52ad8c2d8c519f8802fff25920141049875dbe0f1f8ff9331b46630ad3fcb1c37c6516cf4b5e29d2ce0b0ae64935b40ba29e0669e9545768513a70bee36b699ca80cf37e245e24fb89907cb9a06f087ffffffff0226020000000000001976a91411dbe48cc6b617f9c6adaf4d9ed5f625b1c7cb5988acaa360b00000000001976a914a7158108fd0a615539a37122ed320784295ed1f388ac00000000',
                        'hex'
                    )
                ),
                addressType: AddressType.p2pkh,
                publicKey: undefined,
            },
        ];

        for (const test of testset) {
            const input = getInputFromUtxoAndTx(
                Network.testnet,
                test.utxo,
                test.transaction,
                test.addressType,
                test.publicKey
            );

            assert(input);
        }
    });

    // custom test using partially real data that would otherwise produce an invalid output
    // below the dust limit if we did not manually configure that to the correct value of 546
    it('should not output too small change', async () => {
        const inputScript = Buffer.from('a9147ecd91afdcadf6f1b9e8e026a312e4cce61e63ea87', 'hex');
        const outputOpReturn = Buffer.from(
            '6a200000000000000000000000000000000000000000000000000000000000000000',
            'hex'
        );

        const publicKey = '03b366c69e8237d9be7c4f1ac2a7abc6a79932fbf3de4e2f6c04797d7ef27abfe1';
        const inner = p2wpkh(Buffer.from(publicKey, 'hex'), NETWORK);
        const redeemScript = p2sh(inner);

        const transaction = selectUTXO(
            [
                {
                    txid: Buffer.alloc(32, 0).toString('hex'),
                    index: 0,
                    ...redeemScript,
                    witnessUtxo: {
                        script: inputScript,
                        amount: BigInt(23328), // 0.00023328
                    },
                },
                {
                    txid: Buffer.alloc(32, 0).toString('hex'),
                    index: 0,
                    ...redeemScript,
                    witnessUtxo: {
                        script: inputScript,
                        amount: BigInt(14476), // 0.00014476
                    },
                },
                {
                    txid: Buffer.alloc(32, 0).toString('hex'),
                    index: 0,
                    ...redeemScript,
                    witnessUtxo: {
                        script: inputScript,
                        amount: BigInt(4389), // 0.00004389
                    },
                },
                {
                    txid: Buffer.alloc(32, 0).toString('hex'),
                    index: 0,
                    ...redeemScript,
                    witnessUtxo: {
                        script: inputScript,
                        amount: BigInt(60037), // 0.00060037
                    },
                },
            ],
            [
                {
                    script: outputOpReturn,
                    amount: BigInt(0),
                },
                {
                    address: 'bc1qar0srrr7xfkvy5l643lydnw9re59gtzzwf5mdq',
                    amount: BigInt(100000), // 0.001 BTC
                },
            ],
            'default',
            {
                changeAddress: '3DFVKuT9Ft4rWpysAZ1bHpg55EBy1HVPcr',
                feePerByte: BigInt(Math.ceil(4)),
                bip69: true,
                createTx: true,
                network: NETWORK,
                allowUnknownOutputs: true,
                allowLegacyWitnessUtxo: true,
                // eslint-disable-next-line @typescript-eslint/no-explicit-any
                dust: BigInt(546) as any,
            }
        );

        assert.isDefined(transaction);
    });

    it('should estimate the fee for a transaction', { timeout: 50000 }, async () => {
        // Addresses where randomly picked from blockstream.info
        const paymentAddresses = [
            // P2WPKH: https://blockstream.info/address/bc1qar0srrr7xfkvy5l643lydnw9re59gtzzwf5mdq
            'bc1qar0srrr7xfkvy5l643lydnw9re59gtzzwf5mdq',
            // P2SH-P2WPKH: https://blockstream.info/address/3DFVKuT9Ft4rWpysAZ1bHpg55EBy1HVPcr
            // TODO: As above, add a correct P2SH-P2WPKH address with its pub key
            // '3DFVKuT9Ft4rWpysAZ1bHpg55EBy1HVPcr',
            // P2PKH: https://blockstream.info/address/1Kr6QSydW9bFQG1mXiPNNu6WpJGmUa9i1g
            '1Kr6QSydW9bFQG1mXiPNNu6WpJGmUa9i1g',
            // P2TR https://blockstream.info/address/bc1peqr5a5kfufvsl66444jm9y8qq0s87ph0zv4lfkcs7h40ew02uvsqkhjav0
            'bc1peqr5a5kfufvsl66444jm9y8qq0s87ph0zv4lfkcs7h40ew02uvsqkhjav0',
        ];

        const amounts = [undefined, 2000, 3000];
        const feeRates = [undefined, 10];

        // EVM address for OP return
        const opReturn = '0xC02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2';

        // Refactor to execute in parallel
        await Promise.all(
            feeRates.map(async (feeRate) =>
                Promise.all(
                    amounts.map(async (amount) =>
                        Promise.all(
                            paymentAddresses.map(async (paymentAddress) => {
                                const paymentAddressType = getAddressInfo(paymentAddress).type;

                                let pubkey: string | undefined;

                                if (
                                    paymentAddressType === AddressType.p2sh ||
                                    paymentAddressType === AddressType.p2wsh ||
                                    paymentAddressType === AddressType.p2tr
                                ) {
                                    // Use a random public key for P2SH-P2WPKH
                                    pubkey = '03b366c69e8237d9be7c4f1ac2a7abc6a79932fbf3de4e2f6c04797d7ef27abfe1';
                                }

                                // If the amount is undefined, the fee should be estimated
                                const fee = await estimateTxFee(paymentAddress, amount, pubkey, opReturn, feeRate);

                                expect(fee).toBeGreaterThan(0);
                            })
                        )
                    )
                )
            )
        );
    });

    it('should not spend outputs with inscriptions', { timeout: 50000 }, async () => {
        const paymentAddress = 'bc1peqr5a5kfufvsl66444jm9y8qq0s87ph0zv4lfkcs7h40ew02uvsqkhjav0';
        // Use a random public key
        const pubkey = '03b366c69e8237d9be7c4f1ac2a7abc6a79932fbf3de4e2f6c04797d7ef27abfe1';

        const ordinalsClient = new OrdinalsClient('mainnet');

        // cardinal = return UTXOs not containing inscriptions or runes
        const cardinalOutputs = await ordinalsClient.getOutputsFromAddress(paymentAddress, 'cardinal');

        const cardinalOutputsSet = new Set(cardinalOutputs.map((output) => output.outpoint));

        const maxSpendableBalance = cardinalOutputs.reduce((acc, output) => acc + output.value, 0);

        (selectUTXO as Mock).mockImplementationOnce(() => ({
            tx: {
                toPSBT() {
                    return Uint8Array.from(
                        Buffer.from('675f66d3ebcb97c383b48f6cbc37c8d32d57a489caa9ecb7e3691bd76731adaa', 'hex')
                    );
                },
            },
        }));

        await createBitcoinPsbt(paymentAddress, paymentAddress, maxSpendableBalance, pubkey);

        const [possibleInputs] = (selectUTXO as Mock).mock.lastCall || [];

        expect(possibleInputs.length).toBeGreaterThan(0);
        expect(cardinalOutputs.length).toBeGreaterThan(0);
        expect(
            (possibleInputs as Input[]).filter(
                (input) =>
                    !cardinalOutputsSet.has(
                        OutPoint.toString({
                            txid: input.txid,
                            vout: input.index,
                        })
                    )
            )
        ).toEqual([]);
    });

    it('throws an error if insufficient balance', { timeout: 50000 }, async () => {
        const paymentAddress = 'bc1peqr5a5kfufvsl66444jm9y8qq0s87ph0zv4lfkcs7h40ew02uvsqkhjav0';
        // Use a random public key
        const pubkey = '03b366c69e8237d9be7c4f1ac2a7abc6a79932fbf3de4e2f6c04797d7ef27abfe1';

        const ordinalsClient = new OrdinalsClient('mainnet');

        const allOutputs = await ordinalsClient.getOutputsFromAddress(paymentAddress);

        const totalBalance = allOutputs.reduce((acc, output) => acc + output.value, 0);

        await expect(createBitcoinPsbt(paymentAddress, paymentAddress, totalBalance, pubkey)).rejects.toThrow(
            'Failed to create transaction. Do you have enough funds?'
        );
    });

    it('should return address balance', { timeout: 50000 }, async () => {
        const address = 'bc1peqr5a5kfufvsl66444jm9y8qq0s87ph0zv4lfkcs7h40ew02uvsqkhjav0';

        const balance = await getBalance(address);

        assert(balance.confirmed);
        assert(balance.total);
        assert(
            balance.confirmed === balance.total
                ? balance.unconfirmed === 0n
                : balance.unconfirmed === balance.total - balance.confirmed
        );

        const zeroBalance = await getBalance();

        assert(zeroBalance.confirmed === 0n, 'If no address specified confirmed must be 0');
        assert(zeroBalance.unconfirmed === 0n, 'If no address specified unconfirmed must be 0');
        assert(zeroBalance.total === 0n, 'If no address specified total must be 0');
    });

    it('outputs could not be spent if not confirmed by ord service and indexed', { timeout: 50000 }, async () => {
        const taprootAddress = 'bc1peqr5a5kfufvsl66444jm9y8qq0s87ph0zv4lfkcs7h40ew02uvsqkhjav0';

        const esploraClient = new EsploraClient('mainnet');

        const outputs = await esploraClient.getAddressUtxos(taprootAddress);

        const total = outputs.reduce((acc, output) => acc + output.value, 0);

        const confirmed = outputs.reduce((acc, output) => {
            if (output.confirmed) {
                return acc + output.value;
            }

            return acc;
        }, 0);

        // mock half of the UTXOs contain inscriptions or runes
        vi.spyOn(OrdinalsClient.prototype, 'getOutputsFromAddress').mockResolvedValueOnce(
            // eslint-disable-next-line @typescript-eslint/ban-ts-comment
            // @ts-expect-error
            outputs.slice(Math.ceil(outputs.length / 2)).map((output) => {
                const outpoint = OutPoint.toString(output);

                return { outpoint };
            })
        );
        // mark every requested output as indexed
        // will not be a part of `cardinalOutputsSet` -- could not be spent
        vi.spyOn(OrdinalsClient.prototype, 'getOutputsFromOutPoints').mockResolvedValue(
            // eslint-disable-next-line @typescript-eslint/ban-ts-comment
            // @ts-expect-error
            Array.from(outputs, () => ({ indexed: true, inscriptions: [], runes: {} }))
        );

        const balanceData = await getBalance(taprootAddress);

        expect(balanceData.total).toBeLessThan(BigInt(total));
        expect(balanceData.confirmed).toBeLessThan(BigInt(confirmed));
    });

    it(
        'outputs could not be spent if not confirmed by ord service, not indexed and contain runes or inscriptions',
        { timeout: 50000 },
        async () => {
            const taprootAddress = 'bc1peqr5a5kfufvsl66444jm9y8qq0s87ph0zv4lfkcs7h40ew02uvsqkhjav0';

            const esploraClient = new EsploraClient('mainnet');

            const outputs = await esploraClient.getAddressUtxos(taprootAddress);

            const total = outputs.reduce((acc, output) => acc + output.value, 0);

            const confirmed = outputs.reduce((acc, output) => {
                if (output.confirmed) {
                    return acc + output.value;
                }

                return acc;
            }, 0);

            // mock half of the UTXOs contain inscriptions or runes
            vi.spyOn(OrdinalsClient.prototype, 'getOutputsFromAddress').mockResolvedValueOnce(
                // eslint-disable-next-line @typescript-eslint/ban-ts-comment
                // @ts-expect-error
                outputs.slice(Math.ceil(outputs.length / 2)).map((output) => {
                    const outpoint = OutPoint.toString(output);

                    return { outpoint };
                })
            );
            // mark every requested output as not indexed and containing inscriptions -- not cardinal
            // will not be a part of `cardinalOutputsSet` -- could not be spent
            vi.spyOn(OrdinalsClient.prototype, 'getOutputsFromOutPoints').mockResolvedValue(
                // eslint-disable-next-line @typescript-eslint/ban-ts-comment
                // @ts-expect-error
                Array.from(outputs, () => ({ indexed: false, inscriptions: [null], runes: {} }))
            );

            // no inputs otherwise will loop infinitely
            // eslint-disable-next-line @typescript-eslint/ban-ts-comment
            // @ts-expect-error
            vi.spyOn(EsploraClient.prototype, 'getTransaction').mockResolvedValue({
                status: {
                    confirmed: true,
                },
                vin: [],
            });

            const balanceData = await getBalance(taprootAddress);

            expect(balanceData.total).toEqual(BigInt(total));
            expect(balanceData.confirmed).toEqual(BigInt(confirmed));
        }
    );

    // coinbase reached
    it(
        'outputs could be spent if not confirmed by ord service, not indexed and does not contain runes or inscriptions',
        { timeout: 50000 },
        async () => {
            const taprootAddress = 'bc1peqr5a5kfufvsl66444jm9y8qq0s87ph0zv4lfkcs7h40ew02uvsqkhjav0';

            const esploraClient = new EsploraClient('mainnet');

            const outputs = await esploraClient.getAddressUtxos(taprootAddress);

            const total = outputs.reduce((acc, output) => acc + output.value, 0);

            const confirmed = outputs.reduce((acc, output) => {
                if (output.confirmed) {
                    return acc + output.value;
                }

                return acc;
            }, 0);

            // mock half of the UTXOs contain inscriptions or runes
            vi.spyOn(OrdinalsClient.prototype, 'getOutputsFromAddress').mockResolvedValueOnce(
                // eslint-disable-next-line @typescript-eslint/ban-ts-comment
                // @ts-expect-error
                outputs.slice(Math.ceil(outputs.length / 2)).map((output) => {
                    const outpoint = OutPoint.toString(output);

                    return { outpoint };
                })
            );
            // mark every requested output as not indexed and not containing inscriptions or runes
            vi.spyOn(OrdinalsClient.prototype, 'getInscriptionsFromOutPoint').mockResolvedValue(
                // eslint-disable-next-line @typescript-eslint/ban-ts-comment
                // @ts-expect-error
                { indexed: false, inscriptions: [], runes: {} }
            );

            // no inputs otherwise will loop infinitely
            // eslint-disable-next-line @typescript-eslint/ban-ts-comment
            // @ts-expect-error
            vi.spyOn(EsploraClient.prototype, 'getTransaction').mockResolvedValue({
                status: {
                    confirmed: false,
                },
                vin: [],
            });

            const balanceData = await getBalance(taprootAddress);

            expect(balanceData.total).toEqual(BigInt(total));
            expect(balanceData.confirmed).toEqual(BigInt(confirmed));
        }
    );

    it('processes utxo correctly', { timeout: 50000 }, async () => {
        const esploraClient = new EsploraClient('mainnet');
        const ordinalsClient = new OrdinalsClient('mainnet');

        const utxos: UTXO[] = [
            // regular tx
            // part of cardinals set
            // curl -s -H "Accept: application/json" "https://ordinals-mainnet.gobob.xyz/output/4871cc57fb9dd5359c4d0ef5352b83a21bb7d25729fce56ea8e3aa3c8ff14049:1"
            {
                confirmed: true,
                txid: '4871cc57fb9dd5359c4d0ef5352b83a21bb7d25729fce56ea8e3aa3c8ff14049',
                value: 1,
                vout: 1,
            },
            // regular tx, mocked in esplora call
            // not confirmed & not in cardinals set -> check inputs
            // curl -s -H "Accept: application/json" "https://ordinals-mainnet.gobob.xyz/output/c63275cb82afe248315dcc9534043c16b43471cefd76fdb1c7ae53d71168a3af:1"
            {
                confirmed: false,
                txid: 'c63275cb82afe248315dcc9534043c16b43471cefd76fdb1c7ae53d71168a3af',
                value: 1,
                vout: 1,
            },
            // regular tx
            // 4 `vin`s, 1 contains inscription
            // not confirmed & not included in cardinals set -> check inputs (can not be spent)
            // curl -s -H "Accept: application/json" "https://ordinals-mainnet.gobob.xyz/output/1de1e2025afaa055b4174c7da4646db9a67035666ed64e26420364a15320c217:4"
            {
                confirmed: false,
                txid: '1de1e2025afaa055b4174c7da4646db9a67035666ed64e26420364a15320c217',
                value: 1,
                vout: 4,
            },
            // transfer inscription
            // curl -s -H "Accept: application/json" "https://ordinals-mainnet.gobob.xyz/output/200bead2c2484d69fabffbda3ec55af7f3d809200b53c4d06ac443925df004ef:1"
            {
                confirmed: true,
                txid: '200bead2c2484d69fabffbda3ec55af7f3d809200b53c4d06ac443925df004ef',
                value: 1,
                vout: 1,
            },
            // rune transfer
            // curl -s -H "Accept: application/json" "https://ordinals-mainnet.gobob.xyz/output/b4e912281e8c7b8588adcf1cd0ea8b0bb5f492ea3f008f3ec351f99bdd5f833d:1"
            {
                confirmed: true,
                txid: 'b4e912281e8c7b8588adcf1cd0ea8b0bb5f492ea3f008f3ec351f99bdd5f833d',
                value: 1,
                vout: 1,
            },
        ];

        const cardinalOutputsSet = new Set([
            '4871cc57fb9dd5359c4d0ef5352b83a21bb7d25729fce56ea8e3aa3c8ff14049:1',

            // vin for utxo[1]
            // https://btc-mainnet.gobob.xyz/tx/c63275cb82afe248315dcc9534043c16b43471cefd76fdb1c7ae53d71168a3af
            '8eafa7525377d4b9bafd16c39410f66d3b3a1667d9ba643dbaefe66f8682d35a:1',

            // 4 `vin`s for utxo[2]
            // https://btc-mainnet.gobob.xyz/tx/1de1e2025afaa055b4174c7da4646db9a67035666ed64e26420364a15320c217
            'bd4a3f8c3e836f8ac14756e9b745eea8b3e6374d52e96ab3133dc4ea3d82c0e2:4',
            '4902cc605ccd829554a32fe730ec174c4ea626a2c4676adbb2d13d243785af63:5',
            // contains inscriptions
            // curl -s -H "Accept: application/json" "https://ordinals-mainnet.gobob.xyz/output/8d336ca4f129b6590fa9ed5e6a0bc46de74f586a11ce8c8d72900e3311c9d773:0"
            // '8d336ca4f129b6590fa9ed5e6a0bc46de74f586a11ce8c8d72900e3311c9d773:0',
            '81fdaeb84ca992a5f9d0c27fc7ba861aed15dbf0285ed1e01367fa461f091899:19',
        ]);

        const original = EsploraClient.prototype.getTransaction;

        vi.spyOn(EsploraClient.prototype, 'getTransaction').mockImplementation(async function (tx) {
            const result = await original.call(this, tx);

            // mark as unconfirmed -> continue building tree for `vin`s
            if (tx === utxos[1].txid || tx === utxos[2].txid) {
                result.status.confirmed = false;
            }

            return result;
        });

        const allowedUtxos = await findSafeUtxos(utxos, cardinalOutputsSet, esploraClient, ordinalsClient);

        expect(allowedUtxos).toEqual([utxos[0], utxos[1]]);
    });
});
