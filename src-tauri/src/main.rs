#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod commands;
mod db;
mod models;
mod services;

use sqlx::Pool;
use sqlx::Sqlite;
use tauri::Manager;

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            let app_handle = app.handle();
            let pool = tauri::async_runtime::block_on(async move {
                match db::init_database(&app_handle).await {
                    Ok(p) => {
                        eprintln!("数据库初始化成功");
                        Some(p)
                    }
                    Err(e) => {
                        eprintln!("数据库初始化失败: {}", e);
                        None
                    }
                }
            });
            
            if let Some(pool) = pool {
                app.manage(pool);
            }
            
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::stock_pool::get_stock_pools,
            commands::stock_pool::create_stock_pool,
            commands::stock_pool::update_stock_pool,
            commands::stock_pool::delete_stock_pool,
            commands::stock_pool::add_stock_to_pool,
            commands::stock_pool::remove_stock_from_pool,
            commands::stock_pool::get_stocks_in_pool,
            commands::stock_filter::filter_stocks,
            commands::stock_filter::save_filter_conditions,
            commands::stock_filter::get_filter_conditions,
            commands::analysis::set_monitor_condition,
            commands::analysis::get_monitor_conditions,
            commands::analysis::delete_monitor_condition,
            commands::analysis::get_trade_advices,
            commands::analysis::get_trade_advice_history,
            commands::position::add_transaction,
            commands::position::get_transactions,
            commands::position::delete_transaction,
            commands::position::get_positions,
            commands::position::get_position_summary,
            commands::ai_analysis::analyze_stock_with_ai,
            commands::ai_analysis::get_ai_analyses,
            commands::ai_analysis::save_api_key,
            commands::ai_analysis::get_api_key,
            commands::settings::set_password,
            commands::settings::verify_password,
            commands::settings::backup_database,
            commands::settings::restore_database,
            commands::settings::get_system_config,
            commands::settings::set_system_config,
            commands::stock_info::sync_stock_data,
            commands::stock_info::search_stocks,
            commands::stock_info::get_all_stocks,
            commands::stock_info::get_stock_by_code
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
