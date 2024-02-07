---
sidebar_position: 6
sidebar_label: Using the BOB BTC MetaMask Snap in your app
---

# BOB BTC MetaMask Snap

:::info MetaMask Flask

Unlisted [MetaMask Snaps](https://metamask.io/snaps/) can only be used with [MetaMask Flask](https://metamask.io/flask/). MetaMask Snaps are not currently supported on mobile wallets, and will only run in the desktop version of Chrome or Firefox.

:::

:::tip

Take a look at the [BTC Snap source code](https://github.com/bob-collective/btcsnap) and [our MetaMask Snap demo application](../examples/metamask-ordinals/)

:::

## Create a test profile

You will need to create a new browser profile to use with the MetaMask Flask extension, as having MetaMask and MetaMask Flask installed in the same browser profile can cause issues.

## Using the BOB BTC Snap in your application

### Connecting to the BOB BTC Snap

:::tip

To see how to connect to the BOB BTC Snap and call the available methods, take a look at the [utils.ts](https://github.com/bob-collective/demo-unified-assets-tracker/blob/1475ef915518d45103cd4581c3901ede216a6197/ui/src/utils/btcsnap.ts) file in our demo app.

:::

```typescript
import { MetaMaskInpageProvider } from "@metamask/providers";

declare global {
  interface Window {
    ethereum: MetaMaskInpageProvider;
  }
}

const { ethereum } = window;

const snapId = "npm:@gobob/btcsnap";

export async function checkConnection(): Promise<boolean> {
  const snaps = await ethereum.request({
    method: "wallet_getSnaps",
  });

  const hasMySnap = Object.keys(snaps || []).includes(snapId);

  return hasMySnap;
}

export async function connect(cb: (connected: boolean) => void) {
  let connected = false;
  try {
    const result: any = await ethereum.request({
      method: "wallet_requestSnaps",
      params: {
        [snapId]: {},
      },
    });

    const hasError = !!result?.snaps?.[snapId]?.error;
    connected = !hasError;
  } finally {
    cb(connected);
  }
}
```

### Getting the extended public key

```typescript
export enum BitcoinNetwork {
  Main = "mainnet",
  Test = "testnet",
}

export enum BitcoinScriptType {
  P2PKH = "P2PKH",
  P2SH_P2WPKH = "P2SH-P2WPKH",
  P2WPKH = "P2WPKH",
}

export interface ExtendedPublicKey {
  xpub: string;
  mfp: string;
}

export async function getExtendedPublicKey(
  network: BitcoinNetwork,
  scriptType: BitcoinScriptType
): Promise<ExtendedPublicKey> {
  const networkParams = network === BitcoinNetwork.Main ? "main" : "test";

  try {
    return (await ethereum.request({
      method: "wallet_invokeSnap",
      params: {
        snapId,
        request: {
          method: "btc_getPublicExtendedKey",
          params: {
            network: networkParams,
            scriptType,
          },
        },
      },
    })) as ExtendedPublicKey;
  } catch (err: any) {
    const error = new SnapError(
      err?.message || "Get extended public key failed"
    );
    console.error(error);
    throw error;
  }
}
```

### Using the BOB BTC Snap in a React application

:::tip

Take a look at the [UI code in our demo application](https://github.com/bob-collective/demo-unified-assets-tracker/tree/1475ef915518d45103cd4581c3901ede216a6197/ui) to see how this hook can be used.
:::

```typescript
import { useCallback, useEffect, useState } from "react";
import { addressFromExtPubKey } from "../utils/btcsnap-signer";
import {
  BitcoinNetwork,
  BitcoinScriptType,
  checkConnection,
  connect,
  getExtendedPublicKey,
} from "../utils/btcsnap-utils";
import { useLocalStorage, LocalStorageKey } from "./useLocalStorage";
import { useGetInscriptionIds } from "./useGetInscriptionIds";
import { useQueryClient } from "@tanstack/react-query";
import { BITCOIN_NETWORK } from "../utils/config";

const bitcoinNetwork =
  BITCOIN_NETWORK === "mainnet" ? BitcoinNetwork.Main : BitcoinNetwork.Test;

const getDerivedBtcAddress = async () => {
  const xpub = await getExtendedPublicKey(
    bitcoinNetwork,
    BitcoinScriptType.P2WPKH
  );

  const bitcoinAddress = addressFromExtPubKey(xpub.xpub, bitcoinNetwork)!;

  return {
    bitcoinAddress,
  };
};

const connectionCheck = async () => {
  const isConnected = await checkConnection();

  return isConnected;
};

const useBtcSnap = () => {
  const [isConnected, setIsConnected] = useState<boolean>(false);

  const queryClient = useQueryClient();
  const [bitcoinAddress, setBitcoinAddress, removeBitcoinAddress] =
    useLocalStorage(LocalStorageKey.DERIVED_BTC_ADDRESS);

  const { refetch } = useGetInscriptionIds(bitcoinAddress);

  useEffect(() => {
    if (!bitcoinAddress) return;

    refetch();
  }, [bitcoinAddress, refetch]);

  const connectBtcSnap = useCallback(async () => {
    connect(async (connected: boolean) => {
      if (connected) {
        const { bitcoinAddress } = await getDerivedBtcAddress();
        setBitcoinAddress(bitcoinAddress);
      }
    });
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [setBitcoinAddress]);

  useEffect(() => {
    const checkConnection = async () => {
      const connected = await connectionCheck();

      // This will reset BTC address if user has disconnected
      if (!connected && bitcoinAddress) {
        removeBitcoinAddress();
        queryClient.removeQueries();
      }

      setIsConnected(connected);
    };

    checkConnection();
  }, [
    bitcoinAddress,
    isConnected,
    queryClient,
    removeBitcoinAddress,
    setBitcoinAddress,
  ]);

  return { connectBtcSnap, bitcoinAddress, isConnected };
};

export { useBtcSnap };
```
