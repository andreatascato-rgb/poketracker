# Standard: Tooling (lint, format)

## Obiettivo

Definisce strumenti e configurazione per lint e formattazione nel progetto PokeTracker (frontend Svelte/TypeScript, backend Rust, sidecar C#). Dove applicabile, i path e i comandi sono da considerare obbligatori o fortemente raccomandati.

## Frontend (Svelte + TypeScript)

### Opzione A: ESLint + Prettier (stabile)

- **Setup:** `npx sv add eslint` nel progetto SvelteKit; aggiunge ESLint, plugin Svelte e file di config.
- **Config:** formato flat `eslint.config.js` (non `.eslintrc`). Per TypeScript usare `typescript-eslint`.
- **Prettier:** installare separatamente e configurarlo in modo che ESLint non confligga (es. `eslint-config-prettier`).
- **Estensione:** `eslint-plugin-svelte` per regole Svelte; regole recommended per errori e qualità.

### Opzione B: Biome (unico tool, sperimentale Svelte)

- **Setup:** `biome.json` (o `biome.jsonc`) nella **root** del progetto, accanto a `package.json`.
- **Biome v2.3+:** supporto **sperimentale** per Svelte (parsing, format, lint di script/style/template). Per sintassi Svelte molto specifiche potrebbero esserci limiti.
- **Vantaggi:** un solo tool per lint + format, prestazioni migliori, meno dipendenze. Migrazione da ESLint: `biome migrate eslint`.
- **Scelta:** usare Biome se si accetta lo stato sperimentale Svelte; altrimenti Opzione A.

Per PokeTracker si può partire con **Opzione A** (ESLint via `sv add eslint`) per massima compatibilità Svelte, e valutare Biome quando il supporto Svelte sarà stabile.

### Dove applicare

- Lint/format su `src/` (frontend), inclusi `*.svelte`, `*.ts`, `*.js`.
- Eseguire in CI e, se possibile, in pre-commit (es. script in `package.json`).

## Rust (src-tauri)

- **rustfmt:** formattazione. File di config `rustfmt.toml` o `.rustfmt.toml` in **project root** oppure in **`src-tauri/`**; rustfmt lo cerca dalla directory del manifest (`CARGO_MANIFEST_DIR`) e nelle parent. Per un unico progetto Rust (`src-tauri`) è coerente mettere **`src-tauri/rustfmt.toml`** (o in root se si vuole un solo file condiviso).
- **clippy:** lint. Configurazione tramite **`[lints.clippy]`** in `src-tauri/Cargo.toml` oppure file **`clippy.toml`** / **`.clippy.toml`** nella stessa directory del manifest. Esempio in Cargo.toml:
  ```toml
  [lints.clippy]
  enum_glob_use = "deny"
  ```
- **Comandi:** `cargo fmt` (format), `cargo clippy` (lint). In CI: `cargo fmt -- --check` e `cargo clippy -- -D warnings`.

## C# (src-sidecar)

- **EditorConfig:** stile e analisi. File **`.editorconfig`** nella root del repo o in `src-sidecar/`. Creazione da template: `dotnet new editorconfig` in `src-sidecar/`.
- **dotnet format:** applica le regole definite in `.editorconfig`. Uso:
  ```bash
  dotnet format src-sidecar/PokeTracker.Parser.csproj
  ```
  Opzione `--verify-no-changes` per fare solo check in CI.
- **Posizione:** per un unico progetto C# in `src-sidecar/`, un `.editorconfig` in **`src-sidecar/`** o in **root** è sufficiente; in root si applica a tutto il repo se altri tool (EditorConfig-aware) lo leggono.

## Dove documentare

- Questo standard è il riferimento per **quali** tool usare e **dove** mettere i file di config.
- I dettagli (regole specifiche, severity, exclude) si documentano nei file di config stessi o in brevi commenti inline; le scelte di default “raccomandate” restano qui.

## Riepilogo path e comandi

| Stack    | Lint/format     | File config (es.)        | Comando (es.)                    |
|----------|------------------|--------------------------|----------------------------------|
| Frontend | ESLint + Prettier| `eslint.config.js`, `.prettierrc` | `npm run lint`, `npm run format` |
| Frontend | Biome            | `biome.json` (root)      | `npx biome check .`              |
| Rust     | rustfmt + clippy | `src-tauri/rustfmt.toml`, `[lints.clippy]` in Cargo.toml | `cargo fmt`, `cargo clippy`      |
| C#       | dotnet format    | `src-sidecar/.editorconfig` o root | `dotnet format src-sidecar/...`   |

## Riferimenti

- [Svelte CLI – eslint](https://svelte.dev/docs/cli/eslint)
- [eslint-plugin-svelte](https://sveltejs.github.io/eslint-plugin-svelte/)
- [Biome – Configure](https://biomejs.dev/guides/configure-biome/), [Biome v2.3 Svelte](https://biomejs.dev/blog/biome-v2-3/)
- [rustfmt – Configuration](https://rust-lang.github.io/rustfmt/)
- [Clippy – Configuration](https://doc.rust-lang.org/clippy/configuration.html)
- [dotnet format](https://learn.microsoft.com/en-us/dotnet/core/tools/dotnet-format), [EditorConfig .NET](https://learn.microsoft.com/en-us/dotnet/fundamentals/code-analysis/configuration-files)

## Data Decisione

2026-01-27

## Note

- Per **testing** (unit, e2e, Tauri) si usa la documentazione in [testing-strategy](../project/testing-strategy.md); questo standard riguarda solo lint e format.
- In fase di bootstrap definire gli script in `package.json` (frontend) e, se utile, un unico script/CI che lancia lint/format per frontend, Rust e C#.
