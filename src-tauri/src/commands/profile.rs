//! Comandi profilo: lettura/scrittura su SQLite. Vedi command-with-db, database-migrations-standard.

use rusqlite::OptionalExtension;
use serde::{Deserialize, Serialize};
use tauri::{AppHandle, State};

use crate::commands::save_detect;
use crate::watcher::SavWatcher;
use crate::DbState;

/// Massimo species_id accettato (Gen9 National Dex). Coerente con sidecar PKHeX (range per generazione).
const MAX_SPECIES_ID: i32 = 1025;

/// Range specie mostrate in UI (Gen 1–4, 1–493). Scriviamo sempre tutte le righe 1..=POKEDEX_DISPLAY_SPECIES_MAX
/// in pokedex_state così get_pokedex_state restituisce 493 righe e conteggi/tile (es. Bulbasaur id 1) sono corretti.
const POKEDEX_DISPLAY_SPECIES_MAX: i32 = 493;

/// Coppie (specie_evoluzione, specie_base): se hai catturato l'evoluzione, la base va segnata almeno come catturata.
/// Estendibile a Gen2–Gen9; qui Gen1 (1–151) per completezza Pokedex personale.
const EVOLVES_FROM: &[(i32, i32)] = &[
    (2, 1), (3, 2), (5, 4), (6, 5), (8, 7), (9, 8), (11, 10), (12, 11), (14, 13), (15, 14),
    (17, 16), (18, 17), (20, 19), (21, 20), (22, 21), (24, 23), (25, 24), (26, 25), (28, 27), (29, 28),
    (30, 29), (31, 30), (33, 32), (34, 33), (36, 35), (38, 37), (40, 39), (42, 41), (44, 43), (45, 44),
    (47, 46), (49, 48), (51, 50), (53, 52), (55, 54), (57, 56), (59, 58), (61, 60), (62, 61), (64, 63),
    (65, 64), (67, 66), (68, 67), (70, 69), (71, 70), (73, 72), (75, 74), (76, 75), (78, 77), (80, 79),
    (82, 81), (85, 84), (87, 86), (89, 88), (91, 90), (94, 93), (97, 96), (99, 98), (101, 100),
    (103, 102), (105, 104), (107, 106), (110, 109), (112, 111), (115, 114), (117, 116), (119, 118),
    (121, 120), (124, 123), (126, 125), (128, 127), (131, 130), (134, 133), (136, 135), (139, 138),
    (141, 140), (143, 142), (146, 145), (149, 148),
];

fn prev_evolution(species_id: i32) -> Option<i32> {
    EVOLVES_FROM.iter().find(|(c, _)| *c == species_id).map(|(_, p)| *p)
}

/// Propaga "caught" alle pre-evoluzioni: se una specie è catturata, tutte le forme base vanno almeno "caught".
fn propagate_caught_to_prevolutions(merged: &mut std::collections::HashMap<i32, String>) {
    let caught_ids: Vec<i32> = merged
        .iter()
        .filter(|(_, s)| s.as_str() == "caught")
        .map(|(id, _)| *id)
        .collect();
    for sid in caught_ids {
        let mut current = sid;
        while let Some(prev) = prev_evolution(current) {
            merged.insert(prev, "caught".to_string());
            current = prev;
        }
    }
}

