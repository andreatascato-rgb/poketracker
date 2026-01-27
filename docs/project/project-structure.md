# Struttura Progetto - PokeTracker

## Obiettivo

Definisce l'organizzazione del codice per il progetto PokeTracker (Tauri + Rust + Svelte).

## Struttura Generale

```
poketracker/
├── src-tauri/                 # Backend Rust (Tauri)
│   ├── src/
│   │   ├── main.rs           # Entry point Tauri
│   │   ├── commands/          # Comandi IPC Tauri
│   │   ├── services/          # Logica di business
│   │   ├── models/            # Strutture dati
│   │   ├── db/                # Database layer
│   │   ├── parser/            # Integrazione sidecar parser
│   │   ├── profiles/          # Gestione profili
│   │   ├── monitoring/        # Monitoraggio file system
│   │   └── utils/             # Utility
│   ├── Cargo.toml
│   └── tauri.conf.json
│
├── src-sidecar/               # Parser C# (PKHeX)
│   ├── Program.cs
│   ├── Services/
│   │   └── ParserService.cs
│   └── PokeTracker.Parser.csproj
│
├── src/                       # Frontend Svelte
│   ├── lib/
│   │   ├── components/        # Componenti Svelte
│   │   ├── stores/            # Svelte stores
│   │   ├── routes/            # Routing (se usi SvelteKit)
│   │   ├── services/          # Servizi frontend
│   │   └── utils/             # Utility frontend
│   ├── App.svelte
│   └── main.ts
│
├── docs/                      # Documentazione (già presente)
├── resources/                 # Risorse app (icone, immagini)
└── README.md
```

## Backend Rust (src-tauri/)

### Struttura Dettagliata

```
src-tauri/src/
├── main.rs                    # Setup Tauri, registrazione comandi
│
├── commands/                  # Comandi IPC esposti a frontend
│   ├── mod.rs
│   ├── profile.rs            # Comandi profili
│   ├── save_file.rs          # Comandi file salvataggio
│   ├── pokedex.rs            # Comandi Pokedex
│   ├── wiki.rs               # Comandi Wiki
│   ├── editor.rs             # Comandi editor salvataggi
│   └── management.rs         # Comandi gestione app
│
├── services/                  # Logica di business
│   ├── mod.rs
│   ├── profile_service.rs    # Gestione profili
│   ├── save_monitor.rs       # Monitoraggio file system
│   ├── parser_service.rs     # Integrazione sidecar
│   ├── database_service.rs   # Operazioni database
│   └── sync_service.rs       # Sincronizzazione dati
│
├── models/                    # Strutture dati
│   ├── mod.rs
│   ├── profile.rs            # Modello profilo
│   ├── pokemon.rs            # Modello Pokemon
│   ├── save_file.rs          # Modello salvataggio
│   └── pokedex.rs            # Modello Pokedex
│
├── db/                        # Database layer
│   ├── mod.rs
│   ├── connection.rs         # Gestione connessione SQLite
│   ├── migrations.rs         # Sistema migrazioni
│   ├── repositories/          # Repository pattern
│   │   ├── mod.rs
│   │   ├── profile_repo.rs
│   │   ├── pokemon_repo.rs
│   │   └── pokedex_repo.rs
│   └── schema.rs             # Schema database
│
├── parser/                    # Integrazione parser sidecar
│   ├── mod.rs
│   ├── sidecar.rs            # Gestione processo sidecar
│   └── converter.rs          # Conversione JSON ↔ modelli Rust
│
├── profiles/                  # Gestione profili
│   ├── mod.rs
│   ├── manager.rs            # Manager profili
│   └── storage.rs            # Storage profili
│
├── monitoring/                # Monitoraggio file system
│   ├── mod.rs
│   ├── watcher.rs            # File system watcher
│   └── sync.rs               # Sincronizzazione automatica
│
└── utils/                     # Utility
    ├── mod.rs
    ├── error.rs              # Gestione errori
    └── paths.rs              # Gestione path
```

## Frontend Svelte (src/)

### Struttura Dettagliata

```
src/
├── lib/
│   ├── components/            # Componenti Svelte
│   │   ├── ui/                # Primitivi UI (Button, Input, Card, Badge, …) — vedi ui-primitives-standard, ui-component-catalog
│   │   ├── layout/
│   │   │   ├── TopBar.svelte
│   │   │   ├── Sidebar.svelte
│   │   │   └── ContentArea.svelte
│   │   ├── profile/
│   │   │   ├── ProfileSelector.svelte
│   │   │   └── ProfileManager.svelte
│   │   ├── pokedex/
│   │   │   ├── PokedexView.svelte
│   │   │   └── PokemonCard.svelte
│   │   ├── wiki/
│   │   │   ├── WikiView.svelte
│   │   │   └── WikiEntry.svelte
│   │   ├── editor/
│   │   │   └── SaveEditor.svelte
│   │   └── management/
│   │       └── AppManagement.svelte
│   │
│   ├── stores/                # Svelte stores
│   │   ├── profile.ts        # Store profilo attivo
│   │   ├── pokedex.ts        # Store Pokedex
│   │   ├── wiki.ts           # Store Wiki
│   │   └── app.ts            # Store app state
│   │
│   ├── services/              # Servizi frontend
│   │   ├── tauri.ts          # Wrapper comandi Tauri
│   │   ├── profile.ts        # Servizio profili
│   │   ├── pokedex.ts        # Servizio Pokedex
│   │   └── wiki.ts           # Servizio Wiki
│   │
│   └── utils/                 # Utility frontend
│       ├── types.ts          # TypeScript types
│       └── helpers.ts        # Helper functions
│
├── App.svelte                 # Root component
└── main.ts                    # Entry point
```

## Sidecar C# (src-sidecar/)

### Struttura Dettagliata

```
src-sidecar/
├── Program.cs                 # Entry point console app
├── Services/
│   └── ParserService.cs      # Servizio parsing PKHeX
├── Models/
│   └── SaveData.cs           # Modelli dati
└── PokeTracker.Parser.csproj
```

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

## Note

Questa struttura:
- Scalabile per app complessa
- Mantiene separazione chiara backend/frontend
- Facilita testing e manutenzione
- Segue best practice Tauri + Rust + Svelte
