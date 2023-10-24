use crate::types::token::TokenTypes;
use crate::types::transaction::Tx;
use crate::{
    cores::{endpoint::Endpoint, params::FormParams},
    types::{
        token::{Token, TokenAmount},
        SymbiosisTradeType,
    },
};
use derive_builder::Builder;
use ethers_core::types::{Address, Chain, TransactionRequest};
use http::Method;
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr, TryFromInto, VecSkipError};

/// It returns a calldata to perform a bridging operation.
#[serde_as]
#[derive(Default, Builder, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct BridgingExactIn {
    /// Exact amount of input token to spend.
    #[builder(setter)]
    token_amount_in: TokenAmount,
    /// The desired token out's network.
    #[serde_as(as = "TryFromInto<u64>")]
    #[builder(default = "Chain::Mainnet")]
    chain_id_out: Chain,
    /// The address sent the input token.
    #[builder(setter)]
    from: Address,
    /// The address receive the output token.
    #[builder(setter)]
    to: Address,
}

impl BridgingExactIn {
    /// Create a builder for the endpoint.
    pub fn builder() -> BridgingExactInBuilder {
        BridgingExactInBuilder::default()
    }
}

impl Endpoint for BridgingExactIn {
    fn method(&self) -> http::Method {
        Method::POST
    }

    fn endpoint(&self) -> std::borrow::Cow<'static, str> {
        "v1/bridging/exact_in".into()
    }

    fn body(&self) -> anyhow::Result<Option<(&'static str, Vec<u8>)>> {
        let body = serde_json::to_vec(self)?;
        Ok(Some(("application/json", body)))
    }
}

/// It returns a calldata to perform a bridging operation.
#[serde_as]
#[derive(Default, Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct BridgeResponse {
    tx: Tx,
    fee: TokenAmount,
    token_amount_out: TokenAmount,
    approve_to: Address,
    #[serde(rename = "type")]
    token_type: TokenTypes,
}

impl BridgeResponse {
    /// Get the `Transaction` data to sent to the network.
    pub fn tx(&self) -> Tx {
        self.tx.clone()
    }

    /// Get the fee information to perform this swapping request.
    pub fn fee(&self) -> TokenAmount {
        self.fee.clone()
    }

    /// Get the token with amount can be swapped to after perform this swapping request.
    pub fn token_amount_out(&self) -> TokenAmount {
        self.token_amount_out.clone()
    }

    /// Get the `MetaRouterGateway` contract on appropriate network for user to approve.
    pub fn approve_to(&self) -> Address {
        self.approve_to
    }
}
