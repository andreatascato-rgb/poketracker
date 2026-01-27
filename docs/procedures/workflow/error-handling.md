# Procedure: Gestione Errori User-Facing e Logging

## Obiettivo

Definisce come introdurre o modificare la gestione errori user-facing (toast, inline, modal) e il logging in linea con [error-handling-standard](../../standards/error-handling-standard.md).

## Quando Usare Questa Procedure

- Query: "gestisci errore", "user-facing error", "toast errore", "error boundary", "messaggio di fallimento", "aggiungi toast", "notifica errore"
- Quando si deve mostrare errori all’utente o cambiare come/ dove si loggano
- Se la query corrisponde, questa procedure è OBBLIGATORIA

## Obbligatorietà

Completamento integrale della checklist obbligatorio prima di proporre implementazione. Nessuna eccezione.

## File da Leggere PRIMA

1. **Error Handling Standard**: `docs/standards/error-handling-standard.md:1-60`
   - Quando toast vs inline vs modal: righe 12-18
   - Messaggi utente, logging, a11y toast: righe 20-38

2. **Strategia errori**: `docs/project/error-handling.md:1-80`
   - Categorie e gestione per dominio

3. **Rust/Invoke**: `docs/standards/rust-tauri-standard.md` (Result nei command), `docs/standards/typescript-frontend-standard.md` (try/catch invoke)

## Checklist Obbligatoria

1. [ ] Leggi `docs/standards/error-handling-standard.md:12-18` — Usa toast per azione singola/errore breve; inline per validazione form; modal per errori bloccanti
2. [ ] Messaggi utente: brevi, concreti, actionable; evitare “Si è verificato un errore” (`error-handling-standard.md:20-24`)
3. [ ] Per errori da `invoke`: mappare messaggio backend su messaggio user-facing se necessario; loggare dettaglio tecnico (`error-handling-standard.md:24-26`)
4. [ ] Toast/notifiche: `role="status"` o `role="alert"`, `aria-live="polite"`, rispettare `prefers-reduced-motion` (`error-handling-standard.md:36-38`)
5. [ ] Backend: usare livelli tauri-plugin-log come da `error-handling-standard.md:28-32`; non loggare secret/PII in chiaro
6. [ ] Frontend: `console.error` (o logger) per errori da invoke e UI; contesto (command, payload essenziale) (`error-handling-standard.md:32-34`)

## Riferimenti Standard

- `docs/standards/error-handling-standard.md:1-60` — User-facing, messaggi, logging
- `docs/project/error-handling.md:1-80` — Strategia e categorie
- `docs/standards/rust-tauri-standard.md` — Result, logging Rust
- `docs/standards/typescript-frontend-standard.md` — invoke, try/catch

## Note

- Per “fix bug” usare `docs/procedures/workflow/bug-fix.md`; questa procedure si applica quando si aggiunge o modifica la *gestione* degli errori (dove mostrare, come loggare).
