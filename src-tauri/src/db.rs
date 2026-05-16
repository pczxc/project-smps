use sqlx::{sqlite::{SqlitePoolOptions, SqliteConnectOptions}, Pool, Sqlite};
use tauri::{AppHandle, Manager};
use std::path::PathBuf;

pub async fn init_database(app_handle: &AppHandle) -> Result<Pool<Sqlite>, String> {
    let app_dir = app_handle
        .path()
        .app_local_data_dir()
        .map_err(|e| format!("获取应用目录失败: {}", e))?;
    
    eprintln!("应用数据目录: {:?}", app_dir);
    
    // 确保目录存在
    std::fs::create_dir_all(&app_dir).map_err(|e| format!("创建目录失败: {}", e))?;
    
    let db_path = app_dir.join("stock_manager.db");
    eprintln!("数据库路径: {:?}", db_path);
    
    // 使用 SqliteConnectOptions 直接接受 Path，这更可靠
    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect_with(SqliteConnectOptions::new()
            .filename(&db_path)
            .create_if_missing(true))
        .await
        .map_err(|e| format!("连接数据库失败: {}", e))?;
    
    create_tables(&pool).await?;
    
    eprintln!("数据库初始化成功!");
    
    Ok(pool)
}

async fn create_tables(pool: &Pool<Sqlite>) -> Result<(), String> {
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS stock_pools (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL,
            description TEXT,
            created_at DATETIME DEFAULT CURRENT_TIMESTAMP
        );

        CREATE TABLE IF NOT EXISTS stocks (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            code TEXT NOT NULL,
            name TEXT NOT NULL,
            pool_id INTEGER NOT NULL,
            latest_price REAL,
            change_percent REAL,
            volume INTEGER,
            monitored BOOLEAN DEFAULT 1,
            updated_at DATETIME DEFAULT CURRENT_TIMESTAMP,
            FOREIGN KEY (pool_id) REFERENCES stock_pools(id) ON DELETE CASCADE
        );

        CREATE TABLE IF NOT EXISTS monitor_conditions (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            stock_id INTEGER NOT NULL,
            condition_type TEXT NOT NULL,
            threshold REAL NOT NULL,
            action TEXT NOT NULL,
            created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
            FOREIGN KEY (stock_id) REFERENCES stocks(id) ON DELETE CASCADE
        );

        CREATE TABLE IF NOT EXISTS transactions (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            stock_code TEXT NOT NULL,
            stock_name TEXT NOT NULL,
            type TEXT NOT NULL,
            quantity INTEGER NOT NULL,
            price REAL NOT NULL,
            amount REAL NOT NULL,
            transaction_date DATETIME NOT NULL,
            created_at DATETIME DEFAULT CURRENT_TIMESTAMP
        );

        CREATE TABLE IF NOT EXISTS positions (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            stock_code TEXT NOT NULL UNIQUE,
            stock_name TEXT NOT NULL,
            quantity INTEGER NOT NULL,
            cost_price REAL NOT NULL,
            latest_price REAL,
            floating_profit REAL,
            profit_rate REAL,
            updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
        );

        CREATE TABLE IF NOT EXISTS trade_advices (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            stock_code TEXT NOT NULL,
            stock_name TEXT NOT NULL,
            condition TEXT NOT NULL,
            current_price REAL NOT NULL,
            advice TEXT NOT NULL,
            created_at DATETIME DEFAULT CURRENT_TIMESTAMP
        );

        CREATE TABLE IF NOT EXISTS ai_analyses (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            stock_code TEXT NOT NULL,
            stock_name TEXT NOT NULL,
            content TEXT NOT NULL,
            created_at DATETIME DEFAULT CURRENT_TIMESTAMP
        );

        CREATE TABLE IF NOT EXISTS system_configs (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            key TEXT NOT NULL UNIQUE,
            value TEXT NOT NULL,
            created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
            updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
        );

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

        CREATE INDEX IF NOT EXISTS idx_stocks_pool_id ON stocks(pool_id);
        CREATE INDEX IF NOT EXISTS idx_stocks_code ON stocks(code);
        CREATE INDEX IF NOT EXISTS idx_monitor_conditions_stock_id ON monitor_conditions(stock_id);
        CREATE INDEX IF NOT EXISTS idx_transactions_stock_code ON transactions(stock_code);
        CREATE INDEX IF NOT EXISTS idx_transactions_date ON transactions(transaction_date);
        CREATE INDEX IF NOT EXISTS idx_trade_advices_date ON trade_advices(created_at);
        CREATE INDEX IF NOT EXISTS idx_ai_analyses_stock_code ON ai_analyses(stock_code);
        CREATE INDEX IF NOT EXISTS idx_stock_infos_code ON stock_infos(code);
        CREATE INDEX IF NOT EXISTS idx_stock_infos_name ON stock_infos(name);
        CREATE INDEX IF NOT EXISTS idx_stock_infos_market ON stock_infos(market);
        "#,
    )
    .execute(pool)
    .await
    .map_err(|e| e.to_string())?;

    Ok(())
}

pub fn get_db_path(app_handle: &AppHandle) -> Result<PathBuf, String> {
    let app_dir = app_handle
        .path()
        .app_local_data_dir()
        .map_err(|e| e.to_string())?;
    Ok(app_dir.join("stock_manager.db"))
}
