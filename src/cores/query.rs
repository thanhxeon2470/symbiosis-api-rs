use super::client::Client;
use async_trait::async_trait;
use http::Uri;
use url::Url;

/// Convert url::Url to http::Uri
pub fn url_to_http_uri(url: Url) -> Uri {
    url.as_str()
        .parse::<Uri>()
        .expect("failed to parse a url::Url as an http::Uri")
}

/// A trait which represents an asynchronous query which may be made to a client.
#[async_trait]
pub trait Query<T, C>
where
    C: Client,
{
    /// Perform the query asynchronously against the client.
    async fn query(&self, client: &C) -> anyhow::Result<T>;
}
