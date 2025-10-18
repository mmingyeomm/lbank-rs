use crate::client::Client;
use crate::api::{API, Market as MarketAPI};
use crate::errors::Result;

/// Market API client for synchronous operations
#[derive(Clone)]
pub struct Market {
    pub client: Client,
}

impl Market {
    /// Test server connectivity / health check
    pub fn system_ping(&self) -> Result<String> {
        self.client.post(API::Market(MarketAPI::SystemPing), None)
    }

    /// Get order book depth information (asks/bids)
    /// 
    /// # Parameters
    /// * `symbol` - Trading pair (e.g., "eth_btc")
    /// * `size` - Number of depth entries to return (1-200)
    pub fn depth(&self, symbol: &str, size: u32) -> Result<String> {
        let params = format!("symbol={}&size={}", symbol, size);
        self.client.get(API::Market(MarketAPI::Depth), Some(params))
    }

    /// Get latest price for trading pairs
    /// 
    /// # Parameters
    /// * `symbol` - Optional trading pair. If None, returns all pairs
    pub fn price(&self, symbol: Option<&str>) -> Result<String> {
        let params = symbol.map(|s| format!("symbol={}", s));
        self.client.get(API::Market(MarketAPI::Price), params)
    }

    /// Get current best bid/ask prices and quantities
    /// 
    /// # Parameters
    /// * `symbol` - Trading pair (e.g., "lbk_usdt")
    pub fn book_ticker(&self, symbol: &str) -> Result<String> {
        let params = format!("symbol={}", symbol);
        self.client.get(API::Market(MarketAPI::BookTicker), Some(params))
    }

    /// Get 24-hour ticker data (excludes ETF pairs)
    /// 
    /// # Parameters
    /// * `symbol` - Trading pair or "all" for all pairs
    pub fn ticker_24hr(&self, symbol: &str) -> Result<String> {
        let params = format!("symbol={}", symbol);
        self.client.get(API::Market(MarketAPI::Ticker24hr), Some(params))
    }

    /// Get 24-hour ticker data for Leveraged Tokens (ETF) trading pairs
    /// 
    /// # Parameters
    /// * `symbol` - ETF trading pair or "all" for all ETF pairs
    pub fn etf_ticker_24hr(&self, symbol: &str) -> Result<String> {
        let params = format!("symbol={}", symbol);
        self.client.get(API::Market(MarketAPI::EtfTicker24hr), Some(params))
    }

    /// Get list of recent trades
    /// 
    /// # Parameters
    /// * `symbol` - Trading pair
    /// * `size` - Number of trades to return
    /// * `time` - Optional timestamp filter (returns data after this timestamp)
    pub fn trades(&self, symbol: &str, size: u32, time: Option<u64>) -> Result<String> {
        let mut params = format!("symbol={}&size={}", symbol, size);
        if let Some(t) = time {
            params.push_str(&format!("&time={}", t));
        }
        self.client.get(API::Market(MarketAPI::Trades), Some(params))
    }

    /// Get K-line/candlestick data
    /// 
    /// # Parameters
    /// * `symbol` - Trading pair (e.g., "eth_btc")
    /// * `size` - Number of K-line bars (1-2000)
    /// * `kline_type` - K-line interval type:
    ///   - "minute1", "minute5", "minute15", "minute30"
    ///   - "hour1", "hour4", "hour8", "hour12"
    ///   - "day1", "week1", "month1"
    /// * `time` - Timestamp in seconds
    pub fn kline(&self, symbol: &str, size: u32, kline_type: &str, time: u64) -> Result<String> {
        let params = format!(
            "symbol={}&size={}&type={}&time={}",
            symbol, size, kline_type, time
        );
        self.client.get(API::Market(MarketAPI::Kline), Some(params))
    }
}