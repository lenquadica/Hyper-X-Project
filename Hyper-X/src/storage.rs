use crate::transactions::Transaction;
use std::collections::VecDeque;

pub struct Blockchain {
    pub ledger: VecDeque<Transaction>,
}

impl Blockchain {
    pub fn new() -> Self {
        Self {
            ledger: VecDeque::new(),
        }
    }

    pub fn add_transaction(&mut self, txn: Transaction) {
        self.ledger.push_back(txn);
    }
}
