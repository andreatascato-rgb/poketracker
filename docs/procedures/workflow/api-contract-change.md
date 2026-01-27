# Procedure: Cambio Contratto API (Frontend–Backend)

## Obiettivo

Definisce come modificare i contratti tra frontend e backend (comandi Tauri, tipi condivisi) in modo controllato, in linea con [api-contract-standard](../../standards/api-contract-standard.md).

## Quando Usare Questa Procedure

- Query: "cambia contratto", "aggiungi campo", "breaking change API", "aggiungi parametro comando", "modifica tipo ritorno", "rimuovi comando"
- Quando si deve cambiare signature, argomenti o strutture di ritorno dei command Tauri o i tipi condivisi
- Se la query corrisponde, questa procedure è OBBLIGATORIA

## Obbligatorietà

Completamento integrale della checklist obbligatorio prima di proporre implementazione. Nessuna eccezione.

## File da Leggere PRIMA

1. **API Contract Standard**: `docs/standards/api-contract-standard.md:1-55`
   - Semver, breaking vs non-breaking: righe 7-22
   - Aggiungere campo, breaking change, dove tenere tipi: righe 24-42

2. **Rust command**: `docs/standards/rust-tauri-standard.md:17-33`
3. **TypeScript tipi**: `docs/standards/typescript-frontend-standard.md:64-70`

## Checklist Obbligatoria

1. [ ] Leggi `docs/standards/api-contract-standard.md:7-22` — Classificare il cambio: major (breaking) vs minor (backward-compatible) vs patch
2. [ ] **Aggiungere campo (non-breaking):** in strutture di ritorno usare `Option<T>` o default in Rust; in argomenti parametri opzionali; aggiornare tipi TypeScript e call site che devono usarli (`api-contract-standard.md:24-28`)
3. [ ] **Breaking change:** pianificare deprecation (mantenere vecchio per una release, log, poi rimuovere); bump major; aggiornare frontend e backend insieme; documentare in changelog o doc migrazione (`api-contract-standard.md:30-34`)
4. [ ] Aggiornare tipi in Rust (`src-tauri/src/models/` o moduli command) e in TypeScript (`src/lib/types/` o generati); allineare serde e definizioni (`api-contract-standard.md:36-40`)
5. [ ] Verificare che i call site esistenti continuino a compilare dopo modifiche non-breaking; dopo breaking, aggiornare tutti i chiamanti

## Riferimenti Standard

- `docs/standards/api-contract-standard.md:1-55` — Semver, breaking, tipi
- `docs/standards/rust-tauri-standard.md:17-33` — Command, Result
- `docs/standards/typescript-frontend-standard.md:64-70` — Tipi condivisi

## Note

- Per “aggiungi comando” nuovo usare [command-creation](../../rust/command-creation.md); questa procedure si applica quando si *modifica* un contratto esistente.
