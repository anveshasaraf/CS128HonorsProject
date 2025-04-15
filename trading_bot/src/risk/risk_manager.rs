#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TradeDecision {
    Hold,
    StopLoss,
    TakeProfit,
}

pub struct RiskManager {
    stop_loss_pct: f64,
    take_profit_pct: f64,
}

impl RiskManager {
    pub fn new(stop_loss_pct: f64, take_profit_pct: f64) -> Self {
        RiskManager {
            stop_loss_pct,
            take_profit_pct,
        }
    }

    pub fn evaluate(&self, entry_price: f64, current_price: f64) -> TradeDecision {
        let change_pct = (current_price - entry_price) / entry_price * 100.0;

        if change_pct <= -self.stop_loss_pct {
            TradeDecision::StopLoss
        } else if change_pct >= self.take_profit_pct {
            TradeDecision::TakeProfit
        } else {
            TradeDecision::Hold
        }
    }
}
