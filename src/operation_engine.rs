
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
    pub fn bid(&self, order: crate::strategy_engine::Order){
        let balance = self.get_balance();
        if balance >= order.volume {
            self.place_order(order);
        }
    }
    pub fn ask(&self, order: crate::strategy_engine::Order){
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

    fn place_order(&self, order: crate::strategy_engine::Order) {
        // send to banance
        log::info!("send order: {:?}", order)
    }
}

