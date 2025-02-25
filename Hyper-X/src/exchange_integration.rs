use reqwest::Client;
use serde_json::Value;

pub struct ExchangeIntegration {
    pub client: Client,
    pub binance_api_url: String,
}

impl ExchangeIntegration {
    pub fn new() -> Self {
        Self {
            client: Client::new(),
            binance_api_url: "https://api.binance.com/api/v3/ticker/price?symbol=HYPERXUSDT".to_string(),
        }
    }

    pub async fn get_price_binance(&self) -> Result<f64, reqwest::Error> {
        let response = self.client.get(&self.binance_api_url).send().await?;
        let json: Value = response.json().await?;
        Ok(json["price"].as_str().unwrap().parse::<f64>().unwrap())
    }
}
