use crate::esplora::EsploraClient;
use alloy::{
    network::ReceiptResponse,
    primitives::{Bytes, FixedBytes, U256},
};
use bindings::fullrelaywithverify::FullRelayWithVerify::FullRelayWithVerifyInstance as BitcoinRelayInstance;
use bitcoin::{block::Header as BitcoinHeader, consensus, hashes::Hash, BlockHash};
use eyre::{eyre, Result};
use std::time::Duration;

const HEADERS_PER_BATCH: usize = 5;

fn serialize_headers(headers: &Vec<RelayHeader>) -> Result<Vec<u8>> {
    Ok(headers.iter().map(|header| consensus::serialize(&header.header)).flatten().collect())
}

#[derive(Clone)]
struct RelayHeader {
    hash: BlockHash,
    header: BitcoinHeader,
    height: u32,
}

impl RelayHeader {
    fn serialize_hash(&self) -> FixedBytes<32> {
        FixedBytes::from_slice(self.hash.as_byte_array())
    }

    fn serialize_header(&self) -> Bytes {
        Bytes::from(consensus::serialize(&self.header))
    }
}

pub struct Relayer<
    T: alloy::contract::private::Transport + ::core::clone::Clone,
    P: alloy::contract::private::Provider<T, N>,
    N: alloy::contract::private::Network,
> {
    pub contract: BitcoinRelayInstance<T, P, N>,
    pub esplora_client: EsploraClient,
}

