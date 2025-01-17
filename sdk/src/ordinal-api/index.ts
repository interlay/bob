/**
 * Base path for Ordinals regtest Explorer.
 * @default "http://0.0.0.0:3003"
 */
export const REGTEST_ORD_BASE_PATH = 'http://0.0.0.0:3003';

/**
 * Base path for Ordinals mainnet Explorer.
 * @default "https://ordinals-mainnet.gobob.xyz"
 */
export const MAINNET_ORD_BASE_PATH = 'https://ordinals-mainnet.gobob.xyz';

/**
 * Base path for Ordinals testnet Explorer.
 * @default "https://ordinals-testnet.gobob.xyz"
 */
export const TESTNET_ORD_BASE_PATH = 'https://ordinals-testnet.gobob.xyz';

// https://github.com/ordinals/ord/blob/e39031a46531696e5dd0c853146f8bfab5b7582c/src/inscription_id.rs#L4-L7
export type InscriptionId = {
    txid: string;
    index: number;
};

export module InscriptionId {
    export function toString(id: InscriptionId): string {
        return `${id.txid}i${id.index}`;
    }

    export function fromString(id: string): InscriptionId {
        // TODO: throw error if invalid?
        const [txid, index] = id.split('i');
        return {
            txid,
            index: parseInt(index, 10),
        };
    }
}

// https://github.com/rust-bitcoin/rust-bitcoin/blob/ee787c64810e4bce9c3c5e6e29da83010820a6c2/bitcoin/src/blockdata/transaction.rs#L50-L55
export type OutPoint = {
    txid: string;
    vout: number;
};

export module OutPoint {
    export function toString(id: OutPoint): string {
        return `${id.txid}:${id.vout}`;
    }

    export function fromString(id: string): OutPoint {
        // TODO: throw error if invalid?
        const [txid, vout] = id.split(':');
        return {
            txid,
            vout: parseInt(vout, 10),
        };
    }
}

// https://github.com/ordinals/ord/blob/5a9872bd96801987031217d6203d295b07f5e022/src/sat_point.rs#L4-L7
export type SatPoint = {
    outpoint: OutPoint;
    offset: number;
};

export module SatPoint {
    export function toString(id: SatPoint): string {
        return `${OutPoint.toString(id.outpoint)}:${id.offset}`;
    }

    export function fromString(id: string): SatPoint {
        // TODO: throw error if invalid?
        const [txid, vout, offset] = id.split(':');
        return {
            outpoint: {
                txid,
                vout: parseInt(vout, 10),
            },
            offset: parseInt(offset, 10),
        };
    }
}

/**
 * @ignore
 */
// https://github.com/ordinals/ord/blob/0.21.3/src/api.rs#L147-L151
export interface InscriptionsJson<InscriptionId> {
    /**
     * An array of inscription ids.
     */
    ids: InscriptionId[];

    /**
     * If there are more to return.
     */
    more: boolean;

    /**
     * The current page index.
     */
    page_index: number;
}

/**
 * @ignore
 */
// https://github.com/ordinals/ord/blob/0.21.3/src/api.rs#L154-L165
export interface OutputJson {
    /**
     * The address associated with the UTXO.
     */
    address: string | null;

    /**
     * The indexed value of the output.
     */
    indexed: boolean;

    /**
     * An array of inscription ids.
     */
    inscriptions: string[];

    /**
     * A map of runes.
     */
    runes: {
        [key: string]: {
            amount: number;
            divisibility: number;
            symbol: string | null;
        };
    };

    /**
     * The outpoint.
     */
    outpoint: string;

    /**
     * The SAT ranges.
     */
    sat_ranges: [number, number][] | null;

    /**
     * The scriptPubKey associated with the UTXO.
     */
    script_pubkey: string;

    /**
     * The spent value of the output.
     */
    spent: boolean;

    /**
     * The transaction related to the UTXO.
     */
    transaction: string;

    /**
     * The value of the UTXO.
     */
    value: number;
}

/**
 * @ignore
 */
