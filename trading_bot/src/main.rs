
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
    }
}
