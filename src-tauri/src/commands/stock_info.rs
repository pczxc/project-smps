use crate::models::StockInfo;
use crate::services::stock_sync;
use sqlx::Pool;
use sqlx::Sqlite;
use tauri::State;

#[tauri::command]
pub async fn sync_stock_data(pool: State<'_, Pool<Sqlite>>) -> Result<String, String> {
    eprintln!("开始同步股票数据...");

    // 先确保表存在 - 进行额外的检查和创建
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS stock_infos (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            code TEXT NOT NULL UNIQUE,
            name TEXT NOT NULL,
            market TEXT NOT NULL,
            exchange TEXT NOT NULL,
            list_date TEXT,
            industry TEXT,
            area TEXT,
            synced_at DATETIME DEFAULT CURRENT_TIMESTAMP
        );
        "#,
    )
    .execute(pool.inner())
    .await
    .map_err(|e| format!("确保stock_infos表存在失败: {}", e))?;

    eprintln!("stock_infos表已确保存在");

    // 获取A股数据
    let astocks = stock_sync::fetch_all_astocks().await?;
    eprintln!("获取到 {} 只A股", astocks.len());

    // 获取H股数据
    let hstocks = stock_sync::fetch_all_hstocks().await?;
    eprintln!("获取到 {} 只H股", hstocks.len());

    // 合并数据
    let all_stocks: Vec<_> = astocks
        .into_iter()
        .chain(hstocks.into_iter())
        .map(stock_sync::raw_to_stock_info)
        .collect();

    // 保存到数据库
    let mut saved_count = 0;
    for (index, stock) in all_stocks.into_iter().enumerate() {
        match sqlx::query(
            r#"
            INSERT OR REPLACE INTO stock_infos 
            (code, name, market, exchange, list_date, industry, area, synced_at) 
            VALUES (?, ?, ?, ?, ?, ?, ?, CURRENT_TIMESTAMP)
            "#,
        )
        .bind(&stock.code)
        .bind(&stock.name)
        .bind(&stock.market)
        .bind(&stock.exchange)
        .bind(&stock.list_date)
        .bind(&stock.industry)
        .bind(&stock.area)
        .execute(pool.inner())
        .await
        {
            Ok(_) => {
                saved_count += 1;
                if (index % 10 == 0) {
                    eprintln!("已保存 {} 只股票...", saved_count);
                }
            }
            Err(e) => {
                eprintln!("保存股票({})失败: {}", stock.code, e);
                return Err(format!("保存股票失败: {}", e));
            }
        }
    }

    eprintln!("股票数据同步完成，共保存 {} 只股票", saved_count);
    Ok(format!("成功同步 {} 只股票", saved_count))
}

#[tauri::command]
pub async fn search_stocks(
    query: String,
    pool: State<'_, Pool<Sqlite>>,
) -> Result<Vec<StockInfo>, String> {
    let like_query = format!("%{}%", query);

    let stocks = sqlx::query_as::<_, StockInfo>(
        r#"
        SELECT id, code, name, market, exchange, list_date, industry, area, synced_at
        FROM stock_infos
        WHERE code LIKE ? OR name LIKE ?
        ORDER BY code
        LIMIT 50
        "#,
    )
    .bind(&like_query)
    .bind(&like_query)
    .fetch_all(pool.inner())
    .await
    .map_err(|e| format!("搜索股票失败: {}", e))?;

    Ok(stocks)
}

#[tauri::command]
pub async fn get_all_stocks(pool: State<'_, Pool<Sqlite>>) -> Result<Vec<StockInfo>, String> {
    let stocks = sqlx::query_as::<_, StockInfo>(
        r#"
        SELECT id, code, name, market, exchange, list_date, industry, area, synced_at
        FROM stock_infos
        ORDER BY market, code
        "#,
    )
    .fetch_all(pool.inner())
    .await
    .map_err(|e| format!("获取股票列表失败: {}", e))?;

    Ok(stocks)
}

#[tauri::command]
pub async fn get_stock_by_code(
    code: String,
    pool: State<'_, Pool<Sqlite>>,
) -> Result<Option<StockInfo>, String> {
    let stock = sqlx::query_as::<_, StockInfo>(
        r#"
        SELECT id, code, name, market, exchange, list_date, industry, area, synced_at
        FROM stock_infos
        WHERE code = ?
        "#,
    )
    .bind(code)
    .fetch_optional(pool.inner())
    .await
    .map_err(|e| format!("获取股票信息失败: {}", e))?;

    Ok(stock)
}
