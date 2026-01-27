# Standard: Implementazione UI e Design Tokens

## Obiettivo

Definisce come implementare l’UI in modo coerente con [ui-ux-design](../project/ui-ux-design.md): token (CSS variables), naming, struttura componenti e comportamenti (scroll, layout). Fonte di verità visiva: ui-ux-design; questo standard è il “come” tecnico.

## Design Tokens (CSS Custom Properties)

- **Dove:** variabili globali su `:root` (es. in `src/app.css` o equivalente importato in `+layout.svelte`).
- **Gerarchia** (allineata a Design Tokens Community Group):
  - **Primitivi:** valori “grezzi” (es. `--color-blue-500`, `--space-4`).
  - **Alias/semantici:** valori con contesto (es. `--color-primary`, `--color-background`, `--spacing-md`).
  - **Component:** variabili per singolo componente quando servono (override locali).
- **Naming:** kebab-case; preferire **semantico** rispetto a letterale (es. `--color-primary` invece di `--blue`). Categorie tipiche: `--color-*`, `--spacing-*`, `--font-*`, `--radius-*`, `--shadow-*`, `--z-*`.
- **Riferimento ui-ux-design:** palette, tipografia, spacing e sensazione “in game” / VS Code vanno mappati su token semantici; non definire token generici (es. `--gray-50`) senza un uso chiaro nel design.

## Stili e Componenti

- **Scoped:** usare stili scoped di Svelte per i componenti; per nomi di classe si possono adottare convenzioni tipo BEM (**B**lock **E**lement **M**odifier) solo se utile alla leggibilità (es. `.card`, `.card__title`, `.card--highlight`). Lo scope Svelte evita conflitti anche senza BEM stretto.
- **Globali:** solo per token, reset e comportamenti layout (es. scroll, viewport). Evitare stili globali su classi di componente.
- **Layout:** Top Bar e Sidebar fuori dal contenitore scrollabile; Area contenuto in un wrapper con `overflow: auto`. Scroll limitato all’area contenuto come da [ui-ux-design](../project/ui-ux-design.md).
- **Scrollbar:** nascoste di default (`scrollbar-width: none`; `::-webkit-scrollbar { display: none }` dove serve); scroll attivo con rotella/touch.

## Tema (light/dark)

- Se si introduce tema: usare class/attribute su `html` o `body` (es. `data-theme="dark"`) e variabili che cambiano in base al tema.
- Preferire token semantici che hanno valore diverso per tema (es. `--color-background` = chiaro/scuro) invece di duplicare ogni primitivo.

## Riferimenti

- [Design Tokens Community Group – Format](https://tr.designtokens.org/)
- [Svelte – Scoped styles](https://svelte.dev/docs/svelte/scoped-styles)
- [Svelte – Custom properties](https://svelte.dev/docs/svelte/custom-properties)
- [ui-ux-design](../project/ui-ux-design.md) — fonte di verità per stile, layout, scroll

## Data Decisione

2026-01-27

## Note

- Per “applica stile”, “crea componente UI”, “aggiungi tema” usare questo standard; per struttura e posizione file vedere [project-structure](../project/project-structure.md) e [component-creation](../procedures/svelte/component-creation.md).
