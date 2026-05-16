use reqwest;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct StockQuote {
    pub code: String,
    pub name: String,
    pub price: f64,
    pub change_percent: f64,
    pub volume: i64,
}

pub async fn fetch_stock_quote(stock_code: &str) -> Result<StockQuote, String> {
    Ok(StockQuote {
        code: stock_code.to_string(),
        name: "示例股票".to_string(),
        price: 10.0,
        change_percent: 1.5,
        volume: 1000000,
    })
}

pub async fn fetch_daily_astock_data() -> Result<Vec<StockQuote>, String> {
    Ok(vec![
        StockQuote {
            code: "000001".to_string(),
            name: "平安银行".to_string(),
            price: 12.5,
            change_percent: 1.2,
            volume: 5000000,
        },
        StockQuote {
            code: "000002".to_string(),
            name: "万科A".to_string(),
            price: 15.8,
            change_percent: -0.5,
            volume: 3000000,
        },
    ])
}
