use async_trait::async_trait;
use bytes::Bytes;
use http::request::Builder as RequestBuilder;
use http::Response;
use url::Url;

/// A trait representing a client which can communicate with a [Symbiosis API] instance via REST.
#[async_trait]
pub trait Client {
    /// Get the URL for the endpoint for the client.
    ///
    /// This method adds the hostname for the client's target instance.
    fn endpoint(&self, endpoint: &str) -> anyhow::Result<Url>;

    /// Send a REST query asynchronously.
    async fn send(&self, request: RequestBuilder, body: Vec<u8>)
        -> anyhow::Result<Response<Bytes>>;
}
