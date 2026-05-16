use tokio_cron_scheduler::{Job, JobScheduler};
use tauri::AppHandle;

pub async fn setup_scheduler(app_handle: AppHandle) -> Result<(), String> {
    let scheduler = JobScheduler::new().await.map_err(|e| e.to_string())?;
    
    let job = Job::new("0 30 15 * * *", move |_uuid, _l| {
        let app_handle = app_handle.clone();
        tauri::async_runtime::spawn(async move {
            println!("执行每日收盘后分析任务...");
            
            if let Err(e) = super::market_data::fetch_daily_astock_data().await {
                eprintln!("获取行情数据失败: {}", e);
                return;
            }
            
            println!("每日分析任务完成");
        });
    }).map_err(|e| e.to_string())?;
    
    scheduler.add(job).await.map_err(|e| e.to_string())?;
    scheduler.start().await.map_err(|e| e.to_string())?;
    
    Ok(())
}
