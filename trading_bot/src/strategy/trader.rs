use crate::risk::risk_manager::{RiskManager, TradeDecision};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Signal {
    Buy,
    Sell,
    Hold,
}

pub struct Trader {
    pub risk_manager: RiskManager,
}

impl Trader {
    pub fn new(stop_loss_pct: f64, take_profit_pct: f64) -> Self {
        Trader {
            risk_manager: RiskManager::new(stop_loss_pct, take_profit_pct),
        }
    }

    pub fn generate_signal(&self, price: f64, sma: f64, rsi: f64) -> Signal {
        if rsi < 30.0 && price < sma {
            Signal::Buy
        } else if rsi > 70.0 && price > sma {
            Signal::Sell
        } else {
            Signal::Hold
        }
    }

    pub fn evaluate_trade(&self, entry_price: f64, current_price: f64) -> TradeDecision {
        self.risk_manager.evaluate(entry_price, current_price)
    }
}
