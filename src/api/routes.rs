use crate::cores::endpoint::Endpoint;
use http::Method;
use serde::{Deserialize, Serialize};

/// It returns the list of supported chains.
#[derive(Default, Debug, Clone)]
pub struct GetAvailableRoutes;

impl Endpoint for GetAvailableRoutes {
    fn method(&self) -> http::Method {
        Method::GET
    }

    fn endpoint(&self) -> std::borrow::Cow<'static, str> {
        "v1/available-routes".into()
    }
}

/// Response data of available routes request.
#[derive(Default, Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AvailableRoutesResponse {
    origin_chain_id: u64,
    origin_token: String,
    destination_chain_id: u64,
    destination_token: String,
}
