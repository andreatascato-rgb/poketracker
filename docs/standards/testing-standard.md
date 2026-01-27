# Standard: Testing (Convenzioni e Struttura)

## Obiettivo

Definisce dove mettere i test, come nominarli e come mockare Tauri (invoke) in linea con [testing-strategy](../project/testing-strategy.md). Non sostituisce la strategia, la formalizza in convenzioni operative.

## Dove stanno i test

- **Rust:** unit in moduli `#[cfg(test)]` nello stesso file o in `tests/<nome>.rs` per integration; file di test in `src-tauri/tests/` se si usano integration test separati.
- **Frontend (Svelte/TS):** file `*.test.ts` o `*.spec.ts` vicino al codice, oppure in cartella `__tests__/` o `tests/` per modulo; coerenza per sottoprogetto (es. `src/lib/components/__tests__/` o `ComponentName.test.ts`).
- **E2E:** in cartella dedicata (es. `e2e/`); strumenti e setup documentati in [testing-strategy](../project/testing-strategy.md).

## Naming

- **Rust:** `test_<cosa>`, `#[test] fn <descriptive_name>`; modulo `#[cfg(test)] mod tests { ... }`.
- **Frontend:** `describe('ComponentName'/'module')`; `it('does X when Y')` (o equivalente); nome file `ComponentName.test.ts` o `module.test.ts`.
- Descrizioni chiare: comportamento atteso, non dettagli implementativi.

## Mock Tauri (invoke) in test frontend

- Usare **`@tauri-apps/api/mocks`** e **`mockIPC`** per intercettare le chiamate a `invoke` nei test.
- **Setup:** polyfill per `window.crypto` (es. `getRandomValues`) se si usa jsdom; chiamare `mockIPC((cmd, args) => { ... })` nel test o in `beforeEach`.
- **Teardown:** **`clearMocks()`** (o equivalente) dopo ogni test per evitare che un test influenzi l’altro.
- Esempio pattern: `mockIPC((cmd, args) => { if (cmd === 'my_cmd') return stub; return default; });` poi eseguire il codice che chiama `invoke('my_cmd', ...)` e assertare su risultato e/o su chiamate.

## Isolamento e dati

- **Unit:** nessuna dipendenza da fs/rete reale; mock per Tauri, per servizi, per store se necessario.
- **Integration (Rust):** DB in-memory o file temporanei; pulire stato tra test quando possibile.
- **Dati di test:** fixture in file o costanti; evitare dati “random” dove serve repeatability.

## Riferimenti

- [testing-strategy](../project/testing-strategy.md) — Livelli, strumenti, target
- [Tauri v2 – Mocking](https://v2.tauri.app/develop/tests/mocking)
- [Vitest](https://vitest.dev/), [@testing-library/svelte](https://testing-library.com/docs/svelte-testing-library/intro)

## Data Decisione

2026-01-27

## Note

- Per “aggiungi test”, “unit test”, “mock invoke” usare questo standard e [test-creation](../procedures/workflow/test-creation.md).
