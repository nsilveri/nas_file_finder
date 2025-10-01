use serde::{Deserialize, Serialize};
use tokio_postgres::{NoTls, Error};
use std::process::Command;

#[derive(Debug, Serialize, Deserialize)]
pub struct FileResult {
    pub id: i32,
    pub filename: String,
    pub directory: String,
    pub last_modified: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Configuration {
    pub id: i32,
    pub key: String,
    pub value: String,
    pub description: Option<String>,
    pub updated_at: String,
}

#[derive(Debug, Deserialize)]
pub struct DatabaseConfig {
    pub host: String,
    pub port: u16,
    pub user: String,
    pub password: String,
    pub database: String,
}

#[tauri::command]
pub fn greet(name: String) -> String {
  format!("Hello, {}!", name)
}

#[tauri::command]
pub async fn test_connection(db_config: DatabaseConfig) -> Result<bool, String> {
    // Build connection string from config
    let connection_string = format!(
        "host={} port={} user={} password={} dbname={}",
        db_config.host,
        db_config.port,
        db_config.user,
        db_config.password,
        db_config.database
    );
    
    // Try to connect to the database
    match tokio_postgres::connect(&connection_string, NoTls).await {
        Ok((client, connection)) => {
            // Spawn the connection in a separate task
            tokio::spawn(async move {
                let _ = connection.await;
            });
            
            // Try a simple query to ensure the connection is working
            match client.query("SELECT 1", &[]).await {
                Ok(_) => Ok(true),
                Err(e) => Err(format!("Query test failed: {}", e))
            }
        }
        Err(e) => Err(format!("Connection failed: {}", e))
    }
}

#[tauri::command]
pub async fn search_files(search_terms: Vec<String>, db_config: DatabaseConfig, use_and_logic: bool, search_in_filenames: bool) -> Result<Vec<FileResult>, String> {
    // Build connection string from config
    let connection_string = format!(
        "host={} port={} user={} password={} dbname={}",
        db_config.host,
        db_config.port,
        db_config.user,
        db_config.password,
        db_config.database
    );
    
    // Connect to the database
    let (client, connection) = tokio_postgres::connect(&connection_string, NoTls)
        .await
        .map_err(|e| format!("Failed to connect to database: {}", e))?;

    // Spawn the connection in a separate task
    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("Connection error: {}", e);
        }
    });

    if use_and_logic {
        // AND logic: field must contain ALL search terms
        let mut where_conditions = Vec::new();
        let mut params: Vec<String> = Vec::new();
        
        let field_name = if search_in_filenames { "filename" } else { "directory" };
        
        for (i, term) in search_terms.iter().enumerate() {
            where_conditions.push(format!("{} ILIKE ${}", field_name, i + 1));
            params.push(format!("%{}%", term));
        }
        
        let where_clause = where_conditions.join(" AND ");
        let query = format!(
            "SELECT id, filename, directory, last_modified FROM files WHERE {} LIMIT 100",
            where_clause
        );
        
        // Convert params to the correct type for query
        let param_refs: Vec<&(dyn tokio_postgres::types::ToSql + Sync)> = params
            .iter()
            .map(|p| p as &(dyn tokio_postgres::types::ToSql + Sync))
            .collect();
        
        let rows = client
            .query(&query, &param_refs[..])
            .await
            .map_err(|e| format!("Query error: {}", e))?;

        let mut results = Vec::new();
        for row in rows {
            let file_result = FileResult {
                id: row.get(0),
                filename: row.get(1),
                directory: row.get(2),
                last_modified: row.get::<_, Option<chrono::NaiveDateTime>>(3)
                    .map(|dt| dt.format("%Y-%m-%d %H:%M:%S").to_string()),
            };
            results.push(file_result);
        }

        Ok(results)
    } else {
        // OR logic: field can contain ANY of the search terms
        let mut results = Vec::new();
        
        let field_name = if search_in_filenames { "filename" } else { "directory" };
        let query = format!(
            "SELECT id, filename, directory, last_modified FROM files WHERE {} ILIKE $1 LIMIT 100",
            field_name
        );

        for term in search_terms {
            let pattern = format!("%{}%", term);
            
            let rows = client
                .query(&query, &[&pattern])
                .await
                .map_err(|e| format!("Query error: {}", e))?;

            for row in rows {
                let file_result = FileResult {
                    id: row.get(0),
                    filename: row.get(1),
                    directory: row.get(2),
                    last_modified: row.get::<_, Option<chrono::NaiveDateTime>>(3)
                        .map(|dt| dt.format("%Y-%m-%d %H:%M:%S").to_string()),
                };
                results.push(file_result);
            }
        }

        // Remove duplicates based on id
        results.sort_by_key(|f| f.id);
        results.dedup_by_key(|f| f.id);

        Ok(results)
    }
}

