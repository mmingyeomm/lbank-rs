use crate::client::Client;
use crate::api::{API, General};
use crate::errors::Result;

/// Common API client for synchronous operations
#[derive(Clone)]
pub struct Common {
    pub client: Client,
}

impl Common {
    /// Get list of available trading pairs
    pub fn currency_pairs(&self) -> Result<String> {
        self.client.get(API::General(General::CurrencyPairs), None)
    }

    /// Get basic information (price/quantity accuracy, min transaction quantity) for all trading pairs
    pub fn accuracy(&self) -> Result<String> {
        self.client.get(API::General(General::Accuracy), None)
    }

    /// Get withdrawal configurations for assets (deprecated)
    pub fn withdraw_configs(&self) -> Result<String> {
        self.client.get(API::General(General::WithdrawConfigs), None)
    }

    /// Get coin information including deposit/withdrawal configuration for multiple chains
    pub fn asset_configs(&self) -> Result<String> {
        self.client.get(API::General(General::AssetConfigs), None)
    }

    /// Get current server timestamp (used for signature generation)
    pub fn time(&self) -> Result<String> {
        self.client.get(API::General(General::Timestamp), None)
    }
}
