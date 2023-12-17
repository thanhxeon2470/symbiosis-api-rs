use crate::{cores::params::ParamValue, types::*};
use derive_builder::Builder;
use ethers_core::types::{Chain, U256};
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, skip_serializing_none, NoneAsEmptyString, TryFromInto};
use std::default;

/// [Token] class which contains information about supported cryptocurrency.
#[allow(missing_docs)]
#[serde_as]
#[skip_serializing_none]
#[derive(Clone, Default, Builder, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[builder(setter(strip_option))]
#[builder(default)]
pub struct Token {
    /// The decimals of the token.
    #[builder(default = "18")]
    decimals: u8, // max = 18
    /// The token ticker symbol.
    symbol: Option<String>,
    /// The token name.
    name: Option<String>,
    /// The token chain id.
    #[serde_as(as = "TryFromInto<u64>")]
    #[builder(default = "Chain::Mainnet")]
    chain_id: Chain,
    /// The token address, `None` or empty string if the token is native.
    #[serialize_always]
    #[serde_as(as = "NoneAsEmptyString")]
    address: Option<String>,
    /// The token icon url.
    icon: Option<String>,
    chain_from_id: Option<Chain>,
    /// Indicate that the token is native or not.
    is_native: Option<bool>,
    /// Indicate that the token is created by user or not.
    user_token: Option<bool>,
    // deprecated: bool,
}

impl Token {
    /// Create a builder for [Token].
    pub fn builder() -> TokenBuilder {
        TokenBuilder::default()
    }

    /// Get the token decimals.
    pub fn decimals(&self) -> u8 {
        self.decimals
    }

    /// Get the token symbol.
    pub fn symbol(&self) -> Option<String> {
        self.symbol.clone()
    }

    /// Get the token name.
    pub fn name(&self) -> Option<String> {
        self.name.clone()
    }

    /// Get the token chain_id.
    pub fn chain_id(&self) -> Chain {
        self.chain_id
    }

    /// Get the token address.
    pub fn address(&self) -> Option<String> {
        self.address.clone()
    }

    /// Get the token icon.
    pub fn icon(&self) -> Option<String> {
        self.icon.clone()
    }

    /// Get the token chain_from_id.
    pub fn chain_from_id(&self) -> Option<Chain> {
        self.chain_from_id
    }

    /// Get if the token is native or not.
    pub fn is_native(&self) -> Option<bool> {
        self.is_native
    }

    /// Get if the token is created by user or not.
    pub fn user_token(&self) -> Option<bool> {
        self.user_token
    }
}

/// [TokenAmount] class which contains information about cryptocurrency with associated amount.
#[allow(missing_docs)]
#[serde_as]
#[derive(Clone, Default, Builder, Debug, Serialize, Deserialize)]
pub struct TokenAmount {
    #[serde(flatten)]
    #[builder(setter)]
    token: Token,

    #[serde(
        serialize_with = "serialize_u256_as_dec_string",
        deserialize_with = "deserialize_u256_as_dec_string"
    )]
    #[builder(setter)]
    amount: U256,
}

impl TokenAmount {
    /// Create a builder for [TokenAmount].
    pub fn builder() -> TokenAmountBuilder {
        TokenAmountBuilder::default()
    }

    /// Get the `token` associated with this [TokenAmount] instance.
    pub fn token(&self) -> Token {
        self.token.clone()
    }

    /// Get the `amount` associated with this [TokenAmount] instance.
    pub fn amount(&self) -> U256 {
        self.amount
    }
}

/// Token type.
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum TokenTypes {
    /// EVM compatible token.
    #[default]
    Evm,
    /// TRC20 token.
    Tron,
}

#[cfg(test)]
mod tests {
    use super::TokenTypes;

    #[test]
    fn token_types_as_str() {
        let items = &[(TokenTypes::Evm, "\"evm\""), (TokenTypes::Tron, "\"tron\"")];

        for (i, s) in items {
            assert_eq!(serde_json::to_string(i).unwrap(), *s);
        }
    }
}
