//! Comandi profilo: lettura/scrittura su SQLite. Vedi command-with-db, database-migrations-standard.

use rusqlite::OptionalExtension;
use serde::{Deserialize, Serialize};
use tauri::{AppHandle, State};

use crate::commands::save_detect;
use crate::watcher::SavWatcher;
use crate::DbState;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Profile {
    pub id: String,
    pub name: String,
    /// Data primo caricamento (ISO 8601, es. "2026-01-27T12:00:00").
    pub created_at: String,
    /// Data ultimo aggiornamento (ISO 8601).
    pub updated_at: String,
}

fn slug(name: &str) -> String {
    let s: String = name
        .chars()
        .map(|c| if c.is_alphanumeric() { c } else { '_' })
        .collect();
    if s.is_empty() {
        "profile".into()
    } else {
        s.to_lowercase()
    }
}

/// Restituisce tutti i profili dal DB.
#[tauri::command]
pub fn get_profiles(state: State<'_, DbState>) -> Result<Vec<Profile>, String> {
    let conn = state.0.lock().map_err(|e| e.to_string())?;
    let mut stmt = conn
        .prepare("SELECT id, name, created_at, updated_at FROM profiles ORDER BY name")
        .map_err(|e| e.to_string())?;
    let rows = stmt
        .query_map([], |row| {
            Ok(Profile {
                id: row.get(0)?,
                name: row.get(1)?,
                created_at: row.get(2)?,
                updated_at: row.get(3)?,
            })
        })
        .map_err(|e| e.to_string())?;
    let mut out = Vec::new();
    for row in rows {
        out.push(row.map_err(|e| e.to_string())?);
    }
    Ok(out)
}

/// Restituisce il profilo attivo (legge da app_state).
#[tauri::command]
pub fn get_active_profile(state: State<'_, DbState>) -> Result<Option<Profile>, String> {
    let conn = state.0.lock().map_err(|e| e.to_string())?;
    let active_id: Option<String> = conn
        .query_row(
            "SELECT value FROM app_state WHERE key = 'active_profile_id'",
            [],
            |row| row.get::<_, String>(0),
        )
        .optional()
        .map_err(|e| e.to_string())?;
    let active_id = match active_id {
        Some(id) => id,
        None => return Ok(None),
    };
    let profile = conn
        .query_row(
            "SELECT id, name, created_at, updated_at FROM profiles WHERE id = ?1",
            [&active_id],
            |row| {
                Ok(Profile {
                    id: row.get(0)?,
                    name: row.get(1)?,
                    created_at: row.get(2)?,
                    updated_at: row.get(3)?,
                })
            },
        )
        .optional()
        .map_err(|e| e.to_string())?;
    Ok(profile)
}

/// Crea un nuovo profilo. id derivato da name (slug); se già esiste restituisce errore.
#[tauri::command]
pub fn create_profile(state: State<'_, DbState>, name: String) -> Result<Profile, String> {
    let name = name.trim();
    if name.is_empty() {
        return Err("Il nome non può essere vuoto.".into());
    }
    let id = slug(name);
    let conn = state.0.lock().map_err(|e| e.to_string())?;
    conn.execute(
        "INSERT INTO profiles (id, name, created_at, updated_at) VALUES (?1, ?2, datetime('now'), datetime('now'))",
        [&id, name],
    )
    .map_err(|e| {
        if let rusqlite::Error::SqliteFailure(ref ext, _) = e {
            if ext.code == rusqlite::ErrorCode::ConstraintViolation {
                return "Esiste già un profilo con questo nome.".to_string();
            }
        }
        e.to_string()
    })?;
    // Se è il primo profilo, lo imposta subito come attivo (non si può "non selezionare" un profilo).
    let count: i64 = conn
        .query_row("SELECT COUNT(*) FROM profiles", [], |row| row.get(0))
        .map_err(|e| e.to_string())?;
    if count == 1 {
        conn.execute(
            "INSERT INTO app_state (key, value) VALUES ('active_profile_id', ?1) ON CONFLICT(key) DO UPDATE SET value = ?1",
            [&id],
        )
        .map_err(|e| e.to_string())?;
    }
    let (created_at, updated_at): (String, String) = conn
        .query_row(
            "SELECT created_at, updated_at FROM profiles WHERE id = ?1",
            [&id],
            |row| Ok((row.get(0)?, row.get(1)?)),
        )
        .map_err(|e| e.to_string())?;
    Ok(Profile {
        id: id.clone(),
        name: name.to_string(),
        created_at,
        updated_at,
    })
}

