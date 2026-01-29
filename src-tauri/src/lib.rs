mod commands;
mod db;
mod watcher;

use std::sync::Mutex;
use rusqlite::OptionalExtension;
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
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_shell::init())
        .setup(|app| {
            let path = app.path().app_data_dir().map_err(|e| e.to_string())?;
            let db_path = path.join("data.db");
            let conn = db::open_or_create(&db_path)
                .map_err(|e| format!("db init: {}", e))?;
            let active_id: Option<String> = conn
                .query_row(
                    "SELECT value FROM app_state WHERE key = 'active_profile_id'",
                    [],
                    |row| row.get::<_, String>(0),
                )
                .optional()
                .map_err(|e| format!("db read active_profile: {}", e))?;
            let per_profile_key = active_id.as_ref().map(|id| format!("sav_watched_paths:{}", id));
            let from_per_profile: Option<Vec<String>> = per_profile_key
                .as_ref()
                .and_then(|k| {
                    conn.query_row("SELECT value FROM app_state WHERE key = ?1", [k.as_str()], |row| row.get::<_, String>(0))
                        .optional()
                        .ok()
                        .flatten()
                })
                .and_then(|s| serde_json::from_str(&s).ok());
            let watched_paths: Vec<String> = from_per_profile
                .or_else(|| {
                    conn.query_row(
                        "SELECT value FROM app_state WHERE key = 'sav_watched_paths'",
                        [],
                        |row| row.get::<_, String>(0),
                    )
                    .optional()
                    .ok()
                    .flatten()
                    .and_then(|s| serde_json::from_str(&s).ok())
                })
                .unwrap_or_default();
            app.manage(DbState(Mutex::new(conn)));
            let sav_watcher = watcher::SavWatcher::new(app.handle().clone(), watched_paths)
                .map_err(|e| format!("watcher init: {}", e))?;
            app.manage(sav_watcher);
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            greet,
            commands::profile::get_profiles,
            commands::profile::get_active_profile,
            commands::profile::create_profile,
            commands::profile::set_active_profile,
            commands::profile::rename_profile,
            commands::profile::update_profile,
            commands::profile::delete_profile,
            commands::profile::get_pokedex_state,
            commands::profile::get_sav_entries,
            commands::profile::add_sav_entry,
            commands::profile::remove_sav_entry,
            commands::profile::get_sav_watched_paths,
            commands::profile::set_sav_watched,
            commands::profile::sync_sav_now,
            commands::profile::sync_all_sav_now,
            commands::profile::touch_sav_entry_updated_at,
            commands::profile::sync_pokedex_from_watched_savs_now,
            commands::save_detect::detect_save_game_version,
            commands::export_backup::get_export_dir,
            commands::export_backup::set_export_dir,
            commands::export_backup::open_export_folder,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
