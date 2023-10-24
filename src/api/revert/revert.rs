use crate::{
    cores::endpoint::Endpoint,
    types::{
        token::{TokenAmount, TokenTypes},
        transaction::Tx,
    },
};
use derive_builder::Builder;
use ethers_core::types::{Chain, TxHash};
use http::Method;
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, TryFromInto};

/// It returns a calldata to perform a revert of stuck cross-chain operation (swapping, bridging,
/// zapping). The reverting transaction should be sent to the destination blockchain of the origin
/// (stuck) operation.
#[serde_as]
#[derive(Debug, Builder, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Revert {
    #[builder(setter)]
    transaction_hash: TxHash,
    #[serde_as(as = "TryFromInto<u64>")]
    #[builder(default = "Chain::Mainnet")]
    chain_id: Chain,
}

impl Revert {
    /// Create a builder for the endpoint.
    pub fn builder() -> RevertBuilder {
        RevertBuilder::default()
    }
}

impl Endpoint for Revert {
    fn method(&self) -> http::Method {
        Method::POST
    }

    fn endpoint(&self) -> std::borrow::Cow<'static, str> {
        "v1/revert".into()
    }

    fn body(&self) -> anyhow::Result<Option<(&'static str, Vec<u8>)>> {
        let body = serde_json::to_vec(self)?;
        Ok(Some(("application/json", body)))
    }
}

/// Response data of revert transaction request.
#[derive(Default, Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RevertResponse {
    tx: Tx,
    fee: TokenAmount,
    #[serde(rename = "type")]
    token_type: TokenTypes,
}
