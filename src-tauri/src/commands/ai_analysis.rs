use crate::models::AiAnalysis;
use serde::{Deserialize, Serialize};
use tauri::State;
use sqlx::{Pool, Sqlite};

#[derive(Debug, Serialize, Deserialize)]
pub struct AnalyzeRequest {
    pub stock_code: String,
    pub stock_name: String,
    pub model_type: String,
}

#[tauri::command]
pub async fn analyze_stock_with_ai(
    request: AnalyzeRequest,
    pool: State<'_, Pool<Sqlite>>,
) -> Result<AiAnalysis, String> {
    let content = format!(
        "## {}({}) AI分析报告\n\n### 基本面分析\n该股票基本面良好，具有稳定的盈利能力和成长性。\n\n### 技术面分析\n当前股价处于合理区间，技术指标显示中性偏乐观。\n\n### 近期热点\n该股票所属行业近期受到市场关注，政策利好不断。\n\n### 风险提示\n市场波动风险、行业竞争加剧风险。\n\n### 投资建议\n建议关注，可适当配置。",
        request.stock_name, request.stock_code
    );

    let result = sqlx::query(
        "INSERT INTO ai_analyses (stock_code, stock_name, content) VALUES (?1, ?2, ?3)"
    )
    .bind(&request.stock_code)
    .bind(&request.stock_name)
    .bind(&content)
    .execute(pool.inner())
    .await
    .map_err(|e| e.to_string())?;

    let id = result.last_insert_rowid();
    
    sqlx::query_as::<_, AiAnalysis>("SELECT * FROM ai_analyses WHERE id = ?1")
        .bind(id)
        .fetch_one(pool.inner())
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_ai_analyses(
    stock_code: Option<String>,
    pool: State<'_, Pool<Sqlite>>,
) -> Result<Vec<AiAnalysis>, String> {
    let mut query = String::from("SELECT * FROM ai_analyses WHERE 1=1");
    
    if let Some(code) = stock_code {
        query.push_str(&format!(" AND stock_code = '{}'", code));
    }
    
    query.push_str(" ORDER BY created_at DESC");
    
    sqlx::query_as::<_, AiAnalysis>(&query)
        .fetch_all(pool.inner())
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn save_api_key(
    model_type: String,
    api_key: String,
    pool: State<'_, Pool<Sqlite>>,
) -> Result<(), String> {
    sqlx::query("INSERT INTO system_configs (key, value) VALUES (?1, ?2) ON CONFLICT(key) DO UPDATE SET value = ?2")
        .bind(format!("api_key_{}", model_type))
        .bind(api_key)
        .execute(pool.inner())
        .await
        .map_err(|e| e.to_string())?;
    
    Ok(())
}

#[tauri::command]
pub async fn get_api_key(
    model_type: String,
    pool: State<'_, Pool<Sqlite>>,
) -> Result<Option<String>, String> {
    let result: Option<(String,)> = sqlx::query_as("SELECT value FROM system_configs WHERE key = ?1")
        .bind(format!("api_key_{}", model_type))
        .fetch_optional(pool.inner())
        .await
        .map_err(|e| e.to_string())?;
    
    Ok(result.map(|r| r.0))
}
