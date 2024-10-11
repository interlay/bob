import { Transaction, Script, selectUTXO, TEST_NETWORK, NETWORK, p2wpkh, p2sh } from '@scure/btc-signer';
import { hex, base64 } from '@scure/base';
import { AddressType, getAddressInfo, Network } from 'bitcoin-address-validation';
import { EsploraClient, UTXO } from '../esplora';
import { stripHexPrefix } from '../utils';

const DUST_LIMIT = 546;

export type BitcoinNetworkName = Exclude<Network, 'regtest'>;

const bitcoinNetworks: Record<BitcoinNetworkName, typeof NETWORK> = {
    mainnet: NETWORK,
    testnet: TEST_NETWORK,
};

export const getBtcNetwork = (name: BitcoinNetworkName) => {
    return bitcoinNetworks[name];
};

type Output = { address: string; amount: bigint } | { script: Uint8Array; amount: bigint };

export interface Input {
    txid: string;
    index: number;
    witness_script?: Uint8Array;
    redeemScript?: Uint8Array;
    witnessUtxo?: {
        script: Uint8Array;
        amount: bigint;
    };
    nonWitnessUtxo?: Uint8Array;
}

/**
 * Create a PSBT with an output `toAddress` and an optional OP_RETURN output.
 * May add an additional change output. This returns an **unsigned** PSBT encoded
 * as a Base64 string.
 *
 * @param params An object containing the following properties:
 * - `fromAddress`: The Bitcoin address which is sending to the `toAddress`.
 * - `toAddress`: The Bitcoin address which is receiving the BTC.
 * - `amount`: The amount of BTC (as satoshis) to send.
 * - `publicKey` (optional): Public key needed if using P2SH-P2WPKH.
 * - `opReturnData` (optional): OP_RETURN data to include in an output.
 * - `confirmationTarget` (optional): The number of blocks to include this tx (for fee estimation). Defaults to 3.
 *
 * @returns {Promise<string>} The Base64 encoded PSBT.
 */
export async function createBitcoinPsbt(params: {
    fromAddress: string;
    toAddress: string;
    amount: number;
    publicKey?: string;
    opReturnData?: string;
    confirmationTarget?: number;
    feeRecipient?: string;
    feeAmount?: number;
}): Promise<string> {
    const {
        fromAddress,
        toAddress,
        amount,
        publicKey,
        opReturnData,
        confirmationTarget = 3,
        feeRecipient,
        feeAmount,
    } = params;

    const addressInfo = getAddressInfo(fromAddress);
    const network = addressInfo.network;
    if (network === 'regtest') {
        throw new Error('Bitcoin regtest not supported');
    }

    // We need the public key to generate the redeem and witness script to spend the scripts
    if (addressInfo.type === (AddressType.p2sh || AddressType.p2wsh)) {
        if (!publicKey) {
            throw new Error('Public key is required to spend from the selected address type');
        }
    }

    const esploraClient = new EsploraClient(addressInfo.network);

    const [confirmedUtxos, feeRate] = await Promise.all([
        esploraClient.getAddressUtxos(fromAddress),
        esploraClient.getFeeEstimate(confirmationTarget),
    ]);

    if (confirmedUtxos.length === 0) {
        throw new Error('No confirmed UTXOs');
    }

    // To construct the spending transaction and estimate the fee, we need the transactions for the UTXOs
    const possibleInputs: Input[] = [];

    await Promise.all(
        confirmedUtxos.map(async (utxo) => {
            const hex = await esploraClient.getTransactionHex(utxo.txid);
            const transaction = Transaction.fromRaw(Buffer.from(hex, 'hex'), { allowUnknownOutputs: true });
            const input = getInputFromUtxoAndTx(network, utxo, transaction, addressInfo.type, publicKey);
            possibleInputs.push(input);
        })
    );

    const outputs: Output[] = [
        {
            address: toAddress,
            amount: BigInt(amount),
        },
    ];

    if (feeRecipient && feeAmount && feeAmount > DUST_LIMIT) {
        outputs.push({
            address: feeRecipient,
            amount: BigInt(feeAmount),
        });
    }

    if (opReturnData) {
        outputs.push({
            // OP_RETURN https://github.com/paulmillr/scure-btc-signer/issues/26
            script: Script.encode(['RETURN', hex.decode(stripHexPrefix(opReturnData))]),
            amount: BigInt(0),
        });
    }

    // Outsource UTXO selection to btc-signer
    // https://github.com/paulmillr/scure-btc-signer?tab=readme-ov-file#utxo-selection
    // default = exactBiggest/accumBiggest creates tx with smallest fees, but it breaks
    // big outputs to small ones, which in the end will create a lot of outputs close to dust.
    const transaction = selectUTXO(possibleInputs, outputs, 'default', {
        changeAddress: fromAddress, // Refund surplus to the payment address
        feePerByte: BigInt(Math.ceil(feeRate)), // round up to the nearest integer
        bip69: true, // Sort inputs and outputs according to BIP69
        createTx: true, // Create the transaction
        network: getBtcNetwork(network),
        allowUnknownOutputs: true, // Required for OP_RETURN
        allowLegacyWitnessUtxo: true, // Required for P2SH-P2WPKH
        // eslint-disable-next-line @typescript-eslint/no-explicit-any
        dust: BigInt(DUST_LIMIT) as any, // TODO: update scure-btc-signer
    });

    if (!transaction || !transaction.tx) {
        throw new Error('Failed to create transaction. Do you have enough funds?');
    }

    return base64.encode(transaction.tx.toPSBT(0));
}

// Using the UTXO and the transaction, we can construct the input for the transaction
export function getInputFromUtxoAndTx(
    network: BitcoinNetworkName,
    utxo: UTXO,
    transaction: Transaction,
    addressType: AddressType,
    publicKey?: string
): Input {
    // The output containts the necessary details to spend the UTXO based on the script type
    // Under the hood, @scure/btc-signer parses the output and extracts the script and amount
    const output = transaction.getOutput(utxo.vout);

    // For p2sh, we additionally need the redeem script. This cannot be extracted from the transaction itself
    // We only support P2SH-P2WPKH
    // TODO: add support for P2WSH
    // TODO: add support for P2SH-P2PKH
    let redeemScript = {};

    if (addressType === AddressType.p2sh) {
        if (!publicKey) {
            throw new Error('Bitcoin P2SH not supported without public key');
        }
        const inner = p2wpkh(Buffer.from(publicKey!, 'hex'), getBtcNetwork(network));
        redeemScript = p2sh(inner);
    }

    // For the redeem and witness script, we need to construct the script mixin
    const scriptMixin = {
        ...redeemScript,
    };

    const nonWitnessUtxo = {
        nonWitnessUtxo: Buffer.from(transaction.hex, 'hex'),
    };
    const witnessUtxo = {
        witnessUtxo: {
            script: output.script!,
            amount: output.amount!,
        },
    };
    const witnessMixin = transaction.hasWitnesses ? { ...witnessUtxo, ...nonWitnessUtxo } : nonWitnessUtxo;

    // Construct inputs based on the script type
    const input = {
        txid: utxo.txid,
        index: utxo.vout,
        ...scriptMixin, // Maybe adds the redeemScript and/or witnessScript
        ...witnessMixin, // Adds the witnessUtxo and/or nonWitnessUtxo
    };

    return input;
}
