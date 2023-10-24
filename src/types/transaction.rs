use super::{serde_utils::*, token::TokenAmount};
use chrono::{DateTime, Utc};
use derive_builder::Builder;
use ethers_core::types::{Address, Bytes, Chain, TransactionRequest, TxHash, U256};
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, TryFromInto};

/// Transaction to sent to the network, return from the `Symbiosis` API.  
#[allow(missing_docs)]
#[serde_as]
#[derive(Clone, Default, Builder, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Tx {
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

impl From<Tx> for TransactionRequest {
    fn from(value: Tx) -> Self {
        TransactionRequest::new()
            .to(value.to)
            .value(value.value)
            .data(value.data)
            .chain_id(value.chain_id)
    }
}

impl Tx {
    /// Get the `to` address in this transaction.
    pub fn to(&self) -> Address {
        self.to
    }

    /// Get the `value` to be sent in this transaction.
    pub fn value(&self) -> U256 {
        self.value
    }

    /// Get the `data` to be called in this transaction.
    pub fn data(&self) -> Bytes {
        self.data.clone()
    }

    /// Get the `chain_id` to be sent in this transaction.
    pub fn chain_id(&self) -> Chain {
        self.chain_id
    }
}

/// `Symbiosis` origin transaction, means that the transaction sent to the origin blockchain.
#[serde_as]
#[derive(Clone, Default, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TxOrigin {
    hash: TxHash,
    #[serde_as(as = "TryFromInto<u64>")]
    chain_id: Chain,
    token_amount: TokenAmount,
    time: DateTime<Utc>,
    /// Receiver address on destination blockchain.
    address: Address,
}
