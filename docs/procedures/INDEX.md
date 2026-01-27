# Index Procedure

## Obiettivo

Mappa query dell'utente alle procedure corrispondenti. Consulta questa mappa per identificare quale procedure leggere. Il matching è per **sostanza** della richiesta: varianti in italiano/inglese, singolare/plurale e formulazioni equivalenti vanno considerate match.

## Pattern Matching

### Bootstrap / Inizio sviluppo

- Query: "bootstrap", "avvia progetto", "setup iniziale", "prima implementazione", "init progetto", "crea progetto da zero", "first implementation", "da zero", "inizializza progetto", "scaffold", "crea app tauri", "inizializza", "avvia da zero"
- Procedure: `docs/procedures/workflow/project-bootstrap.md`

### Componenti Svelte

- Query: "crea componente", "aggiungi componente", "nuovo componente", "component svelte", "svelte component", "aggiungi .svelte", "nuovo componente svelte", "crea .svelte", "creare componente", "aggiungere componente"
- Procedure: `docs/procedures/svelte/component-creation.md`

### Store Svelte

- Query: "crea store", "aggiungi store", "nuovo store", "svelte store", "stato globale", "shared state", "aggiungi stato condiviso", "creare store", "aggiungere store"
- Procedure: `docs/procedures/svelte/store-setup.md`

### Pagine / Route SvelteKit

- Query: "crea pagina", "nuova pagina", "aggiungi pagina", "crea route", "nuova route", "aggiungi route", "nuova view", "+page.svelte", "sveltekit route", "creare pagina", "aggiungere route"
- Procedure: `docs/procedures/svelte/page-creation.md`

### Comandi Rust/Tauri

- Query: "crea comando", "aggiungi comando", "nuovo comando", "tauri command", "rust command", "aggiungi invoke", "nuovo comando tauri", "tauri invoke", "creare comando", "aggiungere comando"
- Procedure: `docs/procedures/rust/command-creation.md`

### Nuove Feature

- Query: "nuova feature", "aggiungi feature", "implementa feature", "nuova funzionalità", "aggiungi funzionalità", "implementa funzionalità", "nuova capacità", "aggiungi capacità", "new feature", "implementa"
- Procedure: `docs/procedures/workflow/new-feature.md`

### Bug Fix

- Query: "fix bug", "correggi bug", "risolvi bug", "bug fix", "fix", "correggi", "risolvi", "non funziona", "errore", "fix error", "correggi errore", "risolvi errore"
- Procedure: `docs/procedures/workflow/bug-fix.md`

### Sidecar (es. C# Parser)

- Query: "configura sidecar", "aggiungi sidecar", "sidecar C#", "sidecar parser", "configura parser C#", "aggiungi eseguibile esterno", "externalBin", "tauri sidecar"
- Procedure: `docs/procedures/workflow/sidecar-setup.md`

### Build / Deploy / Release

- Query: "build", "deploy", "release", "distribuisci", "pacchetto", "installer", "tauri build", "crea installer", "release build", "bundle"
- Procedure: `docs/procedures/workflow/deploy-release.md`

### Migrazioni Database

- Query: "aggiungi migrazione", "migrazione db", "cambia schema", "nuova migrazione", "migration", "schema db", "aggiungi tabella", "modifica tabella"
- Procedure: `docs/procedures/workflow/db-migration.md`

### Permissions e Capabilities Tauri

- Query: "aggiungi permesso", "configura permessi", "capability", "permissions", "tauri permissions", "configura capability", "scope", "allow command"
- Procedure: `docs/procedures/workflow/permissions-setup.md`

### Servizi Frontend (Wrapper Invoke)

- Query: "crea servizio", "aggiungi servizio", "servizio frontend", "wrapper invoke", "servizio tauri", "aggiungi servizio che chiama backend"
- Procedure: `docs/procedures/svelte/service-creation.md`

### Rimuovere Risorsa

- Query: "rimuovi", "elimina", "delete", "togli", "rimuovi componente", "elimina comando", "rimuovi feature", "delete file"
- Procedure: `docs/procedures/workflow/remove.md`

### Dipendenze

- Query: "aggiungi dipendenza", "aggiungi pacchetto", "npm install", "cargo add", "dotnet add", "aggiungi libreria", "install package", "aggiungi crate", "aggiungi nuget"
- Procedure: `docs/procedures/workflow/dependency-add.md`

### Test

- Query: "aggiungi test", "scrivi test", "nuovo test", "unit test", "integration test", "test case", "aggiungi unit test", "aggiungi test per"
- Procedure: `docs/procedures/workflow/test-creation.md`

### Refactoring

- Query: "refactor", "refactoring", "ristruttura", "ristrutturazione", "migra codice", "pulizia codice", "estrai componente", "estrai funzione", "rinomina", "sposta file"
- Procedure: `docs/procedures/workflow/refactor.md`

### Creare/Modificare File Markdown

- Query: "crea file md", "modifica file md", "crea standard", "documentazione", "crea guida", "crea procedure", "crea documentazione", "aggiungi doc"
- Procedure: `docs/procedures/markdown-creation.md`

### Modificare .cursorrules

- Query: "modifica cursorrules", "aggiorna cursorrules", "cambia regole", "aggiorna .cursorrules", "aggiungi regola", "modifica regole", "aggiorna regole"
- Procedure: `docs/procedures/cursorrules-update.md`

## Come Usare

1. Identifica pattern nella query utente (anche in forma libera o parziale)
2. Trova corrispondenza in questa mappa (il primo match per categoria conta)
3. Consulta procedure corrispondente
4. Segui checklist nella procedure

## Quando nessuna procedure corrisponde

Se nessun pattern corrisponde alla query:

- Non assumere che sia permesso procedere senza procedure
- Proporre all'utente: (a) riformulare la query per matchare un pattern esistente, (b) creare una nuova procedure per quel tipo di azione, oppure (c) confermare esplicitamente di voler procedere senza procedure (con consapevolezza del rischio)

## Note

- Questa mappa deve essere aggiornata quando vengono aggiunte nuove procedure
- Le cartelle `svelte/`, `rust/`, `workflow/` contengono le procedure rispettive
- In caso di overlap (es. "aggiungi feature X" può richiedere componenti + comandi), usare la procedure **più specifica** per l’azione principale; per feature ampie usare `new-feature.md` che richiama le altre
