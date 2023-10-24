use super::{
    client::Client,
    error::SymbiosisError,
    query::{url_to_http_uri, Query},
};
use crate::cores::params::QueryParams;
use async_trait::async_trait;
use http::{header::CONTENT_TYPE, Method, Request};
use serde::de::DeserializeOwned;
use std::borrow::Cow;

/// A trait for providing the necessary information for a single REST API endpoint.
pub trait Endpoint {
    /// The HTTP method to use for the endpoint.
    fn method(&self) -> Method;
    /// The path to the endpoint.
    fn endpoint(&self) -> Cow<'static, str>;

    /// Query parameters for the endpoint.
    fn parameters(&self) -> QueryParams<'_> {
        QueryParams::default()
    }

    /// The body for the endpoint.
    ///
    /// Returns the `Content-Encoding` header for the data as well as the data itself.
    fn body(&self) -> anyhow::Result<Option<(&'static str, Vec<u8>)>> {
        Ok(None)
    }
}

#[async_trait]
impl<E, T, C> Query<T, C> for E
where
    E: Endpoint + Sync,
    T: DeserializeOwned + 'static,
    C: Client + Sync,
{
    async fn query(&self, client: &C) -> anyhow::Result<T> {
        let mut url = client.endpoint(&self.endpoint())?;
        self.parameters().add_to_url(&mut url);

        let req = Request::builder()
            .method(self.method())
            .uri(url_to_http_uri(url));
        let (req, data) = if let Some((mime, data)) = self.body()? {
            let req = req.header(CONTENT_TYPE, mime);
            (req, data)
        } else {
            (req, Vec::new())
        };

        let resp = client.send(req, data).await?;

        let status = resp.status();
        if !status.is_success() {
            let err: SymbiosisError = serde_json::from_slice(resp.body())?;
            return Err(err.into());
        }

        let v = serde_json::from_slice(resp.body())?;
        serde_json::from_value::<T>(v).map_err(anyhow::Error::from)
    }
}
