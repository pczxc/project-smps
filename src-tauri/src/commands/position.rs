use crate::models::{Transaction, Position, PositionSummary};
use serde::{Deserialize, Serialize};
use tauri::State;
use sqlx::{Pool, Sqlite};

#[derive(Debug, Serialize, Deserialize)]
pub struct AddTransactionRequest {
    pub stock_code: String,
    pub stock_name: String,
    pub type_: String,
    pub quantity: i64,
    pub price: f64,
    pub transaction_date: String,
}

#[tauri::command]
pub async fn add_transaction(
    request: AddTransactionRequest,
    pool: State<'_, Pool<Sqlite>>,
) -> Result<Transaction, String> {
    let amount = request.quantity as f64 * request.price;
    
    let result = sqlx::query(
        "INSERT INTO transactions (stock_code, stock_name, type, quantity, price, amount, transaction_date) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)"
    )
    .bind(&request.stock_code)
    .bind(&request.stock_name)
    .bind(&request.type_)
    .bind(request.quantity)
    .bind(request.price)
    .bind(amount)
    .bind(&request.transaction_date)
    .execute(pool.inner())
    .await
    .map_err(|e| e.to_string())?;

    let id = result.last_insert_rowid();
    
    update_position(&request.stock_code, &request.stock_name, request.quantity, request.price, &request.type_, pool.inner()).await?;
    
    sqlx::query_as::<_, Transaction>("SELECT * FROM transactions WHERE id = ?1")
        .bind(id)
        .fetch_one(pool.inner())
        .await
        .map_err(|e| e.to_string())
}

async fn update_position(
    stock_code: &str,
    stock_name: &str,
    quantity: i64,
    price: f64,
    type_: &str,
    pool: &Pool<Sqlite>,
) -> Result<(), String> {
    let existing: Option<Position> = sqlx::query_as::<_, Position>("SELECT * FROM positions WHERE stock_code = ?1")
        .bind(stock_code)
        .fetch_optional(pool)
        .await
        .map_err(|e| e.to_string())?;

    match existing {
        Some(pos) => {
            if type_ == "buy" {
                let new_quantity = pos.quantity + quantity;
                let new_cost = (pos.cost_price * pos.quantity as f64 + price * quantity as f64) / new_quantity as f64;
                sqlx::query("UPDATE positions SET quantity = ?1, cost_price = ?2, updated_at = CURRENT_TIMESTAMP WHERE stock_code = ?3")
                    .bind(new_quantity)
                    .bind(new_cost)
                    .bind(stock_code)
                    .execute(pool)
                    .await
                    .map_err(|e| e.to_string())?;
            } else {
                let new_quantity = pos.quantity - quantity;
                if new_quantity <= 0 {
                    sqlx::query("DELETE FROM positions WHERE stock_code = ?1")
                        .bind(stock_code)
                        .execute(pool)
                        .await
                        .map_err(|e| e.to_string())?;
                } else {
                    sqlx::query("UPDATE positions SET quantity = ?1, updated_at = CURRENT_TIMESTAMP WHERE stock_code = ?2")
                        .bind(new_quantity)
                        .bind(stock_code)
                        .execute(pool)
                        .await
                        .map_err(|e| e.to_string())?;
                }
            }
        }
        None => {
            if type_ == "buy" {
                sqlx::query("INSERT INTO positions (stock_code, stock_name, quantity, cost_price) VALUES (?1, ?2, ?3, ?4)")
                    .bind(stock_code)
                    .bind(stock_name)
                    .bind(quantity)
                    .bind(price)
                    .execute(pool)
                    .await
                    .map_err(|e| e.to_string())?;
            }
        }
    }

    Ok(())
}

#[tauri::command]
pub async fn get_transactions(
    stock_code: Option<String>,
    start_date: Option<String>,
    end_date: Option<String>,
    pool: State<'_, Pool<Sqlite>>,
) -> Result<Vec<Transaction>, String> {
    let mut query = String::from("SELECT * FROM transactions WHERE 1=1");
    
    if let Some(code) = stock_code {
        query.push_str(&format!(" AND stock_code = '{}'", code));
    }
    if let Some(start) = start_date {
        query.push_str(&format!(" AND date(transaction_date) >= date('{}')", start));
    }
    if let Some(end) = end_date {
        query.push_str(&format!(" AND date(transaction_date) <= date('{}')", end));
    }
    
    query.push_str(" ORDER BY transaction_date DESC");
    
    sqlx::query_as::<_, Transaction>(&query)
        .fetch_all(pool.inner())
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn delete_transaction(
    id: i64,
    pool: State<'_, Pool<Sqlite>>,
) -> Result<(), String> {
    sqlx::query("DELETE FROM transactions WHERE id = ?1")
        .bind(id)
        .execute(pool.inner())
        .await
        .map_err(|e| e.to_string())?;
    
    Ok(())
}

#[tauri::command]
pub async fn get_positions(
    pool: State<'_, Pool<Sqlite>>,
) -> Result<Vec<Position>, String> {
    sqlx::query_as::<_, Position>("SELECT * FROM positions ORDER BY updated_at DESC")
        .fetch_all(pool.inner())
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_position_summary(
    pool: State<'_, Pool<Sqlite>>,
) -> Result<PositionSummary, String> {
    let positions: Vec<Position> = sqlx::query_as::<_, Position>("SELECT * FROM positions")
        .fetch_all(pool.inner())
        .await
        .map_err(|e| e.to_string())?;
    
    let mut total_market_value = 0.0;
    let mut total_cost = 0.0;
    
    for pos in &positions {
        if let Some(latest) = pos.latest_price {
            total_market_value += latest * pos.quantity as f64;
        }
        total_cost += pos.cost_price * pos.quantity as f64;
    }
    
    let total_floating_profit = total_market_value - total_cost;
    let total_return_rate = if total_cost > 0.0 {
        (total_floating_profit / total_cost) * 100.0
    } else {
        0.0
    };
    
    Ok(PositionSummary {
        total_market_value,
        total_floating_profit,
        total_assets: total_market_value,
        total_return_rate,
    })
}
