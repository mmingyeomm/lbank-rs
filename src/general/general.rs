use crate::client::Client;
use crate::api::{API, Spot};
use crate::errors::Result;

/// General API client for synchronous operations
#[derive(Clone)]
pub struct General {
    pub client: Client,
}

impl General {
    /// Ping the LBank API server
    pub fn ping(&self) -> Result<String> {
        self.client.post(API::Spot(Spot::Ping), None)
    }
}
