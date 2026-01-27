//! Comandi profilo: lettura/scrittura su SQLite. Vedi command-with-db, database-migrations-standard.

use rusqlite::OptionalExtension;
use serde::{Deserialize, Serialize};
use tauri::State;

use crate::DbState;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Profile {
    pub id: String,
    pub name: String,
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
        .prepare("SELECT id, name FROM profiles ORDER BY name")
        .map_err(|e| e.to_string())?;
    let rows = stmt
        .query_map([], |row| {
            Ok(Profile {
                id: row.get(0)?,
                name: row.get(1)?,
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
            "SELECT id, name FROM profiles WHERE id = ?1",
            [&active_id],
            |row| Ok(Profile { id: row.get(0)?, name: row.get(1)? }),
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
    conn.execute("INSERT INTO profiles (id, name) VALUES (?1, ?2)", [&id, name])
        .map_err(|e| {
            if let rusqlite::Error::SqliteFailure(ref ext, _) = e {
                if ext.code == rusqlite::ErrorCode::ConstraintViolation {
                    return "Esiste già un profilo con questo nome.".to_string();
                }
            }
            e.to_string()
        })?;
    Ok(Profile {
        id: id.clone(),
        name: name.to_string(),
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
        .execute("UPDATE profiles SET name = ?1 WHERE id = ?2", [new_name, &id])
        .map_err(|e| e.to_string())?;
    if updated == 0 {
        return Err("Profilo non trovato.".into());
    }
    let profile = conn
        .query_row(
            "SELECT id, name FROM profiles WHERE id = ?1",
            [&id],
            |row| Ok(Profile { id: row.get(0)?, name: row.get(1)? }),
        )
        .map_err(|e| e.to_string())?;
    Ok(profile)
}

/// Elimina un profilo. Non permette di eliminare l'ultimo. Se si elimina il profilo attivo, viene attivato un altro.
#[tauri::command]
pub fn delete_profile(state: State<'_, DbState>, id: String) -> Result<(), String> {
    let conn = state.0.lock().map_err(|e| e.to_string())?;
    let count: i64 = conn
        .query_row("SELECT COUNT(*) FROM profiles", [], |row| row.get(0))
        .map_err(|e| e.to_string())?;
    if count <= 1 {
        return Err("Deve restare almeno un profilo.".into());
    }
    let exists = conn
        .query_row("SELECT 1 FROM profiles WHERE id = ?1", [&id], |row| row.get::<_, i32>(0))
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
    conn.execute("DELETE FROM profiles WHERE id = ?1", [&id])
        .map_err(|e| e.to_string())?;
    if active_id.as_deref() == Some(id.as_str()) {
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
}
