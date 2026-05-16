use crate::models::{MonitorCondition, TradeAdvice};
use serde::{Deserialize, Serialize};
use tauri::State;
use sqlx::{Pool, Sqlite};

#[derive(Debug, Serialize, Deserialize)]
pub struct SetMonitorRequest {
    pub stock_id: i64,
    pub condition_type: String,
    pub threshold: f64,
    pub action: String,
}

#[tauri::command]
pub async fn set_monitor_condition(
    request: SetMonitorRequest,
    pool: State<'_, Pool<Sqlite>>,
) -> Result<MonitorCondition, String> {
    let result = sqlx::query(
        "INSERT INTO monitor_conditions (stock_id, condition_type, threshold, action) VALUES (?1, ?2, ?3, ?4)"
    )
    .bind(request.stock_id)
    .bind(&request.condition_type)
    .bind(request.threshold)
    .bind(&request.action)
    .execute(pool.inner())
    .await
    .map_err(|e| e.to_string())?;

    let id = result.last_insert_rowid();
    
    sqlx::query_as::<_, MonitorCondition>("SELECT * FROM monitor_conditions WHERE id = ?1")
        .bind(id)
        .fetch_one(pool.inner())
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_monitor_conditions(
    stock_id: i64,
    pool: State<'_, Pool<Sqlite>>,
) -> Result<Vec<MonitorCondition>, String> {
    sqlx::query_as::<_, MonitorCondition>("SELECT * FROM monitor_conditions WHERE stock_id = ?1")
        .bind(stock_id)
        .fetch_all(pool.inner())
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn delete_monitor_condition(
    id: i64,
    pool: State<'_, Pool<Sqlite>>,
) -> Result<(), String> {
    sqlx::query("DELETE FROM monitor_conditions WHERE id = ?1")
        .bind(id)
        .execute(pool.inner())
        .await
        .map_err(|e| e.to_string())?;
    
    Ok(())
}

#[tauri::command]
pub async fn get_trade_advices(
    pool: State<'_, Pool<Sqlite>>,
) -> Result<Vec<TradeAdvice>, String> {
    let today = chrono::Local::now().format("%Y-%m-%d").to_string();
    sqlx::query_as::<_, TradeAdvice>(
        "SELECT * FROM trade_advices WHERE date(created_at) = date(?1) ORDER BY created_at DESC"
    )
    .bind(today)
    .fetch_all(pool.inner())
    .await
    .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_trade_advice_history(
    start_date: String,
    end_date: String,
    pool: State<'_, Pool<Sqlite>>,
) -> Result<Vec<TradeAdvice>, String> {
    sqlx::query_as::<_, TradeAdvice>(
        "SELECT * FROM trade_advices WHERE date(created_at) BETWEEN date(?1) AND date(?2) ORDER BY created_at DESC"
    )
    .bind(start_date)
    .bind(end_date)
    .fetch_all(pool.inner())
    .await
    .map_err(|e| e.to_string())
}
