use crate::{
    cores::{endpoint::Endpoint, params::QueryParams},
    types::token::TokenAmount,
};
use derive_builder::Builder;
use ethers_core::types::{Address, Chain, TxHash};
use http::Method;
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, TryFromInto};

/// It returns a list of stuck cross-chain operations (swapping, bridging, zapping, interchain
/// communicating) for the specified address from all blockchains supported by the Symbiosis
/// protocol.
#[derive(Debug, Builder, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Stucked {
    #[builder(setter)]
    address: Address,
}

impl Stucked {
    /// Create a builder for the endpoint.
    pub fn builder() -> StuckedBuilder {
        StuckedBuilder::default()
    }
}

impl Endpoint for Stucked {
    fn method(&self) -> http::Method {
        Method::GET
    }

    fn endpoint(&self) -> std::borrow::Cow<'static, str> {
        format!("v1/stucked/{:#x}", self.address).into()
    }
}

/// Response data of stuck cross-chain operations (swapping, bridging, zapping, interchain
/// communicating) request.
#[serde_as]
#[derive(Default, Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct StuckedResponse {
    hash: TxHash,
    #[serde_as(as = "TryFromInto<u64>")]
    chain_id: Chain,
    create_at: String, // TODO: use DateTime
    token_amount: TokenAmount,
}
