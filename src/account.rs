use rust_decimal::Decimal;

pub struct Account{
    pub(crate) balance: Decimal,
    pub(crate) position: Decimal,
    gain:  Decimal,
    alpha:  Decimal,
    beta: Decimal,
    sharp: Decimal,
    max_withdraw:  Decimal,
}

impl Account {
    pub fn new(balance:  Decimal, position:  Decimal) -> Account {
        Account{
            balance,
            position,
            gain: Default::default(),
            alpha: Default::default(),
            beta: Default::default(),
            sharp: Default::default(),
            max_withdraw: Default::default(),
        }
    }
}