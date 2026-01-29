# Index Procedure

## Obiettivo

Mappa query dell'utente alle procedure corrispondenti. Consulta questa mappa per identificare **quale procedura** leggere. Il matching è per **sostanza** della richiesta: varianti in italiano/inglese, singolare/plurale e formulazioni equivalenti vanno considerate match.

## Regole ferree (nessuna eccezione)

- **Non approssimare:** usare solo ciò che è documentato in procedure e standard.
- **Non indovinare:** non assumere path, nomi, strutture; verificare nei file indicati.
- **Non inventare:** nessuna convenzione o implementazione non documentata.
- Per ogni azione: 1) identificare la procedura da questa mappa, 2) leggere la procedura per intero, 3) completare la checklist, 4) leggere tutti gli standard referenziati, 5) solo dopo proporre e (se confermato) implementare.

## Pattern Matching

### Bootstrap / Inizio sviluppo

- Query: "bootstrap", "avvia progetto", "setup iniziale", "prima implementazione", "init progetto", "crea progetto da zero", "first implementation", "da zero", "inizializza progetto", "scaffold", "crea app tauri", "inizializza", "avvia da zero"
- Procedure: `docs/procedures/workflow/project-bootstrap.md`

### Modifica Navigazione / Sidebar / Layout

- Query: "aggiungi voce sidebar", "aggiungi voce menu", "modifica navigazione", "aggiungi sezione Profilo", "cambia ordine voci", "modifica layout", "aggiungi sottovoce Wiki", "rimuovi voce menu", "aggiungi voce Impostazioni"
- Procedure: `docs/procedures/workflow/layout-navigation-change.md`

### Layout Responsive / Breakpoint / Adattivo

- Query: "rendi responsive", "design responsive", "layout mobile", "breakpoint", "layout adattivo", "adatta a viewport", "mobile-first", "sidebar su schermo piccolo", "tipografia fluida", "container query"
- Procedure: `docs/procedures/workflow/responsive-design-change.md`
- Standard: `docs/standards/responsive-design-standard.md`

### Componenti Svelte

- Query: "crea componente", "aggiungi componente", "nuovo componente", "component svelte", "svelte component", "aggiungi .svelte", "nuovo componente svelte", "crea .svelte", "creare componente", "aggiungere componente", "aggiungi componente UI", "componente UI", "aggiungi button", "aggiungi shadcn"
- Procedure: `docs/procedures/svelte/component-creation.md`
- Per componenti UI usare **obbligatoriamente** Tailwind + shadcn-svelte (componenti da `$lib/components/ui`); vedi `docs/standards/ui-stack-standard.md`.

### Store Svelte

- Query: "crea store", "aggiungi store", "nuovo store", "svelte store", "stato globale", "shared state", "aggiungi stato condiviso", "creare store", "aggiungere store"
- Procedure: `docs/procedures/svelte/store-setup.md`

### Pagine / Route SvelteKit

- Query: "crea pagina", "nuova pagina", "aggiungi pagina", "crea route", "nuova route", "aggiungi route", "nuova view", "+page.svelte", "sveltekit route", "creare pagina", "aggiungere route"
- Procedure: `docs/procedures/svelte/page-creation.md`
- Per UI della pagina: `docs/standards/ui-stack-standard.md`, `docs/standards/design-system-standard.md`
- **Se la pagina include una lista o tabella che può essere vuota:** applicare obbligatoriamente la regola empty state in `docs/project/ui-patterns-applied.md` (sezione Empty state) e usare il componente `$lib/components/ui/empty-state`.

### Empty state (liste / tabelle vuote)

- Query: "empty state", "stato vuoto", "lista vuota", "tabella vuota", "nessun dato", "vista quando non ci sono dati", "vuota quando non ci sono"
- Regola: `docs/project/ui-patterns-applied.md` (sezione Empty state). Usare il componente `$lib/components/ui/empty-state`; in empty state mostrare solo il CTA centrato, non il pulsante in header (CardAction). Riferimento: `src/routes/profilo/salvataggi/+page.svelte`, `src/routes/impostazioni/profili/+page.svelte`.

### Form e Validazione

- Query: "aggiungi form", "validazione form", "form schema", "validazione input form", "crea form", "form validation"
- Procedure: `docs/procedures/svelte/form-creation.md`
- Per UI del form (input, button, label): `docs/standards/ui-stack-standard.md`, `docs/standards/design-system-standard.md`

### Comandi Rust/Tauri

