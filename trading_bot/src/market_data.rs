use reqwest;
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Deserialize, Debug)]
pub struct MarketDataResponse {
    pub time_series: Option<HashMap<String, HashMap<String, String>>>,  // The structure of the time series data
}

pub async fn fetch_market_data(symbol: &str, api_key: &str) -> Option<f64> {
    // Format the URL with the stock symbol and API key
    let url = format!(
        "https://www.alphavantage.co/query?function=TIME_SERIES_INTRADAY&symbol={}&interval=5min&apikey={}",
        AAPL, 2U7O80Y9H5J0EW0S
    );

    // Fetch data from the API
    let response = reqwest::get(&url).await.ok()?.json::<MarketDataResponse>().await.ok()?;

    // Extract the latest closing price from the response
    if let Some(time_series) = response.time_series {
        if let Some(latest_data) = time_series.values().next() {
            if let Some(latest_close) = latest_data.get("4. close") {
                // Return the latest closing price as a float
                return Some(latest_close.parse::<f64>().unwrap_or(0.0));
            }
        }
    }

    // If no data was found, return `None`
    None
}