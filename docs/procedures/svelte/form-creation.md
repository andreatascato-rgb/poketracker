# Procedure: Creare Form e Validazione

## Obiettivo

Definisce come aggiungere un form e la validazione (frontend + backend) in linea con [forms-validation-standard](../../standards/forms-validation-standard.md).

## Quando Usare Questa Procedure

- Query: "aggiungi form", "validazione form", "form schema", "validazione input", "crea form", "form validation"
- Quando si deve introdurre un form che invia dati al backend (command Tauri) o modifica dati
- Se la query corrisponde, questa procedure è OBBLIGATORIA

## Obbligatorietà

Completamento integrale della checklist obbligatorio prima di proporre implementazione. Nessuna eccezione.

## File da Leggere PRIMA

1. **Form e validazione**: `docs/standards/forms-validation-standard.md:1-50`
   - Validazione backend obbligatoria, schema condivisi: righe 7-22
   - Frontend (UX, messaggi), backend (Rust): righe 24-38

2. **Input validation**: `docs/standards/input-validation-standard.md:1-40`
   - Never trust frontend, dove validare

3. **Invoke e tipi**: `docs/standards/typescript-frontend-standard.md:36-70`
   - invoke, try/catch, tipi

4. **Command**: `docs/standards/rust-tauri-standard.md:17-33`
   - Result, validazione argomenti

## Checklist Obbligatoria

1. [ ] Leggi `docs/standards/forms-validation-standard.md:7-14` — Ogni dato verso i command va validato in Rust; frontend per UX only
2. [ ] Definire struttura e regole (tipo Rust + validazione in Rust; in frontend tipi/ eventuale Zod allineato)
3. [ ] Backend: validare argomenti all’ingresso del command; restituire `Result<T, String>` (o custom) con messaggio chiaro (`forms-validation-standard.md:34-38`, `input-validation-standard.md`)
4. [ ] Frontend: validazione lato client per feedback immediato; messaggi vicini al campo; stile coerente con [error-handling-standard](../../standards/error-handling-standard.md)
5. [ ] **UI form:** usare componenti da `$lib/components/ui` (Input, Button, ecc.) e Tailwind; rispettare `docs/standards/ui-stack-standard.md` e `docs/standards/design-system-standard.md`; no CSS custom o elementi form da zero
6. [ ] Messaggi di errore: chiari e user-facing come da `error-handling-standard`; inline per errori di campo
7. [ ] Se si usa Zod/Superforms: schema condiviso e `safeParse`; non sostituire validazione backend

## Riferimenti Standard

- `docs/standards/ui-stack-standard.md` — Stack UI (Tailwind + shadcn-svelte) per input, button, label del form
- `docs/standards/design-system-standard.md` — Stile, token (moderno, professionale, dark)
- `docs/standards/forms-validation-standard.md:1-50` — Form, schema, frontend/backend
- `docs/standards/input-validation-standard.md:1-40` — Sicurezza input
- `docs/standards/error-handling-standard.md` — Messaggi utente
- `docs/standards/typescript-frontend-standard.md:36-70` — invoke, tipi
- `docs/standards/rust-tauri-standard.md:17-33` — command, Result

## Note

- Per “aggiungi comando” usare `docs/procedures/rust/command-creation.md`; la validazione degli argomenti del command segue [input-validation-standard](../standards/input-validation-standard.md).
