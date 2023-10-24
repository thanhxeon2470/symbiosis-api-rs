use crate::cores;
use async_trait::async_trait;
use reqwest::{Client, Request, Url};

/// A representation of the Symbiosis API.
#[derive(Clone, Debug)]
pub struct Symbiosis {
    /// The client to use for API calls.
    client: Client,
    /// The base URL to use for API calls.
    base_url: Url,
    /// The authentication information to use when communicating with [Symbiosis API].
    ///
    /// Will be added as `partnerId` query in every requests.
    auth: String,
}

impl Symbiosis {
    /// Create a new Symbiosis instance, with partner id.
    pub fn new(auth: &str) -> Self {
        Self {
            client: Client::new(),
            base_url: Url::parse("https://api-v2.symbiosis.finance/crosschain/").unwrap(),
            auth: auth.to_string(),
        }
    }
}

#[async_trait]
impl cores::client::Client for Symbiosis {
    fn endpoint(&self, endpoint: &str) -> anyhow::Result<Url> {
        self.base_url.join(endpoint).map_err(anyhow::Error::from)
    }

    async fn send(
        &self,
        req: http::request::Builder,
        body: Vec<u8>,
    ) -> anyhow::Result<http::Response<bytes::Bytes>> {
        let http_req = req.body(body)?;
        let mut request: Request = http_req.try_into()?;
        request
            .url_mut()
            .set_query(Some(&format!("partnerId={}", self.auth)));

        let resp = self.client.execute(request).await?;
        let mut http_resp = http::Response::builder()
            .status(resp.status())
            .version(resp.version());
        let headers = http_resp.headers_mut().unwrap();
        for (key, value) in resp.headers() {
            headers.insert(key, value.clone());
        }

        Ok(http_resp.body(resp.bytes().await?)?)
    }
}
