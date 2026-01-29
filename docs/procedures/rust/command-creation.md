# Procedure: Creare Comando Rust/Tauri

## Obiettivo

Definisce come aggiungere un nuovo comando Tauri (Rust) esposto al frontend e come invocarlo dal frontend in modo sicuro e tipizzato.

## Quando Usare Questa Procedura

- Query: "crea comando", "aggiungi comando", "nuovo comando", "tauri command", "rust command", "aggiungi invoke", "nuovo comando tauri", "tauri invoke"
- Quando si deve esporre una nuova funzionalità Rust al frontend via `invoke`
- Se la query corrisponde, questa procedure è OBBLIGATORIA

## Obbligatorietà

Completamento integrale della checklist obbligatorio prima di proporre implementazione. Nessuna eccezione.

## File da Leggere PRIMA

1. **Rust backend Tauri**: `docs/standards/rust-tauri-standard.md:1-88`
   - Error handling nei command: righe 17-33
   - `Result<T, E>` con `E: Serialize`, no panic: righe 19-26

2. **TypeScript/Frontend**: `docs/standards/typescript-frontend-standard.md:36-70`
   - Import `invoke` da `@tauri-apps/api/core`: righe 37-42
   - Gestione errori `try/catch`: righe 48-62
   - Tipi condivisi (opzione A/B): righe 64-70

3. **Struttura progetto**: `docs/project/project-structure.md:48-68`
   - Cartella `src-tauri/src/commands/`, moduli, registrazione in main/lib

## Checklist Obbligatoria

1. [ ] Leggi `docs/standards/rust-tauri-standard.md:17-33` — Command deve restituire `Result<T, E>` con `E: Serialize`; non usare `panic!` nei command
2. [ ] Definisci la funzione con `#[tauri::command]`; parametri e ritorno (de)serializzabili (serde)
3. [ ] Verifica `docs/project/project-structure.md:48-68` — Aggiungi il comando nel modulo appropriato in `src-tauri/src/commands/` (o crea nuovo file e `mod.rs`); registra in `main.rs` (o `lib.rs`) con `.invoke_handler(tauri::generate_handler![...])`
4. [ ] Se serve tipo custom per errore: enum con `#[derive(Serialize)]` o usa `Result<T, String>` (`rust-tauri-standard.md:23-26`)
5. [ ] Frontend: import `invoke` da `@tauri-apps/api/core`; chiamata `await invoke<ReturnType>('command_name', { arg })`; gestione errori con `try/catch` (`typescript-frontend-standard.md:48-62`)
6. [ ] Tipi TypeScript: usare generico `invoke<T>()` o tipi condivisi come da `typescript-frontend-standard.md:64-70`
7. [ ] **Verifica compilazione:** Dopo aver aggiunto/modificato il comando, esegui `cargo check` (o `cargo build`) e verifica che la build passi. Vedi [coding-best-practices](../../standards/coding-best-practices.md) § Workflow.

## Riferimenti Standard

- `docs/standards/versioni-stack-standard.md` — Versioni Rust/Tauri, import `invoke` da `@tauri-apps/api/core`, edition 2021, anti-pattern
- `docs/standards/rust-tauri-standard.md:1-88` — Command, Result, error handling
- `docs/standards/typescript-frontend-standard.md:36-70` — invoke, errori, tipi
- `docs/project/project-structure.md:48-68` — Dove definire e registrare i command

## Note

- Il nome del command nel frontend è la versione snake_case del nome della funzione Rust (es. `my_command` → `invoke('my_command', ...)`).
- Per comandi che usano sidecar o DB, verificare anche `tauri2-sidecar-standard.md` e `rust-tauri-standard.md` (sezione Database).
