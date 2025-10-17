use crate::client::AsyncClient;
use crate::api::{API, Spot};
use crate::errors::Result;
use crate::model::{
    AccountInformation, Balance, Empty, Order, OrderCanceled, TradeHistory, Transaction,
};
use crate::util::{build_signed_request, is_start_time_valid, uuid_spot};
use crate::Account::account::OrderType;
use error_chain::bail;
use std::collections::BTreeMap;

/// Async Account API client for asynchronous operations
#[derive(Clone)]
pub struct AsyncAccount {
    pub client: AsyncClient,
}

struct OrderRequest {
    pub symbol: String,
    pub amount: f64,
    pub price: f64,
    pub order_type: OrderType,
    pub custom_id: Option<String>,
}

impl AsyncAccount {
    /// Get account information including balances
    pub async fn get_account(&self) -> Result<AccountInformation> {
        let parameters: BTreeMap<String, String> = BTreeMap::new();
        let request = build_signed_request(
            parameters,
            &self.client.api_key,
            &self.client.secret_key,
        )?;
        self.client.post_signed(API::Spot(Spot::Account), request).await
    }

    /// Get balance for a single asset
    pub async fn get_balance<S>(&self, asset: S) -> Result<Balance>
    where
        S: Into<String>,
    {
        match self.get_account().await {
            Ok(account) => {
                let cmp_asset = asset.into().to_lowercase();
                
                if let Some(data) = account.data {
                    // Try to parse the asset/free data as a map
                    if let Some(free_val) = data.free {
                        if let Some(free_map) = free_val.as_object() {
                            if let Some(free_amount) = free_map.get(&cmp_asset) {
                                // Get frozen amount
                                let locked = if let Some(freeze_val) = data.freeze {
                                    if let Some(freeze_map) = freeze_val.as_object() {
                                        freeze_map.get(&cmp_asset)
                                            .and_then(|v| v.as_str())
                                            .unwrap_or("0")
                                            .to_string()
                                    } else {
                                        "0".to_string()
                                    }
                                } else {
                                    "0".to_string()
                                };
                                
                                return Ok(Balance {
                                    asset: cmp_asset,
                                    free: free_amount.as_str().unwrap_or("0").to_string(),
                                    locked,
                                });
                            }
                        }
                    }
                }
                bail!("Asset not found");
            }
            Err(e) => Err(e),
        }
    }

    /// Get current open orders for a symbol
    pub async fn get_open_orders<S>(&self, symbol: S) -> Result<Vec<Order>>
    where
        S: Into<String>,
    {
        let mut parameters: BTreeMap<String, String> = BTreeMap::new();
        parameters.insert("symbol".into(), symbol.into());

        let request = build_signed_request(
            parameters,
            &self.client.api_key,
            &self.client.secret_key,
        )?;
        self.client.get_signed(API::Spot(Spot::OpenOrders), Some(request)).await
    }

    /// Get all current open orders
    pub async fn get_all_open_orders(&self) -> Result<Vec<Order>> {
        let parameters: BTreeMap<String, String> = BTreeMap::new();

        let request = build_signed_request(
            parameters,
            &self.client.api_key,
            &self.client.secret_key,
        )?;
        self.client.get_signed(API::Spot(Spot::OpenOrders), Some(request)).await
    }

    /// Check an order's status
    pub async fn order_status<S>(&self, symbol: S, order_id: String) -> Result<Order>
    where
        S: Into<String>,
    {
        let mut parameters: BTreeMap<String, String> = BTreeMap::new();
        parameters.insert("symbol".into(), symbol.into());
        parameters.insert("order_id".into(), order_id);

        let request = build_signed_request(
            parameters,
            &self.client.api_key,
            &self.client.secret_key,
        )?;
        self.client.get_signed(API::Spot(Spot::OrderList), Some(request)).await
    }

