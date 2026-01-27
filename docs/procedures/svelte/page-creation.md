# Procedure: Creare Pagina/Route SvelteKit

## Obiettivo

Definisce come aggiungere una nuova pagina (route) in SvelteKit per PokeTracker, rispettando adapter-static e SSR disattivato.

## Quando Usare Questa Procedure

- Query: "crea pagina", "nuova pagina", "aggiungi pagina", "crea route", "nuova route", "aggiungi route", "nuova view", "+page.svelte", "sveltekit route"
- Quando si deve introdurre una nuova route in `src/routes/` (nuovo `+page.svelte` o sottocartella)
- Se la query corrisponde, questa procedure è OBBLIGATORIA

## Obbligatorietà

Completamento integrale della checklist obbligatorio prima di proporre implementazione. Nessuna eccezione.

## File da Leggere PRIMA

1. **SvelteKit + Tauri**: `docs/standards/svelte-sveltekit-standard.md:48-55`
   - Adapter-static, SSR false, prerender: righe 48-54

2. **Tauri 2 frontend**: `docs/standards/tauri2-standard.md:28-54`
   - `frontendDist`, devUrl, build: righe 44-54

3. **Struttura progetto**: `docs/project/project-structure.md:111-155`
   - `lib/components/`, `lib/stores/`, `lib/services/`; routing SvelteKit in `src/routes/`

## Checklist Obbligatoria

1. [ ] Leggi `docs/standards/svelte-sveltekit-standard.md:48-54` — Nessun prerender per pagine che usano Tauri; layout root con `ssr = false`
2. [ ] Verifica `docs/project/project-structure.md` — Crea `src/routes/<path>/+page.svelte` (e eventuale `+page.ts` per load); rispetta convenzione SvelteKit
3. [ ] Per pagine che usano `invoke` o API Tauri: non prerenderare; usare `export const prerender = false` se necessario (`svelte-sveltekit-standard.md:51`)
4. [ ] Import componenti da `$lib/components/...`; per dati da backend usare load in `+page.ts` con `invoke` o servizi (`typescript-frontend-standard.md:36-62`)
5. [ ] Rispetta runes e `$props()` nei componenti della pagina (`svelte-sveltekit-standard.md:13-41`)

## Riferimenti Standard

- `docs/standards/svelte-sveltekit-standard.md:48-55` — Adapter, SSR, prerender
- `docs/standards/tauri2-standard.md:28-54` — Build frontend per Tauri
- `docs/project/project-structure.md:111-155` — Struttura frontend e routes

## Note

- In PokeTracker il frontend è SPA in finestra Tauri; le pagine non usano SSR né prerendering che chiamano Tauri.
- Per aggiungere solo un componente (non una route), usare la procedure `component-creation.md`.
