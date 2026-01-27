# Procedure: Creare Servizio Frontend (Wrapper Invoke)

## Obiettivo

Definisce come aggiungere un servizio frontend che incapsula le chiamate `invoke` a comandi Tauri (tipi, errori, riuso).

## Quando Usare Questa Procedure

- Query: "crea servizio", "aggiungi servizio", "servizio frontend", "wrapper invoke", "servizio tauri", "aggiungi servizio che chiama backend"
- Quando si deve introdurre un modulo in `src/lib/services/` che espone funzioni che chiamano comandi Tauri
- Se la query corrisponde, questa procedure è OBBLIGATORIA

## Obbligatorietà

Completamento integrale della checklist obbligatorio prima di proporre implementazione. Nessuna eccezione.

## File da Leggere PRIMA

1. **TypeScript/Frontend**: `docs/standards/typescript-frontend-standard.md:36-70`
   - invoke da `@tauri-apps/api/core`, try/catch, tipi: righe 36-62
   - Tipi condivisi e servizi: righe 64-70

2. **Struttura progetto**: `docs/project/project-structure.md:141-146`
   - Cartella `lib/services/` (tauri.ts, profile.ts, pokedex.ts, wiki.ts)

## Checklist Obbligatoria

1. [ ] Leggi `docs/standards/typescript-frontend-standard.md:36-62` — Import `invoke` da `@tauri-apps/api/core`; ogni chiamata con `await` e gestione errori (`try/catch` o `.catch()`)
2. [ ] Verifica `docs/project/project-structure.md:141-146` — Crea il file in `src/lib/services/<nome>.ts` (es. `profile.ts`, `pokedex.ts`)
3. [ ] Esporre funzioni che incapsulano `invoke('<command_name>', { ... })`; non lasciare `invoke` nudo nei componenti se il pattern progetto è “servizi per dominio”
4. [ ] Usare tipi per argomenti e ritorno: generico `invoke<T>()` o tipi condivisi come da `typescript-frontend-standard.md:64-70`
5. [ ] In caso di errore: log, messaggio utente o rilancio; non ignorare reject di `invoke`
6. [ ] Import da `$lib/services/...` nei componenti/store che usano il servizio

## Riferimenti Standard

- `docs/standards/typescript-frontend-standard.md:36-70` — invoke, errori, tipi, servizi
- `docs/project/project-structure.md:141-146` — Dove creare servizi

## Note

- Per “aggiungi comando” (Rust) usare `docs/procedures/rust/command-creation.md`; il servizio frontend va creato quando si espone quel comando alla UI.
- Naming: kebab-case per file (`pokedex.ts`) o coerente con i servizi esistenti in project-structure.
