use log;

const BTC_USDT: &str = "BTC_USDT";
const BINANCE_PRICE_API: &str = "https://api.binance.com/api/v3/ticker/price?symbol=BTCUSDT";
const BINANCE_ORDER_API: &str = "";

#[derive(Debug)]
pub struct OperationEngine {
    exchange_api: String
}

impl OperationEngine{
    pub fn new() -> OperationEngine {
        OperationEngine{exchange_api:BINANCE_ORDER_API.to_string()}
    }
    pub fn run(&self) {
        log::info!("OperationEngine is running, configuration is {:?}", &self);
    }
    fn bid(){

    }
    fn ask(){

    }
    fn isfilled() -> bool{
        false
    }
}

#[derive(Debug)]
pub struct Strategy{
    risk_rate: f32,
    expected_annual_return: f32
}

impl Strategy{
    pub fn new(r: f32, e: f32) -> Strategy {
        Strategy{ risk_rate: r, expected_annual_return: e }
    }
}

enum BidAsk{
    BID,
    ASK
}

pub struct StrategyEngine {
    strategy: Strategy,
    market_data_api: String
}

impl StrategyEngine {
    pub fn new(s: Strategy) -> StrategyEngine {
        StrategyEngine{
            strategy: s,
            market_data_api: BINANCE_PRICE_API.to_string()
        }
    }

    pub fn run(&self){
        log::info!("StrategyEngine is running, the strategy is {:?}", self.strategy);
    }

    fn fetch_market_data() -> MarketData{
        MarketData{
            symbol: BTC_USDT.to_string(),
            timestamp: 0,
            price: 0.0,
            order_book: OrderBook {
                bid_volume_list: vec![],
                ask_volume_list: vec![],
                price_list: vec![],
            },
        }
    }

    fn send_order() -> Order{
        Order{
            id: "test".to_string(),
            timestamp: 0,
            bid_ask: BidAsk::BID,
            volume: 0,
            symbol: BTC_USDT.to_string(),
        }
    }
}

struct Order{
    id: String,
    timestamp: i64,
    bid_ask: BidAsk,
    volume: i32,
    symbol: String
}

struct MarketData {
    symbol: String,
    timestamp: i64,
    price: f32,
    order_book: OrderBook,
}

struct OrderBook {
    bid_volume_list:Vec<i32>,
    ask_volume_list:Vec<i32>,
    price_list:Vec<i32>
}

struct Portfolio {
    balance: f32,
    gain: f32,
    alpha: f32,
    beta:f32,
    sharp:f32,
    max_withdraw: f32,
}
