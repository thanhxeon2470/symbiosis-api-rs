use ethers_core::types::{Chain, U256};
use serde::{Deserialize, Serialize};
use serde_utils::*;
use serde_with::NoneAsEmptyString;
use serde_with::{serde_as, skip_serializing_none, TryFromInto};
use smart_default::SmartDefault;

pub mod request;
pub mod response;
pub mod serde_utils;
pub mod transaction;

#[serde_as]
#[skip_serializing_none]
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Token {
    decimals: u8, // max = 18
    symbol: Option<String>,
    name: Option<String>,
    #[serde_as(as = "TryFromInto<u64>")]
    chain_id: Chain,
    #[serialize_always]
    #[serde_as(as = "NoneAsEmptyString")]
    address: Option<String>,
    icon: Option<String>,
    chain_from_id: Option<Chain>,
    is_native: Option<bool>,
    user_token: Option<bool>,
    // deprecated: bool,
}

impl Default for Token {
    fn default() -> Self {
        Self::builder().build()
    }
}

impl Token {
    pub fn builder() -> TokenBuilder {
        TokenBuilder::default()
    }
    pub fn decimals(&self) -> u8 {
        self.decimals
    }
    pub fn symbol(&self) -> Option<String> {
        self.symbol.clone()
    }
    pub fn name(&self) -> Option<String> {
        self.name.clone()
    }
    pub fn chain_id(&self) -> Chain {
        self.chain_id
    }
    pub fn address(&self) -> Option<String> {
        self.address.clone()
    }
    pub fn icon(&self) -> Option<String> {
        self.icon.clone()
    }
    pub fn chain_from_id(&self) -> Option<Chain> {
        self.chain_from_id
    }
    pub fn is_native(&self) -> Option<bool> {
        self.is_native
    }
    pub fn user_token(&self) -> Option<bool> {
        self.user_token
    }
}

#[derive(SmartDefault, Debug, Serialize)]
pub struct TokenBuilder {
    #[default = 18]
    decimals: u8, // max = 18
    symbol: Option<String>,
    name: Option<String>,
    #[default(Chain::Mainnet)]
    chain_id: Chain,
    address: Option<String>,
    icon: Option<String>,
    chain_from_id: Option<Chain>,
    #[default(Some(true))]
    is_native: Option<bool>,
    user_token: Option<bool>,
    // deprecated: bool,
}

impl TokenBuilder {
    pub fn build(self) -> Token {
        Token {
            decimals: self.decimals,
            symbol: self.symbol,
            name: self.name,
            chain_id: self.chain_id,
            address: self.address,
            icon: self.icon,
            chain_from_id: self.chain_from_id,
            is_native: self.is_native,
            user_token: self.user_token,
            // deprecated: self.deprecated,
        }
    }
    pub fn decimals(mut self, decimals: impl Into<u8>) -> Self {
        self.decimals = decimals.into();
        self
    }
    pub fn symbol(mut self, symbol: impl Into<Option<String>>) -> Self {
        self.symbol = symbol.into();
        self
    }
    pub fn name(mut self, name: impl Into<Option<String>>) -> Self {
        self.name = name.into();
        self
    }
    pub fn chain_id(mut self, chain_id: impl Into<Chain>) -> Self {
        self.chain_id = chain_id.into();
        self
    }
    pub fn address(mut self, address: impl Into<Option<String>>) -> Self {
        self.address = address.into();
        self
    }
    pub fn icons(mut self, icons: impl Into<Option<String>>) -> Self {
        self.icon = icons.into();
        self
    }
    pub fn chain_from_id(mut self, chain_from_id: impl Into<Option<Chain>>) -> Self {
        self.chain_from_id = chain_from_id.into();
        self
    }
    pub fn is_native(mut self, is_native: impl Into<Option<bool>>) -> Self {
        self.is_native = is_native.into();
        self
    }
    pub fn user_token(mut self, user_token: impl Into<Option<bool>>) -> Self {
        self.user_token = user_token.into();
        self
    }
    // pub fn deprecated(mut self, deprecated: impl Into<bool>) -> Self {
    //     self.deprecated = deprecated.into();
    //     self
    // }
}

#[serde_as]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TokenAmount {
    #[serde(flatten)]
    token: Token,

    #[serde(
        serialize_with = "serialize_u256_as_dec_string",
        deserialize_with = "deserialize_u256_as_dec_string"
    )]
    amount: U256,
}

impl Default for TokenAmount {
    fn default() -> Self {
        Self {
            token: Token::builder().build(),
            amount: Default::default(),
        }
    }
}

impl TokenAmount {
    pub fn new(token: impl Into<Token>, amount: impl Into<U256>) -> Self {
        Self {
            token: token.into(),
            amount: amount.into(),
        }
    }

    pub fn token(&self) -> Token {
        self.token.clone()
    }

    pub fn amount(&self) -> U256 {
        self.amount
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum TokenTypes {
    Evm,
    Tron,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum SymbiosisTradeType {
    Dex,
    #[serde(rename = "1inch")]
    OneInch,
    #[serde(rename = "open-ocean")]
    OpenOcean,
    Wrap,
    Izumi,
}
