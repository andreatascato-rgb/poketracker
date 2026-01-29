# Procedure: Aggiungere Dipendenza (npm / Cargo / C#)

## Obiettivo

Definisce dove e come aggiungere una nuova dipendenza (npm, Cargo, .NET) in PokeTracker senza introdurre conflitti di versione o violare gli standard.

## Quando Usare Questa Procedura

- Query: "aggiungi dipendenza", "aggiungi pacchetto", "npm install", "cargo add", "dotnet add", "aggiungi libreria", "install package", "aggiungi crate", "aggiungi nuget"
- Quando si deve aggiungere una dipendenza in package.json, Cargo.toml o .csproj
- Se la query corrisponde, questa procedure è OBBLIGATORIA

## Obbligatorietà

Completamento integrale della checklist obbligatorio prima di proporre implementazione. Nessuna eccezione.

## File da Leggere PRIMA

1. **Tauri 2 versioni**: `docs/standards/tauri2-standard.md:14-16`
   - Allineamento `@tauri-apps/api` e crate `tauri` (stessa minor); plugin stessa versione esatta

2. **Struttura progetto**: `docs/project/project-structure.md:1-50`
   - Root per frontend (package.json), `src-tauri/Cargo.toml`, `src-sidecar/*.csproj`

3. **Tooling**: `docs/standards/tooling-standard.md:1-80`
   - Path e comandi per lint/format (opzionale per dependency-add)

## Checklist Obbligatoria

1. [ ] Identifica il **layer** (frontend npm, backend Cargo, sidecar .NET) e il file da modificare (`package.json`, `src-tauri/Cargo.toml`, `src-sidecar/*.csproj`)
2. [ ] Per **npm**: eseguire `npm install <pkg>` (o `--save-dev` se tool); evitare versioni in conflitto con `@tauri-apps/api` (stessa minor di Tauri)
3. [ ] Per **Cargo**: eseguire `cargo add <crate>` in `src-tauri/`; per plugin Tauri verificare versione compatibile con `tauri` (stessa versione esatta per plugin ufficiali: `tauri2-standard.md:16`)
4. [ ] Per **.NET**: `dotnet add package <pkg>` nel progetto sidecar; target framework come da `csharp-sidecar-standard.md` (es. net8.0)
5. [ ] Dopo l’aggiunta: verificare che build/dev funzionino (`npm run tauri dev` o equivalente)

## Riferimenti Standard

- `docs/standards/tauri2-standard.md:14-16` — Allineamento versioni Tauri e plugin
- `docs/project/project-structure.md:1-50` — Dove sono package.json, Cargo.toml, .csproj
- `docs/standards/csharp-sidecar-standard.md` — Target framework sidecar

## Note

- Non duplicare dipendenze già presenti (controllare package.json / Cargo.toml / .csproj prima).
- Per dipendenze di tipo “nuova feature” (es. “aggiungi grafici”) considerare la procedura `new-feature.md` e poi dependency-add come step interno.
