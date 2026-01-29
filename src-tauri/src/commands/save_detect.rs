//! Comando per riconoscimento gioco/versione e estrazione Pokedex da file .sav tramite sidecar PKHeX.
//! Vedi docs/project/sav-game-version-detection.md, parser-architecture.md, pokedex-personal.md.

use std::path::Path;
use serde::{Deserialize, Serialize};
use tauri::AppHandle;
use tauri_plugin_shell::ShellExt;

#[derive(Debug, Clone, Deserialize)]
pub struct PokedexExtractEntry {
    pub species_id: i32,
    pub status: String,
}

#[derive(Debug, Clone, Deserialize)]
struct PokedexExtractResponse {
    entries: Vec<PokedexExtractEntry>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SaveGameVersion {
    pub game: String,
    pub version: String,
    /// Numero generazione (1–9). Default 0 se assente o non riconosciuta.
    #[serde(default)]
    pub generation: i32,
    /// LanguageID raw dal save (PKHeX); per debug quando version è "—".
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "languageIdRaw")]
    pub language_id_raw: Option<i32>,
}

/// Invoca il sidecar parser con "detect <path>", legge JSON da stdout, restituisce game e version.
/// Valida path (non vuoto, file esistente) prima di spawnare il sidecar.
#[tauri::command]
pub async fn detect_save_game_version(app: AppHandle, path: String) -> Result<SaveGameVersion, String> {
    let path = path.trim();
    if path.is_empty() {
        return Err("Path salvataggio non può essere vuoto.".into());
    }
    if !Path::new(path).is_file() {
        return Err(format!("File non trovato o non è un file: {}", path));
    }
    let sidecar = app
        .shell()
        .sidecar("parser")
        .map_err(|e| e.to_string())?;

    let (mut rx, _child) = sidecar
        .args(["detect", &path])
        .spawn()
        .map_err(|e| e.to_string())?;

    let mut stdout = String::new();
    while let Some(event) = rx.recv().await {
        use tauri_plugin_shell::process::CommandEvent;
        if let CommandEvent::Stdout(line) = event {
            let s = String::from_utf8_lossy(&line);
            stdout.push_str(&s);
        }
    }

    let trimmed = stdout.trim();
    let value: serde_json::Value =
        serde_json::from_str(trimmed).map_err(|e| format!("parse sidecar output: {}", e))?;
    if let Some(msg) = value.get("error").and_then(|v| v.as_str()) {
        return Err(msg.to_string());
    }
    let result: SaveGameVersion =
        serde_json::from_value(value).map_err(|e| format!("sidecar result: {}", e))?;
    Ok(result)
}

/// Estrae stato Pokedex da un file .sav. Invoca il sidecar con `pokedex_auto <path>`: il sidecar
/// rileva il tipo di save e usa il parser dedicato (es. FRLG per Rosso Fuoco/Verde Foglia) o
/// restituisce vuoto. Log su stderr (sidecar) e eprintln (Rust) per capire cosa viene letto/estratto.
pub async fn extract_pokedex_from_save(app: AppHandle, path: String) -> Result<Vec<PokedexExtractEntry>, String> {
    let path = path.trim();
    if path.is_empty() {
        return Err("Path salvataggio non può essere vuoto.".into());
    }
    if !Path::new(path).is_file() {
        return Err(format!("File non trovato o non è un file: {}", path));
    }

    eprintln!("[PokeTracker] extract_pokedex_from_save: invoco sidecar pokedex_auto, path={}", path);

    let sidecar = app
        .shell()
        .sidecar("parser")
        .map_err(|e| e.to_string())?;

    let (mut rx, _child) = sidecar
        .args(["pokedex_auto", path])
        .spawn()
        .map_err(|e| e.to_string())?;

    let mut stdout = String::new();
    let mut stderr = String::new();
    while let Some(event) = rx.recv().await {
        use tauri_plugin_shell::process::CommandEvent;
        match event {
            CommandEvent::Stdout(line) => {
                let s = String::from_utf8_lossy(&line);
                stdout.push_str(&s);
            }
            CommandEvent::Stderr(line) => {
                let s = String::from_utf8_lossy(&line);
                stderr.push_str(&s);
                eprintln!("[parser stderr] {}", s.trim());
            }
            _ => {}
        }
    }

    let trimmed = stdout.trim();
    let value: serde_json::Value =
        serde_json::from_str(trimmed).map_err(|e| format!("parse sidecar output: {}", e))?;
    if let Some(msg) = value.get("error").and_then(|v| v.as_str()) {
        return Err(msg.to_string());
    }
    let response: PokedexExtractResponse =
        serde_json::from_value(value).map_err(|e| format!("sidecar result: {}", e))?;

    let entries = response.entries;
    let seen = entries.iter().filter(|e| e.status.eq_ignore_ascii_case("seen")).count();
    let caught = entries.iter().filter(|e| e.status.eq_ignore_ascii_case("caught")).count();
    eprintln!(
        "[PokeTracker] extract_pokedex_from_save: ricevute {} entries (seen={}, caught={})",
        entries.len(),
        seen,
        caught
    );

    Ok(entries)
}
