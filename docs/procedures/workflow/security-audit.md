# Procedure: Verifica Sicurezza (Security Audit)

## Obiettivo

Definisce come eseguire una verifica di sicurezza in linea con [security-standard](../../standards/security-standard.md) e [security](../../project/security.md).

## Quando Usare Questa Procedura

- Query: "verifica sicurezza", "security audit", "sicurezza generale", "controlla sicurezza", "audit sicurezza", "check sicurezza"
- Quando si deve verificare o migliorare il rispetto delle pratiche di sicurezza (senza focus specifico su sola validazione input)
- Se la query corrisponde, questa procedure è OBBLIGATORIA

## Obbligatorietà

Completamento integrale della checklist obbligatorio prima di proporre implementazione. Nessuna eccezione.

## File da Leggere PRIMA

1. **Security Standard**: `docs/standards/security-standard.md:1-70`
   - Validazione input, file system, DB, sidecar, Tauri, dati sensibili

2. **Strategia sicurezza**: `docs/project/security.md:1-120`
   - Best practice, validazione multi-layer, error handling security

3. **Input validation**: `docs/standards/input-validation-standard.md` — never trust frontend

## Checklist Obbligatoria

1. [ ] Leggi `docs/standards/security-standard.md:9-18` — Validazione input in Rust; path e payload all’ingresso command
2. [ ] File system: path normalizzati e in scope; nessun `..` fuori scope; asset scope Tauri (`security-standard.md:20-26`)
3. [ ] DB: prepared statements; nessuna concatenazione input in SQL; isolamento per profilo (`security-standard.md:28-34`)
4. [ ] Sidecar: validazione argomenti e path; timeout e gestione errori; output JSON validato (`security-standard.md:36-41`)
5. [ ] Tauri: whitelist comandi; permessi minimi necessari; vedi `tauri2-permissions-standard` (`security-standard.md:43-48`)
6. [ ] Dati sensibili: no dati sensibili in log; messaggi user-facing generici; vedi `error-handling-standard` (`security-standard.md:50-56`)
7. [ ] Confronta con `docs/project/security.md` e proponi gap o miglioramenti concreti

## Riferimenti Standard

- `docs/standards/security-standard.md:1-70` — Regole operative sicurezza
- `docs/project/security.md:1-120` — Strategia e best practice
- `docs/standards/input-validation-standard.md` — Validazione input
- `docs/standards/tauri2-permissions-standard.md` — Permessi Tauri

## Note

- Per “validazione input” mirata usare la procedura [input-validation](../../workflow/input-validation.md).
- Questa procedure si applica quando l’obiettivo è una verifica complessiva di sicurezza o un audit.