- Query: "crea comando", "aggiungi comando", "nuovo comando", "tauri command", "rust command", "aggiungi invoke", "nuovo comando tauri", "tauri invoke", "creare comando", "aggiungere comando"
- Procedure: `docs/procedures/rust/command-creation.md`

### Comando che usa DB

- Query: "comando che usa DB", "comando con query", "comando che legge/scrive DB", "aggiungi comando con database", "comando che usa SQLite", "comando con persistenza", "comando che accede al db"
- Procedure: `docs/procedures/workflow/command-with-db.md`

### Nuove Feature

- Query: "nuova feature", "aggiungi feature", "implementa feature", "nuova funzionalità", "aggiungi funzionalità", "implementa funzionalità", "nuova capacità", "aggiungi capacità", "new feature", "implementa"
- Procedure: `docs/procedures/workflow/new-feature.md`
- **Se la feature introduce una vista con lista/tabella che può essere vuota:** applicare la regola empty state in `docs/project/ui-patterns-applied.md` (sezione Empty state) e usare il componente `$lib/components/ui/empty-state`.

### Bug Fix

- Query: "fix bug", "correggi bug", "risolvi bug", "bug fix", "fix", "correggi", "risolvi", "non funziona", "errore", "fix error", "correggi errore", "risolvi errore"
- Procedure: `docs/procedures/workflow/bug-fix.md`
- **Se il bug è "UI non si aggiorna" / "elemento non cambia aspetto"** (es. icona che dovrebbe cambiare colore ma resta uguale): obbligatoria la sezione **"Bug UI non riflette lo stato"** nella procedura (catena stato → reattività → CSS; dopo 2–3 tentativi falliti: ipotesi + strumentazione, non iterare a caso).

### Correzione errore 404 (route non trovata)

- Query: "404", "errore 404", "get X non esiste", "route non trovata", "correggi 404", "fix 404", "continua ad apparire 404"
- Procedure: `docs/procedures/workflow/error-404-correction.md`
- Standard: `docs/standards/fix-without-workaround-standard.md` — fix causa radice, no workaround (redirect/route fantasma).

### Modifica Comportamento (Behavior Change)

- Query: "cambia comportamento", "modifica come funziona", "cambia logica di", "modifica comportamento di", "cambia il funzionamento di", "vuoi che faccia", "invece di X faccia Y", "cambia la logica", "modifica il flusso"
- Procedure: `docs/procedures/workflow/behavior-change.md`

### Loading / Sync / Watcher (UX)

- Query: "loading UX", "stati di caricamento", "overlay sync", "watcher loading", "più sync insieme", "best practice loading", "inline vs overlay", "feedback sync"
- Doc: `docs/project/loading-and-sync-ux.md` — best practice UX per loading, sync e watcher (overlay vs inline, soglie, multi-sync).

### Gestione Errori (User-Facing / Toast / Logging / Archivio)

- Query: "gestisci errore", "user-facing error", "toast errore", "error boundary", "messaggio di fallimento", "aggiungi toast", "notifica errore", "collegare errore a archivio", "aggiungi caso errore sistema", "errore in archivio", "reportSystemError"
- Procedure: `docs/procedures/workflow/error-handling.md`
- Se l’errore deve andare in Archivio Errori (errore di sistema): leggere **Standard operativo** in `docs/project/notifications-and-error-archive.md` e usare `reportSystemError` da `$lib/stores/error-archive.ts`.

### Accessibilità (a11y)

- Query: "accessibilità", "a11y", "aggiungi a11y", "fix a11y", "aria", "keyboard", "screen reader", "aggiungi label", "contrasto"
- Procedure: `docs/procedures/workflow/accessibility-setup.md`

### Validazione Input / Sicurezza

- Query: "validazione input", "sanitizza", "sicurezza input", "input validation", "never trust frontend", "valida argomenti"
- Procedure: `docs/procedures/workflow/input-validation.md`

### Verifica Sicurezza / Security Audit

- Query: "verifica sicurezza", "security audit", "sicurezza generale", "controlla sicurezza", "audit sicurezza", "check sicurezza"
- Procedure: `docs/procedures/workflow/security-audit.md`

### Logging

- Query: "aggiungi logging", "logging", "log level", "configura log", "debug log", "log livello"
- Procedure: `docs/procedures/workflow/logging-setup.md`

### Sidecar (es. C# Parser)

- Query: "configura sidecar", "aggiungi sidecar", "sidecar C#", "sidecar parser", "configura parser C#", "aggiungi eseguibile esterno", "externalBin", "tauri sidecar"
- Procedure: `docs/procedures/workflow/sidecar-setup.md`

