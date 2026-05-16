use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use sqlx::Row;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StockPool {
    pub id: i64,
    pub name: String,
    pub description: Option<String>,
    pub created_at: String,
}

impl<'r> sqlx::FromRow<'r, sqlx::sqlite::SqliteRow> for StockPool {
    fn from_row(row: &'r sqlx::sqlite::SqliteRow) -> Result<Self, sqlx::Error> {
        Ok(Self {
            id: row.try_get("id")?,
            name: row.try_get("name")?,
            description: row.try_get("description")?,
            created_at: {
                let dt: Option<DateTime<Utc>> = row.try_get("created_at")?;
                dt.map(|d| d.to_rfc3339()).unwrap_or_default()
            },
        })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Stock {
    pub id: i64,
    pub code: String,
    pub name: String,
    pub pool_id: i64,
    pub latest_price: Option<f64>,
    pub change_percent: Option<f64>,
    pub volume: Option<i64>,
    pub monitored: bool,
    pub updated_at: String,
}

impl<'r> sqlx::FromRow<'r, sqlx::sqlite::SqliteRow> for Stock {
    fn from_row(row: &'r sqlx::sqlite::SqliteRow) -> Result<Self, sqlx::Error> {
        Ok(Self {
            id: row.try_get("id")?,
            code: row.try_get("code")?,
            name: row.try_get("name")?,
            pool_id: row.try_get("pool_id")?,
            latest_price: row.try_get("latest_price")?,
            change_percent: row.try_get("change_percent")?,
            volume: row.try_get("volume")?,
            monitored: row.try_get("monitored")?,
            updated_at: {
                let dt: Option<DateTime<Utc>> = row.try_get("updated_at")?;
                dt.map(|d| d.to_rfc3339()).unwrap_or_default()
            },
        })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MonitorCondition {
    pub id: i64,
    pub stock_id: i64,
    pub condition_type: String,
    pub threshold: f64,
    pub action: String,
    pub created_at: String,
}

impl<'r> sqlx::FromRow<'r, sqlx::sqlite::SqliteRow> for MonitorCondition {
    fn from_row(row: &'r sqlx::sqlite::SqliteRow) -> Result<Self, sqlx::Error> {
        Ok(Self {
            id: row.try_get("id")?,
            stock_id: row.try_get("stock_id")?,
            condition_type: row.try_get("condition_type")?,
            threshold: row.try_get("threshold")?,
            action: row.try_get("action")?,
            created_at: {
                let dt: Option<DateTime<Utc>> = row.try_get("created_at")?;
                dt.map(|d| d.to_rfc3339()).unwrap_or_default()
            },
        })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transaction {
    pub id: i64,
    pub stock_code: String,
    pub stock_name: String,
    pub type_: String,
    pub quantity: i64,
    pub price: f64,
    pub amount: f64,
    pub transaction_date: String,
    pub created_at: String,
}

impl<'r> sqlx::FromRow<'r, sqlx::sqlite::SqliteRow> for Transaction {
    fn from_row(row: &'r sqlx::sqlite::SqliteRow) -> Result<Self, sqlx::Error> {
        Ok(Self {
            id: row.try_get("id")?,
            stock_code: row.try_get("stock_code")?,
            stock_name: row.try_get("stock_name")?,
            type_: {
                let t: String = row.try_get("type")?;
                t
            },
            quantity: row.try_get("quantity")?,
            price: row.try_get("price")?,
            amount: row.try_get("amount")?,
            transaction_date: {
                let dt: Option<DateTime<Utc>> = row.try_get("transaction_date")?;
                dt.map(|d| d.to_rfc3339()).unwrap_or_default()
            },
            created_at: {
                let dt: Option<DateTime<Utc>> = row.try_get("created_at")?;
                dt.map(|d| d.to_rfc3339()).unwrap_or_default()
            },
        })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Position {
    pub id: i64,
    pub stock_code: String,
    pub stock_name: String,
    pub quantity: i64,
    pub cost_price: f64,
    pub latest_price: Option<f64>,
    pub floating_profit: Option<f64>,
    pub profit_rate: Option<f64>,
    pub updated_at: String,
}

impl<'r> sqlx::FromRow<'r, sqlx::sqlite::SqliteRow> for Position {
    fn from_row(row: &'r sqlx::sqlite::SqliteRow) -> Result<Self, sqlx::Error> {
        Ok(Self {
            id: row.try_get("id")?,
            stock_code: row.try_get("stock_code")?,
            stock_name: row.try_get("stock_name")?,
            quantity: row.try_get("quantity")?,
            cost_price: row.try_get("cost_price")?,
            latest_price: row.try_get("latest_price")?,
            floating_profit: row.try_get("floating_profit")?,
            profit_rate: row.try_get("profit_rate")?,
            updated_at: {
                let dt: Option<DateTime<Utc>> = row.try_get("updated_at")?;
                dt.map(|d| d.to_rfc3339()).unwrap_or_default()
            },
        })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TradeAdvice {
    pub id: i64,
    pub stock_code: String,
    pub stock_name: String,
    pub condition: String,
    pub current_price: f64,
    pub advice: String,
    pub created_at: String,
}

impl<'r> sqlx::FromRow<'r, sqlx::sqlite::SqliteRow> for TradeAdvice {
    fn from_row(row: &'r sqlx::sqlite::SqliteRow) -> Result<Self, sqlx::Error> {
        Ok(Self {
            id: row.try_get("id")?,
            stock_code: row.try_get("stock_code")?,
            stock_name: row.try_get("stock_name")?,
            condition: row.try_get("condition")?,
            current_price: row.try_get("current_price")?,
            advice: row.try_get("advice")?,
            created_at: {
                let dt: Option<DateTime<Utc>> = row.try_get("created_at")?;
                dt.map(|d| d.to_rfc3339()).unwrap_or_default()
            },
        })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AiAnalysis {
    pub id: i64,
    pub stock_code: String,
    pub stock_name: String,
    pub content: String,
    pub created_at: String,
}

impl<'r> sqlx::FromRow<'r, sqlx::sqlite::SqliteRow> for AiAnalysis {
    fn from_row(row: &'r sqlx::sqlite::SqliteRow) -> Result<Self, sqlx::Error> {
        Ok(Self {
            id: row.try_get("id")?,
            stock_code: row.try_get("stock_code")?,
            stock_name: row.try_get("stock_name")?,
            content: row.try_get("content")?,
            created_at: {
                let dt: Option<DateTime<Utc>> = row.try_get("created_at")?;
                dt.map(|d| d.to_rfc3339()).unwrap_or_default()
            },
        })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemConfig {
    pub id: i64,
    pub key: String,
    pub value: String,
    pub created_at: String,
    pub updated_at: String,
}

impl<'r> sqlx::FromRow<'r, sqlx::sqlite::SqliteRow> for SystemConfig {
    fn from_row(row: &'r sqlx::sqlite::SqliteRow) -> Result<Self, sqlx::Error> {
        Ok(Self {
            id: row.try_get("id")?,
            key: row.try_get("key")?,
            value: row.try_get("value")?,
            created_at: {
                let dt: Option<DateTime<Utc>> = row.try_get("created_at")?;
                dt.map(|d| d.to_rfc3339()).unwrap_or_default()
            },
            updated_at: {
                let dt: Option<DateTime<Utc>> = row.try_get("updated_at")?;
                dt.map(|d| d.to_rfc3339()).unwrap_or_default()
            },
        })
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FilterConditions {
    pub price_min: Option<f64>,
    pub price_max: Option<f64>,
    pub change_percent_min: Option<f64>,
    pub change_percent_max: Option<f64>,
    pub volume_min: Option<i64>,
    pub volume_max: Option<i64>,
    pub pe_min: Option<f64>,
    pub pe_max: Option<f64>,
    pub turnover_min: Option<f64>,
    pub turnover_max: Option<f64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PositionSummary {
    pub total_market_value: f64,
    pub total_floating_profit: f64,
    pub total_assets: f64,
    pub total_return_rate: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StockInfo {
    pub id: i64,
    pub code: String,
    pub name: String,
    pub market: String, // A, H, etc.
    pub exchange: String, // SSE, SZSE, HKEX
    pub list_date: Option<String>,
    pub industry: Option<String>,
    pub area: Option<String>,
    pub synced_at: String,
}

impl<'r> sqlx::FromRow<'r, sqlx::sqlite::SqliteRow> for StockInfo {
    fn from_row(row: &'r sqlx::sqlite::SqliteRow) -> Result<Self, sqlx::Error> {
        Ok(Self {
            id: row.try_get("id")?,
            code: row.try_get("code")?,
            name: row.try_get("name")?,
            market: row.try_get("market")?,
            exchange: row.try_get("exchange")?,
            list_date: row.try_get("list_date")?,
            industry: row.try_get("industry")?,
            area: row.try_get("area")?,
            synced_at: {
                let dt: Option<DateTime<Utc>> = row.try_get("synced_at")?;
                dt.map(|d| d.to_rfc3339()).unwrap_or_default()
            },
        })
    }
}
