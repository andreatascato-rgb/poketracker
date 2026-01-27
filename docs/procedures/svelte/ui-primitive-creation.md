# Procedure: Creare Primitivo UI

## Obiettivo

Definisce come aggiungere un nuovo primitivo UI (Button, Input, Card, Badge, IconButton, Link, …) in modo riutilizzabile e coerente con token e catalog. Reuse-first: prima controllare il catalog; se il primitivo esiste, usarlo o estenderlo; creare nuovo solo se manca.

## Quando Usare Questa Procedure

- Query: "crea pulsante", "crea input", "crea card", "crea badge", "aggiungi primitivo UI", "elemento classico UI", "crea Button", "crea componente pulsante", "aggiungi primitivo"
- Quando si deve introdurre un **elemento UI classico** riutilizzabile (pulsante, input, card, badge, icona, link, spinner, …)
- Se la query corrisponde, questa procedure è OBBLIGATORIA

## Obbligatorietà

Completamento integrale della checklist obbligatorio prima di proporre implementazione. Nessuna eccezione.

## File da Leggere PRIMA

1. **Catalog**: `docs/project/ui-component-catalog.md:1-35`
   - Tabella Primitivi: se esiste già il tipo richiesto, usarlo; non creare duplicati

2. **UI Primitives Standard**: `docs/standards/ui-primitives-standard.md:1-70`
   - Gerarchia token → primitivi → compositi; path `lib/components/ui/`; regola reuse-first; mappa standard

3. **UI Implementation**: `docs/standards/ui-implementation-standard.md:7-28`
   - Token semantici; stili scoped; nessun colore/spacing hardcoded

4. **UI/UX Design**: `docs/project/ui-ux-design.md:49-72`
   - Colori, tipografia, layout di riferimento

5. **Component creation** (per convenzioni Svelte): `docs/procedures/svelte/component-creation.md:17-38`
   - Runes, props, path alias; il primitivo è un componente Svelte come gli altri

## Checklist Obbligatoria

1. [ ] Leggi `docs/project/ui-component-catalog.md` — Se nella tabella Primitivi esiste un componente che copre il bisogno (es. Button), proporre di usarlo o di estenderlo con nuove props/varianti; **non** creare un secondo Button/Input/Card uguale
2. [ ] Se non esiste: leggi `docs/standards/ui-primitives-standard.md:9-35` — Path `src/lib/components/ui/NomePrimitivo.svelte`; un file per tipo; varianti via props
3. [ ] Usa **solo** token CSS (es. `var(--color-primary)`, `var(--spacing-md)`); niente colori o spacing hardcoded (`ui-implementation-standard.md:14-16`)
4. [ ] Props con `$props()`: varianti (es. `variant`, `size`) come prop; classe/stile condizionale che mappa a token (`svelte-sveltekit-standard.md:33-41`)
5. [ ] Rispetta runes e convenzioni Svelte 5 (`component-creation.md:32-38`); accessibilità (label, focus, ruoli) come da `docs/standards/accessibility-standard.md`
6. [ ] **Aggiorna il catalog:** aggiungere una riga in `docs/project/ui-component-catalog.md` nella tabella Primitivi con Nome, Path, Props principali, Varianti, Token/Note

## Riferimenti Standard

- `docs/standards/ui-primitives-standard.md:1-70` — Gerarchia, path, reuse-first, mappa
- `docs/project/ui-component-catalog.md` — Catalog primitivi e compositi
- `docs/standards/ui-implementation-standard.md:7-28` — Token, stili
- `docs/project/ui-ux-design.md:49-72` — Colori, tipografia
- `docs/procedures/svelte/component-creation.md` — Convenzioni Svelte
- `docs/standards/accessibility-standard.md` — a11y

## Note

- Naming file: PascalCase (es. `Button.svelte`, `Input.svelte`).
- Per “crea componente” generico che **non** è un elemento classico (es. PokemonCard), usare [component-creation.md](./component-creation.md); lì verificare comunque catalog e primitivi prima di introdurre stili ad hoc.
- Se il primitivo richiede icone: usare slot o prop `icon`; preferire icon set già in uso nell’app.
