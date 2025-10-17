use serde::{Deserialize, Serialize};

/// Empty response for test endpoints
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Empty {}

/// Account information response
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AccountInformation {
    pub result: String,
    pub msg: Option<String>,
    pub error_code: i32,
    pub data: Option<AccountData>,  // Make optional to handle error responses
    pub ts: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AccountData {
    pub info: Option<serde_json::Value>,  // Flexible to handle various response formats
    pub freeze: Option<serde_json::Value>,
    pub asset: Option<serde_json::Value>,
    pub free: Option<serde_json::Value>,
}

/// Balance for a single asset
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Balance {
    pub asset: String,
    pub free: String,
    pub locked: String,
}

/// Order information
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Order {
    pub symbol: String,
    pub order_id: String,
    pub client_order_id: Option<String>,
    pub price: String,
    pub orig_qty: String,
    pub executed_qty: String,
    pub status: String,
    pub time_in_force: Option<String>,
    #[serde(rename = "type")]
    pub order_type: String,
    pub side: String,
    pub time: i64,
}

/// Transaction response after placing an order
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Transaction {
    pub result: String,
    pub msg: Option<String>,
    pub error_code: i32,
    pub data: TransactionData,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TransactionData {
    pub order_id: String,
    pub symbol: String,
    pub price: String,
    pub amount: String,
    #[serde(rename = "type")]
    pub order_type: String,
}

/// Order cancellation response
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct OrderCanceled {
    pub result: String,
    pub msg: Option<String>,
    pub error_code: i32,
    pub data: OrderCanceledData,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct OrderCanceledData {
    pub order_id: String,
    pub symbol: String,
}

/// Trade history
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TradeHistory {
    pub id: i64,
    pub symbol: String,
    pub price: String,
    pub qty: String,
    pub commission: Option<String>,
    pub commission_asset: Option<String>,
    pub time: i64,
    #[serde(rename = "isBuyer")]
    pub is_buyer: bool,
    #[serde(rename = "isMaker")]
    pub is_maker: bool,
}

