
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