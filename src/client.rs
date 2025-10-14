use serde::de::DeserializeOwned;
use crate::config::Config;

// Custom error type for the client
#[derive(Debug)]
pub enum ClientError {
    ReqwestError(reqwest::Error),
    ParseError(String),
}

impl From<reqwest::Error> for ClientError {
    fn from(err: reqwest::Error) -> Self {
        ClientError::ReqwestError(err)
    }
}

pub type Result<T> = std::result::Result<T, ClientError>;

#[derive(Clone)]
pub struct Client {
    pub api_key: String,
    pub secret_key: String,
    pub host: String,
    pub blocking_client: Option<reqwest::blocking::Client>, 
    pub async_client: Option<reqwest::Client>,
    pub verbose: bool,
}

impl Client {
    /// Create a new blocking client
    pub fn new(api_key: Option<String>, secret_key: Option<String>) -> Self {
        let config = Config::default();
        Self::new_with_config(api_key, secret_key, &config)
    }

    /// Create a new blocking client with custom config
    pub fn new_with_config(
        api_key: Option<String>, 
        secret_key: Option<String>,
        config: &Config
    ) -> Self {
        Client {
            api_key: api_key.unwrap_or_default(),
            secret_key: secret_key.unwrap_or_default(),
            host: config.rest_api_endpoint.clone(),
            blocking_client: Some(
                reqwest::blocking::Client::builder()
                    .pool_idle_timeout(None)
                    .build()
                    .unwrap()
            ),
            async_client: None,
            verbose: false,
        }
    }

    /// Create a new async client
    pub fn new_async(api_key: Option<String>, secret_key: Option<String>) -> Self {
        let config = Config::default();
        Self::new_async_with_config(api_key, secret_key, &config)
    }

    /// Create a new async client with custom config
    pub fn new_async_with_config(
        api_key: Option<String>,
        secret_key: Option<String>,
        config: &Config
    ) -> Self {
        Client {
            api_key: api_key.unwrap_or_default(),
            secret_key: secret_key.unwrap_or_default(),
            host: config.rest_api_endpoint.clone(),
            blocking_client: None,
            async_client: Some(
                reqwest::Client::builder()
                    .pool_idle_timeout(None)
                    .build()
                    .unwrap()
            ),
            verbose: false,      }
    }

    pub fn set_verbose(&mut self, verbose: bool) {
        self.verbose = verbose; 
    }

    pub fn is_async(&self) -> bool {
        self.async_client.is_some()
    }

    /// Blocking GET request
    pub fn get_blocking(&self, endpoint: &str, params: Option<&str>) -> Result<String> {
        let client = self.blocking_client.as_ref()
            .expect("Blocking client not initialized. Use Client::new() instead of Client::new_async()");

        let mut url = format!("{}{}", self.host, endpoint);
        if let Some(params) = params {
            if !params.is_empty() {
                url.push_str(&format!("?{}", params));
            }
        }

        if self.verbose {
            println!("Request URL: {}", url);
        }

        let response = client.get(&url).send()?;
        let text = response.text()?;
        
        Ok(text)
    }

    /// Async GET request
    pub async fn get_async(&self, endpoint: &str, params: Option<&str>) -> Result<String> {
        let client = self.async_client.as_ref()
            .expect("Async client not initialized. Use Client::new_async() instead of Client::new()");

        let mut url = format!("{}{}", self.host, endpoint);
        if let Some(params) = params {
            if !params.is_empty() {
                url.push_str(&format!("?{}", params));
            }
        }

        if self.verbose {
            println!("Request URL: {}", url);
        }

        let response = client.get(&url).send().await?;
        let text = response.text().await?;
        
        Ok(text)
    }

    /// Blocking GET request with JSON parsing
    pub fn get_json_blocking<T: DeserializeOwned>(&self, endpoint: &str, params: Option<&str>) -> Result<T> {
        let text = self.get_blocking(endpoint, params)?;
        serde_json::from_str(&text)
            .map_err(|e| ClientError::ParseError(e.to_string()))
    }

    /// Async GET request with JSON parsing
    pub async fn get_json_async<T: DeserializeOwned>(&self, endpoint: &str, params: Option<&str>) -> Result<T> {
        let text = self.get_async(endpoint, params).await?;
        serde_json::from_str(&text)
            .map_err(|e| ClientError::ParseError(e.to_string()))
    }
}