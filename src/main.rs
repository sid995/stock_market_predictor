use dotenv::dotenv;
use reqwest::Error;
use serde::{Deserialize, Serialize};
use std::env;

#[derive(Debug, Serialize, Deserialize)]
struct TimeSeriesDaily {
    #[serde(rename = "Time Series (Daily)")]
    time_series: std::collections::HashMap<String, DailyData>,
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
    let symbol = "IBM";

    match fetch_stock_data(symbol, &api_key).await {
        Ok(data) => {
            println!("Data for {}: {:#?}", symbol, data);
            let average_close = calculate_average_close(&data);
            println!("Average close for {}: {}", symbol, average_close);
        }
        Err(e) => println!("Error fetching data: {}", e),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_average_close() {
        let mut time_series = std::collections::HashMap::new();
        time_series.insert(
            "2023-04-01".to_string(),
            DailyData {
                open: "100.0".to_string(),
                high: "110.0".to_string(),
                low: "90.0".to_string(),
                close: "105.0".to_string(),
                volume: "1000".to_string(),
            },
        );
        time_series.insert(
            "2023-04-02".to_string(),
            DailyData {
                open: "105.0".to_string(),
                high: "115.0".to_string(),
                low: "95.0".to_string(),
                close: "110.0".to_string(),
                volume: "1000".to_string(),
            },
        );
        let data = TimeSeriesDaily { time_series };

        let average_close = calculate_average_close(&data);
        let expected_average_close = 107.5; // (105.0 + 110.0) / 2
        assert_eq!(average_close, expected_average_close);
    }
}
