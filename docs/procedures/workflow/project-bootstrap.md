# Procedure: Bootstrap Progetto (Prima Implementazione)

## Obiettivo

Definisce come avviare da zero il progetto PokeTracker (Tauri 2 + SvelteKit + Rust) rispettando standard e struttura documentati.

## Quando Usare Questa Procedura

- Query: "bootstrap", "avvia progetto", "setup iniziale", "prima implementazione", "init progetto", "crea progetto da zero", "first implementation", "da zero", "inizializza progetto", "scaffold", "crea app tauri"
- Quando il progetto non esiste ancora o si deve (re)inizializzare l’ambiente di sviluppo
- Se la query corrisponde, questa procedure è OBBLIGATORIA

## Obbligatorietà

Completamento integrale della checklist obbligatorio prima di proporre implementazione. Nessuna eccezione.

## File da Leggere PRIMA

1. **Versioni e comandi:** `docs/standards/versioni-stack-standard.md` — versioni minime, import esatti (es. `invoke` da `@tauri-apps/api/core`), anti-pattern Svelte 5 / Tauri 2 / Rust.

2. **Tauri 2**: `docs/standards/tauri2-standard.md:1-75`
   - Creazione progetto: righe 10-16
   - Frontend SvelteKit: righe 28-54
   - Permessi: righe 56-61

3. **Svelte/SvelteKit**: `docs/standards/svelte-sveltekit-standard.md:1-80`
   - Adapter static, SSR false: righe 54-59 (sezione «SvelteKit + Tauri (SPA)»)

4. **Struttura progetto**: `docs/project/project-structure.md:1-50`
   - Cartelle root e src-tauri/src: righe 9-45

5. **Organizzazione file**: `docs/standards/file-organization.md:1-44`
   - Documentazione in `docs/`: righe 9-21

## Checklist Obbligatoria

1. [ ] Leggi `docs/standards/tauri2-standard.md:10-16` — Usa `npm create tauri-app@latest`, scegli SvelteKit
2. [ ] Verifica `docs/standards/tauri2-standard.md:28-54` — Adapter-static, `ssr = false` in layout root, `frontendDist: "../build"`
3. [ ] Verifica `tauri.conf.json` (o equivalente): `devUrl`, `frontendDist`, `beforeDevCommand`/`beforeBuildCommand` come in `tauri2-standard.md:44-53`
4. [ ] Verifica permessi Tauri 2 (`permissions`, non allowlist): `docs/standards/tauri2-standard.md:56-61`
5. [ ] Allinea versioni `@tauri-apps/api` (npm) e crate `tauri` (Cargo): stessa minor; plugin stessa versione esatta (`tauri2-standard.md:16`)
6. [ ] Verifica struttura cartelle rispetto a `docs/project/project-structure.md:9-45` — `src-tauri/`, `src/` (frontend), `docs/`
7. [ ] **Verifica compilazione:** Dopo lo scaffold, esegui `npm run check:rust` (o `cargo check` in `src-tauri/` se Cargo è in PATH; oppure `npm run tauri build`). Se Rust è in `C:\_Main\_app`, usa `npm run check:rust` così lo script imposta PATH. Vedi [coding-best-practices](../../standards/coding-best-practices.md) § Workflow.
8. [ ] Se esistono, tooling e lint come da `docs/standards/tooling-standard.md` (opzionale in bootstrap minimo)

## Riferimenti Standard

- `docs/standards/tauri2-standard.md:1-75` — Tauri 2, SvelteKit, tauri.conf, permessi
- `docs/standards/svelte-sveltekit-standard.md:1-80` — Adapter, SSR, runes
- `docs/project/project-structure.md:1-50` — Struttura cartelle e moduli
- `docs/standards/file-organization.md:1-44` — Dove va la documentazione

## Note

- Se il progetto esiste già e si aggiungono solo parti (feature, comandi, componenti), usare le procedure specifiche (new-feature, command-creation, component-creation).
- Per sidecar C# e parser, dopo il bootstrap usare la procedura `sidecar-setup.md`.
