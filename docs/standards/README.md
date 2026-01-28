# Standard del Progetto

## Obiettivo

Questa cartella contiene tutti gli standard e le convenzioni stabilite per il progetto PokeTracker. Ogni standard definisce regole e best practice da seguire sempre.

## Come funziona

1. Ogni standard viene documentato in un file dedicato
2. Gli standard vengono anche aggiunti a `.cursorrules` per applicazione automatica
3. Quando viene presa una decisione, diventa uno standard da seguire sempre

**Allineamento:** gli standard sono orientati alle best practice 2026; è raccomandata una revisione periodica (annuale o a scadenza definita).

## Standard Attivi

Tutti i file `.md` in questa cartella (escluso questo README) sono standard del progetto.

| File | Descrizione |
|------|-------------|
| [file-organization.md](./file-organization.md) | Organizzazione file di documentazione |
| [cursorrules-standard.md](./cursorrules-standard.md) | Standard per compilare `.cursorrules` |
| [markdown-optimization.md](./markdown-optimization.md) | Markdown ottimizzato per AI |
| [guide-structure.md](./guide-structure.md) | Struttura guide obbligatorie |
| [versioni-stack-standard.md](./versioni-stack-standard.md) | Versioni stack, comandi verificati, anti-pattern (Svelte 5 / Tauri 2 / Rust) |
| [coding-best-practices.md](./coding-best-practices.md) | Best practice bridge/panic-free/Svelte/workflow (checkpoint AI, rimandi agli standard) |
| [tauri2-standard.md](./tauri2-standard.md) | Standard Tauri 2 |
| [tauri2-sidecar-standard.md](./tauri2-sidecar-standard.md) | Standard sidecar Tauri 2 |
| [rust-tauri-standard.md](./rust-tauri-standard.md) | Standard Rust/Tauri |
| [svelte-sveltekit-standard.md](./svelte-sveltekit-standard.md) | Standard Svelte/SvelteKit |
| [typescript-frontend-standard.md](./typescript-frontend-standard.md) | Standard TypeScript frontend |
| [csharp-sidecar-standard.md](./csharp-sidecar-standard.md) | Standard sidecar C# |
| [tooling-standard.md](./tooling-standard.md) | Standard tooling |
| [tauri2-build-deploy-standard.md](./tauri2-build-deploy-standard.md) | Build, deploy e release Tauri 2 |
| [ci-cd-standard.md](./ci-cd-standard.md) | CI/CD, pipeline, quality gates, release flow |
| [tauri2-permissions-standard.md](./tauri2-permissions-standard.md) | Permissions e capabilities Tauri 2 |
| [database-migrations-standard.md](./database-migrations-standard.md) | Migrazioni DB SQLite (Rust) |
| [error-handling-standard.md](./error-handling-standard.md) | Gestione errori user-facing, toast, logging |
| [forms-validation-standard.md](./forms-validation-standard.md) | Form e validazione (frontend + backend) |
| [input-validation-standard.md](./input-validation-standard.md) | Validazione input e sicurezza (never trust frontend) |
| [security-standard.md](./security-standard.md) | Sicurezza (file system, DB, sidecar, Tauri, dati sensibili) |
| [accessibility-standard.md](./accessibility-standard.md) | Accessibilità (a11y), WCAG, ARIA |
| [design-system-standard.md](./design-system-standard.md) | Design system: stile, token, tipografia, layout, icone (standard Poketrack, dark) |
| [interaction-states-standard.md](./interaction-states-standard.md) | Stati interattivi: hover (transizione), active (istantaneo), no scale; per pulsanti, nav, icon-btn |
| [responsive-design-standard.md](./responsive-design-standard.md) | Design responsive: mobile-first, breakpoint, unità fluide, touch, container queries (best practice 2026) |
| [ui-stack-standard.md](./ui-stack-standard.md) | Stack UI: Tailwind + shadcn-svelte (bits-ui), best practice, anti-pattern |
| [breadcrumb-standard.md](./breadcrumb-standard.md) | Breadcrumb: posizione (alto a sinistra), stile, touch, accessibilità |
| [logging-standard.md](./logging-standard.md) | Logging backend e frontend, livelli, formato |
| [i18n-standard.md](./i18n-standard.md) | Internazionalizzazione, traduzioni, locale |
| [performance-standard.md](./performance-standard.md) | Performance, lazy load, virtualizzazione, DB, IPC |
| [api-contract-standard.md](./api-contract-standard.md) | Evoluzione contratto API, semver, breaking vs non-breaking |
| [versioning-standard.md](./versioning-standard.md) | Versionamento SemVer, commit con versione, VERSION-HISTORY |
| [testing-standard.md](./testing-standard.md) | Testing, dove/naming, mock Tauri (mockIPC, clearMocks) |