use crate::client::AsyncClient;
use crate::api::{API, General};
use crate::errors::Result;

/// Common API client for asynchronous operations
#[derive(Clone)]
pub struct AsyncCommon {
    pub client: AsyncClient,
}

impl AsyncCommon {
    /// Get list of available trading pairs
    pub async fn currency_pairs(&self) -> Result<String> {
        self.client.get(API::General(General::CurrencyPairs), None).await
    }

    /// Get basic information (price/quantity accuracy, min transaction quantity) for all trading pairs
    pub async fn accuracy(&self) -> Result<String> {
        self.client.get(API::General(General::Accuracy), None).await
    }

    /// Get withdrawal configurations for assets (deprecated)
    pub async fn withdraw_configs(&self) -> Result<String> {
        self.client.get(API::General(General::WithdrawConfigs), None).await
    }

    /// Get coin information including deposit/withdrawal configuration for multiple chains
    pub async fn asset_configs(&self) -> Result<String> {
        self.client.get(API::General(General::AssetConfigs), None).await
    }

    /// Get current server timestamp (used for signature generation)
    pub async fn time(&self) -> Result<String> {
        self.client.get(API::General(General::Timestamp), None).await
    }
}

