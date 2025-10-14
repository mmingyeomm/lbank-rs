use serde::de::DeserializeOwned;
use crate::{api::API, config::Config, errors::Result};

// Enum to represent either blocking or async HTTP client
#[derive(Clone)]
pub enum HttpClient {
    Blocking(reqwest::blocking::Client),
    Async(reqwest::Client),
}

#[derive(Clone)]
pub struct Client {
    pub api_key: String,
    pub secret_key: String,
    pub host: String,
    pub http_client: HttpClient,
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
            http_client: HttpClient::Blocking(blocking_client),
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
        let async_client = reqwest::Client::builder()
            .pool_idle_timeout(None)
            .build()
            .unwrap();

        Client {
            api_key: api_key.unwrap_or_default(),
            secret_key: secret_key.unwrap_or_default(),
            host: config.rest_api_endpoint.clone(),
            http_client: HttpClient::Async(async_client),
            verbose: false,
        }
    }

    pub fn set_verbose(&mut self, verbose: bool) {
        self.verbose = verbose; 
    }

    pub fn is_async(&self) -> bool {
        matches!(self.http_client, HttpClient::Async(_))
    }

    /// Unified GET request - automatically uses blocking or async based on client type
    pub fn get(&self, endpoint: API, params: Option<String>) -> Result<String> {
        match &self.http_client {
            HttpClient::Blocking(_) => {
                // Direct blocking call
                self.get_blocking(endpoint, params)
            }
            HttpClient::Async(_) => {
                // Block on the async operation
                futures::executor::block_on(self.get_async(endpoint, params))
            }
        }
    }

    /// Unified POST request - automatically uses blocking or async based on client type
    pub fn post(&self, endpoint: API, params: Option<String>) -> Result<String> {
        match &self.http_client {
            HttpClient::Blocking(_) => {
                // Direct blocking call
                self.post_blocking(endpoint, params)
            }
            HttpClient::Async(_) => {
                // Block on the async operation
                futures::executor::block_on(self.post_async(endpoint, params))
            }
        }
    }

    /// Blocking GET request
    pub fn get_blocking(&self, endpoint: API, params: Option<String>) -> Result<String> {
        let HttpClient::Blocking(client) = &self.http_client else {
            panic!("Called get_blocking() on async client. Use Client::new() instead of Client::new_async()");
        };

        let mut url = format!("{}{}", self.host, String::from(endpoint));
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

    /// Blocking POST request
    pub fn post_blocking(&self, endpoint: API, params: Option<String>) -> Result<String> {
        let HttpClient::Blocking(client) = &self.http_client else {
            panic!("Called post_blocking() on async client. Use Client::new() instead of Client::new_async()");
        };

        let url = format!("{}{}", self.host, String::from(endpoint));

        if self.verbose {
            println!("Request URL: {}", url);
            if let Some(ref p) = params {
                println!("Request Body: {}", p);
            }
        }

        let mut request = client.post(&url);
        
        if let Some(body) = params {
            request = request
                .header("Content-Type", "application/x-www-form-urlencoded")
                .body(body);
        }

        let response = request.send()?;
        let text = response.text()?;
        
        Ok(text)
    }

    /// Async GET request
    pub async fn get_async(&self, endpoint: API, params: Option<String>) -> Result<String> {
        let HttpClient::Async(client) = &self.http_client else {
            panic!("Called get_async() on blocking client. Use Client::new_async() instead of Client::new()");
        };

        let mut url = format!("{}{}", self.host,  String::from(endpoint));
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

    /// Async POST request
    pub async fn post_async(&self, endpoint: API, params: Option<String>) -> Result<String> {
        let HttpClient::Async(client) = &self.http_client else {
            panic!("Called post_async() on blocking client. Use Client::new_async() instead of Client::new()");
        };

        let url = format!("{}{}", self.host, String::from(endpoint));

        if self.verbose {
            println!("Request URL: {}", url);
            if let Some(ref p) = params {
                println!("Request Body: {}", p);
            }
        }

        let mut request = client.post(&url);
        
        if let Some(body) = params {
            request = request
                .header("Content-Type", "application/x-www-form-urlencoded")
                .body(body);
        }

        let response = request.send().await?;
        let text = response.text().await?;
        
        Ok(text)
    }

    /// Unified GET request with JSON parsing - automatically uses blocking or async
    pub fn get_json<T: DeserializeOwned>(&self, endpoint: API, params: Option<String>) -> Result<T> {
        let text = self.get(endpoint, params)?;
        Ok(serde_json::from_str(&text)?)
    }

    /// Blocking GET request with JSON parsing
    pub fn get_json_blocking<T: DeserializeOwned>(&self, endpoint: API, params: Option<String>) -> Result<T> {
        let text = self.get_blocking(endpoint, params)?;
        Ok(serde_json::from_str(&text)?)
    }

    /// Async GET request with JSON parsing
    pub async fn get_json_async<T: DeserializeOwned>(&self, endpoint: API, params: Option<String>) -> Result<T> {
        let text = self.get_async(endpoint, params).await?;
        Ok(serde_json::from_str(&text)?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_client_creation_blocking() {
        let client = Client::new(Some("test_key".into()), Some("test_secret".into()));
        
        assert_eq!(client.api_key, "test_key");
        assert_eq!(client.secret_key, "test_secret");
        assert!(!client.is_async());
    }

    #[test]
    fn test_client_creation_async() {
        let client = Client::new_async(Some("test_key".into()), Some("test_secret".into()));
        
        assert_eq!(client.api_key, "test_key");
        assert_eq!(client.secret_key, "test_secret");
        assert!(client.is_async());
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
    fn test_http_client_enum_blocking() {
        let client = Client::new(None, None);
        
        match client.http_client {
            HttpClient::Blocking(_) => { /* Success */ },
            HttpClient::Async(_) => panic!("Expected blocking client"),
        }
    }

    #[test]
    fn test_http_client_enum_async() {
        let client = Client::new_async(None, None);
        
        match client.http_client {
            HttpClient::Async(_) => { /* Success */ },
            HttpClient::Blocking(_) => panic!("Expected async client"),
        }
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