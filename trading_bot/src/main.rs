<<<<<<< HEAD

mod market_data;  // Import the market_data module

use market_data::fetch_market_data;  // Use the function from market_data
use tokio;  // Ensure tokio is imported for async operations

#[tokio::main]
async fn main() {
    let api_key = "2U7O80Y9H5J0EW0S";  // Replace with your actual API key
    let symbol = "AAPL";  // Symbol for Apple

    // Call the function to fetch market data and handle the result
    match fetch_market_data(symbol, api_key).await {
        Some(price) => println!("The latest price for {} is ${}", symbol, price),
        None => eprintln!("Failed to fetch market data for {}", symbol),
=======
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
>>>>>>> ada9a5b71949b73a794708fe7a4e5274000da386
    }
}
