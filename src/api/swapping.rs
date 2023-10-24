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
use ethers_core::types::{Address, TransactionRequest};
use http::Method;
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr, VecSkipError};

/// It returns a calldata to perform a cross-chain swap.
#[derive(Default, Builder, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SwappingExactIn {
    /// Exact amount of input token to spend.
    #[builder(setter)]
    token_amount_in: TokenAmount,
    /// The desired token out.
    #[builder(setter)]
    token_out: Token,
    /// The address sent the input token.
    #[builder(setter)]
    from: Address,
    /// The address receive the output token.
    #[builder(setter)]
    to: Address,
    /// The slippage. Default is 3%.
    #[builder(default = "300")]
    slippage: u64,
    /// The deadline of request in second. Default is 2000000000.
    #[builder(default = "2000000000")]
    deadline: u64,
}

impl SwappingExactIn {
    /// Create a builder for the endpoint.
    pub fn builder() -> SwappingExactInBuilder {
        SwappingExactInBuilder::default()
    }
}

impl Endpoint for SwappingExactIn {
    fn method(&self) -> http::Method {
        Method::POST
    }

    fn endpoint(&self) -> std::borrow::Cow<'static, str> {
        "v1/swapping/exact_in".into()
    }

    fn body(&self) -> anyhow::Result<Option<(&'static str, Vec<u8>)>> {
        let body = serde_json::to_vec(self)?;
        Ok(Some(("application/json", body)))
    }
}

/// Response data of swapping request.
#[serde_as]
#[derive(Default, Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SwapResponse {
    tx: Tx,
    fee: TokenAmount,
    #[serde_as(as = "DisplayFromStr")]
    price_impact: f64,
    token_amount_out: TokenAmount,
    amount_in_usd: TokenAmount,
    approve_to: Address,
    #[serde_as(as = "VecSkipError<_>")]
    route: Vec<Token>,
    in_trade_type: SymbiosisTradeType,
    out_trade_type: SymbiosisTradeType,
    #[serde(rename = "type")]
    token_type: TokenTypes,
}

impl SwapResponse {
    /// Get the `Transaction` data to sent to the network.
    pub fn tx(&self) -> Tx {
        self.tx.clone()
    }

    /// Get the fee information to perform this swapping request.
    pub fn fee(&self) -> TokenAmount {
        self.fee.clone()
    }

    /// Get the percent difference between the mid price and the execution price, i.e. price impact.
    pub fn price_impact(&self) -> f64 {
        self.price_impact
    }

    /// Get the token with amount can be swapped to after perform this swapping request.
    pub fn token_amount_out(&self) -> TokenAmount {
        self.token_amount_out.clone()
    }

    /// Actual amount to be sent is the value of the entered token amount in the USD or WETH
    /// equivalent minus liquidity providers' fees (if any) on the source network.
    pub fn amount_in_usd(&self) -> TokenAmount {
        self.amount_in_usd.clone()
    }

    /// Get the `MetaRouterGateway` contract on appropriate network for user to approve.
    pub fn approve_to(&self) -> Address {
        self.approve_to
    }

    /// Get the route of the trade, i.e. which pairs the trade goes through.
    /// Route is a sequence of intermediate swaps leading to the best price for the trade. In our
    /// example, it's ETH for WETH on Arbitrum One -> Bridging with Symbiosis -> WETH for ETH on
    /// Base.
    pub fn route(&self) -> Vec<Token> {
        self.route.clone()
    }

    /// The type of the trade, either exact in or exact out.
    pub fn in_trade_type(&self) -> SymbiosisTradeType {
        self.in_trade_type
    }

    /// The type of the trade, either exact in or exact out.
    pub fn out_trade_type(&self) -> SymbiosisTradeType {
        self.out_trade_type
    }
}
