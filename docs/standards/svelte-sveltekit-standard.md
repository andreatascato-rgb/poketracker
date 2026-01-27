# Standard: Svelte 5 e SvelteKit 2

## Obiettivo

Definisce versioni, runes, pattern e best practice Svelte 5 + SvelteKit 2 per il frontend PokeTracker, in particolare in abbinamento a Tauri.

## Versioni

- **Svelte 5** stabile da ottobre 2024. Usare **Svelte 5** (non 4) per nuovi componenti.
- **SvelteKit 2** (late 2023): Vite 5, shallow routing. Compatibile con Svelte 5.
- Per Tauri si usa **adapter-static** + **SSR disattivato** (vedi [tauri2-standard.md](./tauri2-standard.md)).

## Runes (reattività esplicita)

Svelte 5 sostituisce la reattività implicita con le **runes**. Preferirle sempre nel nuovo codice.

| Svelte 4 | Svelte 5 | Uso |
|----------|----------|-----|
| `let x = 0` | `let x = $state(0)` | Stato reattivo |
| `$: y = x * 2` | `const y = $derived(x * 2)` | Valore derivato |
| `$: { ... }` | `$effect(() => { ... })` | Side effect |

- **`$state`**: variabili reattive, nessun wrapper da usare nel template.
- **`$derived`**: valori calcolati; si ricalcolano al prossimo read quando cambiano le dipendenze.
- **`$effect`**: side effect (DOM, fetch, librerie esterne). Esegue **solo in browser**; per cleanup restituire una funzione:
  ```javascript
  $effect(() => {
    const id = setInterval(() => { ... }, 1000);
    return () => clearInterval(id);
  });
  ```

## Props e bindable

- **`$props()`**: input dei componenti. Destrutturare con default e rest:
  ```javascript
  let { count = 0, className, ...rest } = $props();
  ```
- **`$bindable()`**: per props modificabili dal parent (two-way):
  ```javascript
  let { value = $bindable() } = $props();
  ```

## onMount vs $effect

- In Svelte 5 preferire **`$effect`** per logica che deve reagire allo stato o alla mount (es. init dopo render).
- **`onMount`** resta disponibile ma in migrazione viene mappato su legacy; per nuovo codice usare `$effect` con eventuale teardown.
- Per eventi su `window`: usare **`<svelte:window>`** (binding e listener con cleanup automatico).

## Separazione logica (UI vs business)

- I file **`.svelte`** contengono solo **UI**, stato locale (runes) e **chiamate a servizi/store**; la **logica di business** resta in Rust ed è esposta tramite command. I componenti usano i servizi (che incapsulano `invoke`), non `invoke` sparso nei template. Vedi [architecture-overview](../project/architecture-overview.md), [typescript-frontend-standard](./typescript-frontend-standard.md).

## SvelteKit + Tauri (SPA)

- **Adapter:** `@sveltejs/adapter-static` con `fallback: 'index.html'`.
- **Layout root** `src/routes/+layout.ts`: `export const ssr = false;` (necessario per API Tauri che usano `window`).
- **Prerender:** non prerenderare pagine che usano Tauri; in SPA con adapter-static usare `prerender: { entries: [] }` o equivalente per evitare build-time access alle API.
- **Output:** cartella `build/`; in `tauri.conf.json` impostare `frontendDist: "../build"` (vedi tauri2-standard).

## Migrazione da Svelte 4

- **Strumento:** `npx sv migrate svelte-5`. Revisionare a mano il risultato.
- È possibile **mescolare** vecchia e nuova sintassi durante la transizione.
- Obiettivo: portare tutto a runes per reattività più prevedibile e meno “magia” sul `$:`.

## SvelteKit 2 – note

- **Node:** 18.13+.
- **Migrazione a Kit 2:** `npx svelte-migrate sveltekit-2` (da Kit 1.x).
- **Shallow routing:** associare stato a voci di history senza full navigation (utile per modali/popup).

## Riferimenti

- [Svelte 5 – Runes](https://svelte.dev/docs/svelte/v5-migration-guide)
- [Svelte 5 – $effect](https://svelte.dev/docs/svelte/%24effect)
- [Svelte 5 – $props](https://svelte.dev/docs/svelte/$props)
- [SvelteKit – adapter-static](https://kit.svelte.dev/docs/adapter-static)
- [Tauri 2 – SvelteKit](https://v2.tauri.app/start/frontend/sveltekit/) (coerenza con tauri2-standard)

## Data Decisione

2026-01-27

## Note

- Per PokeTracker il frontend è **solo client-side** (SPA in finestra Tauri); non serve SSR né prerendering delle pagine che chiamano Tauri.
- `$effect` con return di funzione è il pattern corretto per cleanup (interval, listener, observer) e riduce rischi di memory leak.
