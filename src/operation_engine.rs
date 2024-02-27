use crate::account::Account;
use crate::common::BidAsk;
use crate::strategy_engine::Order;

#[derive(Debug)]
pub struct OperationEngine {
    exchange_api: String
}

impl OperationEngine{
    pub fn new() -> OperationEngine {
        OperationEngine{exchange_api: crate::common::BINANCE_ORDER_API.to_string()}
    }
    pub fn run(&self) {
        log::info!("OperationEngine is running, configuration is {:?}", &self);
    }
    // TODO: use polymorphism for bid and ask?
    pub fn bid(&self, order: Order, account: &mut Account){
        let balance = account.balance;
        if balance >= order.volume * order.price {
            self.place_order(order, account);
        }
    }
    pub fn ask(&self, order: Order, account: &mut Account){
        let position = account.position;
        if position >= order.volume {
            self.place_order(order, account);
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
    fn place_order(&self, order: crate::strategy_engine::Order, account: &mut Account) {
        if order.bid_ask == BidAsk::BID {
            account.balance -= order.price * order.volume;
            account.position += order.volume;
        } else if order.bid_ask == BidAsk::ASK {
            account.position -= order.volume;
            account.balance += order.price * order.volume;
        }
        log::info!("send order: {:?}", order)
    }
}

