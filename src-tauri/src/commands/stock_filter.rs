use crate::models::{FilterConditions, Stock};
use serde::{Deserialize, Serialize};
use tauri::State;
use sqlx::{Pool, Sqlite};

#[derive(Debug, Serialize, Deserialize)]
pub struct FilterResult {
    pub stocks: Vec<Stock>,
}

#[tauri::command]
pub async fn filter_stocks(
    conditions: FilterConditions,
    pool: State<'_, Pool<Sqlite>>,
) -> Result<Vec<Stock>, String> {
    let mut query = String::from("SELECT * FROM stocks WHERE 1=1");
    
    if let Some(min) = conditions.price_min {
        query.push_str(&format!(" AND latest_price >= {}", min));
    }
    if let Some(max) = conditions.price_max {
        query.push_str(&format!(" AND latest_price <= {}", max));
    }
    if let Some(min) = conditions.change_percent_min {
        query.push_str(&format!(" AND change_percent >= {}", min));
    }
    if let Some(max) = conditions.change_percent_max {
        query.push_str(&format!(" AND change_percent <= {}", max));
    }
    if let Some(min) = conditions.volume_min {
        query.push_str(&format!(" AND volume >= {}", min));
    }
    if let Some(max) = conditions.volume_max {
        query.push_str(&format!(" AND volume <= {}", max));
    }
    
    query.push_str(" ORDER BY updated_at DESC");
    
    sqlx::query_as::<_, Stock>(&query)
        .fetch_all(pool.inner())
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn save_filter_conditions(
    name: String,
    conditions: FilterConditions,
    pool: State<'_, Pool<Sqlite>>,
) -> Result<(), String> {
    let conditions_json = serde_json::to_string(&conditions).map_err(|e| e.to_string())?;
    
    sqlx::query("INSERT INTO system_configs (key, value) VALUES (?1, ?2) ON CONFLICT(key) DO UPDATE SET value = ?2")
        .bind(format!("filter_{}", name))
        .bind(conditions_json)
        .execute(pool.inner())
        .await
        .map_err(|e| e.to_string())?;
    
    Ok(())
}

#[tauri::command]
pub async fn get_filter_conditions(
    pool: State<'_, Pool<Sqlite>>,
) -> Result<Vec<(String, FilterConditions)>, String> {
    let configs: Vec<(String, String)> = sqlx::query_as("SELECT key, value FROM system_configs WHERE key LIKE 'filter_%'")
        .fetch_all(pool.inner())
        .await
        .map_err(|e| e.to_string())?;
    
    let mut result = Vec::new();
    for (key, value) in configs {
        let name = key.replace("filter_", "");
        let conditions: FilterConditions = serde_json::from_str(&value).map_err(|e| e.to_string())?;
        result.push((name, conditions));
    }
    
    Ok(result)
}
