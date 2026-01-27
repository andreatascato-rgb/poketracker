# Procedure: Validazione Input e Sicurezza

## Obiettivo

Definisce come validare e sanitizzare l’input in ingresso ai command Tauri in linea con [input-validation-standard](../../standards/input-validation-standard.md).

## Quando Usare Questa Procedure

- Query: "validazione input", "sanitizza", "sicurezza input", "input validation", "never trust frontend", "valida argomenti"
- Quando si deve validare o sanitizzare i payload di `invoke` o l’input utente in Rust
- Se la query corrisponde, questa procedure è OBBLIGATORIA

## Obbligatorietà

Completamento integrale della checklist obbligatorio prima di proporre implementazione. Nessuna eccezione.

## File da Leggere PRIMA

1. **Input Validation Standard**: `docs/standards/input-validation-standard.md:1-50`
   - Never trust frontend, dove validare: righe 7-18
   - Path, stringhe, numeri, strumenti: righe 20-32

2. **Rust command**: `docs/standards/rust-tauri-standard.md:17-33`
   - Result, errori nei command

3. **Sicurezza**: `docs/project/security.md`
   - Strategia sicurezza

## Checklist Obbligatoria

1. [ ] Leggi `docs/standards/input-validation-standard.md:7-14` — Ogni dato da `invoke` è untrusted; validare sempre in Rust prima di business logic e persistenza
2. [ ] Validare all’ingresso del command: tipo, range, formato, lunghezza (`input-validation-standard.md:16-18`)
3. [ ] Path: normalizzare e validare (no `..` fuori scope, caratteri invalidi); API sicure per join (`input-validation-standard.md:19-20`)
4. [ ] Stringhe: lunghezza max, encoding UTF-8; sanitizzare se usate in contesti sensibili (`input-validation-standard.md:21-22`)
5. [ ] Numeri: range e tipo; evitare overflow dove rilevante (`input-validation-standard.md:23`)
6. [ ] Query DB: parametri preparati (rusqlite/sqlx); mai concatenare input utente in SQL (`input-validation-standard.md:30`)
7. [ ] Messaggi user-facing brevi e actionable; dettaglio tecnico solo in log (`input-validation-standard.md:34-38`)

## Riferimenti Standard

- `docs/standards/input-validation-standard.md:1-50` — Validazione, sanitizzazione
- `docs/standards/rust-tauri-standard.md:17-33` — command, Result
- `docs/project/security.md` — Strategia

## Note

- Per “aggiungi comando” usare `docs/procedures/rust/command-creation.md` e applicare questa procedure per gli argomenti del command.
