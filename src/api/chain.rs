use crate::cores::endpoint::Endpoint;
use http::Method;
use serde::{Deserialize, Serialize};

/// It returns the list of supported chains.
#[derive(Default, Debug, Clone)]
pub struct GetSupportedChains;

impl Endpoint for GetSupportedChains {
    fn method(&self) -> http::Method {
        Method::GET
    }

    fn endpoint(&self) -> std::borrow::Cow<'static, str> {
        "v1/chains".into()
    }
}

/// Symbiosis supported chain information, included in response body.
#[derive(Default, Serialize, Deserialize, Debug, Clone)]
pub struct SymbiosisChain {
    id: u64,
    name: String,
    explorer: String,
    icon: String,
}
