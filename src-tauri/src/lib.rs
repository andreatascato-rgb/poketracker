mod commands;
mod db;

use std::sync::Mutex;
use tauri::Manager;

/// Stato condiviso: connessione SQLite (database-storage: data.db in app_data_dir).
/// Usato dai command che accedono al DB (command-with-db: accesso da state, non connessioni ad hoc).
pub struct DbState(pub Mutex<rusqlite::Connection>);

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Ciao, {}! Da Rust.", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            let path = app.path().app_data_dir().map_err(|e| e.to_string())?;
            let db_path = path.join("data.db");
            let conn = db::open_or_create(&db_path)
                .map_err(|e| format!("db init: {}", e))?;
            app.manage(DbState(Mutex::new(conn)));
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            greet,
            commands::profile::get_profiles,
            commands::profile::get_active_profile,
            commands::profile::create_profile,
            commands::profile::set_active_profile,
            commands::profile::rename_profile,
            commands::profile::delete_profile,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
