use crate::client::Client;
use crate::api::API;
use crate::api::Spot;
use crate::errors::Result;

#[derive(Clone)]
pub struct General {
    pub client: Client,
}

impl General {
    /// Ping the LBank API server
    pub fn ping(&self) -> Result<String> {
        // LBank ping requires POST request
        self.client.post(API::Spot(Spot::Ping), None)
    }
}