/// Imposta il profilo attivo. Restituisce errore se l'id non esiste.
#[tauri::command]
pub fn set_active_profile(state: State<'_, DbState>, id: String) -> Result<(), String> {
    let conn = state.0.lock().map_err(|e| e.to_string())?;
    let exists = conn
        .query_row("SELECT 1 FROM profiles WHERE id = ?1", [&id], |row| row.get::<_, i32>(0))
        .optional()
        .map_err(|e| e.to_string())?
        .is_some();
    if !exists {
        return Err("Profilo non trovato.".into());
    }
    conn.execute(
        "INSERT INTO app_state (key, value) VALUES ('active_profile_id', ?1) ON CONFLICT(key) DO UPDATE SET value = ?1",
        [&id],
    )
    .map_err(|e| e.to_string())?;
    Ok(())
}

/// Rinomina un profilo. Restituisce errore se l'id non esiste o il nome è vuoto.
#[tauri::command]
pub fn rename_profile(state: State<'_, DbState>, id: String, new_name: String) -> Result<Profile, String> {
    let new_name = new_name.trim();
    if new_name.is_empty() {
        return Err("Il nome non può essere vuoto.".into());
    }
    let conn = state.0.lock().map_err(|e| e.to_string())?;
    let updated = conn
        .execute(
            "UPDATE profiles SET name = ?1, updated_at = datetime('now') WHERE id = ?2",
            [new_name, &id],
        )
        .map_err(|e| e.to_string())?;
    if updated == 0 {
        return Err("Profilo non trovato.".into());
    }
    let profile = conn
        .query_row(
            "SELECT id, name, created_at, updated_at FROM profiles WHERE id = ?1",
            [&id],
            |row| {
                Ok(Profile {
                    id: row.get(0)?,
                    name: row.get(1)?,
                    created_at: row.get(2)?,
                    updated_at: row.get(3)?,
                })
            },
        )
        .map_err(|e| e.to_string())?;
    Ok(profile)
}

/// Elimina un profilo e tutti i dati ad esso associati.
/// Se si elimina l'ultimo profilo, app_state.active_profile_id viene azzerato e l'app mostra onboarding.
/// Se si elimina il profilo attivo ma restano altri profili, viene attivato il primo disponibile.
/// Quando saranno presenti dati per-profilo (pokedex, salvataggi, cartelle), vanno eliminati qui prima del DELETE.
#[tauri::command]
pub fn delete_profile(state: State<'_, DbState>, id: String) -> Result<(), String> {
    let id = id.trim();
    if id.is_empty() {
        return Err("Id profilo non valido.".into());
    }
    let conn = state.0.lock().map_err(|e| e.to_string())?;
    conn.execute("BEGIN IMMEDIATE", []).map_err(|e| e.to_string())?;
    let outcome: Result<(), String> = (|| {
        let exists = conn
            .query_row("SELECT 1 FROM profiles WHERE id = ?1", [id], |row| row.get::<_, i32>(0))
            .optional()
            .map_err(|e| e.to_string())?
            .is_some();
        if !exists {
            return Err("Profilo non trovato.".into());
        }
        let active_id: Option<String> = conn
            .query_row(
                "SELECT value FROM app_state WHERE key = 'active_profile_id'",
                [],
                |row| row.get::<_, String>(0),
            )
            .optional()
            .map_err(|e| e.to_string())?;
        conn.execute("DELETE FROM profiles WHERE id = ?1", [id])
            .map_err(|e| e.to_string())?;
        let remaining: i64 = conn
            .query_row("SELECT COUNT(*) FROM profiles", [], |row| row.get(0))
            .map_err(|e| e.to_string())?;
        if remaining == 0 {
            conn.execute("DELETE FROM app_state WHERE key = 'active_profile_id'", [])
                .map_err(|e| e.to_string())?;
        } else if active_id.as_deref() == Some(id) {
            let fallback: Option<String> = conn
                .query_row("SELECT id FROM profiles LIMIT 1", [], |row| row.get::<_, String>(0))
                .optional()
                .map_err(|e| e.to_string())?;
            if let Some(fid) = fallback {
                conn.execute(
                    "INSERT INTO app_state (key, value) VALUES ('active_profile_id', ?1) ON CONFLICT(key) DO UPDATE SET value = ?1",
                    [&fid],
                )
                .map_err(|e| e.to_string())?;
            }
        }
        Ok(())
    })();
    match &outcome {
        Ok(()) => {
            conn.execute("COMMIT", []).map_err(|e| e.to_string())?;
        }
        Err(_) => {
            let _ = conn.execute("ROLLBACK", []);
        }
    }
    outcome
}

