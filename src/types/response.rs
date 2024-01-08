use super::{Token, TokenTypes};
use crate::types::{transaction::Transaction, SymbiosisTradeType, TokenAmount};
use ethers_core::types::Address;
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, VecSkipError};

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum Response {
    Swap(SwapBody),
    UnprocessEntity(UnprocessEntityBody),
    BadRequest(BadRequestBody),
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SwapBody {
    #[serde(flatten)]
    info: SwapInfo,
}

impl SwapBody {
    pub fn info(&self) -> SwapInfo {
        self.info.clone()
    }
}

#[serde_as]
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SwapInfo {
    tx: Transaction,
    fee: TokenAmount,
    price_impact: String, // TODO
    token_amount_out: TokenAmount,
    amount_in_usd: TokenAmount,
    approve_to: Address,
    #[serde_as(as = "VecSkipError<_>")]
    route: Vec<Token>,
    in_trade_type: Option<SymbiosisTradeType>,
    out_trade_type: Option<SymbiosisTradeType>,
    #[serde(rename = "type")]
    token_type: TokenTypes,
}

impl SwapInfo {
    pub fn tx(&self) -> Transaction {
        self.tx.clone()
    }
    pub fn fee(&self) -> TokenAmount {
        self.fee.clone()
    }
    pub fn price_impact(&self) -> String {
        self.price_impact.clone()
    }
    pub fn token_amount_out(&self) -> TokenAmount {
        self.token_amount_out.clone()
    }
    pub fn amount_in_usd(&self) -> TokenAmount {
        self.amount_in_usd.clone()
    }
    pub fn approve_to(&self) -> Address {
        self.approve_to
    }
    pub fn route(&self) -> Vec<Token> {
        self.route.clone()
    }
    pub fn in_trade_type(&self) -> Option<SymbiosisTradeType> {
        self.in_trade_type.clone()
    }
    pub fn out_trade_type(&self) -> Option<SymbiosisTradeType> {
        self.out_trade_type.clone()
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UnprocessEntityBody {
    code: u64,
    message: String,
    errors: Vec<EntityError>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EntityError {
    field: String,
    message: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BadRequestBody {
    code: u64,
    message: String,
}
