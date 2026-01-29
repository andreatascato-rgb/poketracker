# Procedure: Aggiungere Test (unit / integration)

## Obiettivo

Definisce come aggiungere test (unit o integration) in PokeTracker rispettando la strategia di testing e la struttura del progetto.

## Quando Usare Questa Procedura

- Query: "aggiungi test", "scrivi test", "nuovo test", "unit test", "integration test", "test case", "aggiungi unit test", "aggiungi test per"
- Quando si deve introdurre o estendere test per frontend (Vitest/Playwright), backend Rust (cargo test) o sidecar
- Se la query corrisponde, questa procedure è OBBLIGATORIA

## Obbligatorietà

Completamento integrale della checklist obbligatorio prima di proporre implementazione. Nessuna eccezione.

## File da Leggere PRIMA

1. **Testing Standard**: `docs/standards/testing-standard.md:1-55`
   - Dove stanno i test, naming, mock Tauri: righe 7-32

2. **Strategia test**: `docs/project/testing-strategy.md:1-80`
   - Livelli, strumenti, target

3. **Tooling**: `docs/standards/tooling-standard.md:1-80`
   - Comandi e path per test (npm script, cargo test, ecc.)

4. **Struttura progetto**: `docs/project/project-structure.md:1-50`
   - Cartelle src-tauri, src (frontend), src-sidecar

## Checklist Obbligatoria

1. [ ] Leggi `docs/standards/testing-standard.md:7-22` — Dove mettere test (Rust `#[cfg(test)]`/`tests/`; frontend `*.test.ts` o `__tests__/`); naming
2. [ ] Per test frontend che usano `invoke`: usare `mockIPC` da `@tauri-apps/api/mocks`; polyfill `window.crypto` se jsdom; **clearMocks** dopo ogni test (`testing-standard.md:24-32`)
3. [ ] Verifica `docs/standards/tooling-standard.md` — Path e comandi per eseguire i test
4. [ ] Crea o estendi file di test nella posizione prevista come da `testing-standard.md`
5. [ ] Il test deve essere **riproducibile** e **isolato**; evitare dipendenze da stato globale o ordine non deterministico
6. [ ] Dopo l’aggiunta: eseguire la suite di test e verificare che passi

## Riferimenti Standard

- `docs/standards/testing-standard.md:1-55` — Dove, naming, mock Tauri
- `docs/project/testing-strategy.md:1-80` — Strategia, livelli
- `docs/standards/tooling-standard.md:1-80` — Comandi e path
- `docs/project/project-structure.md:1-50` — Struttura cartelle

## Note

- Se `docs/project/testing-strategy.md` non esiste ancora, proporre dove collocare i test in base a `project-structure.md` e tooling-standard, poi chiedere conferma.
- Per “fix bug” con aggiunta di test di regressione, usare prima `bug-fix.md` e trattare l’aggiunta del test come step della checklist o come richiesta esplicita.
