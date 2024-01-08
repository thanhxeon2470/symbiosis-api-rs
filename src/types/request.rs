use super::{Token, TokenAmount};
use ethers_core::types::Address;
use serde::Serialize;
use smart_default::SmartDefault;

#[derive(Clone, Debug, Serialize)]
#[serde(untagged)]
pub enum Request {
    SwappingExactIn(SwappingExactInBody),
}

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SwappingExactInBody {
    token_amount_in: TokenAmount,
    token_out: Token,
    from: Address,
    to: Address,
    slippage: usize,
    deadline: u64,
}

impl SwappingExactInBody {
    pub fn builder() -> SwappingExactInBodyBuilder {
        SwappingExactInBodyBuilder::default()
    }
}

#[derive(SmartDefault, Debug, Serialize)]
pub struct SwappingExactInBodyBuilder {
    token_amount_in: TokenAmount,
    token_out: Token,
    from: Address,
    to: Address,
    #[default = 300] // 3%
    slippage: usize,
    #[default = 2000000000]
    deadline: u64,
}

impl SwappingExactInBodyBuilder {
    pub fn token_amount_in(mut self, token_amount_in: impl Into<TokenAmount>) -> Self {
        self.token_amount_in = token_amount_in.into();
        self
    }
    pub fn token_out(mut self, token_out: impl Into<Token>) -> Self {
        self.token_out = token_out.into();
        self
    }
    pub fn from(mut self, from: impl Into<Address>) -> Self {
        self.from = from.into();
        self
    }
    pub fn to(mut self, to: impl Into<Address>) -> Self {
        self.to = to.into();
        self
    }
    pub fn slippage(mut self, slippage: impl Into<usize>) -> Self {
        self.slippage = slippage.into();
        self
    }
    pub fn deadline(mut self, deadline: impl Into<u64>) -> Self {
        self.deadline = deadline.into();
        self
    }
    pub fn build(self) -> SwappingExactInBody {
        SwappingExactInBody {
            token_amount_in: self.token_amount_in,
            token_out: self.token_out,
            from: self.from,
            to: self.to,
            slippage: self.slippage,
            deadline: self.deadline,
        }
    }
}
