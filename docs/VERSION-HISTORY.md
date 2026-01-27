# Storico versioni PokeTracker

Elenco delle versioni pushate, con descrizione e change. Ordinamento: dalla più recente in alto.

Per come assegnare e registrare le versioni: `docs/standards/versioning-standard.md`.  
Per il flusso prima di commit/push: `docs/procedures/workflow/versioning-release.md`.

---

## Formato entrate

Ogni versione ha questa struttura:

```markdown
## [X.Y.Z] - YYYY-MM-DD

### Descrizione
Breve testo che riassume la release.

### Change
- Change 1
- Change 2
```

---

## Versioni

### [0.0.1] - 2026-01-27

#### Descrizione
Setup documentazione: standard, procedure, versionamento e storico versioni. Base per sviluppo PokeTracker.

#### Change
- Standard: versionamento, api-contract, ci-cd, i18n, performance, security, testing, ui-primitives
- Procedure workflow: versioning-release, api-contract-change, behavior-change, i18n-setup, layout-navigation-change, performance-optimization, security-audit; aggiornamenti deploy-release, test-creation
- Procedure svelte: ui-primitive-creation; aggiornamento component-creation
- Project: architecture-overview, glossary, ui-component-catalog; aggiornamento README e project-structure
- Versionamento: standard, procedura, file `docs/VERSION-HISTORY.md`
- `.cursorrules` e indici doc: riferimento versioning, README standards/procedures
