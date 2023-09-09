use serde::{Deserialize, Serialize};
use std::collections::VecDeque;

#[derive(Debug, Deserialize, Serialize,Clone)]
pub struct TickerEvent {
    
    pub s: String,
    pub b: String,
    pub B: String,
    pub a: String,
    pub A: String,
    pub T: i64,
    pub E: i64,
}

#[derive(Debug)]
pub struct RollingOHLC {
    pub symbol: String,
    pub window_size: i64,
    pub data: VecDeque<TickerEvent>,
}

impl RollingOHLC {
    pub fn new(symbol: &str, window_size: i64) -> Self {
        Self {
            symbol: symbol.to_string(),
            window_size,
            data: VecDeque::new(),
        }
    }

    pub fn add_event(&mut self, event: TickerEvent) {
        self.data.push_back(event);
        self.trim_old_data();
    }

    pub fn calculate_ohlc(&self) -> Option<(i64, f64, f64, f64, f64)> {
        if self.data.is_empty() {
            return None;
        }

        let now = self.data.back().unwrap().T;
        let start_time = now - self.window_size * 1000;
        let mut min_price = f64::MAX;
        let mut max_price = f64::MIN;
        let mut open = 0.0;
        let mut close = 0.0;

        for event in &self.data {
            if event.T < start_time {
                continue;
            }

            // Parse the String values into f64
            let price = event.b.parse::<f64>().unwrap_or(0.0);

            if price < min_price {
                min_price = price;
            }

            if price > max_price {
                max_price = price;
            }

            if open == 0.0 {
                open = price;
            }

            close = price;
        }

        Some((now, open, max_price, min_price, close))
    }

    pub fn trim_old_data(&mut self) {
        let now = self.data.back().map(|event| event.T).unwrap_or(0);
        let cutoff = now - self.window_size * 1000;
        self.data.retain(|event| event.T >= cutoff);
    }
}
