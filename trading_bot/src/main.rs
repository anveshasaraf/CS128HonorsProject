mod strategy;
mod risk;

use strategy::trader::{Trader, Signal};
use risk::risk_manager::TradeDecision;

fn main() {
    let trader = Trader::new(5.0, 10.0); // stop-loss: 5%, take-profit: 10%

    let current_price = 95.0;
    let sma = 100.0;
    let rsi = 28.0;

    let signal = trader.generate_signal(current_price, sma, rsi);
    println!("Trade Signal: {:?}", signal);

    if signal == Signal::Buy {
        // Simulate a later market movement
        let future_price = 104.0;
        let decision = trader.evaluate_trade(current_price, future_price);
        println!("Risk Evaluation: {:?}", decision);
    }
}