// https://github.com/ordinals/ord/blob/0.21.3/src/api.rs#L228-L233
export interface AddressInfo {
    /**
     * An array of output ids.
     */
    outputs: string[];
    /**
     * An array of inscription ids.
     */
    inscriptions: string[];
    /**
     * Balance in satoshi.
     */
    sat_balance: number;
    /**
     * A list of runes.
     */
    runes_balances: [string, `${number}`, string][];
}

/**
 * @ignore
 */
// https://github.com/ordinals/ord/blob/0.21.3/src/api.rs#L197-L213
export interface SatJson<InscriptionId> {
    /**
     * The number of the ordinal.
     */
    number: number;

    /**
     * The decimal representation of the ordinal.
     */
    decimal: string;

    /**
     * The degree representation of the ordinal.
     */
    degree: string;

    /**
     * The name of the ordinal.
     */
    name: string;

    /**
     * The block associated with the ordinal.
     */
    block: number;

    charms: number[];

    /**
     * The cycle associated with the ordinal.
     */
    cycle: number;

    /**
     * The epoch associated with the ordinal.
     */
    epoch: number;

    /**
     * The period associated with the ordinal.
     */
    period: number;

    /**
     * The offset associated with the ordinal.
     */
    offset: number;

    /**
     * The rarity of the ordinal.
     */
    rarity: string;

    /**
     * The percentile representation of the ordinal.
     */
    percentile: string;

    /**
     * The SAT point of the ordinal.
     */
    satpoint: string | null;

    /**
     * The timestamp of the ordinal.
     */
    timestamp: number;

    /**
     * An array of inscription ids.
     */
    inscriptions: InscriptionId[];
}

/**
 * @ignore
 */
// https://github.com/ordinals/ord/blob/0.21.3/src/api.rs#L93-L113
export interface InscriptionJson<InscriptionId, SatPoint> {
    /**
     * The address associated with the inscription.
     */
    address: string | null;

    charms: string[];

    child_count: number;

    /**
     * An array of child IDs.
     */
    children: InscriptionId[];

    /**
     * The length of the content.
     */
    content_length: number | null;

    /**
     * The content type of the inscription.
     */
    content_type: string | null;

    effective_content_type: string | null;

    /**
     * The genesis fee of the inscription.
     */
    fee: number;

    /**
     * The genesis height of the inscription.
     */
    height: number;

    /**
     * The ID of the inscription.
     */
    id: InscriptionId;

    /**
     * The next inscription ID.
     */
    next: InscriptionId | null;

    /**
     * The number of the inscription.
     */
    number: number;

    /**
     * The parent inscription IDs.
     */
    parents: string[];

    /**
     * The previous inscription ID.
     */
    previous: InscriptionId | null;

    /**
     * The rune associated with the inscription.
     */
    rune: string | null;

    /**
     * The SAT associated with the inscription.
     */
    sat: number | null;

    /**
     * The SAT point of the inscription, this is the current UTXO.
     */
    satpoint: SatPoint;

    /**
     * The timestamp of the inscription.
     */
    timestamp: number;

    /**
     * The output value of the inscription.
     */
    value: number | null;
}

export class OrdinalsClient {
    private basePath: string;

    constructor(networkOrUrl: string = 'mainnet') {
        switch (networkOrUrl) {
            case 'mainnet':
                this.basePath = MAINNET_ORD_BASE_PATH;
                break;
            case 'testnet':
                this.basePath = TESTNET_ORD_BASE_PATH;
                break;
            case 'regtest':
                this.basePath = REGTEST_ORD_BASE_PATH;
                break;
            default:
                this.basePath = networkOrUrl;
        }
    }

    /**
     * Retrieves address information.
     * @param {string} address - The address to request information about.
     * @returns {Promise<AddressInfo>} A Promise that resolves to the address information.
     *
     * @example
     * ```typescript
     * const client = new OrdinalsClient("regtest");
     * const addressInfo = await client.getAssetsByAddress("enter_your_address_here");
     * console.log("AddressInfo:", addressInfo);
     * ```
     */
    getAssetsByAddress(address: string): Promise<AddressInfo> {
        return this.getJson<AddressInfo>(`${this.basePath}/address/${address}`);
    }

