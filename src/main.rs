use dotenv::dotenv;
use reqwest::Error;
use serde::{Deserialize, Serialize};
use std::env;
use std::{
    collections::HashMap,
    io::{self, Write},
};

#[derive(Debug, Serialize, Deserialize)]
struct TimeSeriesDaily {
    #[serde(rename = "Time Series (Daily)")]
    time_series: HashMap<String, DailyData>,
}

#[derive(Debug, Serialize, Deserialize)]
struct DailyData {
    #[serde(rename = "1. open")]
    open: String,
    #[serde(rename = "2. high")]
    high: String,
    #[serde(rename = "3. low")]
    low: String,
    #[serde(rename = "4. close")]
    close: String,
    #[serde(rename = "5. volume")]
    volume: String,
}

async fn fetch_stock_data(symbol: &str, api_key: &str) -> Result<TimeSeriesDaily, Error> {
    let url = format!(
        "https://www.alphavantage.co/query?function=TIME_SERIES_DAILY&symbol={}&apikey={}",
        symbol, api_key
    );

    let response = reqwest::get(url).await?.json::<TimeSeriesDaily>().await?;
    Ok(response)
}

fn calculate_average_close(data: &TimeSeriesDaily) -> f64 {
    let sum: f64 = data
        .time_series
        .values()
        .map(|d| d.close.parse::<f64>().unwrap_or(0.0))
        .sum();
    let count = data.time_series.len() as f64;

    sum / count
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    let api_key = env::var("ALPHA_VANTAGE_API_KEY").expect("ALPHA_VANTAGE_API_KEY not set");

    println!("Enter the stock symbol you wish to analyze:");
    let mut symbol = String::new();
    io::stdin()
        .read_line(&mut symbol)
        .expect("Failed to read line");
    let symbol = symbol.trim();

    match fetch_stock_data(symbol, &api_key).await {
        Ok(data) => {
            println!("Data for {}: {:#?}", symbol, data);
            let average_close = calculate_average_close(&data);
            println!("Average close for {}: {}", symbol, average_close);
        }
        Err(e) => println!("Error fetching data: {}", e),
    }
}