### Versionamento / Commit-Push con versione

- Query: "commit e push", "push con versione", "rilascia versione", "versionamento", "decidere versione", "fare release", "scrivere nel commit la versione", "aggiorna changelog", "aggiungi a VERSION-HISTORY", "preparare push con versione", "quale versione dare"
- Procedure: `docs/procedures/workflow/versioning-release.md`

### Build / Deploy / Release / CI

- Query: "build", "deploy", "release", "distribuisci", "pacchetto", "installer", "tauri build", "crea installer", "release build", "bundle", "configura CI", "pipeline", "GitHub Actions", "pipeline release"
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

### Integra libreria / Primo uso

- Query: "integrazione libreria", "integrazione libreria X", "primo uso libreria", "setup libreria", "come usare libreria", "configura libreria", "integra pacchetto", "integra libreria"
- Procedure: `docs/procedures/workflow/library-integration.md`

### Test

- Query: "aggiungi test", "scrivi test", "nuovo test", "unit test", "integration test", "test case", "aggiungi unit test", "aggiungi test per"
- Procedure: `docs/procedures/workflow/test-creation.md`

### Estrai componente

- Query: "estrai componente", "estrai in nuovo file", "spezza componente", "estrai blocco in componente", "estrai blocco in nuovo componente"
- Procedure: `docs/procedures/workflow/extract-component.md`

### Refactoring

- Query: "refactor", "refactoring", "ristruttura", "ristrutturazione", "migra codice", "pulizia codice", "estrai funzione", "rinomina", "sposta file"
- Procedure: `docs/procedures/workflow/refactor.md`

### Creare/Modificare File Markdown

- Query: "crea file md", "modifica file md", "crea standard", "documentazione", "crea guida", "crea procedure", "crea documentazione", "aggiungi doc"
- Procedure: `docs/procedures/markdown-creation.md`

### Modificare .cursorrules

- Query: "modifica cursorrules", "aggiorna cursorrules", "cambia regole", "aggiorna .cursorrules", "aggiungi regola", "modifica regole", "aggiorna regole"
- Procedure: `docs/procedures/cursorrules-update.md`

### i18n / Traduzioni

- Query: "traduci", "aggiungi lingua", "i18n", "localizzazione", "stringhe tradotte", "nuova lingua", "internazionalizzazione"
- Procedure: `docs/procedures/workflow/i18n-setup.md`

### Performance

- Query: "ottimizza", "performance", "lazy load", "virtualizza lista", "indice DB", "ottimizza performance", "lista lenta", "rendering lento"
- Procedure: `docs/procedures/workflow/performance-optimization.md`

### Contratto API (Frontend–Backend)

- Query: "cambia contratto", "aggiungi campo", "breaking change API", "aggiungi parametro comando", "modifica tipo ritorno"
- Procedure: `docs/procedures/workflow/api-contract-change.md`

## Come Usare

1. Identifica pattern nella query utente (anche in forma libera o parziale).
2. Trova corrispondenza in questa mappa (il primo match per categoria conta).
3. **Leggi per intero** la procedura corrispondente (non riassunti).
4. **Completa la checklist** della procedura punto per punto; apri i file indicati.
5. Leggi tutti gli standard referenziati nella procedura.
6. Solo dopo: proponi implementazione; implementa solo dopo conferma utente.

## Quando nessuna procedura corrisponde

Se nessun pattern corrisponde alla query:

- Non assumere che sia permesso procedere senza procedure
- Proporre all'utente: (a) riformulare la query per matchare un pattern esistente, (b) creare una nuova procedure per quel tipo di azione, oppure (c) confermare esplicitamente di voler procedere senza procedure (con consapevolezza del rischio)

**Se l'utente conferma di procedere senza procedure:** prima di implementare consultare obbligatoriamente `docs/standards/README.md` e gli standard lì elencati rilevanti per il dominio (es. performance → `performance-standard.md`, sicurezza → `security-standard.md`, build/CI → `ci-cd-standard.md`). Per termini di dominio: `docs/project/glossary.md`. Per vista architettura: `docs/project/architecture-overview.md`.

## Note

- Questa mappa deve essere aggiornata quando vengono aggiunte nuove procedure
- Le cartelle `svelte/`, `rust/`, `workflow/` contengono le procedure rispettive
- Per "crea componente" usare **Componenti Svelte** (`component-creation.md`)
- In caso di overlap (es. "aggiungi feature X" può richiedere componenti + comandi), usare la procedura **più specifica** per l’azione principale; per feature ampie usare `new-feature.md` che richiama le altre