    /// Place a test order (sandboxed - validated but not executed)
    pub async fn test_order_status<S>(&self, symbol: S, order_id: String) -> Result<()>
    where
        S: Into<String>,
    {
        let mut parameters: BTreeMap<String, String> = BTreeMap::new();
        parameters.insert("symbol".into(), symbol.into());
        parameters.insert("order_id".into(), order_id);

        let request = build_signed_request(
            parameters,
            &self.client.api_key,
            &self.client.secret_key,
        )?;
        self.client
            .get_signed::<Empty>(API::Spot(Spot::OrderTest), Some(request))
            .await
            .map(|_| ())
    }

    /// Place a LIMIT order - BUY
    pub async fn limit_buy<S, F>(&self, symbol: S, amount: F, price: f64) -> Result<Transaction>
    where
        S: Into<String>,
        F: Into<f64>,
    {
        let buy = OrderRequest {
            symbol: symbol.into(),
            amount: amount.into(),
            price,
            order_type: OrderType::BuyLimit,
            custom_id: None,
        };
        let order = self.build_order(buy);
        let request = build_signed_request(
            order,
            &self.client.api_key,
            &self.client.secret_key,
        )?;
        self.client.post_signed(API::Spot(Spot::Order), request).await
    }

    /// Place a test LIMIT order - BUY (sandboxed)
    pub async fn test_limit_buy<S, F>(&self, symbol: S, amount: F, price: f64) -> Result<()>
    where
        S: Into<String>,
        F: Into<f64>,
    {
        let buy = OrderRequest {
            symbol: symbol.into(),
            amount: amount.into(),
            price,
            order_type: OrderType::BuyLimit,
            custom_id: None,
        };
        let order = self.build_order(buy);
        let request = build_signed_request(
            order,
            &self.client.api_key,
            &self.client.secret_key,
        )?;
        self.client
            .post_signed::<Empty>(API::Spot(Spot::OrderTest), request)
            .await
            .map(|_| ())
    }

    /// Place a LIMIT order - SELL
    pub async fn limit_sell<S, F>(&self, symbol: S, amount: F, price: f64) -> Result<Transaction>
    where
        S: Into<String>,
        F: Into<f64>,
    {
        let sell = OrderRequest {
            symbol: symbol.into(),
            amount: amount.into(),
            price,
            order_type: OrderType::SellLimit,
            custom_id: None,
        };
        let order = self.build_order(sell);
        let request = build_signed_request(
            order,
            &self.client.api_key,
            &self.client.secret_key,
        )?;
        self.client.post_signed(API::Spot(Spot::Order), request).await
    }

    /// Place a test LIMIT order - SELL (sandboxed)
    pub async fn test_limit_sell<S, F>(&self, symbol: S, amount: F, price: f64) -> Result<()>
    where
        S: Into<String>,
        F: Into<f64>,
    {
        let sell = OrderRequest {
            symbol: symbol.into(),
            amount: amount.into(),
            price,
            order_type: OrderType::SellLimit,
            custom_id: None,
        };
        let order = self.build_order(sell);
        let request = build_signed_request(
            order,
            &self.client.api_key,
            &self.client.secret_key,
        )?;
        self.client
            .post_signed::<Empty>(API::Spot(Spot::OrderTest), request)
            .await
            .map(|_| ())
    }

    /// Place a MARKET order - BUY
    pub async fn market_buy<S, F>(&self, symbol: S, amount: F) -> Result<Transaction>
    where
        S: Into<String>,
        F: Into<f64>,
    {
        let buy = OrderRequest {
            symbol: symbol.into(),
            amount: amount.into(),
            price: 0.0,
            order_type: OrderType::BuyMarket,
            custom_id: None,
        };
        let order = self.build_order(buy);
        let request = build_signed_request(
            order,
            &self.client.api_key,
            &self.client.secret_key,
        )?;
        self.client.post_signed(API::Spot(Spot::Order), request).await
    }

