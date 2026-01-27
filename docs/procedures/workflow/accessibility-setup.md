# Procedure: Accessibilità (a11y)

## Obiettivo

Definisce come aggiungere o correggere l’accessibilità in linea con [accessibility-standard](../../standards/accessibility-standard.md).

## Quando Usare Questa Procedure

- Query: "accessibilità", "a11y", "aggiungi a11y", "fix a11y", "aria", "keyboard", "screen reader", "aggiungi label", "contrasto"
- Quando si deve introdurre o correggere comportamenti e markup per utenti con disabilità
- Se la query corrisponde, questa procedure è OBBLIGATORIA

## Obbligatorietà

Completamento integrale della checklist obbligatorio prima di proporre implementazione. Nessuna eccezione.

## File da Leggere PRIMA

1. **Accessibility Standard**: `docs/standards/accessibility-standard.md:1-55`
   - Contrasto, focus, semantica, ARIA: righe 7-18
   - Svelte/SvelteKit, toast, form: righe 20-38

2. **Contrasto:** WCAG AA come target minimo.

## Checklist Obbligatoria

1. [ ] Leggi `docs/standards/accessibility-standard.md:7-18` — Contrasto adeguato (WCAG AA); ordine tab e focus visibile; semantica HTML; ARIA solo se necessario
2. [ ] Ogni pagina: `<title>` descrittivo in `<svelte:head>` (`accessibility-standard.md:20-22`)
3. [ ] Risolvere i warning di accessibilità di Svelte (click su non-interattivi, ARIA errato) (`accessibility-standard.md:23-24`)
4. [ ] Per azioni su elementi non nativamente interattivi: handler tastiera (Enter/Space) e ruoli/ARIA appropriati (`accessibility-standard.md:25-26`)
5. [ ] Toast/notifiche: `role="status"` o `role="alert"`, `aria-live="polite"`; `prefers-reduced-motion` (`accessibility-standard.md:28-32`)
6. [ ] Form: ogni controllo con `<label>` o `aria-label`/`aria-labelledby`; errori di validazione associati al campo (`accessibility-standard.md:34-37`)
7. [ ] Validare con strumenti automatici (axe, Lighthouse) su pagine critiche (`accessibility-standard.md:39-41`)

## Riferimenti Standard

- `docs/standards/accessibility-standard.md:1-55` — Requisiti a11y

## Note

- Per “crea componente” usare `docs/procedures/svelte/component-creation.md` e applicare i requisiti a11y di questo standard fin da subito.
