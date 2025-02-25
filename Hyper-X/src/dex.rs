use std::collections::VecDeque;

#[derive(Debug, Clone)]
pub struct Order {
    pub trader: String,
    pub amount: u64,
    pub price: u64,
    pub is_buy: bool,
}

pub struct DEX {
    pub buy_orders: VecDeque<Order>,
    pub sell_orders: VecDeque<Order>,
}

impl DEX {
    pub fn new() -> Self {
        Self {
            buy_orders: VecDeque::new(),
            sell_orders: VecDeque::new(),
        }
    }

    pub fn place_order(&mut self, trader: String, amount: u64, price: u64, is_buy: bool) {
        let order = Order {
            trader,
            amount,
            price,
            is_buy,
        };

        if is_buy {
            self.buy_orders.push_back(order);
        } else {
            self.sell_orders.push_back(order);
        }
    }

    pub fn match_orders(&mut self) {
        while let (Some(buy), Some(sell)) = (self.buy_orders.front(), self.sell_orders.front()) {
            if buy.price >= sell.price {
                println!(
                    "✅ Trade Executed: {} bough
