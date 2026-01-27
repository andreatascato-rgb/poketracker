# Tracker Ricerche e File – TEMPORANEO

> **Nota:** File temporaneo per guidare il lavoro. Rimuovere o archiviare al termine.

## Obiettivo

Riepilogo di tutte le ricerche da fare (standard e best practice) e dei file da creare.  
Workflow: **una ricerca → un file per quel tema → aggiorna questo tracker → prossima ricerca.**

---

## Parte 1 – Ricerche su standard e best practice

### 1. Tauri 2

- **Stato:** [x] Fatto
- **Cosa cercare:** breaking changes vs 1.x, setup frontend consigliato, template ufficiali Tauri + Svelte/SvelteKit, best practice 2025-2026
- **File creato:** `docs/standards/tauri2-standard.md`

### 2. Svelte 5 / SvelteKit 2

- **Stato:** [x] Fatto
- **Cosa cercare:** runes, nuovi pattern reattività, integrazione con Tauri (SPA vs SSR, build, dev server)
- **File creato:** `docs/standards/svelte-sveltekit-standard.md`

### 3. Sidecar in Tauri 2

- **Stato:** [x] Fatto
- **Cosa cercare:** come si configura e invoca un sidecar (es. .exe C#), path, permessi, best practice
- **File creato:** `docs/standards/tauri2-sidecar-standard.md`

### 4. Rust (backend Tauri)

- **Stato:** [x] Fatto
- **Cosa cercare:** edition, crates consigliate, DB (rusqlite/sqlx), error handling, logging
- **File creato:** `docs/standards/rust-tauri-standard.md`

### 5. C# sidecar

- **Stato:** [x] Fatto
- **Cosa cercare:** target framework, contratto I/O con Rust (stdin/stdout, JSON), dove buildare, come Tauri lo trova
- **File creato:** `docs/standards/csharp-sidecar-standard.md`

### 6. TypeScript / Frontend

- **Stato:** [x] Fatto
- **Cosa cercare:** strict mode, path alias, tipi condivisi con Rust, @tauri-apps/api, pattern invocazione comandi
- **File creato:** `docs/standards/typescript-frontend-standard.md`

### 7. Tooling (lint, format, test)

- **Stato:** [x] Fatto
- **Cosa cercare:** ESLint/Prettier/Biome per Svelte+TS, rustfmt/clippy, C# formattazione, dove documentare
- **File creato:** `docs/standards/tooling-standard.md`

### 8. Build / Deploy / Release Tauri 2

- **Stato:** [x] Fatto
- **Cosa cercare:** tauri build, --no-bundle, formati installer (MSI/DMG/AppImage), tauri-action, code signing, updater
- **File creato:** `docs/standards/tauri2-build-deploy-standard.md`

### 9. Permissions e Capabilities Tauri 2

- **Stato:** [x] Fatto
- **Cosa cercare:** permissions, capabilities, identifier, scope, src-tauri/permissions/, src-tauri/capabilities/
- **File creato:** `docs/standards/tauri2-permissions-standard.md`

### 10. Migrazioni DB (Rust / SQLite)

- **Stato:** [x] Fatto
- **Cosa cercare:** rusqlite_migration, sqlx migrate, M::up, to_latest, user_version, best practice
- **File creato:** `docs/standards/database-migrations-standard.md`

---

## Parte 2 – File creati (output delle ricerche)

| # | Argomento   | File creato | Note |
|---|-------------|-------------|------|
| 1 | Tauri 2     | `docs/standards/tauri2-standard.md` | versione, create-tauri-app, SvelteKit, tauri.conf, permissions |
| 2 | Svelte/Kit  | `docs/standards/svelte-sveltekit-standard.md` | runes $state/$derived/$effect, $props/$bindable, adapter-static, ssr=false, $effect cleanup, onMount vs $effect |
| 3 | Sidecar     | `docs/standards/tauri2-sidecar-standard.md` | externalBin, target triple, shell plugin, sidecar(), permissions shell:allow-execute, C# build path |
| 4 | Rust        | `docs/standards/rust-tauri-standard.md` | edition 2021, serde/thiserror/anyhow, Result+Serialize, tauri-plugin-log, DB tauri-plugin-sql vs rusqlite/sqlx |
| 5 | C# sidecar  | `docs/standards/csharp-sidecar-standard.md` | .NET 8 LTS, PublishSingleFile, nome parser-x86_64-pc-windows-msvc.exe, args in / JSON stdout, UTF-8, src-tauri/binaries/ |
| 6 | TS/Frontend | `docs/standards/typescript-frontend-standard.md` | strict, $lib/alias, invoke da @tauri-apps/api/core, try/catch, tipi condivisi (typegen vs manuale), servizi |
| 7 | Tooling     | `docs/standards/tooling-standard.md` | ESLint+Prettier vs Biome, rustfmt.toml/clippy in src-tauri, .editorconfig + dotnet format C#, tabella path/comandi |

---

## Parte 3 – Procedure da definire (dopo le ricerche)

- [x] `docs/procedures/workflow/project-bootstrap.md`
- [x] Aggiornare `docs/procedures/INDEX.md` con pattern “bootstrap / inizio sviluppo”
- [x] Procedure atomiche: component-creation, store-setup, page-creation, command-creation, new-feature, bug-fix, sidecar-setup, dependency-add, test-creation, refactor

---

## Log operativo

- **Creato:** 2026-01-27 — File tracker iniziale.
- **Ricerca 1 (Tauri 2):** completata — creato `docs/standards/tauri2-standard.md`. Contenuti: Tauri 2 stabile, create-tauri-app, SvelteKit + adapter-static + ssr=false, tauri.conf (devUrl, frontendDist), breaking changes vs 1.x, permissions al posto allowlist, riferimenti doc ufficiale.
- **Ricerca 2 (Svelte 5 / SvelteKit 2):** completata — creato `docs/standards/svelte-sveltekit-standard.md`. Contenuti: Svelte 5 stabile ott 2024, runes $state/$derived/$effect, $props/$bindable, $effect con teardown return, onMount vs $effect e `<svelte:window>`, SvelteKit 2 + adapter-static + ssr=false per Tauri, migrazione sv migrate svelte-5, riferimenti doc.
- **Ricerca 3 (Sidecar Tauri 2):** completata — creato `docs/standards/tauri2-sidecar-standard.md`. Contenuti: externalBin in bundle, target triple e naming, path src-tauri/binaries/, shell plugin e sidecar(), args e spawn, permissions shell:allow-execute e scope sidecar, note su stdin non sempre affidabile, sidecar C# e path build, riferimenti doc.
- **Ricerca 4 (Rust backend Tauri):** completata — creato `docs/standards/rust-tauri-standard.md`. Contenuti: edition 2021 (bug 2024 con tauri-build), serde/thiserror/anyhow, error nei command Result+Serialize e no panic, tauri-plugin-log, DB: tauri-plugin-sql vs rusqlite/sqlx, riferimenti a database-storage e parser-architecture.
- **Ricerca 5 (C# sidecar):** completata — creato `docs/standards/csharp-sidecar-standard.md`. Contenuti: .NET 8 LTS, console app, PublishSingleFile+SelfContained, nome exe con target triple, path src-tauri/binaries/, contratto args in / JSON stdout, UTF-8, post-build o script per copy, riferimenti a tauri2-sidecar e parser-architecture.
- **Ricerca 6 (TypeScript / Frontend):** completata — creato `docs/standards/typescript-frontend-standard.md`. Contenuti: TypeScript strict, tsconfig extends .svelte-kit, path alias $lib e kit.alias, invoke da @tauri-apps/api/core, await e try/catch, tipi condivisi (tauri-typegen/tauri-plugin-typegen vs manuale), servizi per incapsulare invoke, riferimenti a rust-tauri e project-structure.
- **Ricerca 7 (Tooling):** completata — creato `docs/standards/tooling-standard.md`. Contenuti: frontend ESLint+Prettier (sv add eslint) vs Biome v2.3 (Svelte sperimentale), Rust rustfmt.toml e clippy in src-tauri, C# .editorconfig e dotnet format, tabella path/comandi, riferimento a testing-strategy.
