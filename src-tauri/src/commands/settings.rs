use tauri::{State, Manager};
use sqlx::{Pool, Sqlite};
use std::path::PathBuf;

#[tauri::command]
pub async fn set_password(
    password: String,
    pool: State<'_, Pool<Sqlite>>,
) -> Result<(), String> {
    use argon2::{Argon2, PasswordHasher};
    use argon2::password_hash::SaltString;
    use rand::rngs::OsRng;

    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let password_hash = argon2
        .hash_password(password.as_bytes(), &salt)
        .map_err(|e| e.to_string())?
        .to_string();

    sqlx::query("INSERT INTO system_configs (key, value) VALUES ('app_password', ?1) ON CONFLICT(key) DO UPDATE SET value = ?1")
        .bind(password_hash)
        .execute(pool.inner())
        .await
        .map_err(|e| e.to_string())?;

    Ok(())
}

#[tauri::command]
pub async fn verify_password(
    password: String,
    pool: State<'_, Pool<Sqlite>>,
) -> Result<bool, String> {
    use argon2::{Argon2, PasswordHash, PasswordVerifier};

    let result: Option<(String,)> = sqlx::query_as("SELECT value FROM system_configs WHERE key = 'app_password'")
        .fetch_optional(pool.inner())
        .await
        .map_err(|e| e.to_string())?;

    match result {
        Some(row) => {
            let hash = &row.0;
            let parsed_hash = PasswordHash::new(hash).map_err(|e| e.to_string())?;
            Ok(Argon2::default().verify_password(password.as_bytes(), &parsed_hash).is_ok())
        }
        None => Ok(true),
    }
}

#[tauri::command]
pub async fn backup_database(
    app_handle: tauri::AppHandle,
) -> Result<String, String> {
    let app_dir = app_handle
        .path()
        .app_local_data_dir()
        .map_err(|e| e.to_string())?;
    
    let db_path = app_dir.join("stock_manager.db");
    let backup_dir = app_dir.join("backups");
    
    std::fs::create_dir_all(&backup_dir).map_err(|e| e.to_string())?;
    
    let timestamp = chrono::Local::now().format("%Y%m%d_%H%M%S").to_string();
    let backup_path = backup_dir.join(format!("stock_manager_backup_{}.db", timestamp));
    
    std::fs::copy(&db_path, &backup_path).map_err(|e| e.to_string())?;
    
    Ok(backup_path.to_string_lossy().to_string())
}

#[tauri::command]
pub async fn restore_database(
    backup_path: String,
    app_handle: tauri::AppHandle,
) -> Result<(), String> {
    let app_dir = app_handle
        .path()
        .app_local_data_dir()
        .map_err(|e| e.to_string())?;
    
    let db_path = app_dir.join("stock_manager.db");
    let backup = PathBuf::from(backup_path);
    
    if !backup.exists() {
        return Err("备份文件不存在".to_string());
    }
    
    std::fs::copy(&backup, &db_path).map_err(|e| e.to_string())?;
    
    Ok(())
}

#[tauri::command]
pub async fn get_system_config(
    key: String,
    pool: State<'_, Pool<Sqlite>>,
) -> Result<Option<String>, String> {
    let result: Option<(String,)> = sqlx::query_as("SELECT value FROM system_configs WHERE key = ?1")
        .bind(key)
        .fetch_optional(pool.inner())
        .await
        .map_err(|e| e.to_string())?;
    
    Ok(result.map(|r| r.0))
}

#[tauri::command]
pub async fn set_system_config(
    key: String,
    value: String,
    pool: State<'_, Pool<Sqlite>>,
) -> Result<(), String> {
    sqlx::query("INSERT INTO system_configs (key, value) VALUES (?1, ?2) ON CONFLICT(key) DO UPDATE SET value = ?2")
        .bind(key)
        .bind(value)
        .execute(pool.inner())
        .await
        .map_err(|e| e.to_string())?;
    
    Ok(())
}