/// Voce salvataggio: path file, gioco, versione e generazione da detect, data ultimo aggiornamento.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SavEntry {
    pub path: String,
    pub game: String,
    pub version: String,
    /// Generazione (1–9). 0 se non impostata (entry vecchie).
    #[serde(default)]
    pub generation: i32,
    /// Data ultimo aggiornamento (ISO 8601).
    pub updated_at: String,
}

/// Restituisce le voci salvataggio (path, game, version, updated_at). Salvate in app_state come JSON array.
/// Se sav_entries è vuoto e esistono sav_folders, li migra a voci con game/version vuoti.
#[tauri::command]
pub fn get_sav_entries(state: State<'_, DbState>) -> Result<Vec<SavEntry>, String> {
    let conn = state.0.lock().map_err(|e| e.to_string())?;
    let value: Option<String> = conn
        .query_row(
            "SELECT value FROM app_state WHERE key = 'sav_entries'",
            [],
            |row| row.get::<_, String>(0),
        )
        .optional()
        .map_err(|e| e.to_string())?;
    let mut entries: Vec<SavEntry> = match value {
        Some(ref s) => serde_json::from_str(s).unwrap_or_default(),
        None => Vec::new(),
    };
    if entries.is_empty() {
        let legacy: Option<String> = conn
            .query_row(
                "SELECT value FROM app_state WHERE key = 'sav_folders'",
                [],
                |row| row.get::<_, String>(0),
            )
            .optional()
            .map_err(|e| e.to_string())?;
        if let Some(ref s) = legacy {
            let paths: Vec<String> = serde_json::from_str(s).unwrap_or_default();
            let now = chrono::Utc::now().format("%Y-%m-%dT%H:%M:%S").to_string();
            entries = paths
                .into_iter()
                .map(|path| SavEntry {
                    path,
                    game: String::new(),
                    version: String::new(),
                    generation: 0,
                    updated_at: now.clone(),
                })
                .collect();
            let json = serde_json::to_string(&entries).map_err(|e| e.to_string())?;
            conn.execute(
                "INSERT INTO app_state (key, value) VALUES ('sav_entries', ?1) ON CONFLICT(key) DO UPDATE SET value = ?1",
                [&json],
            )
            .map_err(|e| e.to_string())?;
        }
    }
    Ok(entries)
}

