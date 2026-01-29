# Standard: Versioni stack e comandi verificati

## Obiettivo

Definisce **versioni minime/raccomandate** e **comandi/pattern esatti** da usare (e da **non** usare) per evitare errori tipo "comando non valido per Svelte 5", "sintassi Rust obsoleta", "import Tauri sbagliato". È il riferimento per bootstrap, procedure e AI. **Revisione periodica:** aggiornare alla prima implementazione e almeno annualmente (o quando si aggiorna lo stack).

## Versioni di riferimento (2026-01-27)

Range o minimi verificati alla data indicata. Per versioni **esatte** da usare in un nuovo scaffold, preferire le **latest stable** delle minor sotto indicate; allineare sempre tra loro i pacchetti dello stesso ecosistema (vedi tauri2-standard).

| Stack | Versione | Note |
|-------|----------|------|
| **Node** | 18.13+ (consigliato 20 LTS) | Richiesto da SvelteKit 2. |
| **Tauri** | 2.x (solo 2, mai 1.x) | Crate `tauri` e `@tauri-apps/api`: **stessa minor**; plugin **stessa versione esatta**. |
| **Svelte** | 5.x (mai 4 per nuovo codice) | Runes obbligatorie per reattività. |
| **SvelteKit** | 2.x | Adapter-static, `ssr = false` in layout root. |
| **Rust** | edition **2021** (non 2024) | Con Tauri c’è bug con edition 2024 in `tauri-build`; usare 2021. |
| **TypeScript** | 5.x, `strict: true` | Da tsconfig. |

- **Bootstrap:** `npm create tauri-app@latest` e **selezionare SvelteKit** dai prompt (non usare template `svelte` se si vuole SvelteKit). Non esiste flag `--template sveltekit`; si sceglie dall’interattivo.

## Comandi e import esatti

### Tauri 2 (frontend)

- **Invoke:** sempre da `@tauri-apps/api/core`:
  ```typescript
  import { invoke } from '@tauri-apps/api/core';
  const x = await invoke<MyType>('command_name', { arg: value });
  ```
- **Non** usare `import { invoke } from '@tauri-apps/api'` (in Tauri 2 `invoke` è nel namespace `core`).
- **Permessi:** solo modello **permissions** (non allowlist). Prefisso `core:` per permessi core.

### Svelte 5 (reattività e componenti)

| Da usare | Da NON usare (Svelte 4 / deprecato) |
|----------|-------------------------------------|
| `let x = $state(0)` | `let x = 0` per stato reattivo |
| `const y = $derived(x * 2)` | `$: y = x * 2` |
| `$effect(() => { … })` e `return () => cleanup` | `$: { … }` per side effect |
| `let { a, b } = $props()` | `export let a; export let b` |
| `let { v = $bindable() } = $props()` | `export let v; bind:v` per two-way |
| `$effect` per init/cleanup in browser | `onMount` + logica reattiva (preferire `$effect`) |

- **Migrazione da Svelte 4:** `npx sv migrate svelte-5`. **Sempre revisionare a mano** il risultato (es. sostituire eventuali `run()` con `$derived`/`$effect` dove appropriato).
- **Layout root:** `src/routes/+layout.ts` (o `.svelte` con logic in `.ts`), `export const ssr = false;`.

### SvelteKit 2 + Tauri

- **Adapter:** `@sveltejs/adapter-static` con `fallback: 'index.html'`.
- **Output build:** cartella `build/` → in `tauri.conf.json` usare `"frontendDist": "../build"`.
- **Config build:** `devUrl` tipicamente `http://localhost:5173`; `beforeDevCommand` / `beforeBuildCommand` come in [tauri2-standard](./tauri2-standard.md).

### Rust (backend Tauri)

- **Edition:** in `Cargo.toml`, `edition = "2021"`. **Non** `"2024"` (bug con tauri-build).
- **Command:** ritornare `Result<T, E>` con `E: Serialize`; **non** `panic!` nei command.
- **Invoke lato Rust:** i command sono definiti con `#[tauri::command]` e registrati sul `Builder`; i nomi sono stringhe usate dal frontend in `invoke('nome_command', …)`.
- **Percorso Rust/Cargo (cartella spostata):** se Rust non è in PATH o è in una cartella custom, copia `.env.example` in `.env` e imposta `RUST_ROOT` al percorso della cartella che contiene `.cargo` e `.rustup` (es. `RUST_ROOT=C:\_Main\_app`). Gli script `run-tauri.mjs` e `cargo-check.mjs` leggono `.env` e usano quel percorso per `npm run tauri` e `npm run check:rust`.

## Da NON usare (anti-pattern)

- **Tauri:** allowlist; `import { invoke } from '@tauri-apps/api'` senza `/core`; permessi non dichiarati in capabilities.
- **Svelte 5:** `$:` per derived/effect; `let x = 0` come stato reattivo; `export let` per props (usare `$props()`); ignorare il risultato di `npx sv migrate svelte-5` senza revisione.
- **Svelte 5 — Store in file `.ts`:** **non** usare `$state` (e altre runes) in `lib/stores/*.ts` o in qualsiasi `.ts` non processato da Svelte. A runtime `$state` non è definito → `ReferenceError` e schermata bianca. Per store in `.ts` usare **`writable`/`readable`** da `svelte/store`. Nei componenti `.svelte` la sottoscrizione `$storeName` funziona normalmente.
- **Rust:** `edition = "2024"`; `panic!` nei command; ritorno non-Result dai command senza gestione errore lato frontend.
- **SvelteKit + Tauri:** SSR attivo per route che usano `invoke`; prerender di pagine che chiamano Tauri; `frontendDist` diverso da `../build` se si usa adapter-static con default.

## Dove trovare i dettagli

- **Tauri:** [tauri2-standard.md](./tauri2-standard.md)
- **Svelte/SvelteKit:** [svelte-sveltekit-standard.md](./svelte-sveltekit-standard.md)
- **Rust/command:** [rust-tauri-standard.md](./rust-tauri-standard.md)
- **Invoke e tipi frontend:** [typescript-frontend-standard.md](./typescript-frontend-standard.md)
- **Bootstrap:** [project-bootstrap](../procedures/workflow/project-bootstrap.md) → checklist che usa questi standard

## Data decisione e revisione

- **Data:** 2026-01-27
- **Prossima revisione:** alla prima implementazione (bootstrap) e poi almeno annuale, o quando si aggiornano Tauri/Svelte/SvelteKit/Rust.