#[tauri::command]
pub fn open_directory(directory_path: String) -> Result<(), String> {
    // Open directory in Windows Explorer
    #[cfg(target_os = "windows")]
    {
        Command::new("explorer.exe")
            .arg(&directory_path)
            .spawn()
            .map_err(|e| format!("Failed to open directory: {}", e))?;
    }
    
    // Open directory in macOS Finder
    #[cfg(target_os = "macos")]
    {
        Command::new("open")
            .arg(&directory_path)
            .spawn()
            .map_err(|e| format!("Failed to open directory: {}", e))?;
    }
    
    // Open directory in Linux file manager
    #[cfg(target_os = "linux")]
    {
        Command::new("xdg-open")
            .arg(&directory_path)
            .spawn()
            .map_err(|e| format!("Failed to open directory: {}", e))?;
    }
    
    Ok(())
}

#[tauri::command]
pub async fn get_configurations(db_config: DatabaseConfig) -> Result<Vec<Configuration>, String> {
    // Build connection string from config
    let connection_string = format!(
        "host={} port={} user={} password={} dbname={}",
        db_config.host,
        db_config.port,
        db_config.user,
        db_config.password,
        db_config.database
    );
    
    // Connect to the database
    let (client, connection) = tokio_postgres::connect(&connection_string, NoTls)
        .await
        .map_err(|e| format!("Failed to connect to database: {}", e))?;

    // Spawn the connection in a separate task
    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("Connection error: {}", e);
        }
    });

    let query = "SELECT id, key, value, description, updated_at FROM configurations ORDER BY key";
    let rows = client
        .query(query, &[])
        .await
        .map_err(|e| format!("Query error: {}", e))?;

    let mut results = Vec::new();
    for row in rows {
        let config = Configuration {
            id: row.get(0),
            key: row.get(1),
            value: row.get(2),
            description: row.get(3),
            updated_at: row.get::<_, chrono::NaiveDateTime>(4)
                .format("%Y-%m-%d %H:%M:%S").to_string(),
        };
        results.push(config);
    }

    Ok(results)
}

#[tauri::command]
pub async fn update_configuration(db_config: DatabaseConfig, key: String, value: String) -> Result<(), String> {
    // Build connection string from config
    let connection_string = format!(
        "host={} port={} user={} password={} dbname={}",
        db_config.host,
        db_config.port,
        db_config.user,
        db_config.password,
        db_config.database
    );
    
    // Connect to the database
    let (client, connection) = tokio_postgres::connect(&connection_string, NoTls)
        .await
        .map_err(|e| format!("Failed to connect to database: {}", e))?;

    // Spawn the connection in a separate task
    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("Connection error: {}", e);
        }
    });

    let query = "UPDATE configurations SET value = $1, updated_at = CURRENT_TIMESTAMP WHERE key = $2";
    client
        .execute(query, &[&value, &key])
        .await
        .map_err(|e| format!("Update error: {}", e))?;

    Ok(())
}