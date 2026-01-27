# Standard: Gestione Errori (User-Facing e Logging)

## Obiettivo

Definisce quando mostrare errori all’utente (toast, inline, modal), come formattare i messaggi, come loggare e come mappare `Result`/eccezioni dal backend. Si appoggia a [error-handling](../project/error-handling.md) per la strategia e a [rust-tauri-standard](./rust-tauri-standard.md) per i command.

## Principi

- **Separazione:** messaggio utente (breve, actionable) vs log tecnico (dettaglio, stack, contesto).
- **Niente panic in produzione** nei command Rust; sempre `Result<T, E>` con `E` serializzabile.
- **Frontend:** ogni chiamata a `invoke` deve gestire il reject (try/catch o .catch()); non lasciare errori “unhandled”.

## Quando Usare Cosa (User-Facing)

- **Toast:** conferme rapide, errori di azione singola (es. “Salvataggio fallito”), warning non bloccanti. Messaggio 1–2 frasi; evitare pile di toast; raggruppare errori simili se possibile. Posizione fissa (es. in basso a destra); auto-dismiss con durata adeguata; rispettare `prefers-reduced-motion`.
- **Inline:** errori di validazione form, campi obbligatori, errori legati a un campo/sezione. Mostrare vicino al controllo interessato.
- **Modal / bloccante:** errori critici che richiedono azione (es. “Sessione scaduta”, “Impossibile continuare”). Usare quando l’utente deve leggere e decidere prima di proseguire.

## Messaggi Utente

- Brevi e concreti; evitare messaggi generici (“Si è verificato un errore”); preferire “Impossibile salvare: file in uso” con eventuale azione suggerita.
- Per errori da backend: usare il messaggio restituito dal command (String o campo `message` se tipo strutturato) solo se adatto all’utente; altrimenti mappare l’errore in un messaggio generico ma chiaro e loggare il dettaglio tecnico.

## Logging

- **Backend (Rust):** usare tauri-plugin-log come da [rust-tauri-standard](./rust-tauri-standard.md). Livelli: `error` per fallimenti utente/operazione; `warn` per situazioni anomale recuperabili; `info` per flussi principali; `debug`/`trace` per diagnostica.
- **Frontend:** `console.error` (o logger strutturato se introdotto) per errori da `invoke` e per errori di UI; includere contesto (es. nome command, payload essenziale). Non loggare segreti o dati personali in chiaro.
- **Formato:** in produzione preferire log strutturati (es. JSON) se possibile; in sviluppo plain text leggibile.

## Accessibilità Toast/Notifiche

- Ruolo appropriato (es. `role="status"` per messaggi informativi) e `aria-live="polite"` per annunci da screen reader.
- Rispettare `prefers-reduced-motion` (ridurre o disattivare animazioni).
- Non usare solo il colore per indicare errore/successo; accompagnare con icona o testo.

## Riferimenti

- [error-handling](../project/error-handling.md) — strategia e categorie errori
- [rust-tauri-standard](./rust-tauri-standard.md) — Result nei command, logging Rust
- [typescript-frontend-standard](./typescript-frontend-standard.md) — invoke, try/catch
- [web.dev – Toast](https://web.dev/patterns/components/toast)

## Data Decisione

2026-01-27

## Note

- Per “gestisci errore”, “aggiungi error boundary”, “messaggio errore utente” applicare questo standard e la procedure corrispondente.
