use anyhow::Result;
use bitcoincore_rpc::{Auth, Client, RpcApi};
use bitcoincore_rpc::bitcoin::{Block, BlockHash};

pub struct LitecoinClient {
    inner: Client,
}

impl LitecoinClient {
    pub fn new(url: &str, user: &str, pass: &str) -> Result<Self> {
        let inner = Client::new(url, Auth::UserPass(user.into(), pass.into()))?;
        Ok(Self { inner })
    }

    pub fn best_block(&self) -> bitcoincore_rpc::Result<(BlockHash, i32)> {
        let hash = self.inner.get_best_block_hash()?;
        let info = self.inner.get_block_info(&hash)?;
        Ok((hash, info.height as i32))
    }

    pub fn get_chain_info(
        &self,
    ) -> bitcoincore_rpc::Result<bitcoincore_rpc::json::GetBlockchainInfoResult> {
        self.inner.get_blockchain_info()
    }

    pub fn get_block(&self, hash: &BlockHash) -> bitcoincore_rpc::Result<Block> {
        self.inner.get_block(hash)
    }

    pub fn get_drivechain_info(&self) -> bitcoincore_rpc::Result<serde_json::Value> {
        self.inner.call("getdrivechaininfo", &[])
    }
}
