use super::serde_utils::*;
use crate::types::TryFromInto;
use ethers_core::types::{Address, Bytes, Chain, TransactionRequest, U256};
use serde::{Deserialize, Serialize};
use serde_with::serde_as;
use smart_default::SmartDefault;

#[serde_as]
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Transaction {
    to: Address,
    #[serde(
        serialize_with = "serialize_u256_as_dec_string",
        deserialize_with = "deserialize_u256_as_dec_string"
    )]
    value: U256,
    data: Bytes,
    #[serde_as(as = "TryFromInto<u64>")]
    chain_id: Chain,
}

impl From<Transaction> for TransactionRequest {
    fn from(value: Transaction) -> Self {
        TransactionRequest::new()
            .to(value.to)
            .value(value.value)
            .data(value.data)
            .chain_id(value.chain_id)
    }
}

#[derive(SmartDefault, Debug, Serialize)]
pub struct TransactionBuilder {
    to: Address,
    value: U256,
    #[default(Bytes::default())]
    data: Bytes,
    chain_id: Chain,
}

impl Transaction {
    pub fn builder() -> TransactionBuilder {
        TransactionBuilder::default()
    }
    pub fn to(&self) -> Address {
        self.to
    }
    pub fn value(&self) -> U256 {
        self.value
    }
    pub fn data(&self) -> Bytes {
        self.data.clone()
    }
    pub fn chain_id(&self) -> Chain {
        self.chain_id
    }
}

impl TransactionBuilder {
    pub fn to(mut self, to: impl Into<Address>) -> Self {
        self.to = to.into();
        self
    }
    pub fn value(mut self, value: impl Into<U256>) -> Self {
        self.value = value.into();
        self
    }
    pub fn data(mut self, data: impl Into<Bytes>) -> Self {
        self.data = data.into();
        self
    }
    pub fn chain_id(mut self, chain_id: impl Into<Chain>) -> Self {
        self.chain_id = chain_id.into();
        self
    }
}
