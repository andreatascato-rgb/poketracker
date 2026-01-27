# Procedure: Aggiungere Test (unit / integration)

## Obiettivo

Definisce come aggiungere test (unit o integration) in PokeTracker rispettando la strategia di testing e la struttura del progetto.

## Quando Usare Questa Procedure

- Query: "aggiungi test", "scrivi test", "nuovo test", "unit test", "integration test", "test case", "aggiungi unit test", "aggiungi test per"
- Quando si deve introdurre o estendere test per frontend (Vitest/Playwright), backend Rust (cargo test) o sidecar
- Se la query corrisponde, questa procedure è OBBLIGATORIA

## Obbligatorietà

Completamento integrale della checklist obbligatorio prima di proporre implementazione. Nessuna eccezione.

## File da Leggere PRIMA

1. **Strategia test**: `docs/project/testing-strategy.md:1-80`
   - Dove si collocano i test, framework, convenzioni

2. **Tooling**: `docs/standards/tooling-standard.md:1-80`
   - Comandi e path per test (npm script, cargo test, ecc.)

3. **Struttura progetto**: `docs/project/project-structure.md:1-50`
   - Cartelle src-tauri, src (frontend), src-sidecar

## Checklist Obbligatoria

1. [ ] Leggi `docs/project/testing-strategy.md` — Identifica layer (frontend/backend/sidecar) e framework (Vitest, cargo test, ecc.)
2. [ ] Verifica `docs/standards/tooling-standard.md` — Path e comandi per eseguire i test
3. [ ] Crea o estendi file di test nella posizione prevista (es. `*.test.ts`, `*_test.rs`, `tests/` in Rust)
4. [ ] Il test deve essere **riproducibile** e **isolato**; evitare dipendenze da stato globale o ordine di esecuzione non deterministico
5. [ ] Dopo l’aggiunta: eseguire la suite di test e verificare che passi

## Riferimenti Standard

- `docs/project/testing-strategy.md:1-80` — Strategia, framework, convenzioni
- `docs/standards/tooling-standard.md:1-80` — Comandi e path
- `docs/project/project-structure.md:1-50` — Struttura cartelle

## Note

- Se `docs/project/testing-strategy.md` non esiste ancora, proporre dove collocare i test in base a `project-structure.md` e tooling-standard, poi chiedere conferma.
- Per “fix bug” con aggiunta di test di regressione, usare prima `bug-fix.md` e trattare l’aggiunta del test come step della checklist o come richiesta esplicita.
