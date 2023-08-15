use crate::error::Error;
use reqwest::header;
use serde::de::DeserializeOwned;
use std::time::Duration;

pub type Result<T> = std::result::Result<T, Error>;

const DEFAULT_TIMEOUT: Duration = Duration::from_secs(10);

const DEFAULT_BASE_URL: &str = "https://futures.kraken.com";

const DEFAULT_USER_AGENT: &str = "rust-kraken-futures-client/0.2";

#[derive(Default)]
pub struct ClientBuilder {
    base_url: Option<String>,
    user_agent: Option<String>,
    public_key: Option<String>,
    private_key: Option<String>,
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

    pub fn public_key(mut self, public_key: &str) -> Self {
        self.public_key = Some(public_key.to_string());
        self
    }

    pub fn private_key(mut self, private_key: &str) -> Self {
        self.private_key = Some(private_key.to_string());
        self
    }

    pub fn auth(mut self, public_key: &str, private_key: &str) -> Self {
        self.public_key = Some(public_key.to_string());
        self.private_key = Some(private_key.to_string());
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
        Client {
            base_url: self
                .base_url
                .unwrap_or_else(|| DEFAULT_BASE_URL.to_string()),
            user_agent: self
                .user_agent
                .unwrap_or_else(|| DEFAULT_USER_AGENT.to_string()),
            public_key: self.public_key,
            private_key: self.private_key,
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
    user_agent: String,
    #[allow(dead_code)]
    public_key: Option<String>,
    #[allow(dead_code)]
    private_key: Option<String>,
    http_client: reqwest::Client,
}

impl Default for Client {
    fn default() -> Self {
        Self::builder().build()
    }
}

impl Client {
    pub fn new(public_key: &str, private_key: &str) -> Self {
        Self::builder()
            .public_key(public_key)
            .private_key(private_key)
            .build()
    }

    pub fn builder() -> ClientBuilder {
        ClientBuilder::default()
    }

    async fn unwrap_response<Resp>(&self, resp: reqwest::Response) -> Result<Resp>
    where
        Resp: DeserializeOwned,
    {
        let resp: Resp = resp.json().await?;
        Ok(resp)
    }

    /// Sends a public request to the API.
    pub async fn send_public<Resp>(&self, url: &str) -> Result<Resp>
    where
        Resp: DeserializeOwned,
    {
        let url = format!("{}{}", self.base_url, url);

        println!("=== {url}");

        let resp = self
            .http_client
            .get(&url)
            .header(header::USER_AGENT, &self.user_agent)
            .send()
            .await?;

        self.unwrap_response(resp).await
    }

    // /// Sends a private request to the API.
    // pub async fn send_private<Resp>(&self, url: &str, query: Option<String>) -> Result<Resp>
    // where
    //     Resp: DeserializeOwned,
    // {
    //     let resp = if let Some(public_key) = &self.public_key {
    //         if let Some(private_key) = &self.private_key {
    //             let pathname = url;
    //             let url = format!("{}{}", self.base_url, url);

    //             let nonce = sign::compute_nonce().to_string();

    //             let formdata = if let Some(query) = query {
    //                 format!("{}&nonce={}", query, nonce)
    //             } else {
    //                 format!("nonce={}", nonce)
    //             };

    //             self.http_client
    //                 .post(&url)
    //                 .header(header::CONTENT_TYPE, "application/x-www-form-urlencoded")
    //                 .header(header::USER_AGENT, &self.user_agent)
    //                 .header("API-Key", public_key)
    //                 .header(
    //                     "API-Sign",
    //                     sign::compute_signature(private_key, pathname, &nonce, &formdata)?,
    //                 )
    //                 .body(formdata.into_bytes())
    //                 .send()
    //                 .await?
    //         } else {
    //             return Err(Error::Unauthorized);
    //         }
    //     } else {
    //         return Err(Error::Unauthorized);
    //     };

    //     self.unwrap_response(resp).await
    // }
}