    /**
     * Retrieves an inscription based on its ID.
     * @param {string} id - The ID of the inscription to retrieve.
     * @returns {Promise<InscriptionDataFromId>} A Promise that resolves to the inscription data.
     *
     * @example
     * ```typescript
     * const client = new OrdinalsClient("regtest");
     * let inscriptionId = InscriptionId.fromString("enter_your_inscription_id_here");
     * const inscription = await client.getInscriptionFromId(inscriptionId);
     * console.log("Inscription:", inscription);
     * ```
     */
    async getInscriptionFromId(id: InscriptionId): Promise<InscriptionJson<InscriptionId, SatPoint>> {
        console.log(`${this.basePath}/inscription/${InscriptionId.toString(id)}`);
        const inscriptionJson = await this.getJson<InscriptionJson<string, string>>(
            `${this.basePath}/inscription/${InscriptionId.toString(id)}`
        );
        return {
            ...inscriptionJson,
            children: inscriptionJson.children.map(InscriptionId.fromString),
            id: InscriptionId.fromString(inscriptionJson.id),
            next: inscriptionJson.next != null ? InscriptionId.fromString(inscriptionJson.next) : null,
            previous: inscriptionJson.previous != null ? InscriptionId.fromString(inscriptionJson.previous) : null,
            satpoint: SatPoint.fromString(inscriptionJson.satpoint),
        };
    }

    /**
     * Retrieves a list of inscriptions.
     * @returns {Promise<InscriptionsJson<InscriptionId>>} A Promise that resolves to a inscriptions data.
     *
     * @example
     * ```typescript
     * const client = new OrdinalsClient("regtest");
     * const inscriptions = await client.getInscriptions();
     * console.log("Inscriptions:", inscriptions);
     * ```
     */
    async getInscriptions(): Promise<InscriptionsJson<InscriptionId>> {
        // TODO: add filtering, sorting and pagination based on different parameters
        const inscriptionsJson = await this.getJson<InscriptionsJson<string>>(`${this.basePath}/inscriptions`);
        return this.parseInscriptionsJson(inscriptionsJson);
    }

    /**
     * Retrieves an inscription based on its block height.
     * @param {number} height - The block height of the inscription to retrieve.
     * @returns {Promise<InscriptionsJson<InscriptionId>>} A Promise that resolves to the inscription data.
     *
     * @example
     * ```typescript
     * const client = new OrdinalsClient("regtest");
     * let block: number = "enter_your_block_number_here";
     * const inscriptions = await client.getInscriptionsFromBlock(block);
     * console.log("Inscriptions:", inscriptions);
     * ```
     */
    async getInscriptionsFromBlock(height: number): Promise<InscriptionsJson<InscriptionId>> {
        const inscriptionsJson = await this.getJson<InscriptionsJson<string>>(
            `${this.basePath}/inscriptions/block/${height}`
        );
        return this.parseInscriptionsJson(inscriptionsJson);
    }

    /**
     * Retrieves inscriptions based on the UTXO (Unspent Transaction Output).
     * @param {OutPoint} outPoint - The ID and output index of the UTXO.
     * @returns {Promise<OutputJson>} A Promise that resolves to the inscription data.
     *
     * @example
     * ```typescript
     * const client = new OrdinalsClient("regtest");
     * let txid: string = "enter_your_utxo_txid_here";
     * let vout = 0; // enter the UTXO index here
     * const output = await client.getInscriptionsFromOutPoint({ txid, vout });
     * console.log("Output:", output);
     * ```
     */
    getInscriptionsFromOutPoint(outPoint: OutPoint): Promise<OutputJson> {
        return this.getJson<OutputJson>(`${this.basePath}/output/${OutPoint.toString(outPoint)}`);
    }

