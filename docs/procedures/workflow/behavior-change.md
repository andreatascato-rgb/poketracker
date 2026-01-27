# Procedure: Modifica Comportamento (Behavior Change)

## Obiettivo

Definisce come modificare il comportamento esistente di una parte dell'app (logica, flusso, regole) **senza** correggere un bug né aggiungere una nuova feature. Esempi: "cambia come funziona X", "modifica la logica di Y", "cambia l'ordine di Z", "vuoi che invece di A faccia B".

## Quando Usare Questa Procedure

- Query: "cambia comportamento", "modifica come funziona", "cambia logica di", "modifica comportamento di", "cambia il funzionamento di", "vuoi che faccia", "invece di X faccia Y", "cambia la logica", "modifica il flusso"
- Quando l'utente chiede di **cambiare intenzionalmente** il comportamento attuale (non correggere un errore, non aggiungere capacità nuova)
- Se la query corrisponde, questa procedure è OBBLIGATORIA

## Obbligatorietà

Completamento integrale della checklist obbligatorio prima di proporre implementazione. Nessuna eccezione.

## Differenza da altre procedure

| Situazione | Procedure da usare |
|------------|---------------------|
| Comportamento **errato** da correggere (bug) | [bug-fix](./bug-fix.md) |
| **Nuova** capacità / funzionalità | [new-feature](./new-feature.md) |
| **Modifica** di comportamento esistente (cambio di specifica/regola) | Questa procedure (behavior-change) |

## File da Leggere PRIMA

1. **Vista architettura**: `docs/project/architecture-overview.md:1-80`
   - Stack, flusso dati, dove vivono profilo/Pokedex/wiki/comandi

2. **Struttura progetto**: `docs/project/project-structure.md:1-110`
   - Layer e cartelle (frontend, backend, sidecar, db)

3. **Glossario**: `docs/project/glossary.md` — Termini di dominio coinvolti (profilo, Pokedex, save, ecc.)

4. **Procedure e standard dell'area toccata**: da `docs/procedures/INDEX.md` in base al layer (es. component-creation se si modifica UI, command-creation se si modifica comando, db-migration se si modifica schema)

## Checklist Obbligatoria

1. [ ] **Identifica l’ambito** della modifica: cosa cambia (logica, flusso, regola, UX) e in quale layer (frontend, backend, DB, sidecar). Leggi `docs/project/architecture-overview.md` per capire dove vive quel comportamento
2. [ ] **Impact analysis:** elenca i punti che possono essere toccati (file, comandi, componenti, store, DB, doc). Se la modifica riguarda contratto frontend–backend (parametri, tipo ritorno), consultare [api-contract-change](./api-contract-change.md)
3. [ ] **Leggi il contesto attuale:** apri i file e le parti di codice coinvolte; verifica gli standard applicabili (`docs/standards/` per quel layer)
4. [ ] **Proponi la modifica** in modo concreto: cosa si cambia, dove, e perché rispetta gli standard; evidenzia eventuali breaking change o impatti su altri moduli
5. [ ] **Doc e test:** indica quali documenti vanno aggiornati (es. features, core-functionality, glossary) e quali test o verifiche manuali confermano il nuovo comportamento
6. [ ] **Conferma con l’utente** prima di implementare se la modifica è ampia o cambia contratto/API

## Riferimenti Standard

- `docs/project/architecture-overview.md` — Dove vive il comportamento, flusso dati
- `docs/project/project-structure.md` — Layer e cartelle
- `docs/project/glossary.md` — Termini di dominio
- `docs/procedures/INDEX.md` — Procedure per layer (component-creation, command-creation, db-migration, api-contract-change, ecc.)
- `docs/standards/api-contract-standard.md` — Se si modificano parametri o tipo ritorno di comandi

## Note

- **Modifica minima:** preferire cambi locali al punto in cui vive il comportamento; evitare refactor ampi non richiesti.
- **Breaking change:** se la modifica cambia parametri/tipo di un comando o di un’API usata da altri, seguire [api-contract-change](./api-contract-change.md) e documentare la breaking change.
- Se la richiesta è in realtà un bug (“non funziona così”) usare [bug-fix](./bug-fix.md); se è una nuova capacità usare [new-feature](./new-feature.md).