    /// Place a test MARKET order - BUY (sandboxed)
    pub async fn test_market_buy<S, F>(&self, symbol: S, amount: F) -> Result<()>
    where
        S: Into<String>,
        F: Into<f64>,
    {
        let buy = OrderRequest {
            symbol: symbol.into(),
            amount: amount.into(),
            price: 0.0,
            order_type: OrderType::BuyMarket,
            custom_id: None,
        };
        let order = self.build_order(buy);
        let request = build_signed_request(
            order,
            &self.client.api_key,
            &self.client.secret_key,
        )?;
        self.client
            .post_signed::<Empty>(API::Spot(Spot::OrderTest), request)
            .await
            .map(|_| ())
    }

    /// Place a MARKET order - SELL
    pub async fn market_sell<S, F>(&self, symbol: S, amount: F) -> Result<Transaction>
    where
        S: Into<String>,
        F: Into<f64>,
    {
        let sell = OrderRequest {
            symbol: symbol.into(),
            amount: amount.into(),
            price: 0.0,
            order_type: OrderType::SellMarket,
            custom_id: None,
        };
        let order = self.build_order(sell);
        let request = build_signed_request(
            order,
            &self.client.api_key,
            &self.client.secret_key,
        )?;
        self.client.post_signed(API::Spot(Spot::Order), request).await
    }

    /// Place a test MARKET order - SELL (sandboxed)
    pub async fn test_market_sell<S, F>(&self, symbol: S, amount: F) -> Result<()>
    where
        S: Into<String>,
        F: Into<f64>,
    {
        let sell = OrderRequest {
            symbol: symbol.into(),
            amount: amount.into(),
            price: 0.0,
            order_type: OrderType::SellMarket,
            custom_id: None,
        };
        let order = self.build_order(sell);
        let request = build_signed_request(
            order,
            &self.client.api_key,
            &self.client.secret_key,
        )?;
        self.client
            .post_signed::<Empty>(API::Spot(Spot::OrderTest), request)
            .await
            .map(|_| ())
    }

    /// Place a custom order with full control
    #[allow(clippy::too_many_arguments)]
    pub async fn custom_order<S, F>(
        &self,
        symbol: S,
        amount: F,
        price: f64,
        order_type: OrderType,
        custom_id: Option<String>,
    ) -> Result<Transaction>
    where
        S: Into<String>,
        F: Into<f64>,
    {
        let order_req = OrderRequest {
            symbol: symbol.into(),
            amount: amount.into(),
            price,
            order_type,
            custom_id,
        };
        let order = self.build_order(order_req);
        let request = build_signed_request(
            order,
            &self.client.api_key,
            &self.client.secret_key,
        )?;
        self.client.post_signed(API::Spot(Spot::Order), request).await
    }

    /// Place a test custom order (sandboxed)
    #[allow(clippy::too_many_arguments)]
    pub async fn test_custom_order<S, F>(
        &self,
        symbol: S,
        amount: F,
        price: f64,
        order_type: OrderType,
        custom_id: Option<String>,
    ) -> Result<()>
    where
        S: Into<String>,
        F: Into<f64>,
    {
        let order_req = OrderRequest {
            symbol: symbol.into(),
            amount: amount.into(),
            price,
            order_type,
            custom_id,
        };
        let order = self.build_order(order_req);
        let request = build_signed_request(
            order,
            &self.client.api_key,
            &self.client.secret_key,
        )?;
        self.client
            .post_signed::<Empty>(API::Spot(Spot::OrderTest), request)
            .await
            .map(|_| ())
    }

    /// Cancel an order by order_id
    pub async fn cancel_order<S>(&self, symbol: S, order_id: String) -> Result<OrderCanceled>
    where
        S: Into<String>,
    {
        let mut parameters: BTreeMap<String, String> = BTreeMap::new();
        parameters.insert("symbol".into(), symbol.into());
        parameters.insert("order_id".into(), order_id);

        let request = build_signed_request(
            parameters,
            &self.client.api_key,
            &self.client.secret_key,
        )?;
        self.client.delete_signed(API::Spot(Spot::CancelOrder), Some(request)).await
    }

