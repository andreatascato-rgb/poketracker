# Documentazione PokeTracker

## Obiettivo

Questa cartella contiene tutta la documentazione del progetto PokeTracker, organizzata per garantire massima leggibilità da parte di AI e sviluppatori.

## Struttura

```
docs/
├── standards/     # Standard e convenzioni del progetto
├── procedures/    # Procedure operative per azioni specifiche
├── project/       # Informazioni sul progetto e applicazione
├── temp/          # File temporanei (valutazioni, confronti, bozze)
└── README.md      # Questo file
```

## Standard

Tutti gli standard del progetto sono documentati in `docs/standards/`:

- **[file-organization.md](./standards/file-organization.md)** - Organizzazione file di documentazione
- **[cursorrules-standard.md](./standards/cursorrules-standard.md)** - Standard per compilare `.cursorrules`
- **[markdown-optimization.md](./standards/markdown-optimization.md)** - Standard per markdown ottimizzato per AI
- **[guide-structure.md](./standards/guide-structure.md)** - Struttura guide obbligatorie

Vedi [standards/README.md](./standards/README.md) per la lista completa.

## Procedure

Le procedure operative per azioni specifiche sono in `docs/procedures/`:

- **[INDEX.md](./procedures/INDEX.md)** - Mappa query → procedure corrispondenti

Le procedure verranno create quando necessario durante lo sviluppo.

## Progetto

Le informazioni sul progetto e sull'applicazione sono in `docs/project/`. Vedi **[project/README.md](./project/README.md)** per obiettivo, entry point ed elenco raggruppato.

- **[overview.md](./project/overview.md)** - Panoramica generale dell'app
- **[glossary.md](./project/glossary.md)** - Glossario termini di dominio e tecnici
- **[architecture-overview.md](./project/architecture-overview.md)** - Vista architettura (stack, flusso dati)
- **[offline-data-strategy.md](./project/offline-data-strategy.md)** - Strategia offline e dati locali
- **[core-functionality.md](./project/core-functionality.md)** - Funzionalità core dell'app
- **[features.md](./project/features.md)** - Elenco e documentazione delle features
- **[pokedex-personal.md](./project/pokedex-personal.md)** - Feature: Pokedex Personale
- **[wiki-offline.md](./project/wiki-offline.md)** - Feature: Wiki Offline
- **[save-editor.md](./project/save-editor.md)** - Feature: Editor Salvataggi (PKHeX-like)
- **[multi-profile.md](./project/multi-profile.md)** - Feature: Gestione Multi-Profilo
- **[self-management.md](./project/self-management.md)** - Feature: Gestione Interna App
- **[database-storage.md](./project/database-storage.md)** - Database e Storage
- **[parser-architecture.md](./project/parser-architecture.md)** - Architettura Parser
- **[knowledge-database.md](./project/knowledge-database.md)** - Database Conoscenza
- **[project-structure.md](./project/project-structure.md)** - Struttura Progetto
- **[error-handling.md](./project/error-handling.md)** - Gestione Errori
- **[testing-strategy.md](./project/testing-strategy.md)** - Strategia Testing
- **[deployment.md](./project/deployment.md)** - Deployment e Distribuzione
- **[performance.md](./project/performance.md)** - Ottimizzazione Performance
- **[security.md](./project/security.md)** - Sicurezza

## File temporanei

Valutazioni, confronti, bozze e analisi di passaggio vanno in **[temp/](./temp/README.md)**. Quando non servono più, i file in `temp/` possono essere eliminati.

## Come Usare

1. Consulta gli standard in `docs/standards/` per comprendere le convenzioni
2. Usa `docs/procedures/INDEX.md` per trovare la procedura corrispondente alla tua query
3. Segui la checklist nella procedura prima di implementare

## Note

Tutta la documentazione segue lo standard definito in `docs/standards/markdown-optimization.md` per garantire massima leggibilità da parte di AI.
