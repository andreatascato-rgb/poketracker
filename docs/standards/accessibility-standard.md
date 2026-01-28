# Standard: Accessibilità (a11y)

## Obiettivo

Definisce i requisiti minimi di accessibilità per PokeTracker (WCAG-oriented): focus, landmark, ARIA, contrasto e testing. Si appoggia a Svelte/SvelteKit per le funzionalità built-in.

## Principi

- **Contrasto:** testo e UI devono rispettare contrasto adeguato (WCAG AA come target minimo per testo normale e large).
- **Focus:** ordine di tab logico; focus visibile; nessun trap senza uscita. Dopo navigazione (es. cambio route), focus gestito in modo prevedibile (SvelteKit focus su body o su elemento principale).
- **Semantica:** usare elementi HTML appropriati (button per azioni, link per navigazione, label per form, heading per gerarchia).
- **ARIA:** usare solo quando l’HTML nativo non basta (es. regioni live per annunci dinamici, `aria-label` dove il testo visibile non è sufficiente). Evitare `accesskey` (conflitti tastiera).

## Svelte / SvelteKit

- **Route e titolo:** ogni pagina deve avere `<title>` descrittivo in `<svelte:head>` per annunci a screen reader.
- **Avvisi compile-time:** rispettare i warning di accessibilità di Svelte (es. click su non-interattivi senza handler tastiera, uso errato di ARIA).
- **Eventi:** per azioni su elementi non nativamente interattivi (es. `div` con `on:click`) fornire equivalente da tastiera (es. `on:keydown` con Enter/Space) e ruoli/ARIA appropriati.

## Riduzione movimento (prefers-reduced-motion)

- Quando l'utente preferisce meno movimento (`prefers-reduced-motion: reduce`), transizioni e animazioni vanno ridotte o disattivate.
- **Implementazione globale:** in `src/app.css` un blocco `@media (prefers-reduced-motion: reduce)` applica `transition-duration: 0.01ms` e `animation-duration: 0.01ms` a tutta l'app. Nuove animazioni/transizioni devono rispettare questa preferenza (il blocco globale le sovrascrive; per eccezioni documentare il motivo).
- Riferimento design system: `docs/standards/design-system-standard.md` (§ Transizioni).

## Toast e Notifiche

- Usare `role="status"` o `role="alert"` a seconda della priorità; `aria-live="polite"` (o `assertive` per alert critici).
- Rispettare `prefers-reduced-motion` per animazioni (v. sezione "Riduzione movimento" sopra).
- Non affidarsi solo al colore per indicare successo/errore; usare icona o testo.

## Form

- Ogni controllo deve avere `<label>` associato (o `aria-label` / `aria-labelledby` se non possibile).
- Messaggi di errore di validazione associati al campo (`aria-describedby` o id espliciti).

## Testing

- Validare con strumenti automatici (es. axe-core, Lighthouse) su pagine critiche.
- Test manuale con tastiera (solo Tab/Shift+Tab/Enter/Space/Esc) e, se possibile, con screen reader.

## Riferimenti

- [SvelteKit – Accessibility](https://kit.svelte.dev/docs/accessibility)
- [Svelte – Accessibility warnings](https://svelte.dev/docs/accessibility-warnings)
- [WCAG 2.1](https://www.w3.org/WAI/WCAG21/quickref/)

## Data Decisione

2026-01-27

## Note

- Per “aggiungi accessibilità”, “a11y”, “fix a11y” usare questo standard e la procedure corrispondente.