/// Aggiunge una voce salvataggio (path file, game, version e generation da detect). Evita duplicati per path.
#[tauri::command]
pub fn add_sav_entry(
    state: State<'_, DbState>,
    path: String,
    game: String,
    version: String,
    generation: Option<i32>,
) -> Result<(), String> {
    let path = path.trim().to_string();
    if path.is_empty() {
        return Ok(());
    }
    let conn = state.0.lock().map_err(|e| e.to_string())?;
    let value: Option<String> = conn
        .query_row(
            "SELECT value FROM app_state WHERE key = 'sav_entries'",
            [],
            |row| row.get::<_, String>(0),
        )
        .optional()
        .map_err(|e| e.to_string())?;
    let mut entries: Vec<SavEntry> = match value {
        Some(ref s) => serde_json::from_str(s).unwrap_or_default(),
        None => Vec::new(),
    };
    if entries.iter().any(|e| e.path == path) {
        return Ok(());
    }
    let updated_at = chrono::Utc::now().format("%Y-%m-%dT%H:%M:%S").to_string();
    let gen = generation.unwrap_or(0);
    entries.push(SavEntry {
        path,
        game: game.trim().to_string(),
        version: version.trim().to_string(),
        generation: gen,
        updated_at,
    });
    let json = serde_json::to_string(&entries).map_err(|e| e.to_string())?;
    conn.execute(
        "INSERT INTO app_state (key, value) VALUES ('sav_entries', ?1) ON CONFLICT(key) DO UPDATE SET value = ?1",
        [&json],
    )
    .map_err(|e| e.to_string())?;
    Ok(())
}

/// Sincronizza ora: valida il file con il sidecar e aggiorna updated_at della voce salvataggio.
/// Usato quando l’utente attiva il watcher (presa dati) o quando il watcher rileva modifica al file.
#[tauri::command]
pub async fn sync_sav_now(state: State<'_, DbState>, app: AppHandle, path: String) -> Result<(), String> {
    let path = path.trim().to_string();
    if path.is_empty() {
        return Err("Percorso non valido.".into());
    }
    eprintln!("[PokeTracker] sync_sav_now avviato: {}", path);
    save_detect::detect_save_game_version(app, path.clone()).await?;
    let conn = state.0.lock().map_err(|e| e.to_string())?;
    let value: Option<String> = conn
        .query_row(
            "SELECT value FROM app_state WHERE key = 'sav_entries'",
            [],
            |row| row.get::<_, String>(0),
        )
        .optional()
        .map_err(|e| e.to_string())?;
    let mut entries: Vec<SavEntry> = match value {
        Some(ref s) => serde_json::from_str(s).unwrap_or_default(),
        None => return Err("Nessun salvataggio configurato per questo percorso.".into()),
    };
    let updated_at = chrono::Utc::now().format("%Y-%m-%dT%H:%M:%S").to_string();
    let mut found = false;
    for e in entries.iter_mut() {
        if e.path == path {
            e.updated_at = updated_at.clone();
            found = true;
            break;
        }
    }
    if !found {
        return Err("Salvataggio non trovato.".into());
    }
    let json = serde_json::to_string(&entries).map_err(|e| e.to_string())?;
    conn.execute(
        "INSERT INTO app_state (key, value) VALUES ('sav_entries', ?1) ON CONFLICT(key) DO UPDATE SET value = ?1",
        [&json],
    )
    .map_err(|e| e.to_string())?;
    eprintln!("[PokeTracker] sync_sav_now completato: {}", path);
    Ok(())
}

/// Restituisce i path dei salvataggi per cui il watcher è attivo (persistito in app_state come JSON array).
#[tauri::command]
pub fn get_sav_watched_paths(state: State<'_, DbState>) -> Result<Vec<String>, String> {
    let conn = state.0.lock().map_err(|e| e.to_string())?;
    let value: Option<String> = conn
        .query_row(
            "SELECT value FROM app_state WHERE key = 'sav_watched_paths'",
            [],
            |row| row.get::<_, String>(0),
        )
        .optional()
        .map_err(|e| e.to_string())?;
    let paths: Vec<String> = match value {
        Some(ref s) => serde_json::from_str(s).unwrap_or_default(),
        None => Vec::new(),
    };
    Ok(paths)
}

