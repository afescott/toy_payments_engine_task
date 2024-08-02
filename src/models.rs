use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

//An incoming csv row
#[derive(Deserialize, Serialize, Debug)]
pub struct Record {
    #[serde(rename = "type")]
    pub transaction_type: Transaction,
    #[serde(rename = "client id")]
    pub client_id: u16,
    #[serde(rename = "transaction id")]
    pub transaction_id: u32,
    pub amount: Option<Decimal>,
}

//A clients's account
#[derive(Deserialize, Serialize, Copy, Clone, Debug, Default)]
pub struct Account {
    // The total funds that are available for trading, staking, withdrawal, etc.
    // Equal to total - held amounts
    pub available: Decimal,
    // The total funds that are held for dispute.
    // Equal to total - available amounts
    pub held: Decimal,
    // The total funds that are available or held. This should be equal to available + held
    pub total: Decimal,
    // Whether an account is locked
    pub locked: bool,
}

impl Account {
    pub fn new(available: Decimal) -> Self {
        Self {
            available,
            total: Decimal::default(),
            held: Decimal::default(),
            locked: false,
        }
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub enum Transaction {
    #[serde(rename = "withdrawal")]
    Withdrawal,
    #[serde(rename = "deposit")]
    Deposit,
    #[serde(rename = "dispute")]
    Dispute,
    #[serde(rename = "resolve")]
    Resolve,
    #[serde(rename = "chargeback")]
    Chargeback,
}
