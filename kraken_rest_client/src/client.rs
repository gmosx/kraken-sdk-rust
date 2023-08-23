use crate::error::Error;
use crate::sign;
use reqwest::header;
use serde::{de::DeserializeOwned, Deserialize};
use std::time::Duration;

#[derive(Debug, Deserialize)]
struct ResponseWrapper<T> {
    pub error: Vec<String>,
    pub result: Option<T>,
}

pub type Result<T> = std::result::Result<T, Error>;

const DEFAULT_TIMEOUT: Duration = Duration::from_secs(10);

const DEFAULT_BASE_URL: &str = "https://api.kraken.com";

const DEFAULT_USER_AGENT: &str = "rust-kraken-client/0.16";

#[derive(Default)]
pub struct ClientBuilder {
    base_url: Option<String>,
    user_agent: Option<String>,
    api_key: Option<String>,
    api_secret: Option<String>,
    http_client: Option<reqwest::Client>,
    timeout: Option<Duration>,
}

impl ClientBuilder {
    pub fn base_url(mut self, base_url: &str) -> Self {
        self.base_url = Some(base_url.to_string());
        self
    }

    pub fn user_agent(mut self, user_agent: &str) -> Self {
        self.user_agent = Some(user_agent.to_string());
        self
    }

    pub fn api_key(mut self, api_key: &str) -> Self {
        self.api_key = Some(api_key.to_string());
        self
    }

    pub fn api_secret(mut self, api_secret: &str) -> Self {
        self.api_secret = Some(api_secret.to_string());
        self
    }

    pub fn auth(mut self, api_key: &str, api_secret: &str) -> Self {
        self.api_key = Some(api_key.to_string());
        self.api_secret = Some(api_secret.to_string());
        self
    }

    pub fn http_client(mut self, http_client: reqwest::Client) -> Self {
        self.http_client = Some(http_client);
        self
    }

    pub fn timeout(mut self, timeout: Duration) -> Self {
        self.timeout = Some(timeout);
        self
    }

    pub fn build(self) -> Client {
        // #TODO handle the unwrap
        Client {
            base_url: self
                .base_url
                .unwrap_or_else(|| DEFAULT_BASE_URL.to_string()),
            user_agent: self
                .user_agent
                .unwrap_or_else(|| DEFAULT_USER_AGENT.to_string()),
            api_key: self.api_key,
            api_secret: self.api_secret,
            http_client: self.http_client.unwrap_or_else(|| {
                reqwest::Client::builder()
                    .timeout(self.timeout.unwrap_or(DEFAULT_TIMEOUT))
                    .build()
                    .unwrap()
            }),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Client {
    base_url: String,
    /// You must supply a user agent string while creating a request header else you
    /// will not be able to connect to the API.
    user_agent: String,
    api_key: Option<String>,
    api_secret: Option<String>,
    http_client: reqwest::Client,
}

impl Default for Client {
    fn default() -> Self {
        Self::builder().build()
    }
}

impl Client {
    pub fn new(api_key: &str, api_secret: &str) -> Self {
        Self::builder()
            .api_key(api_key)
            .api_secret(api_secret)
            .build()
    }

    pub fn builder() -> ClientBuilder {
        ClientBuilder::default()
    }

    async fn unwrap_response<Resp>(&self, resp: reqwest::Response) -> Result<Resp>
    where
        Resp: DeserializeOwned,
    {
        let resp: ResponseWrapper<Resp> = resp.json().await?;

        if !resp.error.is_empty() {
            return Err(Error::Api(resp.error.join(",")));
        }

        if let Some(result) = resp.result {
            Ok(result)
        } else {
            Err(Error::internal("no result field in response"))
        }
    }

    // #TODO the parameter is path, not url!
    /// Sends a public request to the API.
    pub async fn send_public<Resp>(&self, url: &str) -> Result<Resp>
    where
        Resp: DeserializeOwned,
    {
        let url = format!("{}{}", self.base_url, url);

        let resp = self
            .http_client
            .get(&url)
            .header(header::USER_AGENT, &self.user_agent)
            .send()
            .await?;

        self.unwrap_response(resp).await
    }

    // #TODO the parameter is path, not url!
    /// Sends a private request to the API.
    pub async fn send_private<Resp>(&self, url: &str, query: Option<String>) -> Result<Resp>
    where
        Resp: DeserializeOwned,
    {
        let resp = if let Some(api_key) = &self.api_key {
            if let Some(api_secret) = &self.api_secret {
                let pathname = url;
                let url = format!("{}{}", self.base_url, url);

                let nonce = sign::compute_nonce().to_string();

                let body = if let Some(query) = query {
                    format!("{}&nonce={}", query, nonce)
                } else {
                    format!("nonce={}", nonce)
                };

                self.http_client
                    .post(&url)
                    .header(header::CONTENT_TYPE, "application/x-www-form-urlencoded")
                    .header(header::USER_AGENT, &self.user_agent)
                    .header("API-Key", api_key)
                    .header(
                        "API-Sign",
                        sign::compute_signature(api_secret, pathname, &nonce, &body)?,
                    )
                    .body(body)
                    .send()
                    .await?
            } else {
                return Err(Error::Unauthorized);
            }
        } else {
            return Err(Error::Unauthorized);
        };

        self.unwrap_response(resp).await
    }

    // #TODO the parameter is path, not url!
    /// Sends a private request to the API.
    pub async fn send_private_json<Resp>(&self, url: &str, body: String) -> Result<Resp>
    where
        Resp: DeserializeOwned,
    {
        let resp = if let Some(api_key) = &self.api_key {
            if let Some(api_secret) = &self.api_secret {
                let pathname = url;
                let url = format!("{}{}", self.base_url, url);

                let nonce = sign::compute_nonce().to_string();

                let formdata = if let Some(query) = query {
                    format!("{}&nonce={}", query, nonce)
                } else {
                    format!("nonce={}", nonce)
                };

                self.http_client
                    .post(&url)
                    .header(header::CONTENT_TYPE, "application/json")
                    .header(header::USER_AGENT, &self.user_agent)
                    .header("API-Key", api_key)
                    .header(
                        "API-Sign",
                        sign::compute_signature(api_secret, pathname, &nonce, &formdata)?,
                    )
                    .body(body.into_bytes())
                    .send()
                    .await?
            } else {
                return Err(Error::Unauthorized);
            }
        } else {
            return Err(Error::Unauthorized);
        };

        self.unwrap_response(resp).await
    }
}
