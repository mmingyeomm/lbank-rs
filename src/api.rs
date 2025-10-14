use crate::{client::Client, config::Config, general::General};

pub enum API {
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

impl From<API> for String {
    fn from(item: API) -> Self {
        String::from(match item {
            API::Spot(route) => match route {
                Spot::Ping => "/v2/supplement/system_ping.do",
                Spot::Time => "/v2/timestamp.do",
                Spot::ExchangeInfo => "/v2/accuracy.do",
                Spot::Depth => "/v2/depth.do",
                Spot::Trades => "/v2/supplement/trades.do",
                Spot::HistoricalTrades => "/v2/supplement/transaction_history.do",
                Spot::AggTrades => "/v2/supplement/trades.do",
                Spot::Klines => "/v2/kline.do",
                Spot::AvgPrice => "/v2/ticker.do",
                Spot::Ticker24hr => "/v2/ticker/24hr.do",
                Spot::Price => "/v2/supplement/ticker/price.do",
                Spot::BookTicker => "/v2/supplement/ticker/bookTicker.do",
                Spot::Order => "/v2/supplement/create_order.do",
                Spot::OrderTest => "/v2/supplement/create_order_test.do",
                Spot::OpenOrders => "/v2/supplement/orders_info_no_deal.do",
                Spot::AllOrders => "/v2/supplement/orders_info_history.do",
                Spot::Oco => "/v2/supplement/create_order.do",
                Spot::OrderList => "/v2/supplement/orders_info.do",
                Spot::AllOrderList => "/v2/supplement/orders_info_history.do",
                Spot::OpenOrderList => "/v2/supplement/orders_info_no_deal.do",
                Spot::Account => "/v2/supplement/user_info_account.do",
                Spot::MyTrades => "/v2/supplement/transaction_history.do",
                Spot::UserDataStream => "/v2/subscribe/get_key.do",
            },
            API::Savings(route) => match route {
                Sapi::AllCoins => "/v2/supplement/user_info.do",
                Sapi::AssetDetail => "/v2/supplement/asset_detail.do",
                Sapi::DepositAddress => "/v2/supplement/get_deposit_address.do",
                Sapi::SpotFuturesTransfer => "/v2/supplement/withdraw.do",
            },
            API::Futures(route) => match route {
                Futures::Ping => "/fapi/v1/ping",
                Futures::Time => "/fapi/v1/time",
                Futures::ExchangeInfo => "/fapi/v1/exchangeInfo",
                Futures::Depth => "/fapi/v1/depth",
                Futures::Trades => "/fapi/v1/trades",
                Futures::HistoricalTrades => "/fapi/v1/historicalTrades",
                Futures::AggTrades => "/fapi/v1/aggTrades",
                Futures::Klines => "/fapi/v1/klines",
                Futures::ContinuousKlines => "/fapi/v1/continuousKlines",
                Futures::IndexPriceKlines => "/fapi/v1/indexPriceKlines",
                Futures::MarkPriceKlines => "/fapi/v1/markPriceKlines",
                Futures::PremiumIndex => "/fapi/v1/premiumIndex",
                Futures::FundingRate => "/fapi/v1/fundingRate",
                Futures::Ticker24hr => "/fapi/v1/ticker/24hr",
                Futures::TickerPrice => "/fapi/v1/ticker/price",
                Futures::BookTicker => "/fapi/v1/ticker/bookTicker",
                Futures::AllForceOrders => "/fapi/v1/allForceOrders",
                Futures::AllOpenOrders => "/fapi/v1/allOpenOrders",
                Futures::AllOrders => "/fapi/v1/allOrders",
                Futures::UserTrades => "/fapi/v1/userTrades",
                Futures::PositionSide => "/fapi/v1/positionSide/dual",
                Futures::Order => "/fapi/v1/order",
                Futures::PositionRisk => "/fapi/v2/positionRisk",
                Futures::Balance => "/fapi/v2/balance",
                Futures::OpenInterest => "/fapi/v1/openInterest",
                Futures::OpenInterestHist => "/futures/data/openInterestHist",
                Futures::TopLongShortAccountRatio => "/futures/data/topLongShortAccountRatio",
                Futures::TopLongShortPositionRatio => "/futures/data/topLongShortPositionRatio",
                Futures::GlobalLongShortAccountRatio => "/futures/data/globalLongShortAccountRatio",
                Futures::TakerlongshortRatio => "/futures/data/takerlongshortRatio",
                Futures::LvtKlines => "/fapi/v1/lvtKlines",
                Futures::IndexInfo => "/fapi/v1/indexInfo",
                Futures::ChangeInitialLeverage => "/fapi/v1/leverage",
                Futures::MarginType => "/fapi/v1/marginType",
                Futures::PositionMargin => "/fapi/v1/positionMargin",
                Futures::Account => "/fapi/v2/account",
                Futures::OpenOrders => "/fapi/v1/openOrders",
                Futures::UserDataStream => "/fapi/v1/listenKey",
                Futures::Income => "/fapi/v1/income",
            },
        })
    }
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



