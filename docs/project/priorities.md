# Priorità di progetto

## Obiettivo

Definisce l’ordine di priorità quando si fanno trade-off o scelte di implementazione. L’AI applica questo ordine di default; non serve che l’utente ripeta le priorità a ogni task.

## Ordine di priorità (decrescente)

1. **Correttezza e dati** — Comportamento e logica corretti; dati coerenti, validazione backend (never trust frontend), integrità DB. Nessun compromesso su sicurezza e integrità dati.
2. **Performance e caricamento** — Tempi di risposta accettabili, niente blocchi UI, lazy load dove serve, virtualizzazione liste lunghe, query e IPC efficienti. Vedi `docs/standards/performance-standard.md`.
3. **Funzionalità** — Feature complete e usabili; flussi chiari, feedback utente (es. toast/errori), accessibilità di base (focus, contrasto, semantica). Vedi `docs/standards/accessibility-standard.md`, `docs/standards/error-handling-standard.md`.
In dubbio: privilegiare il livello più alto della lista (es. non sacrificare correttezza per performance).

## Principi

- **Non tralasciare** — L’obiettivo è consegnare **tutti** i livelli: corretti, veloci, fruibili e ben presentati. Le priorità servono per **trade-off**, non per abbandonare dati o funzionalità.
- **Accessibilità** — È parte della funzionalità (punto 3). In linea con best practice 2026 (WCAG).
- **Standard prima** — Per ogni ambito (performance, a11y, sicurezza) applicare gli standard in `docs/standards/`; le priorità indicano solo chi “vince” in caso di conflitto esplicito.
- **Override esplicito** — Se in un task l’utente dice esplicitamente “priorità X” (es. “questa volta va bene solo funzionalità, stile dopo”), quella scelta prevale su questo ordine.

## Riferimenti

- Standard performance: `docs/standards/performance-standard.md`
- Standard accessibilità: `docs/standards/accessibility-standard.md`
- Strategia progetto: `docs/project/overview.md`, `docs/project/core-functionality.md`

## Data decisione

2026-01-27
