use serde::de::DeserializeOwned;
use crate::{api::API, config::Config, errors::Result};

/// Blocking HTTP client for synchronous operations
#[derive(Clone)]
pub struct Client {
    pub api_key: String,
    pub secret_key: String,
    pub host: String,
    pub http_client: reqwest::blocking::Client,
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
        let blocking_client = reqwest::blocking::Client::builder()
            .pool_idle_timeout(None)
            .build()
            .unwrap();

        Client {
            api_key: api_key.unwrap_or_default(),
            secret_key: secret_key.unwrap_or_default(),
            host: config.rest_api_endpoint.clone(),
            http_client: blocking_client,
            verbose: false,
        }
    }

    pub fn set_verbose(&mut self, verbose: bool) {
        self.verbose = verbose;
    }

    /// Blocking GET request
    pub fn get(&self, endpoint: API, params: Option<String>) -> Result<String> {
        let mut url = format!("{}{}", self.host, String::from(endpoint));
        if let Some(params) = params {
            if !params.is_empty() {
                url.push_str(&format!("?{}", params));
            }
        }

        if self.verbose {
            println!("Request URL: {}", url);
        }

        let response = self.http_client.get(&url).send()?;
        let text = response.text()?;

        Ok(text)
    }

    /// Blocking POST request
    pub fn post(&self, endpoint: API, params: Option<String>) -> Result<String> {
        let url = format!("{}{}", self.host, String::from(endpoint));

        if self.verbose {
            println!("Request URL: {}", url);
            if let Some(ref p) = params {
                println!("Request Body: {}", p);
            }
        }

        let mut request = self.http_client.post(&url);

        if let Some(body) = params {
            request = request
                .header("Content-Type", "application/x-www-form-urlencoded")
                .body(body);
        }

        let response = request.send()?;
        let text = response.text()?;

        Ok(text)
    }

    /// Blocking GET request with JSON parsing
    pub fn get_json<T: DeserializeOwned>(&self, endpoint: API, params: Option<String>) -> Result<T> {
        let text = self.get(endpoint, params)?;
        Ok(serde_json::from_str(&text)?)
    }

    /// Blocking POST request with JSON parsing
    pub fn post_json<T: DeserializeOwned>(&self, endpoint: API, params: Option<String>) -> Result<T> {
        let text = self.post(endpoint, params)?;
        Ok(serde_json::from_str(&text)?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_client_creation() {
        let client = Client::new(Some("test_key".into()), Some("test_secret".into()));

        assert_eq!(client.api_key, "test_key");
        assert_eq!(client.secret_key, "test_secret");
    }

    #[test]
    fn test_client_with_default_credentials() {
        let client = Client::new(None, None);

        assert_eq!(client.api_key, "");
        assert_eq!(client.secret_key, "");
    }

    #[test]
    fn test_set_verbose() {
        let mut client = Client::new(None, None);
        assert!(!client.verbose);

        client.set_verbose(true);
        assert!(client.verbose);
    }

    #[test]
    fn test_client_with_custom_config() {
        let mut config = Config::default();
        config.rest_api_endpoint = "https://custom.api.com".to_string();

        let client = Client::new_with_config(
            Some("key".into()),
            Some("secret".into()),
            &config
        );

        assert_eq!(client.host, "https://custom.api.com");
    }
}
