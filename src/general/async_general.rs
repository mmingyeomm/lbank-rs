use crate::client::AsyncClient;
use crate::api::{API, Spot};
use crate::errors::Result;

/// General API client for asynchronous operations
#[derive(Clone)]
pub struct AsyncGeneral {
    pub client: AsyncClient,
}

impl AsyncGeneral {
    /// Ping the LBank API server
    pub async fn ping(&self) -> Result<String> {
        self.client.post(API::Spot(Spot::Ping), None).await
    }
}
