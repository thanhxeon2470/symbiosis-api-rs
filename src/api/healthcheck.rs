use crate::cores::endpoint::Endpoint;
use http::Method;

/// It returns the health of the API: 200 if everything is up and running.
#[derive(Default, Debug, Clone)]
pub struct HealthCheck;

impl Endpoint for HealthCheck {
    fn method(&self) -> http::Method {
        Method::GET
    }

    fn endpoint(&self) -> std::borrow::Cow<'static, str> {
        "health-check".into()
    }
}
