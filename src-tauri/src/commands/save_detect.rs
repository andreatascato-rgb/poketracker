//! Comando per riconoscimento gioco/versione da file .sav tramite sidecar PKHeX.
//! Vedi docs/project/sav-game-version-detection.md, parser-architecture.md.

use serde::{Deserialize, Serialize};
use tauri::AppHandle;
use tauri_plugin_shell::ShellExt;

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
#[tauri::command]
pub async fn detect_save_game_version(app: AppHandle, path: String) -> Result<SaveGameVersion, String> {
    let sidecar = app
        .shell()
        .sidecar("parser")
        .map_err(|e| e.to_string())?;

    let (mut rx, _child) = sidecar
        .args(&["detect", &path])
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
