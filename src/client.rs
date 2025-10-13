use serde::de::DeserializeOwned;

use crate::config::Config;




#[derive(Clone)]

pub struct Client {
    pub api_key: String,
    pub secret_key: String,
    pub inner_client: reqwest::blocking::Client,
    pub verbose: bool,
    pub async_mode: bool,
}


impl Client {

    pub fn new(api_key: Option<String>, secret_key: Option<String>) -> Self {

        Client {
            api_key: api_key.unwrap_or_default(),
            secret_key: secret_key.unwrap_or_default(),
            inner_client: reqwest::blocking::Client::new(),
            verbose: false,
            async_mode: false,
        }
    }

    pub fn set_verbose(&mut self, verbose: bool) {
        self.verbose = verbose; 
    }

    pub fn enable_async(&mut self) {
        self.async_mode = true;
    }

    pub fn is_async(&self) -> bool {
        self.async_mode
    }


    pub fn get<T: DeserializeOwned>(&self, endpoint: API, request: Option<String>) -> Result<T> {
        let mut url: String = format!("{}{}", self.host, String::from(endpoint));
        if let Some(request) = request {
            if !request.is_empty() {
                url.push_str(format!("?{}", request).as_str());
            }
        }

        let client = &self.inner_client;
        if self.verbose {
            println!("Request URL: {}", url);
        }
        let response = client.get(url.as_str()).send()?;

        self.handler(response)
    }




}