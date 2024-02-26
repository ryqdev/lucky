use std::io::Read;
use std::os::unix::raw::off_t;
use std::string::ToString;
use log;
use error_chain::error_chain;
use serde_json::{Value};
use serde::Deserialize;
use crate::trader::BidAsk::{ASK, BID};


error_chain! {
    foreign_links {
        Io(std::io::Error);
        HttpRequest(reqwest::Error);
    }
}


const BTC_USDT: &str = "BTC_USDT";
const BINANCE_PRICE_API: &str = "https://api.binance.com/api/v3/ticker/price?symbol=BTCUSDT";
const BINANCE_ORDER_API: &str = "";

const ERROR_FETCH_DATA: &str = "Error fetching price";
const ERROR_PRICE: f32 = -1.0;

const LOWER_BOUND_PRICE: f32 = 50000.0;
const UPPER_BOUND_PRICE: f32 = 52000.0;

#[derive(Deserialize)]
struct FetchPrice {
    symbol: String,
    price: String
}

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
    // TODO: use polymorphism for bid and ask?
    pub fn bid(&self, order: Order){
        let balance = self.get_balance();
        if balance >= order.volume {
            self.place_order(order);
        }
    }
    pub fn ask(&self, order: Order){
        let position = self.get_position();
        if position >= order.volume {
            self.place_order(order);
        }
    }
    fn isfilled() -> bool{
        false
    }
    fn get_balance(&self) -> f32 {
        return 100.0
    }

    fn get_position(&self) -> f32 {
        return 100.0
    }

    fn place_order(&self, order: Order) {
        // send to banance
        log::info!("send order: {:?}", order)
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

#[derive(PartialEq, Debug)]
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
        loop{
            let market_data = self.fetch_market_data();
            if market_data.price < LOWER_BOUND_PRICE {
                self.send_order(Order{
                    id: "test_bid".to_string(),
                    timestamp: 0,
                    bid_ask: BidAsk::BID,
                    volume: 1.0,
                    symbol: BTC_USDT.to_string(),
                })
            } else if market_data.price > UPPER_BOUND_PRICE {
                self.send_order(Order{
                    id: "test_ask".to_string(),
                    timestamp: 0,
                    bid_ask: BidAsk::ASK,
                    volume: 1.0,
                    symbol: BTC_USDT.to_string(),
                })
            }
        }
    }

    fn fetch_market_data(&self) -> MarketData{
        let body = self.fetch_price().unwrap_or_else(|error| {
            log::error!("fetch price failed, error: {}", error);
            String::from( ERROR_FETCH_DATA)
        });

        if body !=  ERROR_FETCH_DATA {
            log::info!("Fetch: {}", body);
        }

        let price = self.fetch_price_from_json(&body).unwrap_or_else(|error| {
            log::error!("parse json failed, error: {}", error);
            ERROR_PRICE
        } );

        if price != ERROR_PRICE {
            log::info!("Price: {}", price);
        }

        MarketData{
            symbol: BTC_USDT.to_string(),
            timestamp: 0,
            price,
            order_book: OrderBook {
                bid_volume_list: vec![],
                ask_volume_list: vec![],
                price_list: vec![],
            },
        }
    }

    fn fetch_price_from_json(&self, body: &str) -> serde_json::Result<(f32)> {
        let fp: FetchPrice = serde_json::from_str(body)?;
        Ok((fp.price).parse().unwrap())
    }

    fn fetch_price(&self) -> Result<String> {
        let mut res = reqwest::blocking::get("https://api.binance.com/api/v3/ticker/price?symbol=BTCUSDT")?;
        let mut body = String::new();
        res.read_to_string(&mut body)?;

        Ok(body)
    }

    fn send_order(&self, order: Order){
        //TODO: new an OperationEngine whenever send order? Might be not suitable
        let op = OperationEngine::new();
        if order.bid_ask == BID {
            op.bid(order)
        } else if order.bid_ask == ASK{
            op.ask(order)
        } else {
            log::error!("Unknown order type")
        }
    }
}

#[derive(Debug)]
struct Order{
    id: String,
    timestamp: i64,
    bid_ask: BidAsk,
    volume: f32,
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

pub struct Account{
    balance: f32,
    position: f32,
    gain: f32,
    alpha: f32,
    beta:f32,
    sharp:f32,
    max_withdraw: f32,
}

impl Account {
    pub fn new(balance: f32, position: f32) -> Account {
        Account{
            balance,
            position,
            gain: 0.0,
            alpha: 0.0,
            beta: 0.0,
            sharp: 0.0,
            max_withdraw: 0.0,
        }
    }
}