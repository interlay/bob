#![allow(unused)]

use bitcoin::{block::Header, consensus, hashes::hex::FromHex, BlockHash, Network};
use eyre::Result;
use reqwest::{Client, Url};
use std::str::FromStr;

const ESPLORA_MAINNET_URL: &str = "https://blockstream.info/api/";
const ESPLORA_TESTNET_URL: &str = "https://blockstream.info/testnet/api/";
const ESPLORA_LOCALHOST_URL: &str = "http://localhost:3002";

#[derive(Clone)]
pub struct EsploraClient {
    url: Url,
    cli: Client,
}

impl EsploraClient {
    pub fn new(esplora_url: Option<String>, network: Network) -> Result<Self> {
        Ok(Self {
            url: esplora_url
                .unwrap_or_else(|| {
                    match network {
                        Network::Bitcoin => ESPLORA_MAINNET_URL,
                        Network::Testnet => ESPLORA_TESTNET_URL,
                        _ => ESPLORA_LOCALHOST_URL,
                    }
                    .to_owned()
                })
                .parse()?,
            cli: Client::new(),
        })
    }

    async fn get(&self, path: &str) -> Result<String> {
        let url = self.url.join(path)?;
        Ok(self.cli.get(url).send().await?.error_for_status()?.text().await?)
    }

    pub async fn get_block_hash(&self, height: u32) -> Result<BlockHash> {
        let response = self.get(&format!("block-height/{height}")).await?;
        Ok(BlockHash::from_str(&response)?)
    }

    pub async fn get_raw_block_header(&self, hash: &BlockHash) -> Result<Vec<u8>> {
        Ok(Vec::<u8>::from_hex(&self.get(&format!("block/{hash}/header")).await?)?)
    }

    pub async fn get_block_header(&self, hash: &BlockHash) -> Result<Header> {
        Ok(consensus::deserialize(&self.get_raw_block_header(hash).await?)?)
    }

    pub async fn get_raw_block_header_at_height(&self, height: u32) -> Result<Vec<u8>> {
        self.get_raw_block_header(&self.get_block_hash(height).await?).await
    }

    pub async fn get_block_header_at_height(&self, height: u32) -> Result<Header> {
        self.get_block_header(&self.get_block_hash(height).await?).await
    }
}
