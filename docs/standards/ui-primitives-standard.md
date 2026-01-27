# Standard: Primitivi UI e Design System

## Obiettivo

Definisce la gerarchia token → primitivi → compositi, dove vivono i primitivi UI, e la regola **reuse-first**: prima controllare il [catalog](../project/ui-component-catalog.md); se un primitivo esiste, usarlo o estenderlo; creare nuovo solo se manca. Un solo linguaggio visivo per tutta l’app (2026).

## Gerarchia

1. **Design tokens** (CSS variables) — fonte: [ui-implementation-standard](./ui-implementation-standard.md), [ui-ux-design](../project/ui-ux-design.md).
2. **Primitivi UI** — Button, Input, Card, Badge, IconButton, Link, ecc. Usano solo token; nessuno stile ad hoc.
3. **Componenti compositi** — Domain-specific (es. PokemonCard, ProfileSelector). Costruiti da primitivi + eventuale logica; stessi token e stessi primitivi.

## Dove vivono i primitivi

- **Path:** `src/lib/components/ui/` (sottocartella dedicata ai primitivi).
- **Naming:** PascalCase (es. `Button.svelte`, `Input.svelte`, `Card.svelte`).
- **Un solo file per tipo di primitivo;** le varianti (size, variant) sono props, non componenti diversi.

## Regola reuse-first

- **Prima di creare un elemento classico** (pulsante, input, card, badge, icona, link, …): controllare [ui-component-catalog](../project/ui-component-catalog.md). Se esiste un primitivo adatto, usarlo. Se serve una variante (es. nuovo colore/size), estendere il primitivo tramite props.
- **Prima di creare un componente personalizzato:** verificare il catalog; preferire composizione da primitivi esistenti invece di nuovi stili ad hoc.
- **Nuovo primitivo** solo se il catalog non copre il bisogno e il pattern è riutilizzabile in più punti. Alla creazione: usare token, definire props/varianti, aggiornare il catalog.

## Tipi di primitivi (catalog)

Il catalog elenca i primitivi attivi. Tipi tipici:

- **Button** — azioni (primary, secondary, danger, link; size sm/md/lg).
- **Input** — campi testo (text, number, search; con label, errore).
- **Card** — contenitore (con optional header/footer).
- **Badge** — etichette, stati, count.
- **IconButton** — azione con sola icona.
- **Link** — navigazione (interno/esterno).
- **Spinner / Skeleton** — caricamento.
- **Toast / Alert** — notifiche (per implementazione vedi [error-handling-standard](./error-handling-standard.md)).

Estendere il catalog quando si aggiunge un nuovo tipo; non inventare componenti “simili” senza aggiungerli.

## Mappa: cosa crei → quali standard

| Cosa crei | Standard da applicare |
|-----------|------------------------|
| **Elemento classico** (pulsante, input, card, badge, icona, link, …) | [ui-primitives-standard](./ui-primitives-standard.md) (questo), [ui-implementation-standard](./ui-implementation-standard.md), [ui-ux-design](../project/ui-ux-design.md), [component-creation](../procedures/svelte/component-creation.md), [accessibility-standard](./accessibility-standard.md). Procedure: [ui-primitive-creation](../procedures/svelte/ui-primitive-creation.md). |
| **Componente personalizzato** (PokemonCard, ProfileSelector, …) | [component-creation](../procedures/svelte/component-creation.md), [ui-implementation-standard](./ui-implementation-standard.md), questo standard (componi da primitivi), [ui-ux-design](../project/ui-ux-design.md), [accessibility-standard](./accessibility-standard.md), [svelte-sveltekit-standard](./svelte-sveltekit-standard.md), [typescript-frontend-standard](./typescript-frontend-standard.md). |

## Token e varianti

- I primitivi usano **solo** token semantici (es. `var(--color-primary)`, `var(--spacing-md)`); niente colori o spacing hardcoded.
- Le varianti (primary/secondary, sm/md/lg) sono **props**; nel componente, classi o stili condizionali mappano prop → token.
- Per “applica stile”, “tema”, “token” vedi [ui-implementation-standard](./ui-implementation-standard.md).

## Documentazione per riuso

- Ogni primitivo deve essere **registrato** in [ui-component-catalog](../project/ui-component-catalog.md) con: Nome, Path, Props principali, Varianti, Token usati.
- Così l’AI e i dev sanno cosa esiste e come riutilizzarlo in modo identico.

## Riferimenti

- [ui-implementation-standard](./ui-implementation-standard.md) — token, layout, stili
- [ui-ux-design](../project/ui-ux-design.md) — fonte di verità visiva
- [ui-component-catalog](../project/ui-component-catalog.md) — elenco primitivi e compositi
- [project-structure](../project/project-structure.md) — struttura `lib/components/`
- [accessibility-standard](./accessibility-standard.md) — a11y per tutti i componenti UI

## Data Decisione

2026-01-27

## Note

- Per “crea pulsante”, “crea input”, “aggiungi primitivo”, “elemento classico UI” usare la procedure [ui-primitive-creation](../procedures/svelte/ui-primitive-creation.md).
- Per “crea componente” generico: [component-creation](../procedures/svelte/component-creation.md); se l’output è un elemento UI classico, seguire prima ui-primitive-creation.
