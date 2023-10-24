use crate::{
    cores::endpoint::Endpoint,
    types::{token::TokenAmount, transaction::TxOrigin},
};
use derive_builder::Builder;
use ethers_core::types::{Chain, TxHash};
use http::Method;
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, TryFromInto};
use std::default;

/// Symbiosis transaction request, specified by transaction hash and chain id.
#[allow(missing_docs)]
#[serde_as]
#[derive(Default, Builder, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TxHashWithChainId {
    #[builder(setter)]
    transaction_hash: TxHash,
    #[serde_as(as = "TryFromInto<u64>")]
    #[builder(default = "Chain::Mainnet")]
    chain_id: Chain,
}

impl TxHashWithChainId {
    /// Create a builder for the endpoint.
    pub fn builder() -> TxHashWithChainIdBuilder {
        TxHashWithChainIdBuilder::default()
    }
}

/// It returns the status of the origin transaction specified by the blockchain ID and transaction
/// hash. The origin transaction means the transaction sent to the origin blockchain.
/// The possible statuses:
///     -1: Not found,
///     1: Pending,
///     0: Success (in this case, the answer contains as well the destination chain ID and the hash
///        of the transaction on the destination chain),
///     2: Stucked,
///     3: Reverted
#[allow(missing_docs)]
#[derive(Default, Builder, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct GetSingleTx {
    #[builder(setter)]
    tx: TxHashWithChainId,
}

impl GetSingleTx {
    /// Create a builder for the endpoint.
    pub fn builder() -> GetSingleTxBuilder {
        GetSingleTxBuilder::default()
    }
}

impl Endpoint for GetSingleTx {
    fn method(&self) -> http::Method {
        Method::GET
    }

    fn endpoint(&self) -> std::borrow::Cow<'static, str> {
        format!(
            "v1/tx/{}/{:#x}",
            self.tx.chain_id as u64, self.tx.transaction_hash
        )
        .into()
    }
}

/// It returns the status of the origin transactions specified by the blockchain ID and transaction
/// hash. The origin transaction means the transaction sent to the origin blockchain.
#[allow(missing_docs)]
#[derive(Default, Builder, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct GetBatchTx {
    #[builder(setter)]
    txs: Vec<TxHashWithChainId>,
}

impl GetBatchTx {
    /// Create a builder for the endpoint.
    pub fn builder() -> GetBatchTxBuilder {
        GetBatchTxBuilder::default()
    }
}

impl Endpoint for GetBatchTx {
    fn method(&self) -> http::Method {
        Method::POST
    }

    fn endpoint(&self) -> std::borrow::Cow<'static, str> {
        "v1/batch-tx".into()
    }

    fn body(&self) -> anyhow::Result<Option<(&'static str, Vec<u8>)>> {
        let body = serde_json::to_vec(self)?;
        Ok(Some(("application/json", body)))
    }
}

/// Response data for get transaction request.
#[derive(Default, Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TxResponse {
    status: TxStatus,
    tx: TxOrigin,
    tx_in: TxOrigin,
    transit_token_sent: Option<TokenAmount>,
}

/// Status of the origin transaction in Symbiosis transaction explorer.
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "text", content = "code")]
pub enum TxStatus {
    /// Transaction is not existed.
    NotFound(i8),
    /// Transaction was succeed (in this case, the answer contains as well the destination chain ID
    /// and the hash of the transaction on the destination chain).
    Success(i8),
    /// Transaction is pending.
    Pending(i8),
    /// Transaction was stucked.
    Stucked(i8),
    /// Transaction was reverted.
    Reverted(i8),
}

impl Default for TxStatus {
    fn default() -> Self {
        Self::NotFound(-1)
    }
}
