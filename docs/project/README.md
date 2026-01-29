# Documentazione Progetto PokeTracker

## Obiettivo

Questa cartella contiene le informazioni sul progetto e sull'applicazione PokeTracker: requisiti, funzionalità, architettura e strategie trasversali.

## Entry point

Per una panoramica generale dell'app, inizia da **[overview.md](./overview.md)**. Per termini di dominio e tecnici: **[glossary.md](./glossary.md)**.  
Per priorità in caso di trade-off: **[priorities.md](./priorities.md)**.  
Per partire dallo sviluppo: **[piano-sviluppo-iniziale.md](./piano-sviluppo-iniziale.md)** — prime 5 azioni in ordine di priorità.

## Documenti

### Piano di sviluppo

- **[piano-sviluppo-iniziale.md](./piano-sviluppo-iniziale.md)** — Piano breve: bootstrap → layout → profilo → DB → prima pagina contenuto

### Panoramica e core

- **[overview.md](./overview.md)** - Panoramica generale dell'app
- **[glossary.md](./glossary.md)** - Glossario termini di dominio e tecnici
- **[priorities.md](./priorities.md)** - Priorità di progetto (trade-off)
- **[poketrack-reference.md](./poketrack-reference.md)** - Riferimento visivo: cosa adottiamo dall'app Poketrack (layout, token, roadmap)
- **[ui-patterns-applied.md](./ui-patterns-applied.md)** - Pattern UI applicati (empty state, card, CTA, padding)
- **[core-functionality.md](./core-functionality.md)** - Funzionalità core
- **[features.md](./features.md)** - Elenco e documentazione delle features

### Features (dettaglio)

- **[pokedex-personal.md](./pokedex-personal.md)** - Pokedex personale
- **[pokedex-sprites.md](./pokedex-sprites.md)** - Sprite Pokedex (offline, gen 1–4, script download)
- **[wiki-offline.md](./wiki-offline.md)** - Wiki offline
- **[save-editor.md](./save-editor.md)** - Editor salvataggi (PKHeX-like)
- **[multi-profile.md](./multi-profile.md)** - Gestione multi-profilo
- **[self-management.md](./self-management.md)** - Gestione interna app

### Architettura e dati

- **[architecture-overview.md](./architecture-overview.md)** - Vista architettura (stack, flusso dati, dove vivono i dati)
- **[ux-layout-decision-2026.md](./ux-layout-decision-2026.md)** - Valutazione UX: perché sidebar+Top Bar è la soluzione migliore per il 2026, miglioramenti concreti
- **[offline-data-strategy.md](./offline-data-strategy.md)** - Strategia offline e dati locali
- **[project-structure.md](./project-structure.md)** - Struttura progetto
- **[parser-architecture.md](./parser-architecture.md)** - Architettura parser
- **[pokedex-parser-strategy.md](./pokedex-parser-strategy.md)** - Strategia: parser Pokedex per gioco/versione (solo tracciamento Pokedex)
- **[sav-game-version-detection.md](./sav-game-version-detection.md)** - Spec: riconoscimento gioco/versione da .sav (PKHeX, sidecar C#)
- **[knowledge-database.md](./knowledge-database.md)** - Database conoscenza
- **[database-storage.md](./database-storage.md)** - Database e storage

### Strategie trasversali

- **[loading-and-sync-ux.md](./loading-and-sync-ux.md)** - Best practice UX per stati di caricamento, sync e watcher (overlay vs inline, soglie, multi-sync)
- **[error-handling.md](./error-handling.md)** - Gestione errori
- **[notifications-and-error-archive.md](./notifications-and-error-archive.md)** - Notifiche, Archivio sidebar, sottosezione Errori; standard per errori → avviso + notifica + registro (log completo da incollare)
- **[testing-strategy.md](./testing-strategy.md)** - Strategia testing
- **[deployment.md](./deployment.md)** - Deployment e distribuzione
- **[performance.md](./performance.md)** - Ottimizzazione performance
- **[security.md](./security.md)** - Sicurezza

## Come usare

1. Leggi [overview.md](./overview.md) per il quadro generale
2. Usa [features.md](./features.md) per orientarti sulle funzionalità
3. Approfondi i singoli documenti in base al contesto

## Note

Per la struttura complessiva di `docs/` vedi [../README.md](../README.md).

