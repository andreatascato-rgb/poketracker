mod commands;
mod db;
mod watcher;

use std::sync::Mutex;
use rusqlite::OptionalExtension;
use tauri::Manager;

/// Stato condiviso: connessione SQLite (database-storage: data.db in app_data_dir).
/// Usato dai command che accedono al DB (command-with-db: accesso da state, non connessioni ad hoc).
pub struct DbState(pub Mutex<rusqlite::Connection>);

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
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
            let watched_paths: Vec<String> = {
                let raw_per: Option<String> = per_profile_key
                    .as_ref()
                    .and_then(|k| {
                        conn.query_row("SELECT value FROM app_state WHERE key = ?1", [k.as_str()], |row| row.get::<_, String>(0))
                            .optional()
                            .ok()
                            .flatten()
                    });
                if let Some(ref s) = raw_per {
                    serde_json::from_str(s).map_err(|e| format!("parse sav_watched_paths (per-profile): {}", e))?
                } else {
                    let raw_legacy: Option<String> = conn
                        .query_row(
                            "SELECT value FROM app_state WHERE key = 'sav_watched_paths'",
                            [],
                            |row| row.get::<_, String>(0),
                        )
                        .optional()
                        .ok()
                        .flatten();
                    if let Some(ref s) = raw_legacy {
                        serde_json::from_str(s).map_err(|e| format!("parse sav_watched_paths (legacy): {}", e))?
                    } else {
                        Vec::new()
                    }
                }
            };
            app.manage(DbState(Mutex::new(conn)));
            let sav_watcher = watcher::SavWatcher::new(app.handle().clone(), watched_paths)
                .map_err(|e| format!("watcher init: {}", e))?;
            app.manage(sav_watcher);
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::profile::get_profiles,
            commands::profile::get_active_profile,
            commands::profile::create_profile,
            commands::profile::set_active_profile,
            commands::profile::rename_profile,
            commands::profile::update_profile,
            commands::profile::update_profile_avatar,
            commands::profile::delete_profile,
            commands::profile::get_pokedex_state,
            commands::profile::get_sav_entries,
            commands::profile::add_sav_entry,
            commands::profile::update_sav_entry,
            commands::profile::remove_sav_entry,
            commands::profile::get_sav_watched_paths,
            commands::profile::set_sav_watched,
            commands::profile::sync_sav_now,
            commands::profile::sync_all_sav_now,
            commands::profile::touch_sav_entry_updated_at,
            commands::profile::sync_pokedex_from_watched_savs_now,
            commands::save_detect::detect_save_game_version,
            commands::save_detect::get_trainer_data,
            commands::export_backup::get_export_dir,
            commands::export_backup::set_export_dir,
            commands::export_backup::open_export_folder,
            commands::error_archive::get_error_archive_entries,
            commands::error_archive::add_error_archive_entry,
            commands::error_archive::remove_error_archive_entry,
        ])
        .run(tauri::generate_context!())
        .map_err(|e| e.into())
}