    /**
     * Retrieves outputs based on the address.
     * @param {String} address - The Bitcoin address to check.
     * @param {('cardinal' | 'inscribed' | 'runic' | 'any')} [type] - Optional type of UTXOs to be returned. If omitted returns all UTXOs.
     * @returns {Promise<OutputJson[]>} A Promise that resolves to the list of output data.
     *
     * @example
     * ```typescript
     * const client = new OrdinalsClient("regtest");
     * const address: string = "enter_address_here";
     * const type: 'cardinal' | 'inscribed' | 'runic' = "enter_type_here";
     * const outputs = await client.getOutputsFromAddress(address, type?);
     * console.log("Outputs:", outputs);
     * ```
     */
    getOutputsFromAddress(address: string, type?: 'cardinal' | 'inscribed' | 'runic' | 'any'): Promise<OutputJson[]> {
        const searchParams = new URLSearchParams();
        if (type) searchParams.append('type', type);
        // https://docs.ordinals.com/guides/api.html#description-19
        return this.getJson<OutputJson[]>(`${this.basePath}/outputs/${address}?${searchParams}`);
    }

    /**
     * Retrieves outputs based on the out points list.
     * @param {string[]} outpoints - The list of out points to check.
     * @returns {Promise<OutputJson[]>} A Promise that resolves to the list of output data.
     *
     * @example
     * ```typescript
     * const client = new OrdinalsClient("regtest");
     * const outpoints: string[] = "enter_outpoints_here";
     * const outputs = await client.getOutputsFromOutPoints(outpoints);
     * console.log("Outputs:", outputs);
     * ```
     */
    getOutputsFromOutPoints(outpoints: string[]): Promise<OutputJson[]> {
        // https://docs.ordinals.com/guides/api.html#description-19
        return this.postJson<OutputJson[]>(`${this.basePath}/outputs`, outpoints);
    }

    /**
     * Retrieves an inscription based on its sat (something specific to your use case).
     * @param {number} sat - The sat of the inscription to retrieve.
     * @returns {Promise<SatJson<InscriptionId>>} A Promise that resolves to the SatJson data type.
     *
     * @example
     * ```typescript
     * const client = new OrdinalsClient("regtest");
     * let sat: number = 0 // enter the sat number here
     * const inscriptions = await client.getInscriptionsFromSat(sat);
     * console.log("Inscriptions:", inscriptions);
     * ```
     */
    async getInscriptionsFromSat(sat: number): Promise<SatJson<InscriptionId>> {
        const satJson = await this.getJson<SatJson<string>>(`${this.basePath}/sat/${sat}`);
        return {
            ...satJson,
            inscriptions: satJson.inscriptions.map((id) => InscriptionId.fromString(id)),
        };
    }

    /**
     * Retrieves a list of inscriptions starting from a specified block and moving forward.
     * @param {number} startHeight - The start block height.
     * @returns {Promise<InscriptionsJson<InscriptionId>>} A Promise that resolves to the inscription data.
     *
     * @example
     * ```typescript
     * const client = new OrdinalsClient("regtest");
     * let startBlock: number = "enter_your_block_number_here";
     * const inscriptions = await client.getInscriptionsFromStartBlock(block);
     * console.log("Inscriptions:", inscriptions);
     * ```
     */
    async getInscriptionsFromStartBlock(startHeight: number): Promise<InscriptionsJson<InscriptionId>> {
        const inscriptionsJson = await this.getJson<InscriptionsJson<string>>(
            `${this.basePath}/inscriptions/${startHeight}`
        );
        return this.parseInscriptionsJson(inscriptionsJson);
    }

    /**
     * @ignore
     */
    private async getJson<T>(url: string): Promise<T> {
        const response = await fetch(url, {
            headers: {
                Accept: 'application/json',
            },
        });
        if (!response.ok) {
            throw new Error(response.statusText);
        }
        return response.json() as Promise<T>;
    }

    /**
     * @ignore
     */
    private async postJson<T>(url: string, body: object): Promise<T> {
        const response = await fetch(url, {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json',
                Accept: 'application/json',
            },
            body: JSON.stringify(body),
        });
        if (!response.ok) {
            throw new Error(response.statusText);
        }
        return response.json() as Promise<T>;
    }

    /**
     * @ignore
     */
    private parseInscriptionsJson(inscriptionsJson: InscriptionsJson<string>): InscriptionsJson<InscriptionId> {
        const ids = inscriptionsJson.ids.map((id) => InscriptionId.fromString(id));
        return {
            ...inscriptionsJson,
            ids,
        };
    }
}