/// Attiva o disattiva il watcher per un path. Il path deve essere presente in sav_entries.
#[tauri::command]
pub fn set_sav_watched(
    state: State<'_, DbState>,
    watcher: State<'_, SavWatcher>,
    path: String,
    watched: bool,
) -> Result<(), String> {
    let path = path.trim().to_string();
    if path.is_empty() {
        return Err("Percorso non valido.".into());
    }
    let conn = state.0.lock().map_err(|e| e.to_string())?;
    let entries_value: Option<String> = conn
        .query_row(
            "SELECT value FROM app_state WHERE key = 'sav_entries'",
            [],
            |row| row.get::<_, String>(0),
        )
        .optional()
        .map_err(|e| e.to_string())?;
    let entries: Vec<SavEntry> = match entries_value {
        Some(ref s) => serde_json::from_str(s).unwrap_or_default(),
        None => Vec::new(),
    };
    if !entries.iter().any(|e| e.path == path) {
        return Err("Salvataggio non trovato. Aggiungilo prima di attivare il watcher.".into());
    }
    let watched_value: Option<String> = conn
        .query_row(
            "SELECT value FROM app_state WHERE key = 'sav_watched_paths'",
            [],
            |row| row.get::<_, String>(0),
        )
        .optional()
        .map_err(|e| e.to_string())?;
    let mut paths: Vec<String> = match watched_value {
        Some(ref s) => serde_json::from_str(s).unwrap_or_default(),
        None => Vec::new(),
    };
    if watched {
        if !paths.contains(&path) {
            paths.push(path.clone());
        }
    } else {
        paths.retain(|p| p != &path);
    }
    let json = serde_json::to_string(&paths).map_err(|e| e.to_string())?;
    conn.execute(
        "INSERT INTO app_state (key, value) VALUES ('sav_watched_paths', ?1) ON CONFLICT(key) DO UPDATE SET value = ?1",
        [&json],
    )
    .map_err(|e| e.to_string())?;
    if watched {
        watcher.add(&path).map_err(|e| e.to_string())?;
        eprintln!("[PokeTracker] watcher attivato: {}", path);
    } else {
        watcher.remove(&path).map_err(|e| e.to_string())?;
        eprintln!("[PokeTracker] watcher disattivato: {}", path);
    }
    Ok(())
}

/// Rimuove la voce salvataggio con il path indicato. Rimuove anche il path da sav_watched_paths se presente.
#[tauri::command]
pub fn remove_sav_entry(state: State<'_, DbState>, path: String) -> Result<(), String> {
    let path = path.trim().to_string();
    if path.is_empty() {
        return Ok(());
    }
    let conn = state.0.lock().map_err(|e| e.to_string())?;
    let value: Option<String> = conn
        .query_row(
            "SELECT value FROM app_state WHERE key = 'sav_entries'",
            [],
            |row| row.get::<_, String>(0),
        )
        .optional()
        .map_err(|e| e.to_string())?;
    let mut entries: Vec<SavEntry> = match value {
        Some(ref s) => serde_json::from_str(s).unwrap_or_default(),
        None => return Ok(()),
    };
    let before = entries.len();
    entries.retain(|e| e.path != path);
    if entries.len() == before {
        return Ok(());
    }
    let json = serde_json::to_string(&entries).map_err(|e| e.to_string())?;
    conn.execute(
        "INSERT INTO app_state (key, value) VALUES ('sav_entries', ?1) ON CONFLICT(key) DO UPDATE SET value = ?1",
        [&json],
    )
    .map_err(|e| e.to_string())?;
    // Rimuovi path da sav_watched_paths
    let watched_value: Option<String> = conn
        .query_row(
            "SELECT value FROM app_state WHERE key = 'sav_watched_paths'",
            [],
            |row| row.get::<_, String>(0),
        )
        .optional()
        .map_err(|e| e.to_string())?;
    if let Some(ref s) = watched_value {
        let mut paths: Vec<String> = serde_json::from_str(s).unwrap_or_default();
        paths.retain(|p| p != &path);
        let watched_json = serde_json::to_string(&paths).map_err(|e| e.to_string())?;
        conn.execute(
            "INSERT INTO app_state (key, value) VALUES ('sav_watched_paths', ?1) ON CONFLICT(key) DO UPDATE SET value = ?1",
            [&watched_json],
        )
        .map_err(|e| e.to_string())?;
    }
    Ok(())
}
