use std::collections::HashMap;

use crate::models::{Account, Record, Transaction};

// Representing a series of clients, accounts and record history
#[derive(Debug, Default)]
pub struct ClientAccounts {
    // Transaction ids and records
    client_transaction: HashMap<u32, Record>,
    // client ids & associated accounts
    pub client_account: HashMap<u16, Account>,
}

impl ClientAccounts {
    pub fn trans(&mut self, record: Record) {
        let is_withdraw_or_deposit = match record.transaction_type {
            Transaction::Withdrawal => {
                self.client_account
                    .entry(record.client_id)
                    .and_modify(|r| {
                        if let Some(amount) = record.amount {
                            if r.available > amount {
                                r.available -= amount;
                                true
                            } else {
                                println!("Not enough funds");
                                false
                            }
                        } else {
                            false
                        };
                    })
                    .or_default();
                true
            }
            Transaction::Deposit => {
                if let Some(amount) = record.amount {
                    self.client_account
                        .entry(record.client_id)
                        .and_modify(|acc| acc.available += amount)
                        .or_insert(Account::new(amount));
                    true
                } else {
                    println!("No funds provided");
                    false
                }
            }
            Transaction::Dispute => {
                let record = self.client_transaction.get(&record.transaction_id);

                if let Some(record) = record {
                    self.client_account
                        .entry(record.client_id)
                        .and_modify(|acc| {
                            if let Some(amount) = record.amount {
                                acc.available -= amount;
                                acc.held += amount;
                            }
                        })
                        .or_default();
                } else {
                    println!("Referenced disputed transaction doesn't exist");
                };
                false
            }
            Transaction::Resolve => {
                let record = self.client_transaction.get(&record.transaction_id);

                if let Some(record) = record {
                    self.client_account
                        .entry(record.client_id)
                        .and_modify(|acc| {
                            if let Some(amount) = record.amount {
                                acc.available += amount;
                                acc.held -= amount;
                            }
                        })
                        .or_default();
                } else {
                    println!("Referenced resolved transaction doesn't exist");
                };
                false
            }
            Transaction::Chargeback => {
                let record = self.client_transaction.get(&record.transaction_id);
                if let Some(record) = record {
                    self.client_account
                        .entry(record.client_id)
                        .and_modify(|r| {
                            if let Some(amount) = record.amount {
                                r.total -= amount;
                                r.held -= amount;
                                r.locked = true;
                            }
                        })
                        .or_default();
                } else {
                    println!("Referenced chargeback transaction doesn't exist");
                };
                false
            }
        };
        if is_withdraw_or_deposit {
            self.client_transaction
                .insert(record.transaction_id, record);
        }
    }
}
