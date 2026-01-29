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

### [0.2.1] - 2026-01-29

#### Descrizione
Fix script e configurazione: run-tauri, cargo-check, .env.example, Cargo.lock.

#### Change
- Script run-tauri e cargo-check aggiornati
- .env.example e Cargo.lock allineati

### [0.2.0] - 2026-01-29

#### Descrizione
Pokedex, Wiki, Backup dati, Errori in Impostazioni, Statistiche profilo, export e script di sviluppo; refactor navigazione e documentazione.

#### Change
- Pokedex: componenti, route profilo/pokedex, sprites statici, dati e servizi; probe PKHeX e parser-inventory
- Wiki: route wiki con mosse, nature, pokemon
- Impostazioni: layout dedicato; pagine backup-dati e errori (archivio/errori spostato qui)
- Profilo: statistiche, salvataggi; rimosso dashboard
- Backend: comando export_backup; sidecar PkHexPokedexProbe
- Script: run-tauri, cargo-check, download-pokedex-sprites, add-cargo-to-path, reinstall-rust-user; .env.example
- UI: dropdown-menu, ProfileSelector e store aggiornati; alert-dialog
- Docs: procedure e standard aggiornati; parser-inventory, pokedex-parser-strategy, fix-without-workaround; temp analyses

### [0.1.0] - 2026-01-28

#### Descrizione
Shell UI Poketrack (Top Bar + Sidebar) con navigazione e componenti UI base; avvio foundation feature (profilo, sync, archivio errori, sidecar).

#### Change
- Layout: top bar + sidebar (collassabile), breadcrumb, stati interattivi e icone uniche
- Profilo: selector e flusso gestione profili (UI + command/DB)
- Sync/Watcher: store sync e watcher lato Tauri; UX loading/sync documentata
- UI primitives: tooltip/sonner/empty-state/breadcrumb e standard responsive/interactions
- Sidecar/parser: integrazione binario e prime logiche rilevamento save

### [0.0.3] - 2026-01-27

#### Descrizione
Pagina Impostazioni: titolo di pagina rimosso, card Profili con standard titolo+azione; documentato standard in ui-patterns-applied.

#### Change
- Impostazioni: rimosso titolo "Impostazioni" dal contenuto (ridondante con voce menu)
- Card Profili: sottotitolo senza max-w-[66ch]; titolo "Profili" con text-lg, min-h-8, flex items-center sulla stessa riga del bottone "Nuovo allenatore"
- docs/project/ui-patterns-applied.md: standard "Titolo con azione sulla riga" (CardTitle + Button size="sm" → text-lg, min-h-8, flex items-center, font-semibold)

### [0.0.2] - 2026-01-27

#### Descrizione
Versionamento autonomo: l’AI decide bump e esegue commit + push senza conferma; conferma solo per MAJOR o casi ambigui.

#### Change
- Standard versionamento: flusso decisionale autonomo, eccezioni (MAJOR/ambiguo)
- Procedura versioning-release: checklist autonoma, nessuna conferma per MINOR/PATCH

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
