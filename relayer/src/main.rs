use alloy::{
    network::EthereumWallet, primitives::Address, providers::ProviderBuilder,
    signers::local::PrivateKeySigner,
};
use bindings::fullrelaywithverify::FullRelayWithVerify as BitcoinRelay;
use clap::Parser;
use esplora::EsploraClient;
use eyre::Result;
use relayer::Relayer;
use reqwest::Url;

mod esplora;
mod relayer;

/// Relayer
#[derive(Debug, Parser)]
#[command(name = "app")]
struct App {
    /// Ethereum RPC URL
    #[clap(long, env = "ETH_RPC_URL", default_value = "http://localhost:8545")]
    eth_rpc_url: String,

    /// The relayer's private key for submitting txs
    #[clap(long, env = "PRIVATE_KEY")]
    private_key: String,

    /// The relay address
    #[clap(long, env = "RELAY_ADDRESS")]
    relay_address: Address,
}

#[tokio::main]
async fn main() -> Result<()> {
    // env_logger::init();
    let app = App::parse();

    let privk = app.private_key.trim().strip_prefix("0x").expect("Requires private key");
    let signer: PrivateKeySigner = privk.parse().expect("should parse private key");
    let wallet = EthereumWallet::from(signer);
    let rpc_url: Url = app.eth_rpc_url.parse()?;
    let provider =
        ProviderBuilder::new().with_recommended_fillers().wallet(wallet).on_http(rpc_url);

    let esplora_client = EsploraClient::new(None, bitcoin::Network::Bitcoin)?;

    let relayer = Relayer::new(BitcoinRelay::new(app.relay_address, provider), esplora_client);
    relayer.run().await?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloy::{
        network::EthereumWallet,
        node_bindings::Anvil,
        primitives::{Bytes, FixedBytes, U256},
        providers::ProviderBuilder,
        signers::local::PrivateKeySigner,
    };
    use bitcoin::hashes::Hash;

    #[tokio::test]
    async fn test() -> Result<()> {
        let anvil = Anvil::new().try_spawn()?;
        let signer: PrivateKeySigner = anvil.keys()[0].clone().into();
        let wallet = EthereumWallet::from(signer.clone());
        let rpc_url = anvil.endpoint_url();

        let provider =
            ProviderBuilder::new().with_recommended_fillers().wallet(wallet).on_http(rpc_url);

        let esplora_client = esplora::EsploraClient::new(None, bitcoin::Network::Bitcoin)?;

        let period_start_height = 201600;
        // change this to test different headers
        let genesis_height = period_start_height + 2014;

        let period_start_block_hash = esplora_client.get_block_hash(period_start_height).await?;
        let genesis_block_header = esplora_client
            .get_raw_block_header(&esplora_client.get_block_hash(genesis_height).await?)
            .await?;

        let contract = BitcoinRelay::deploy(
            provider.clone(),
            Bytes::from(genesis_block_header.clone()),
            U256::from(genesis_height),
            FixedBytes::new(period_start_block_hash.to_byte_array()),
        )
        .await?;

        // let block_headers = esplora_client
        //     .get_raw_block_header(&esplora_client.get_block_hash(genesis_height + 1).await?)
        //     .await?;

        // let receipt = contract
        //     .addHeaders(Bytes::from(genesis_block_header), Bytes::from(block_headers))
        //     .send()
        //     .await?
        //     .get_receipt()
        //     .await?;

        // assert!(receipt.status());

        Relayer::new(contract, esplora_client).run_once().await?;

        Ok(())
    }
}