// https://github.com/summa-tx/relays/tree/master/maintainer/maintainer/header_forwarder
impl<
        T: alloy::contract::private::Transport + ::core::clone::Clone,
        P: alloy::contract::private::Provider<T, N>,
        N: alloy::contract::private::Network,
    > Relayer<T, P, N>
{
    pub fn new(contract: BitcoinRelayInstance<T, P, N>, esplora_client: EsploraClient) -> Self {
        Self { contract, esplora_client }
    }

    async fn find_latest_height(&self) -> Result<u32> {
        let latest_digest = self.contract.getBestKnownDigest().call().await?._0;
        let mut latest_height: u32 =
            self.contract.findHeight(latest_digest).call().await?._0.try_into().unwrap();

        let mut latest = self
            .esplora_client
            .get_block_header(&BlockHash::from_byte_array(latest_digest.0))
            .await?;

        let mut better_or_same =
            self.esplora_client.get_block_header_at_height(latest_height).await?;

        while latest != better_or_same {
            println!("wrong header");
            latest = self.esplora_client.get_block_header(&latest.prev_blockhash).await?;
            latest_height -= 1;
            better_or_same = self.esplora_client.get_block_header_at_height(latest_height).await?;
        }

        Ok(latest_height)
    }

    #[allow(unused)]
    pub async fn run_once(&self) -> Result<()> {
        let latest_height = self.find_latest_height().await?;
        let headers: Vec<RelayHeader> = self.pull_headers(latest_height).await?;
        self.push_headers(headers).await?;
        Ok(())
    }

    pub async fn run(&self) -> Result<()> {
        let mut latest_height = self.find_latest_height().await?;

        loop {
            let headers: Vec<RelayHeader> = self.pull_headers(latest_height).await?;
            latest_height += headers.len() as u32;
            self.push_headers(headers).await?;
            tokio::time::sleep(Duration::from_secs(2)).await;
        }
    }

    async fn pull_headers(&self, mut latest_height: u32) -> Result<Vec<RelayHeader>> {
        latest_height += 1;

        // let futures = (latest_height..latest_height + HEADERS_PER_BATCH as u32)
        //     .map(|height| async move {
        //         println!("fetching height {}", height);
        //         let hash = self.esplora_client.get_block_hash(height).await?;
        //         let header = self.esplora_client.get_block_header(&hash).await?;
        //         Ok(RelayHeader { hash, header, height }) as Result<RelayHeader>
        //     })
        //     .collect::<Vec<_>>();
        // Ok(try_join_all(futures).await?)

        let mut relay_headers = Vec::new();
        for height in latest_height..latest_height + HEADERS_PER_BATCH as u32 {
            let hash = self.esplora_client.get_block_hash(height).await?;
            let header = self.esplora_client.get_block_header(&hash).await?;
            relay_headers.push(RelayHeader { hash, header, height });
        }
        Ok(relay_headers)
    }

    async fn push_headers(&self, headers: Vec<RelayHeader>) -> Result<()> {
        let start_mod = headers.first().unwrap().height % 2016;
        let end_header = headers.last().unwrap().clone();
        let end_mod = end_header.height % 2016;

        // difficulty change first
        if start_mod == 0 {
            println!("difficulty change first");
            self.add_diff_change(headers).await?;
        }
        // spanning difficulty change
        else if start_mod > end_mod {
            println!("spanning difficulty change");
            let (pre_change, post_change): (Vec<RelayHeader>, Vec<RelayHeader>) =
                headers.into_iter().partition(|header| header.height % 2016 >= start_mod);

            if pre_change.len() != 0 {
                self.add_headers(pre_change).await?;
            }

            if post_change.len() != 0 {
                self.add_diff_change(post_change).await?;
            }
        }
        // no difficulty change
        else {
            println!("adding headers");
            self.add_headers(headers).await?;
        }

        println!("updating head");
        self.update_best_digest(end_header).await?;

        Ok(())
    }

    async fn add_diff_change(&self, headers: Vec<RelayHeader>) -> Result<()> {
        let epoch_start = headers.first().unwrap().height - 2016;
        let old_start = self.esplora_client.get_raw_block_header_at_height(epoch_start).await?;
        let old_end =
            self.esplora_client.get_raw_block_header_at_height(epoch_start + 2015).await?;

        let receipt = self
            .contract
            .addHeadersWithRetarget(
                Bytes::from(old_start),
                Bytes::from(old_end),
                Bytes::from(serialize_headers(&headers)?),
            )
            .send()
            .await?
            .get_receipt()
            .await?;
        assert!(receipt.status());

        Ok(())
    }

    async fn add_headers(&self, headers: Vec<RelayHeader>) -> Result<()> {
        let latest_digest = headers.first().unwrap().header.prev_blockhash;
        let anchor = self.esplora_client.get_raw_block_header(&latest_digest).await?;

        let receipt = self
            .contract
            .addHeaders(Bytes::from(anchor), Bytes::from(serialize_headers(&headers)?))
            .send()
            .await?
            .get_receipt()
            .await?;
        assert!(receipt.status());

        Ok(())
    }

    async fn find_lca(
        &self,
        new_best: &RelayHeader,
        current_best: RelayHeader,
    ) -> Result<RelayHeader> {
        let mut ancestor = current_best;

        for _ in 0..20 {
            let is_ancestor = self
                .contract
                .isAncestor(ancestor.serialize_hash(), new_best.serialize_hash(), U256::from(240))
                .call()
                .await?;

            if is_ancestor._0 {
                return Ok(ancestor);
            }

            ancestor = RelayHeader {
                hash: ancestor.header.prev_blockhash,
                header: self
                    .esplora_client
                    .get_block_header(&ancestor.header.prev_blockhash)
                    .await?,
                height: ancestor.height.saturating_sub(1),
            };
        }

        Err(eyre!("Could not find ancenstor"))
    }

    async fn update_best_digest(&self, new_best: RelayHeader) -> Result<()> {
        let current_best_digest_raw = self.contract.getBestKnownDigest().call().await?._0;
        let current_best_digest = BlockHash::from_byte_array(current_best_digest_raw.0);
        let current_best = RelayHeader {
            hash: current_best_digest,
            header: self.esplora_client.get_block_header(&current_best_digest).await?,
            height: self
                .contract
                .findHeight(current_best_digest_raw)
                .call()
                .await?
                ._0
                .try_into()
                .unwrap(),
        };

        let ancestor = self.find_lca(&new_best, current_best.clone()).await?;
        let delta = new_best.height - ancestor.height + 1;

        let receipt = self
            .contract
            .markNewHeaviest(
                ancestor.serialize_hash(),
                current_best.serialize_header(),
                new_best.serialize_header(),
                U256::from(delta),
            )
            .send()
            .await?
            .get_receipt()
            .await?;
        assert!(receipt.status());

        Ok(())
    }
}