    /// Cancel an order by client order ID
    pub async fn cancel_order_with_client_id<S>(
        &self,
        symbol: S,
        custom_id: String,
    ) -> Result<OrderCanceled>
    where
        S: Into<String>,
    {
        let mut parameters: BTreeMap<String, String> = BTreeMap::new();
        parameters.insert("symbol".into(), symbol.into());
        parameters.insert("custom_id".into(), custom_id);

        let request = build_signed_request(
            parameters,
            &self.client.api_key,
            &self.client.secret_key,
        )?;
        self.client
            .delete_signed(API::Spot(Spot::CancelClientOrders), Some(request))
            .await
    }

    /// Place a test cancel order (sandboxed)
    pub async fn test_cancel_order<S>(&self, symbol: S, order_id: String) -> Result<()>
    where
        S: Into<String>,
    {
        let mut parameters: BTreeMap<String, String> = BTreeMap::new();
        parameters.insert("symbol".into(), symbol.into());
        parameters.insert("order_id".into(), order_id);
        
        let request = build_signed_request(
            parameters,
            &self.client.api_key,
            &self.client.secret_key,
        )?;
        self.client
            .delete_signed::<Empty>(API::Spot(Spot::OrderTest), Some(request))
            .await
            .map(|_| ())
    }

    /// Get trade history for a symbol
    pub async fn trade_history<S>(&self, symbol: S) -> Result<Vec<TradeHistory>>
    where
        S: Into<String>,
    {
        let mut parameters: BTreeMap<String, String> = BTreeMap::new();
        parameters.insert("symbol".into(), symbol.into());

        let request = build_signed_request(
            parameters,
            &self.client.api_key,
            &self.client.secret_key,
        )?;
        self.client.get_signed(API::Spot(Spot::MyTrades), Some(request)).await
    }

    /// Get trade history starting from selected time
    pub async fn trade_history_from<S>(&self, symbol: S, start_time: u64) -> Result<Vec<TradeHistory>>
    where
        S: Into<String>,
    {
        if !is_start_time_valid(&start_time) {
            bail!("Start time should be less than the current time");
        }

        let mut parameters: BTreeMap<String, String> = BTreeMap::new();
        parameters.insert("symbol".into(), symbol.into());
        parameters.insert("start_time".into(), start_time.to_string());
        
        let request = build_signed_request(
            parameters,
            &self.client.api_key,
            &self.client.secret_key,
        )?;
        self.client.get_signed(API::Spot(Spot::MyTrades), Some(request)).await
    }

    /// Get trade history from start_time to end_time
    pub async fn trade_history_from_to<S>(
        &self,
        symbol: S,
        start_time: u64,
        end_time: u64,
    ) -> Result<Vec<TradeHistory>>
    where
        S: Into<String>,
    {
        if end_time <= start_time {
            bail!("End time should be greater than start time");
        }
        if !is_start_time_valid(&start_time) {
            bail!("Start time should be less than the current time");
        }
        self.get_trades(symbol, start_time, end_time).await
    }

    /// Internal method to get trades within a time range
    async fn get_trades<S>(&self, symbol: S, start_time: u64, end_time: u64) -> Result<Vec<TradeHistory>>
    where
        S: Into<String>,
    {
        let mut trades = match self.trade_history_from(symbol, start_time).await {
            Ok(trades) => trades,
            Err(e) => return Err(e),
        };
        trades.retain(|trade| trade.time <= end_time as i64);
        Ok(trades)
    }

    /// Build order parameters for LBank API
    fn build_order(&self, order: OrderRequest) -> BTreeMap<String, String> {
        let mut order_parameters: BTreeMap<String, String> = BTreeMap::new();

        order_parameters.insert("symbol".into(), order.symbol);
        order_parameters.insert("type".into(), order.order_type.to_string());
        order_parameters.insert("amount".into(), order.amount.to_string());

        if order.price != 0.0 {
            order_parameters.insert("price".into(), order.price.to_string());
        }

        if let Some(custom_id) = order.custom_id {
            order_parameters.insert("custom_id".into(), custom_id);
        } else {
            let uuid = uuid_spot();
            order_parameters.insert("custom_id".into(), uuid);
        }

        order_parameters
    }
}
