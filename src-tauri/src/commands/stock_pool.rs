use crate::models::{StockPool, Stock};
use serde::{Deserialize, Serialize};
use tauri::State;
use sqlx::{Pool, Sqlite};

#[derive(Debug, Serialize, Deserialize)]
pub struct CreatePoolRequest {
    pub name: String,
    pub description: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AddStockRequest {
    pub pool_id: i64,
    pub code: String,
    pub name: String,
}

#[tauri::command]
pub async fn get_stock_pools(pool: State<'_, Pool<Sqlite>>) -> Result<Vec<StockPool>, String> {
    sqlx::query_as::<_, StockPool>("SELECT * FROM stock_pools ORDER BY created_at DESC")
        .fetch_all(pool.inner())
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn create_stock_pool(
    request: CreatePoolRequest,
    pool: State<'_, Pool<Sqlite>>,
) -> Result<StockPool, String> {
    let result = sqlx::query("INSERT INTO stock_pools (name, description) VALUES (?1, ?2)")
        .bind(&request.name)
        .bind(&request.description)
        .execute(pool.inner())
        .await
        .map_err(|e| e.to_string())?;

    let id = result.last_insert_rowid();
    
    sqlx::query_as::<_, StockPool>("SELECT * FROM stock_pools WHERE id = ?1")
        .bind(id)
        .fetch_one(pool.inner())
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn update_stock_pool(
    id: i64,
    name: String,
    description: Option<String>,
    pool: State<'_, Pool<Sqlite>>,
) -> Result<(), String> {
    sqlx::query("UPDATE stock_pools SET name = ?1, description = ?2 WHERE id = ?3")
        .bind(name)
        .bind(description)
        .bind(id)
        .execute(pool.inner())
        .await
        .map_err(|e| e.to_string())?;
    
    Ok(())
}

#[tauri::command]
pub async fn delete_stock_pool(
    id: i64,
    pool: State<'_, Pool<Sqlite>>,
) -> Result<(), String> {
    sqlx::query("DELETE FROM stock_pools WHERE id = ?1")
        .bind(id)
        .execute(pool.inner())
        .await
        .map_err(|e| e.to_string())?;
    
    Ok(())
}

#[tauri::command]
pub async fn add_stock_to_pool(
    request: AddStockRequest,
    pool: State<'_, Pool<Sqlite>>,
) -> Result<Stock, String> {
    let result = sqlx::query("INSERT INTO stocks (code, name, pool_id) VALUES (?1, ?2, ?3)")
        .bind(&request.code)
        .bind(&request.name)
        .bind(request.pool_id)
        .execute(pool.inner())
        .await
        .map_err(|e| e.to_string())?;

    let id = result.last_insert_rowid();
    
    sqlx::query_as::<_, Stock>("SELECT * FROM stocks WHERE id = ?1")
        .bind(id)
        .fetch_one(pool.inner())
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn remove_stock_from_pool(
    stock_id: i64,
    pool: State<'_, Pool<Sqlite>>,
) -> Result<(), String> {
    sqlx::query("DELETE FROM stocks WHERE id = ?1")
        .bind(stock_id)
        .execute(pool.inner())
        .await
        .map_err(|e| e.to_string())?;
    
    Ok(())
}

#[tauri::command]
pub async fn get_stocks_in_pool(
    pool_id: i64,
    pool: State<'_, Pool<Sqlite>>,
) -> Result<Vec<Stock>, String> {
    sqlx::query_as::<_, Stock>("SELECT * FROM stocks WHERE pool_id = ?1 ORDER BY updated_at DESC")
        .bind(pool_id)
        .fetch_all(pool.inner())
        .await
        .map_err(|e| e.to_string())
}
