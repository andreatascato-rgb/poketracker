# Standard: TypeScript e Frontend (Tauri + SvelteKit)

## Obiettivo

Definisce strict mode, path alias, tipi condivisi con Rust e pattern di invocazione dei comandi Tauri nel frontend TypeScript/Svelte per PokeTracker.

## TypeScript Strict Mode

- Abilitare **`strict: true`** in `tsconfig.json` (o nelle `compilerOptions` del progetto).
- In SvelteKit il `tsconfig.json` deve **estendere** `.svelte-kit/tsconfig.json`; SvelteKit espone già `$lib` e path di base. Aggiungere o confermare:
  ```json
  {
    "extends": "./.svelte-kit/tsconfig.json",
    "compilerOptions": {
      "strict": true
    }
  }
  ```
- Opzioni utili insieme a `strict`: `forceConsistentCasingInFileNames`, `resolveJsonModule`, `skipLibCheck`.

## Path Alias

- **`$lib`**: alias di default in SvelteKit per `src/lib` (o la cartella lib configurata). Usare `import x from '$lib/...'` senza path relativi lunghi.
- **Alias aggiuntivi:** in `svelte.config.js` sotto `kit.alias`:
  ```javascript
  kit: {
    alias: {
      '$components': 'src/lib/components',
      '$utils': 'src/lib/utils'
    }
  }
  ```
  SvelteKit propaga gli alias a Vite e al tipo-check; i path in `tsconfig` sono di solito generati/ereditati da `.svelte-kit/tsconfig.json`. Se si usano alias custom, verificare che siano definiti sia in `kit.alias` sia in `compilerOptions.paths` (o che il generated config li includa).
- Riferimento struttura: [project-structure](../project/project-structure.md) (es. `lib/components/`, `lib/stores/`, `lib/services/`).

## Invocazione comandi Tauri

- Importare **`invoke`** da `@tauri-apps/api/core`:
  ```typescript
  import { invoke } from '@tauri-apps/api/core';
  ```
- **`invoke`** restituisce una **Promise**; usare sempre **`await`** (o `.then()`) per evitare unhandled rejection.
- Chiamata tipica:
  ```typescript
  const result = await invoke<ReturnType>('command_name', { arg1: value1 });
  ```
  Il secondo argomento è l’oggetto dei parametri (serializzabile in JSON); il tipo di ritorno si può specificare col generico `invoke<T>()`.

## Gestione errori

- I command Rust che restituiscono `Result<T, E>` portano gli errori sul frontend come **Promise reject**.
- Pattern consigliato:
  ```typescript
  try {
    const data = await invoke<MyData>('my_command', { id });
    // usare data
  } catch (err) {
    console.error('Command failed:', err);
    // messaggio utente, fallback, ecc.
  }
  ```
- Oppure `.catch()` sulla Promise. Non lasciare chiamate a `invoke` senza `await` o senza gestione errori.

## Tipi condivisi con Rust

- **Non** usare **`any`** per argomenti o ritorno di `invoke`: sempre interfacce/tipi espliciti che rispecchiano le struct Rust (parametri e return type dei command devono essere tipizzati).
- Due approcci:

### Opzione A: Generazione automatica

- **tauri-typegen** o **tauri-plugin-typegen** (per Tauri 2.2+): generano tipi TypeScript a partire da `#[tauri::command]` e dai tipi (de)serializzabili (serde) in Rust.
- I comandi e i relativi parametri/return type diventano interfacce/funzioni tipizzate in TypeScript; si riduce la deriva tra backend e frontend.
- Output tipico in una cartella tipo `src/generated/` o simile; importare da lì nei servizi che chiamano `invoke`.

### Opzione B: Tipi manuali

- Definire in TypeScript interfacce/tipi che riflettono i parametri e i return type dei command (es. in `src/lib/utils/types.ts` o per-modulo).
- Aggiornarli manualmente quando si cambiano le signature in Rust; più semplice da introdurre, ma richiede disciplina per restare allineati.

Per PokeTracker si può partire con **Opzione B** e introdurre **Opzione A** quando i command e i tipi crescono.

## Convenzioni di uso di invoke

- Incapsulare le chiamate Tauri in **servizi** (es. `lib/services/tauri.ts` o `lib/services/profile.ts`), non sparare `invoke` direttamente dai componenti quando la logica si ripete.
- Usare nomi di command chiari e coerenti con i moduli Rust (es. `get_profile`, `save_pokedex`).
- Per dati complessi, tipizzare sia gli argomenti che il ritorno:
  ```typescript
  interface GetProfileArgs { id: string; }
  interface Profile { name: string; /* ... */ }
  const p = await invoke<Profile>('get_profile', { id } satisfies GetProfileArgs);
  ```

## Riferimenti

- [Tauri 2 – Calling Rust](https://v2.tauri.app/develop/calling-rust/)
- [Tauri 2 – JavaScript API (core)](https://v2.tauri.app/reference/javascript/api/namespacecore)
- [SvelteKit – $lib](https://svelte.dev/docs/kit/$lib)
- [SvelteKit – Configuration (alias)](https://svelte.dev/docs/kit/configuration#alias)
- [TypeScript – strict](https://www.typescriptlang.org/tsconfig/strict)
- [rust-tauri-standard](./rust-tauri-standard.md) (Result e Serialize lato Rust)
- [project-structure](../project/project-structure.md) (layout frontend)

## Data Decisione

2026-01-27

## Note

- Mantenere la **versione** di `@tauri-apps/api` allineata alla minor della crate `tauri` (vedi [tauri2-standard](./tauri2-standard.md)).
- In ambiente Tauri non eseguire `invoke` durante SSR; usare le chiamate solo in codice eseguito in browser (es. in `$effect`, in `onMount` o dopo aver verificato che si è in client). Con `ssr = false` nel layout root questo è già garantito per le pagine che usano Tauri.
