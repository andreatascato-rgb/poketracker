# Struttura Progetto - PokeTracker

## Obiettivo

Definisce l'organizzazione del codice per il progetto PokeTracker (Tauri + Rust + Svelte).

## Struttura Generale

```
poketracker/
├── src-tauri/                 # Backend Rust (Tauri)
│   ├── src/
│   │   ├── main.rs            # Entry point Tauri
│   │   ├── lib.rs             # Setup app, registrazione command
│   │   ├── commands/          # Comandi IPC Tauri (profile, save_detect, export_backup)
│   │   ├── db/                # Database SQLite (connection, migrations)
│   │   └── watcher.rs         # Monitoraggio file system (salvataggi)
│   ├── binaries/              # Sidecar parser (parser-x86_64-pc-windows-msvc.exe)
│   ├── Cargo.toml
│   └── tauri.conf.json
│
├── src-sidecar/               # Parser C# (PKHeX.Core) — read-only
│   ├── Program.cs             # Entry point; comandi detect, pokedex (API PKHeX tipata)
│   └── PokeTracker.Parser.csproj
│
├── src/                       # Frontend Svelte/SvelteKit
│   ├── lib/                   # Codice condiviso
│   │   ├── components/        # Componenti Svelte (ui, layout, profile, pokedex)
│   │   ├── stores/            # Svelte stores (profile, error-archive, sync)
│   │   ├── data/              # Dati statici (pokedex-types, pokedex-species)
│   │   └── utils.ts           # Utility frontend
│   ├── routes/                # Routing SvelteKit (+page.svelte, +layout.svelte)
│   ├── app.html
│   └── (entry SvelteKit)
│
├── docs/                      # Documentazione
├── static/                    # Asset (favicon, pokedex-sprites, profile-sprites)
└── README.md
```

## Backend Rust (src-tauri/)

### Struttura Dettagliata (attuale)

```
src-tauri/src/
├── main.rs                    # Entry point Tauri
├── lib.rs                     # Registrazione command, setup DB, watcher
│
├── commands/                  # Comandi IPC esposti a frontend
│   ├── mod.rs
│   ├── profile.rs            # Profili, salvataggi, Pokedex state (DB + sidecar)
│   ├── save_detect.rs        # Invoco sidecar: detect, pokedex (JSON → Result)
│   └── export_backup.rs      # Export dati (cartella, path)
│
├── db/                        # Database SQLite
│   ├── mod.rs
│   ├── connection.rs         # Connessione, init, migrazioni
│   └── migrations.rs         # Schema (profiles, app_state, pokedex_state)
│
└── watcher.rs                 # Monitoraggio file system (salvataggi)
```

## Frontend Svelte (src/)

### Struttura Dettagliata (attuale)

```
src/
├── lib/
│   ├── components/            # Componenti Svelte
│   │   ├── ui/                # shadcn-svelte (button, card, dialog, empty-state, breadcrumb, …)
│   │   ├── layout/            # ContentArea, Sidebar, TopBar
│   │   ├── profile/           # ProfileSelector
│   │   └── pokedex/           # PokedexTile
│   ├── stores/                # Svelte stores
│   │   ├── profile.ts         # Profilo attivo, loadProfiles
│   │   ├── error-archive.ts   # reportSystemError, archivio errori
│   │   └── sync.svelte.ts     # Stato sync
│   ├── data/                  # Dati statici
│   │   ├── pokedex-types.ts   # Tipi specie
│   │   └── pokedex-species.ts # Lista specie (getSpeciesList)
│   └── utils.ts               # Utility
│
├── routes/                    # Routing SvelteKit
│   ├── +layout.svelte         # Layout root (TopBar, Sidebar, breadcrumb)
│   ├── +layout.ts
│   ├── +page.svelte           # Home
│   ├── profilo/               # Profilo, salvataggi, pokedex, statistiche
│   ├── impostazioni/          # Profili, backup-dati, errori
│   ├── wiki/                  # Mosse, nature, pokemon
│   └── editor/
│
└── app.html
```

**Obiettivo struttura:** introdurre `lib/services/` (wrapper invoke tipati) e usare i servizi dalle pagine invece di `invoke` sparso; tipi condivisi in `lib/types/` o nei servizi.

## Sidecar C# (src-sidecar/)

### Struttura Dettagliata (attuale)

```
src-sidecar/
├── Program.cs                 # Entry point; comandi detect, pokedex (API PKHeX tipata, no reflection)
└── PokeTracker.Parser.csproj  # PKHeX.Core, net8.0, PublishSingleFile → src-tauri/binaries/
```

- **detect:** SaveUtil.GetVariantSAV, SaveDetectHelper (game, version, generation, language).
- **pokedex:** PokedexHelper usa solo `SaveFile.GetSeen(speciesId)` e `SaveFile.GetCaught(speciesId)`; range specie da `SaveFile.Generation` (nessun 1–493 hardcoded).

## Principi Organizzativi

### Separazione Responsabilità

- **Backend Rust**: Logica business, database, file system, integrazione sidecar
- **Frontend Svelte**: UI, state management, presentazione dati
- **Sidecar C#**: Parsing file salvataggio (isolato)

### Pattern Utilizzati

**Backend:**
- **Repository Pattern**: Per accesso database
- **Service Layer**: Per logica di business
- **Command Pattern**: Per comandi Tauri IPC

**Frontend:**
- **Component-based**: Componenti Svelte riutilizzabili
- **Store Pattern**: Svelte stores per state management
- **Service Layer**: Wrapper per comandi Tauri

### Convenzioni Naming

**Rust:**
- File: `snake_case.rs`
- Funzioni: `snake_case()`
- Struct: `PascalCase`
- Moduli: `snake_case`

**Svelte:**
- Componenti: `PascalCase.svelte`
- Stores: `camelCase.ts`
- Servizi: `camelCase.ts`

## Sviluppo locale (dev)

- **Avvio:** `npm run tauri dev`. Tenere il processo in esecuzione e **non chiudere** la finestra dell’app.
- **Task Cursor/VS Code:** è definito il task **"Tauri Dev"** in `.vscode/tasks.json` (cartella gitignored). Per usarlo: *Run Task* → *Tauri Dev*. Evita di digitare il comando a ogni sessione.
- **Modifiche frontend (Svelte, CSS, TS):** Vite invia aggiornamenti via HMR. Se in Tauri non vedi le modifiche, usare il **pulsante Ricarica** (icona refresh) in top bar a destra — visibile solo in dev — per ricaricare la WebView senza riavviare `tauri dev`.
- **Modifiche Rust (`src-tauri`):** Tauri osserva i file, ricompila e riavvia l’app; la finestra si chiude e riapre da sola.

## Note

Questa struttura:
- Scalabile per app complessa
- Mantiene separazione chiara backend/frontend
- Facilita testing e manutenzione
- Segue best practice Tauri + Rust + Svelte
