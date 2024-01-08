use anyhow::bail;
use reqwest::{Client, Method, Url};
use std::str::FromStr;
use tracing::{debug, trace};
use types::{request::Request, response::Response};

pub mod types;

#[derive(Clone, Debug)]
pub struct SymbiosisApi {
    partner_id: Option<String>,
    url: Url,
    req: Option<(String, Method, Request)>,
}

impl SymbiosisApi {
    /// POST /v1/swapping/exact_in
    pub fn post_v1_swapping_exact_in(mut self, req: Request) -> anyhow::Result<Self> {
        match req {
            Request::SwappingExactIn(body) => {
                self.req = Some((
                    "/v1/swapping/exact_in".to_string(),
                    Method::POST,
                    Request::SwappingExactIn(body),
                ));
                anyhow::Ok(self)
            }
            _ => bail!("expect: SwappingExactIn, found {:?}", req),
        }
    }

    /// - serialize the request
    /// - send the request
    /// - receive the response
    pub async fn send(self) -> anyhow::Result<Response> {
        if let Some((route, method, body)) = self.req {
            let url = match &self.partner_id {
                Some(id) => {
                    format!("{}{}?partnerId={}", self.url, route, id)
                }
                None => format!("{}{}", self.url, route),
            };

            debug!(?url, ?method, ?body);
            trace!("request body to string {:?}", serde_json::to_string(&body));

            let response = Client::new()
                .request(method, url)
                .json(&body)
                .send()
                .await?;

            debug!(?response);

            response
                .json::<Response>()
                .await
                .map_err(anyhow::Error::from)
        } else {
            bail!("req is None")
        }
    }
}

impl SymbiosisApi {
    pub fn builder() -> SymbiosisApiBuilder {
        SymbiosisApiBuilder::default()
    }
}

#[derive(Debug)]
pub struct SymbiosisApiBuilder {
    partner_id: Option<String>,
    url: Url,
}

impl Default for SymbiosisApiBuilder {
    fn default() -> Self {
        let url = "https://api-v2.symbiosis.finance/crosschain";
        Self {
            partner_id: None,
            url: Url::from_str(url).unwrap(),
        }
    }
}

impl SymbiosisApiBuilder {
    pub fn with_partner_id(mut self, partner_id: impl Into<String>) -> Self {
        self.partner_id = Some(partner_id.into());
        self
    }
    pub fn with_url(mut self, url: &str) -> Self {
        self.url = Url::from_str(url).expect("cannot parse url {url}");
        self
    }
    pub fn build(self) -> SymbiosisApi {
        SymbiosisApi {
            partner_id: self.partner_id,
            url: self.url,
            req: None,
        }
    }
}
