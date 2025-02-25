use std::collections::HashMap;
use std::sync::{Arc, Mutex};

pub struct Tokenomics {
    pub total_supply: u64,
    pub balances: HashMap<String, u64>,
    pub staking: HashMap<String, u64>,
    pub staking_rewards: HashMap<String, u64>,
}

impl Tokenomics {
    pub fn new() -> Self {
        Self {
            total_supply,21_000_000 // 1 Billion Hyper-X tokens
            balances: HashMap::new(),
            staking: HashMap::new(),
            staking_rewards: HashMap::new(),
        }
    }

    pub fn stake_tokens(&mut self, user: &str, amount: u64) -> bool {
        if let Some(balance) = self.balances.get_mut(user) {
            if *balance >= amount {
                *balance -= amount;
                *self.staking.entry(user.to_string()).or_insert(0) += amount;
                return true;
            }
        }
        false
    }

    pub fn claim_rewards(&mut self, user: &str) -> u64 {
        let reward = self.staking.get(user).map(|&staked| staked / 10).unwrap_or(0);
        *self.balances.entry(user.to_string()).or_insert(0) += reward;
        reward
    }

    pub fn distribute_rewards(&mut self) {
        for (user, &staked_amount) in &self.staking {
            let reward = staked_amount / 10;
            *self.balances.entry(user.clone()).or_insert(0) += reward;
        }
    }
}
