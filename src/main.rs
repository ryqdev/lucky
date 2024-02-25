use std::os::macos::raw::time_t;
use log;

fn main() {
    let operation_system = OperationEngine::new();
    operation_system.run();

    let strategy = Strategy::new(5.0, 5.0);
    let engine = StrategyEngine::new(strategy);
    engine.run();
}

const BTC_USDT: &str = "BTC_USDT";
const ETH_USDT: &str = "ETH_USDT";

enum BidAsk{
    BID,
    ASK
}

struct Strategy{
    risk_rate: f32,
    expected_annual_return: f32
}

impl Strategy{
    pub fn new(r: f32, e: f32) -> Strategy {
        Strategy{ risk_rate: r, expected_annual_return: e }
    }
}

struct StrategyEngine {
    strategy: Strategy
}

impl StrategyEngine {
    pub fn new(s: Strategy) -> StrategyEngine {
        StrategyEngine{
            strategy: s
        }
    }

    pub fn run(){
        log::info!("StrategyEngine is running")
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

struct OperationEngine {}

impl OperationEngine{
    pub fn new() -> OperationEngine {
        OperationEngine{}
    }
    pub fn run() {
        log::info!("OperationEngine is running")
    }
    fn bid(){

    }
    fn ask(){

    }
    fn isfilled() -> bool{
        false
    }
}

struct Order{
    id: String,
    timestamp: time_t,
    bid_ask: BidAsk,
    volume: i32,
    symbol: String
}

struct MarketData {
    symbol: String,
    timestamp: time_t,
    price: f32,
    order_book: OrderBook,
}

struct OrderBook {
    bid_volume_list:Vec<i32>,
    ask_volume_list:Vec<i32>,
    price_list:Vec<i32>
}

