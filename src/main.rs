mod trader;
use trader::{OperationEngine, Strategy, StrategyEngine};

fn main() {
    env_logger::init();

    let strategy = Strategy::new(5.0, 5.0);
    let engine = StrategyEngine::new(strategy);
    engine.run();
}
