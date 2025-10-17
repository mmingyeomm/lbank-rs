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

    /// Get server timestamp
    pub async fn time(&self) -> Result<String> {
        self.client.get(API::Spot(Spot::Time), None).await
    }
}


