# Procedure: Creare Componente Svelte

## Obiettivo

Definisce come aggiungere un nuovo componente Svelte (Svelte 5 runes) nel frontend PokeTracker, nella cartella e con convenzioni corrette.

## Quando Usare Questa Procedure

- Query: "crea componente", "aggiungi componente", "nuovo componente", "component svelte", "svelte component", "aggiungi .svelte", "nuovo componente svelte", "crea .svelte"
- Quando si deve introdurre un nuovo file `.svelte` in `src/lib/components/` (o sottocartella)
- Se la query corrisponde, questa procedure è OBBLIGATORIA

## Obbligatorietà

Completamento integrale della checklist obbligatorio prima di proporre implementazione. Nessuna eccezione.

## File da Leggere PRIMA

1. **Svelte 5 / SvelteKit**: `docs/standards/svelte-sveltekit-standard.md:1-80`
   - Runes `$state`, `$derived`, `$effect`: righe 13-31
   - Props `$props()`, `$bindable()`: righe 33-41
   - Preferenza `$effect` vs onMount: righe 43-47

2. **Struttura progetto**: `docs/project/project-structure.md:117-136`
   - Cartella `lib/components/` e sottocartelle (ui, layout, profile, pokedex, wiki, editor, management)

3. **TypeScript/Frontend**: `docs/standards/typescript-frontend-standard.md:21-34`
   - Path alias `$lib`, `$components` se configurati

4. **Se il componente è UI**: `docs/standards/ui-stack-standard.md` — Stack obbligatorio Tailwind + shadcn-svelte; usare componenti da `$lib/components/ui` (Button, Input, Card, ecc.) quando disponibili; non introdurre CSS custom o componenti UI da zero.

## Checklist Obbligatoria

1. [ ] Leggi `docs/standards/svelte-sveltekit-standard.md:13-31` — Usa runes (`$state`, `$derived`, `$effect`) per reattività; niente `let` reattivo senza runes nei nuovi componenti
2. [ ] Leggi `docs/standards/svelte-sveltekit-standard.md:33-41` — Props con `$props()`; per two-way usare `$bindable()`
3. [ ] Verifica `docs/project/project-structure.md:117-136` — Crea il file in `src/lib/components/<dominio>/NomeComponente.svelte` (es. `pokedex/PokemonCard.svelte`); usa PascalCase per il file
4. [ ] Per effetti (interval, listener, subscribe): usare `$effect` con funzione di cleanup in return (`svelte-sveltekit-standard.md:24-30`)
5. [ ] Per eventi su `window`: usare `<svelte:window>` e binding/listener con cleanup (`svelte-sveltekit-standard.md:46-47`)
6. [ ] Import da `$lib/...` o alias configurati (`typescript-frontend-standard.md:21-34`)
7. [ ] Se il componente invoca comandi Tauri: usare `invoke` da `@tauri-apps/api/core` e `try/catch` (`typescript-frontend-standard.md:36-62`)
8. [ ] **Se il componente è UI visibile:** leggere `docs/standards/ui-stack-standard.md`; usare Tailwind (classi utility) e componenti da `$lib/components/ui` (Button, Input, Card, ecc.) quando disponibili; non introdurre CSS custom o componenti UI da zero al posto di Tailwind/shadcn

## Riferimenti Standard

- `docs/standards/ui-stack-standard.md` — Se il componente è UI: usare Tailwind + shadcn-svelte, best practice e anti-pattern
- `docs/standards/design-system-standard.md` — Se il componente è UI visibile: token, tipografia, icone, stile (moderno, professionale, dark)
- `docs/standards/versioni-stack-standard.md` — Versioni Svelte 5, comandi esatti, anti-pattern (non usare `$:`, `let` reattivo, `export let`)
- `docs/standards/svelte-sveltekit-standard.md:1-80` — Runes, props, $effect, adapter
- `docs/project/project-structure.md:117-136` — Dove creare componenti
- `docs/standards/typescript-frontend-standard.md:1-70` — Alias, invoke, errori
## Note

- Naming file: PascalCase (es. `PokemonCard.svelte`).
- Sottocartelle per dominio: layout, profile, pokedex, wiki, editor, management.
- **Componenti UI:** vanno in `layout/` (TopBar, Sidebar, ContentArea) o in altre sottocartelle di dominio; per elementi riutilizzabili (button, input, card) usare quelli in `$lib/components/ui` da shadcn-svelte; per nuovi primitivi UI usare `npx shadcn-svelte@latest add <nome>` e metterli in `$lib/components/ui/`.
- Per aggiungere una nuova pagina/route SvelteKit, usare la procedure `page-creation.md` se disponibile, altrimenti rispettare `src/routes/` e `+page.svelte`/`+layout.svelte`.
