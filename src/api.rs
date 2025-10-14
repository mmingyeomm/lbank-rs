use crate::{client::Client, config::Config, general::General};

pub enum api {
    Spot(Spot),
    Savings(Sapi),
    Futures(Futures),
}



pub enum Spot {
    Ping,
    Time,
    ExchangeInfo,
    Depth,
    Trades,
    HistoricalTrades,
    AggTrades,
    Klines,
    AvgPrice,
    Ticker24hr,
    Price,
    BookTicker,
    Order,
    OrderTest,
    OpenOrders,
    AllOrders,
    Oco,
    OrderList,
    AllOrderList,
    OpenOrderList,
    Account,
    MyTrades,
    UserDataStream,
}

pub enum Sapi {
    AllCoins,
    AssetDetail,
    DepositAddress,
    SpotFuturesTransfer,
}

pub enum Futures {
    Ping,
    Time,
    ExchangeInfo,
    Depth,
    Trades,
    HistoricalTrades,
    AggTrades,
    Klines,
    ContinuousKlines,
    IndexPriceKlines,
    MarkPriceKlines,
    PremiumIndex,
    FundingRate,
    Ticker24hr,
    TickerPrice,
    BookTicker,
    AllForceOrders,
    AllOpenOrders,
    AllOrders,
    UserTrades,
    Order,
    PositionRisk,
    Balance,
    PositionSide,
    OpenInterest,
    OpenInterestHist,
    TopLongShortAccountRatio,
    TopLongShortPositionRatio,
    GlobalLongShortAccountRatio,
    TakerlongshortRatio,
    LvtKlines,
    IndexInfo,
    ChangeInitialLeverage,
    MarginType,
    PositionMargin,
    Account,
    OpenOrders,
    UserDataStream,
    Income,
}

pub trait LBank { 
    fn new(api_key: Option<String>, api_secret: Option<String>) -> Self;
    fn new_with_config(api_key: Option<String>, api_secret: Option<String>, config: &Config) -> Self;
    fn new_async(api_key: Option<String>, api_secret: Option<String>) -> Self;
    fn new_async_with_config(api_key: Option<String>, api_secret: Option<String>, config: &Config) -> Self;
    fn set_verbose(&mut self, verbose: bool);
}


impl LBank for General {
    fn new(api_key: Option<String>, secret_key: Option<String>) -> General {
        Self::new_with_config(api_key, secret_key, &Config::default())
    }

    fn new_with_config(
        api_key: Option<String>, 
        secret_key: Option<String>, 
        config: &Config,
    ) -> General {
        General {
            client: Client::new_with_config(api_key, secret_key, config),
        }
    }

    fn new_async(api_key: Option<String>, secret_key: Option<String>) -> General {
        Self::new_async_with_config(api_key, secret_key, &Config::default())
    }

    fn new_async_with_config(
        api_key: Option<String>,
        secret_key: Option<String>,
        config: &Config,
    ) -> General {
        General {
            client: Client::new_async_with_config(api_key, secret_key, config),
        }
    }

    fn set_verbose(&mut self, verbose: bool) {
        self.client.set_verbose(verbose);
    }
}



