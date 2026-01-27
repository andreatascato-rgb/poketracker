# Vista Architettura - PokeTracker

## Obiettivo

Descrive la vista tecnica d'insieme: stack, flusso dati e dove vivono le responsabilità. Usare per capire "dove va questa logica?", "chi chiama chi?", "dove sono i dati?". Dettagli su cartelle e file: [project-structure](./project-structure.md).

## Stack

| Layer | Tecnologia | Path principale | Ruolo |
|-------|------------|-----------------|--------|
| **Frontend** | Svelte 5 + SvelteKit 2 | `src/` | UI, routing, stato locale; chiama backend via `invoke`. |
| **Backend** | Rust + Tauri 2 | `src-tauri/` | Comandi IPC, logica di business, DB, integrazione sidecar. |
| **Sidecar** | C# + PKHeX | `src-sidecar/` | Parsing file `.sav`; eseguito come processo esterno da Rust. |
| **Dati** | SQLite (Rust) + eventuale knowledge DB | `src-tauri/src/db/` | Persistenza profili, Pokedex, dati estratti; knowledge per wiki. |

- **Build:** Tauri bundlea frontend (output SvelteKit) + binari Rust + sidecar C#. Vedi [tauri2-build-deploy-standard](../standards/tauri2-build-deploy-standard.md), [deployment](./deployment.md).
- **Offline:** Tutti i dati e la logica sono locali; nessuna dipendenza da rete per core e wiki. Vedi [core-functionality](./core-functionality.md).

## Flusso dati (da UI a disco)

1. **Utente** agisce nell’UI (Svelte): click, form, navigazione.
2. **Componenti / store** chiamano i **servizi frontend** (`src/lib/services/`) che incapsulano `invoke`.
3. **`invoke('command_name', { ... })`** invia la richiesta al **backend Tauri** (Rust).
4. **Comando** (`src-tauri/src/commands/`) riceve i parametri, delega alla **logica di business** (`services/`) e/o al **DB** (`db/`), eventualmente al **sidecar** (`parser/`).
5. **Sidecar** (se usato): Rust avvia il processo C#, passa argomenti (es. path file), riceve JSON; converte in modelli Rust e prosegue.
6. **DB** (Rust): lettura/scrittura SQLite tramite `db/connection.rs`, `db/repositories/`, migrazioni in `db/migrations.rs`. Vedi [database-storage](./database-storage.md), [database-migrations-standard](../standards/database-migrations-standard.md).
7. **Risposta** del comando torna al frontend via IPC; i componenti/store aggiornano lo stato e l’UI.

**Profilo attivo:** il frontend e il backend condividono l’idea di “profilo corrente” (selettore in Top Bar). I comandi che operano su dati di profilo ricevono l’id profilo (o lo leggono da contesto) e filtrano per quello. Dati profilo e Pokedex vivono nel DB Rust, isolati per profilo. Vedi [multi-profile](./multi-profile.md), [core-functionality](./core-functionality.md).

## Dove vivono cosa

| Dato / responsabilità | Layer | Path o componente tipo |
|------------------------|--------|--------------------------|
| **Profilo attivo, preferenze UI** | Frontend | Store `app` / `profile`; selettore in Top Bar. |
| **Profili, cartelle salvataggi, Pokedex per profilo** | Backend + DB | `profiles/`, `db/repositories/`, comandi `profile`, `pokedex`. |
| **File `.sav`**, parsing | Sidecar + Rust | Sidecar C# (PKHeX); Rust in `parser/` chiama sidecar e converte risultati. Vedi [parser-architecture](./parser-architecture.md). |
| **Knowledge (Pokemon, mosse, nature, wiki)** | Backend + DB / risorse | Knowledge DB o bundle; comandi `wiki`; frontend legge via invoke. Vedi [knowledge-database](./knowledge-database.md). |
| **Navigazione, layout** | Frontend | `lib/components/layout/` (TopBar, Sidebar, ContentArea); route in `src/routes/`. Vedi [ui-ux-design](./ui-ux-design.md). |
| **Primitivi UI** | Frontend | `lib/components/ui/`. Vedi [ui-component-catalog](./ui-component-catalog.md). |

## Comunicazione Frontend ↔ Backend

- **Solo** tramite **comandi Tauri** esposti in `src-tauri` e invocati con `invoke` da Svelte.
- **Niente** chiamate HTTP verso server esterno per la logica core; tutto passa da IPC Tauri.
- **Servizi frontend** (`lib/services/`) nascondono i dettagli di `invoke` e incapsulano errore/retry; i componenti usano i servizi, non `invoke` sparso. Vedi [typescript-frontend-standard](../standards/typescript-frontend-standard.md), [rust-tauri-standard](../standards/rust-tauri-standard.md).

## Sidecar e parser

- **Sidecar:** eseguibile C# (es. da `src-tauri/binaries/` o path configurato) lanciato da Rust per operazioni su file `.sav`.
- **Flusso:** Rust prepara argomenti → spawn processo sidecar → sidecar usa PKHeX → output JSON → Rust deserializza e usa nei modelli/servizi.
- **Scope e permessi:** definiti in Tauri (capabilities, scope sidecar). Vedi [tauri2-sidecar-standard](../standards/tauri2-sidecar-standard.md), [parser-architecture](./parser-architecture.md).

## Riferimenti

- [project-structure](./project-structure.md) — Albero cartelle e file
- [core-functionality](./core-functionality.md) — Offline, persistenza, profili, monitoraggio save
- [parser-architecture](./parser-architecture.md) — Sidecar, PKHeX, integrazione Rust
- [knowledge-database](./knowledge-database.md) — Knowledge DB, dimensioni, uso wiki
- [database-storage](./database-storage.md) — Storage e DB
- [glossary](./glossary.md) — Termini (profilo, invoke, sidecar, ecc.)