/// Riempie merged con specie 1..=POKEDEX_DISPLAY_SPECIES_MAX mancanti come "unseen".
/// Così in DB scriviamo sempre 493 righe e il frontend riceve tutte le specie (conteggi e Bulbasaur id 1 corretti).
fn fill_pokedex_display_range(merged: &mut std::collections::HashMap<i32, String>) {
    for sid in 1..=POKEDEX_DISPLAY_SPECIES_MAX {
        merged.entry(sid).or_insert_with(|| "unseen".to_string());
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Profile {
    pub id: String,
    pub name: String,
    /// Data primo caricamento (ISO 8601, es. "2026-01-27T12:00:00").
    pub created_at: String,
    /// Data ultimo aggiornamento (ISO 8601).
    pub updated_at: String,
    /// Id sprite/avatar scelto per il profilo (es. "kenney-0"); null se non impostato.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avatar_id: Option<String>,
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

/// Chiave app_state per l'elenco salvataggi del profilo (docs/project/multi-profile.md).
fn sav_entries_key(profile_id: &str) -> String {
    format!("sav_entries:{}", profile_id)
}

/// Chiave app_state per i path monitorati dal watcher del profilo.
fn sav_watched_paths_key(profile_id: &str) -> String {
    format!("sav_watched_paths:{}", profile_id)
}

/// Restituisce tutti i profili dal DB.
#[tauri::command]
pub fn get_profiles(state: State<'_, DbState>) -> Result<Vec<Profile>, String> {
    let conn = state.0.lock().map_err(|e| e.to_string())?;
    let mut stmt = conn
        .prepare("SELECT id, name, created_at, updated_at, avatar_id FROM profiles ORDER BY name")
        .map_err(|e| e.to_string())?;
    let rows = stmt
        .query_map([], |row| {
            Ok(Profile {
                id: row.get(0)?,
                name: row.get(1)?,
                created_at: row.get(2)?,
                updated_at: row.get(3)?,
                avatar_id: row.get(4)?,
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
            "SELECT id, name, created_at, updated_at, avatar_id FROM profiles WHERE id = ?1",
            [&active_id],
            |row| {
                Ok(Profile {
                    id: row.get(0)?,
                    name: row.get(1)?,
                    created_at: row.get(2)?,
                    updated_at: row.get(3)?,
                    avatar_id: row.get(4)?,
                })
            },
        )
        .optional()
        .map_err(|e| e.to_string())?;
    Ok(profile)
}

/// Crea un nuovo profilo. id derivato da name (slug); se già esiste restituisce errore.
#[tauri::command]
pub fn create_profile(
    state: State<'_, DbState>,
    name: String,
    avatar_id: Option<String>,
) -> Result<Profile, String> {
    let name = name.trim();
    if name.is_empty() {
        return Err("Il nome non può essere vuoto.".into());
    }
    let id = slug(name);
    let conn = state.0.lock().map_err(|e| e.to_string())?;
    conn.execute(
        "INSERT INTO profiles (id, name, created_at, updated_at, avatar_id) VALUES (?1, ?2, datetime('now'), datetime('now'), ?3)",
        rusqlite::params![&id, name, avatar_id],
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
    let (created_at, updated_at, avatar_id): (String, String, Option<String>) = conn
        .query_row(
            "SELECT created_at, updated_at, avatar_id FROM profiles WHERE id = ?1",
            [&id],
            |row| Ok((row.get(0)?, row.get(1)?, row.get(2)?)),
        )
        .map_err(|e| e.to_string())?;
    Ok(Profile {
        id: id.clone(),
        name: name.to_string(),
        created_at,
        updated_at,
        avatar_id,
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
    profile_by_id(&conn, &id)
}

/// Aggiorna nome e/o avatar di un profilo. Restituisce errore se l'id non esiste o il nome è vuoto.
#[tauri::command]
pub fn update_profile(
    state: State<'_, DbState>,
    id: String,
    new_name: String,
    avatar_id: Option<String>,
) -> Result<Profile, String> {
    let new_name = new_name.trim();
    if new_name.is_empty() {
        return Err("Il nome non può essere vuoto.".into());
    }
    let conn = state.0.lock().map_err(|e| e.to_string())?;
    let updated = conn
        .execute(
            "UPDATE profiles SET name = ?1, avatar_id = ?2, updated_at = datetime('now') WHERE id = ?3",
            rusqlite::params![new_name, avatar_id, &id],
        )
        .map_err(|e| e.to_string())?;
    if updated == 0 {
        return Err("Profilo non trovato.".into());
    }
    profile_by_id(&conn, &id)
}

fn profile_by_id(conn: &rusqlite::Connection, id: &str) -> Result<Profile, String> {
    conn.query_row(
        "SELECT id, name, created_at, updated_at, avatar_id FROM profiles WHERE id = ?1",
        [id],
        |row| {
            Ok(Profile {
                id: row.get(0)?,
                name: row.get(1)?,
                created_at: row.get(2)?,
                updated_at: row.get(3)?,
                avatar_id: row.get(4)?,
            })
        },
    )
    .map_err(|e| e.to_string())
}

/// Elimina un profilo e tutti i dati ad esso associati: pokedex_state, sav_entries e sav_watched_paths del profilo.
/// Rimuove i path del profilo dal watcher. Se si elimina l'ultimo profilo, app_state.active_profile_id viene azzerato.
/// Se si elimina il profilo attivo ma restano altri profili, viene attivato il primo disponibile.
#[tauri::command]
pub fn delete_profile(state: State<'_, DbState>, watcher: State<'_, SavWatcher>, id: String) -> Result<(), String> {
    let id = id.trim();
    if id.is_empty() {
        return Err("Id profilo non valido.".into());
    }
    let conn = state.0.lock().map_err(|e| e.to_string())?;
    conn.execute("BEGIN IMMEDIATE", []).map_err(|e| e.to_string())?;
    let outcome: Result<Vec<String>, String> = (|| {
        let exists = conn
            .query_row("SELECT 1 FROM profiles WHERE id = ?1", [id], |row| row.get::<_, i32>(0))
            .optional()
            .map_err(|e| e.to_string())?
            .is_some();
        if !exists {
            return Err("Profilo non trovato.".into());
        }
        // Leggi path monitorati del profilo da rimuovere dal watcher dopo il commit
        let watched_key = sav_watched_paths_key(id);
        let watched_json: Option<String> = conn
            .query_row("SELECT value FROM app_state WHERE key = ?1", [&watched_key], |row| row.get::<_, String>(0))
            .optional()
            .map_err(|e| e.to_string())?;
        let watched_paths: Vec<String> = watched_json
            .as_ref()
            .and_then(|s| serde_json::from_str(s).ok())
            .unwrap_or_default();
        // Elimina salvataggi e path del profilo (dati di essi)
        conn.execute("DELETE FROM app_state WHERE key = ?1", [&sav_entries_key(id)])
            .map_err(|e| e.to_string())?;
        conn.execute("DELETE FROM app_state WHERE key = ?1", [&watched_key])
            .map_err(|e| e.to_string())?;
        conn.execute("DELETE FROM pokedex_state WHERE profile_id = ?1", [id])
            .map_err(|e| e.to_string())?;
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
        Ok(watched_paths)
    })();
    match &outcome {
        Ok(paths) => {
            conn.execute("COMMIT", []).map_err(|e| e.to_string())?;
            for path in paths {
                let _ = watcher.remove(path);
            }
        }
        Err(_) => {
            let _ = conn.execute("ROLLBACK", []);
        }
    }
    outcome.map(|_| ())
}

/// Voce stato Pokedex per una specie (docs/project/pokedex-personal.md).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PokedexStateEntry {
    pub species_id: i32,
    pub status: String,
}

/// Restituisce lo stato Pokedex del profilo (solo righe presenti; assenza = unseen in frontend).
#[tauri::command]
pub fn get_pokedex_state(state: State<'_, DbState>, profile_id: String) -> Result<Vec<PokedexStateEntry>, String> {
    let profile_id = profile_id.trim();
    if profile_id.is_empty() {
        return Err("Id profilo non valido.".into());
    }
    let conn = state.0.lock().map_err(|e| e.to_string())?;
    let mut stmt = conn
        .prepare("SELECT species_id, status FROM pokedex_state WHERE profile_id = ?1 ORDER BY species_id")
        .map_err(|e| e.to_string())?;
    let rows = stmt
        .query_map([profile_id], |row| {
            let species_id: i32 = row.get(0)?;
            let raw: String = row.get(1)?;
            let status = raw.trim().to_lowercase();
            let status = if status == "caught" || status == "seen" || status == "unseen" {
                status
            } else {
                "unseen".to_string()
            };
            Ok(PokedexStateEntry { species_id, status })
        })
        .map_err(|e| e.to_string())?;
    let mut out = Vec::new();
    for row in rows {
        out.push(row.map_err(|e| e.to_string())?);
    }
    Ok(out)
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

/// Restituisce le voci salvataggio del profilo attivo (path, game, version, updated_at). Salvate in app_state come JSON array per profilo.
/// Migra da chiave legacy 'sav_entries' al primo accesso.
#[tauri::command]
pub fn get_sav_entries(state: State<'_, DbState>) -> Result<Vec<SavEntry>, String> {
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
        None => return Ok(Vec::new()),
    };
    let key = sav_entries_key(&active_id);
    let value: Option<String> = conn
        .query_row("SELECT value FROM app_state WHERE key = ?1", [&key], |row| row.get::<_, String>(0))
        .optional()
        .map_err(|e| e.to_string())?;
    let mut entries: Vec<SavEntry> = match value {
        Some(ref s) => serde_json::from_str(s).unwrap_or_default(),
        None => {
            // Migrazione: chiave legacy globale → per profilo
            let legacy: Option<String> = conn
                .query_row(
                    "SELECT value FROM app_state WHERE key = 'sav_entries'",
                    [],
                    |row| row.get::<_, String>(0),
                )
                .optional()
                .map_err(|e| e.to_string())?;
            let migrated: Vec<SavEntry> = legacy
                .as_ref()
                .and_then(|s| serde_json::from_str(s).ok())
                .unwrap_or_default();
            if !migrated.is_empty() {
                let json = serde_json::to_string(&migrated).map_err(|e| e.to_string())?;
                conn.execute(
                    "INSERT INTO app_state (key, value) VALUES (?1, ?2) ON CONFLICT(key) DO UPDATE SET value = ?2",
                    rusqlite::params![&key, &json],
                )
                .map_err(|e| e.to_string())?;
                conn.execute("DELETE FROM app_state WHERE key = 'sav_entries'", [])
                    .map_err(|e| e.to_string())?;
            }
            migrated
        }
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
                "INSERT INTO app_state (key, value) VALUES (?1, ?2) ON CONFLICT(key) DO UPDATE SET value = ?2",
                rusqlite::params![&key, &json],
            )
            .map_err(|e| e.to_string())?;
        }
    }
    Ok(entries)
}

/// Aggiunge una voce salvataggio al profilo attivo (path file, game, version e generation da detect). Evita duplicati per path.
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
    let active_id: Option<String> = conn
        .query_row(
            "SELECT value FROM app_state WHERE key = 'active_profile_id'",
            [],
            |row| row.get::<_, String>(0),
        )
        .optional()
        .map_err(|e| e.to_string())?;
    let active_id = active_id.ok_or("Nessun profilo attivo.")?;
    let key = sav_entries_key(&active_id);
    let value: Option<String> = conn
        .query_row("SELECT value FROM app_state WHERE key = ?1", [&key], |row| row.get::<_, String>(0))
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
        "INSERT INTO app_state (key, value) VALUES (?1, ?2) ON CONFLICT(key) DO UPDATE SET value = ?2",
        rusqlite::params![&key, &json],
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
    save_detect::detect_save_game_version(app.clone(), path.clone()).await?;
    let pokedex_entries = save_detect::extract_pokedex_from_save(app, path.clone()).await?;
    let conn = state.0.lock().map_err(|e| e.to_string())?;
    let active_id: Option<String> = conn
        .query_row(
            "SELECT value FROM app_state WHERE key = 'active_profile_id'",
            [],
            |row| row.get::<_, String>(0),
        )
        .optional()
        .map_err(|e| e.to_string())?;
    let active_id = active_id.ok_or("Nessun profilo attivo.")?;
    let key = sav_entries_key(&active_id);
    let value: Option<String> = conn
        .query_row("SELECT value FROM app_state WHERE key = ?1", [&key], |row| row.get::<_, String>(0))
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
        "INSERT INTO app_state (key, value) VALUES (?1, ?2) ON CONFLICT(key) DO UPDATE SET value = ?2",
        rusqlite::params![&key, &json],
    )
    .map_err(|e| e.to_string())?;

    // Carica stato attuale pokedex (per merge multi-save).
    let mut current: std::collections::HashMap<i32, String> = std::collections::HashMap::new();
    let mut stmt = conn
        .prepare("SELECT species_id, status FROM pokedex_state WHERE profile_id = ?1")
        .map_err(|e| e.to_string())?;
    let rows = stmt
        .query_map([&active_id], |row| Ok((row.get::<_, i32>(0)?, row.get::<_, String>(1)?)))
        .map_err(|e| e.to_string())?;
    for row in rows {
        let (sid, raw) = row.map_err(|e| e.to_string())?;
        let s = raw.trim().to_lowercase();
        let status = if s == "caught" || s == "seen" || s == "unseen" {
            s
        } else {
            "unseen".to_string()
        };
        current.insert(sid, status);
    }

    // Merge: per ogni specie best(current, new) con caught > seen > unseen.
    // Normalizza status (trim + lowercase) così "Seen"/" seen " non vengono scartati.
    fn normalize_status(s: &str) -> Option<String> {
        let s = s.trim().to_lowercase();
        if s == "unseen" || s == "seen" || s == "caught" {
            Some(s)
        } else {
            None
        }
    }
    fn rank(s: &str) -> u8 {
        match s {
            "caught" => 3,
            "seen" => 2,
            _ => 1,
        }
    }
    let mut merged: std::collections::HashMap<i32, String> = std::collections::HashMap::new();
    for e in &pokedex_entries {
        let Some(s) = normalize_status(&e.status) else { continue };
        if e.species_id < 1 || e.species_id > MAX_SPECIES_ID {
            continue;
        }
        let best = current
            .get(&e.species_id)
            .map(|c| if rank(c) >= rank(&s) { c.as_str() } else { s.as_str() })
            .unwrap_or(s.as_str());
        merged.insert(e.species_id, best.to_string());
    }
    for (sid, status) in &current {
        if *sid < 1 || *sid > MAX_SPECIES_ID {
            continue;
        }
        merged.entry(*sid).or_insert_with(|| status.clone());
    }
    propagate_caught_to_prevolutions(&mut merged);
    fill_pokedex_display_range(&mut merged);

    conn.execute("DELETE FROM pokedex_state WHERE profile_id = ?1", [&active_id])
        .map_err(|e| e.to_string())?;
    for sid in 1..=POKEDEX_DISPLAY_SPECIES_MAX {
        let status = merged.get(&sid).map(|s| s.as_str()).unwrap_or("unseen");
        conn.execute(
            "INSERT INTO pokedex_state (profile_id, species_id, status) VALUES (?1, ?2, ?3)",
            rusqlite::params![&active_id, sid, status],
        )
        .map_err(|e| e.to_string())?;
    }

    eprintln!("[PokeTracker] sync_sav_now completato: {}", path);
    Ok(())
}

/// Sincronizza il Pokedex da **tutti** i salvataggi del profilo attivo: estrae da ogni path, merge (caught > seen > unseen), aggiorna pokedex_state.
/// Usare per "Aggiorna Pokedex" o bootstrap quando ci sono save ma nessun dato Pokedex ancora.
#[tauri::command]
pub async fn sync_all_sav_now(state: State<'_, DbState>, app: AppHandle) -> Result<(), String> {
    let (active_id, paths): (String, Vec<String>) = {
        let conn = state.0.lock().map_err(|e| e.to_string())?;
        let active_id: Option<String> = conn
            .query_row(
                "SELECT value FROM app_state WHERE key = 'active_profile_id'",
                [],
                |row| row.get::<_, String>(0),
            )
            .optional()
            .map_err(|e| e.to_string())?;
        let active_id = active_id.ok_or("Nessun profilo attivo.")?;
        let key = sav_entries_key(&active_id);
        let value: Option<String> = conn
            .query_row("SELECT value FROM app_state WHERE key = ?1", [&key], |row| row.get::<_, String>(0))
            .optional()
            .map_err(|e| e.to_string())?;
        let entries: Vec<SavEntry> = value
            .as_ref()
            .and_then(|s| serde_json::from_str(s).ok())
            .unwrap_or_default();
        let paths: Vec<String> = entries.into_iter().map(|e| e.path).collect();
        (active_id, paths)
    };

    if paths.is_empty() {
        return Err("Nessun salvataggio configurato. Aggiungi almeno un file .sav da Salvataggi.".into());
    }

    eprintln!("[PokeTracker] sync_all_sav_now: {} path", paths.len());

    fn normalize_status(s: &str) -> Option<String> {
        let s = s.trim().to_lowercase();
        if s == "unseen" || s == "seen" || s == "caught" {
            Some(s)
        } else {
            None
        }
    }
    fn rank(s: &str) -> u8 {
        match s {
            "caught" => 3,
            "seen" => 2,
            _ => 1,
        }
    }

    let mut merged: std::collections::HashMap<i32, String> = std::collections::HashMap::new();
    for path in &paths {
        let path = path.trim().to_string();
        if path.is_empty() {
            continue;
        }
        match save_detect::extract_pokedex_from_save(app.clone(), path.clone()).await {
            Ok(entries) => {
                for e in entries {
                    let Some(s) = normalize_status(&e.status) else { continue };
                    if e.species_id < 1 || e.species_id > MAX_SPECIES_ID {
                        continue;
                    }
                    let cur = merged.get(&e.species_id).map(|c| c.as_str()).unwrap_or("unseen");
                    let best = if rank(&s) >= rank(cur) { s.as_str() } else { cur };
                    merged.insert(e.species_id, best.to_string());
                }
            }
            Err(e) => {
                eprintln!("[PokeTracker] sync_all_sav_now skip {}: {}", path, e);
            }
        }
    }

    let conn = state.0.lock().map_err(|e| e.to_string())?;
    let mut stmt = conn
        .prepare("SELECT species_id, status FROM pokedex_state WHERE profile_id = ?1")
        .map_err(|e| e.to_string())?;
    let rows = stmt
        .query_map([&active_id], |row| Ok((row.get::<_, i32>(0)?, row.get::<_, String>(1)?)))
        .map_err(|e| e.to_string())?;
    for row in rows {
        let (sid, raw) = row.map_err(|e| e.to_string())?;
        let s = raw.trim().to_lowercase();
        let status = if s == "caught" || s == "seen" || s == "unseen" {
            s
        } else {
            "unseen".to_string()
        };
        if merged.get(&sid).map(|m| rank(m) < rank(&status)).unwrap_or(true) {
            merged.insert(sid, status);
        }
    }
    propagate_caught_to_prevolutions(&mut merged);
    fill_pokedex_display_range(&mut merged);

    conn.execute("DELETE FROM pokedex_state WHERE profile_id = ?1", [&active_id])
        .map_err(|e| e.to_string())?;
    for sid in 1..=POKEDEX_DISPLAY_SPECIES_MAX {
        let status = merged.get(&sid).map(|s| s.as_str()).unwrap_or("unseen");
        conn.execute(
            "INSERT INTO pokedex_state (profile_id, species_id, status) VALUES (?1, ?2, ?3)",
            rusqlite::params![&active_id, sid, status],
        )
        .map_err(|e| e.to_string())?;
    }

    eprintln!("[PokeTracker] sync_all_sav_now completato.");
    Ok(())
}

/// Aggiorna solo il campo updated_at della voce salvataggio per il path indicato (profilo attivo).
/// Usato quando sav-file-changed rileva un cambio: aggiorna il timestamp senza rieseguire tutto il parsing.
#[tauri::command]
pub fn touch_sav_entry_updated_at(state: State<'_, DbState>, path: String) -> Result<(), String> {
    let path = path.trim().to_string();
    if path.is_empty() {
        return Ok(());
    }
    let conn = state.0.lock().map_err(|e| e.to_string())?;
    let active_id: Option<String> = conn
        .query_row(
            "SELECT value FROM app_state WHERE key = 'active_profile_id'",
            [],
            |row| row.get::<_, String>(0),
        )
        .optional()
        .map_err(|e| e.to_string())?;
    let active_id = active_id.ok_or("Nessun profilo attivo.")?;
    let key = sav_entries_key(&active_id);
    let value: Option<String> = conn
        .query_row("SELECT value FROM app_state WHERE key = ?1", [&key], |row| row.get::<_, String>(0))
        .optional()
        .map_err(|e| e.to_string())?;
    let mut entries: Vec<SavEntry> = match value {
        Some(ref s) => serde_json::from_str(s).unwrap_or_default(),
        None => return Ok(()),
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
        return Ok(());
    }
    let json = serde_json::to_string(&entries).map_err(|e| e.to_string())?;
    conn.execute(
        "INSERT INTO app_state (key, value) VALUES (?1, ?2) ON CONFLICT(key) DO UPDATE SET value = ?2",
        rusqlite::params![&key, &json],
    )
    .map_err(|e| e.to_string())?;
    Ok(())
}

/// Sincronizza il Pokedex **solo** dai sav watchati: estrae unseen/seen/caught da ogni path con watcher attivo, merge (caught > seen > unseen), aggiorna pokedex_state.
/// Usato su sav-file-changed così il parsing è fatto su TUTTI i pokemon di TUTTI i sav watchati.
#[tauri::command]
pub async fn sync_pokedex_from_watched_savs_now(
    state: State<'_, DbState>,
    app: AppHandle,
) -> Result<(), String> {
    let (active_id, paths): (String, Vec<String>) = {
        let conn = state.0.lock().map_err(|e| e.to_string())?;
        let active_id: Option<String> = conn
            .query_row(
                "SELECT value FROM app_state WHERE key = 'active_profile_id'",
                [],
                |row| row.get::<_, String>(0),
            )
            .optional()
            .map_err(|e| e.to_string())?;
        let active_id = active_id.ok_or("Nessun profilo attivo.")?;
        let key = sav_watched_paths_key(&active_id);
        let value: Option<String> = conn
            .query_row("SELECT value FROM app_state WHERE key = ?1", [&key], |row| row.get::<_, String>(0))
            .optional()
            .map_err(|e| e.to_string())?;
        let paths: Vec<String> = value
            .as_ref()
            .and_then(|s| serde_json::from_str(s).ok())
            .unwrap_or_default();
        (active_id, paths)
    };

    if paths.is_empty() {
        return Ok(());
    }

    eprintln!(
        "[PokeTracker] sync_pokedex_from_watched_savs_now: {} path",
        paths.len()
    );

    fn normalize_status(s: &str) -> Option<String> {
        let s = s.trim().to_lowercase();
        if s == "unseen" || s == "seen" || s == "caught" {
            Some(s)
        } else {
            None
        }
    }
    fn rank(s: &str) -> u8 {
        match s {
            "caught" => 3,
            "seen" => 2,
            _ => 1,
        }
    }

    let mut merged: std::collections::HashMap<i32, String> = std::collections::HashMap::new();
    for path in &paths {
        let path = path.trim().to_string();
        if path.is_empty() {
            continue;
        }
        match save_detect::extract_pokedex_from_save(app.clone(), path.clone()).await {
            Ok(entries) => {
                for e in entries {
                    let Some(s) = normalize_status(&e.status) else { continue };
                    if e.species_id < 1 || e.species_id > MAX_SPECIES_ID {
                        continue;
                    }
                    let cur = merged.get(&e.species_id).map(|c| c.as_str()).unwrap_or("unseen");
                    let best = if rank(&s) >= rank(cur) { s.as_str() } else { cur };
                    merged.insert(e.species_id, best.to_string());
                }
            }
            Err(e) => {
                eprintln!(
                    "[PokeTracker] sync_pokedex_from_watched_savs_now skip {}: {}",
                    path, e
                );
            }
        }
    }

    let conn = state.0.lock().map_err(|e| e.to_string())?;
    let mut stmt = conn
        .prepare("SELECT species_id, status FROM pokedex_state WHERE profile_id = ?1")
        .map_err(|e| e.to_string())?;
    let rows = stmt
        .query_map([&active_id], |row| Ok((row.get::<_, i32>(0)?, row.get::<_, String>(1)?)))
        .map_err(|e| e.to_string())?;
    for row in rows {
        let (sid, raw) = row.map_err(|e| e.to_string())?;
        let s = raw.trim().to_lowercase();
        let status = if s == "caught" || s == "seen" || s == "unseen" {
            s
        } else {
            "unseen".to_string()
        };
        if merged
            .get(&sid)
            .map(|m| rank(m) < rank(&status))
            .unwrap_or(true)
        {
            merged.insert(sid, status);
        }
    }
    propagate_caught_to_prevolutions(&mut merged);
    fill_pokedex_display_range(&mut merged);

    conn.execute("DELETE FROM pokedex_state WHERE profile_id = ?1", [&active_id])
        .map_err(|e| e.to_string())?;
    for sid in 1..=POKEDEX_DISPLAY_SPECIES_MAX {
        let status = merged.get(&sid).map(|s| s.as_str()).unwrap_or("unseen");
        conn.execute(
            "INSERT INTO pokedex_state (profile_id, species_id, status) VALUES (?1, ?2, ?3)",
            rusqlite::params![&active_id, sid, status],
        )
        .map_err(|e| e.to_string())?;
    }

    eprintln!("[PokeTracker] sync_pokedex_from_watched_savs_now completato.");
    Ok(())
}

/// Restituisce i path dei salvataggi del profilo attivo per cui il watcher è attivo (persistito in app_state come JSON array per profilo).
#[tauri::command]
pub fn get_sav_watched_paths(state: State<'_, DbState>) -> Result<Vec<String>, String> {
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
        None => return Ok(Vec::new()),
    };
    let key = sav_watched_paths_key(&active_id);
    let value: Option<String> = conn
        .query_row("SELECT value FROM app_state WHERE key = ?1", [&key], |row| row.get::<_, String>(0))
        .optional()
        .map_err(|e| e.to_string())?;
    let paths: Vec<String> = match value {
        Some(ref s) => serde_json::from_str(s).unwrap_or_default(),
        None => {
            // Migrazione: chiave legacy globale → per profilo
            let legacy: Option<String> = conn
                .query_row(
                    "SELECT value FROM app_state WHERE key = 'sav_watched_paths'",
                    [],
                    |row| row.get::<_, String>(0),
                )
                .optional()
                .map_err(|e| e.to_string())?;
            let migrated: Vec<String> = legacy
                .as_ref()
                .and_then(|s| serde_json::from_str(s).ok())
                .unwrap_or_default();
            if !migrated.is_empty() {
                let json = serde_json::to_string(&migrated).map_err(|e| e.to_string())?;
                conn.execute(
                    "INSERT INTO app_state (key, value) VALUES (?1, ?2) ON CONFLICT(key) DO UPDATE SET value = ?2",
                    rusqlite::params![&key, &json],
                )
                .map_err(|e| e.to_string())?;
                conn.execute("DELETE FROM app_state WHERE key = 'sav_watched_paths'", [])
                    .map_err(|e| e.to_string())?;
            }
            migrated
        }
    };
    Ok(paths)
}

/// Attiva o disattiva il watcher per un path del profilo attivo. Il path deve essere presente in sav_entries.
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
    let active_id: Option<String> = conn
        .query_row(
            "SELECT value FROM app_state WHERE key = 'active_profile_id'",
            [],
            |row| row.get::<_, String>(0),
        )
        .optional()
        .map_err(|e| e.to_string())?;
    let active_id = active_id.ok_or("Nessun profilo attivo.")?;
    let entries_key = sav_entries_key(&active_id);
    let watched_key = sav_watched_paths_key(&active_id);
    let entries_value: Option<String> = conn
        .query_row("SELECT value FROM app_state WHERE key = ?1", [&entries_key], |row| row.get::<_, String>(0))
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
        .query_row("SELECT value FROM app_state WHERE key = ?1", [&watched_key], |row| row.get::<_, String>(0))
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
        "INSERT INTO app_state (key, value) VALUES (?1, ?2) ON CONFLICT(key) DO UPDATE SET value = ?2",
        rusqlite::params![&watched_key, &json],
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

/// Rimuove la voce salvataggio del profilo attivo con il path indicato. Rimuove anche il path da sav_watched_paths se presente.
#[tauri::command]
pub fn remove_sav_entry(state: State<'_, DbState>, path: String) -> Result<(), String> {
    let path = path.trim().to_string();
    if path.is_empty() {
        return Ok(());
    }
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
        None => return Ok(()),
    };
    let entries_key = sav_entries_key(&active_id);
    let watched_key = sav_watched_paths_key(&active_id);
    let value: Option<String> = conn
        .query_row("SELECT value FROM app_state WHERE key = ?1", [&entries_key], |row| row.get::<_, String>(0))
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
        "INSERT INTO app_state (key, value) VALUES (?1, ?2) ON CONFLICT(key) DO UPDATE SET value = ?2",
        rusqlite::params![&entries_key, &json],
    )
    .map_err(|e| e.to_string())?;
    // Rimuovi path da sav_watched_paths del profilo
    let watched_value: Option<String> = conn
        .query_row("SELECT value FROM app_state WHERE key = ?1", [&watched_key], |row| row.get::<_, String>(0))
        .optional()
        .map_err(|e| e.to_string())?;
    if let Some(ref s) = watched_value {
        let mut paths: Vec<String> = serde_json::from_str(s).unwrap_or_default();
        paths.retain(|p| p != &path);
        let watched_json = serde_json::to_string(&paths).map_err(|e| e.to_string())?;
        conn.execute(
            "INSERT INTO app_state (key, value) VALUES (?1, ?2) ON CONFLICT(key) DO UPDATE SET value = ?2",
            rusqlite::params![&watched_key, &watched_json],
        )
        .map_err(|e| e.to_string())?;
    }
    Ok(())
}
