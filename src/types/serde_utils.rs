use ethers_core::types::*;
use serde::{Deserialize, Deserializer, Serializer};

// TODO: this work is a little bit tricky. Please refactor it to a nicer version

// serialize_option_chain
// deserialize_option_chain

/// serialize_U256_as_dec_string
/// input: U256, output: decimal String
pub fn serialize_u256_as_dec_string<S>(x: &U256, s: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    s.serialize_str(x.to_string().as_str())
}

/// deserialize_U256_as_dec_string
/// input: decimal String, output: U256
pub fn deserialize_u256_as_dec_string<'de, D>(deserializer: D) -> Result<U256, D::Error>
where
    D: Deserializer<'de>,
{
    String::deserialize(deserializer)
        .map(|string| U256::from_dec_str(string.as_str()).unwrap_or_default())
}
