# Standard: Tauri 2

## Obiettivo

Definisce versioni, configurazione e best practice Tauri 2 per PokeTracker. Riferimento per bootstrap e sviluppo.

## Versione e Creazione Progetto

- **Tauri 2** stabile da ottobre 2024. Usare sempre **Tauri 2** (non 1.x).
- Creare il progetto con:
  ```bash
  npm create tauri-app@latest
  ```
- Scegliere **SvelteKit** come frontend (o **Svelte** via Vite se si preferisce setup più minimale).
- Mantenere **versioni allineate**: `@tauri-apps/api` (npm) e crate `tauri` (Cargo) devono avere la stessa **minor**; per i plugin, stessa versione esatta (es. `2.2.1` su entrambi).

## Breaking Changes Rispetto a Tauri 1

Se si migra da Tauri 1, usare `npm run tauri migrate` e poi verificare manualmente la [guida ufficiale](https://v2.tauri.app/start/migrate/from-tauri-1).

- **Config:** `package > productName` e `package > version` sono in **root**; la chiave `tauri` è rinominata in **`app`**.
- **Allowlist rimossa:** sostituita dal sistema **permissions**. I permessi core usano il prefisso `core:` (es. `core:path:default`, `core:event:default`) oppure il set `core:default`.
- **Asset:** `assetScope` si configura in `app > security > assetProtocol > scope` (array di glob).
- **Drop file:** `fileDropEnabled` rinominato in **`dragDropEnabled`**.
- **Updater:** sotto `plugins > updater`; per i bundle abilitare `bundle > createUpdaterArtifacts`.

## Frontend: SvelteKit con Tauri 2

Tauri non supporta soluzioni server-side. Per SvelteKit:

1. **Adapter statico**
   - `npm install --save-dev @sveltejs/adapter-static`
   - In `svelte.config.js` usare `adapter-static` con `fallback: 'index.html'`.

2. **SSR disattivato**
   - In `src/routes/+layout.ts` (root): `export const ssr = false;`
   - Necessario per usare le API Tauri che dipendono da `window`.

3. **Niente prerendering** delle pagine che usano Tauri: le `load` in build non hanno accesso alle API Tauri.

### Configurazione `tauri.conf.json` (build)

```json
{
  "build": {
    "beforeDevCommand": "npm run dev",
    "beforeBuildCommand": "npm run build",
    "devUrl": "http://localhost:5173",
    "frontendDist": "../build"
  }
}
```

- **SvelteKit:** `frontendDist` = `../build` (output dell’adapter static).
- **Vite + Svelte:** `frontendDist` = `../dist`, dev server tipicamente su porta 5173.

## Permessi e Sicurezza

- Usare il modello **permissions** di Tauri 2, non l’allowlist.
- Permessi core: prefisso `core:` o set `core:default`.
- `assetProtocol.scope`: array di glob per i path serviti come asset; `deny` ha priorità su `allow`.

## Mobile (Opzionale)

Se in futuro si targetta iOS/Android:

- In `Cargo.toml`: sezione `[lib]` con `crate-type = ["staticlib","cdylib","rlib"]`.
- Rinominare `main.rs` in `lib.rs` e usare `#[cfg_attr(mobile, tauri::mobile_entry_point)]`.
- Creare un nuovo `main.rs` che invoca la funzione di run condivisa.
- Per il dev server in rete usare `TAURI_DEV_HOST` (non solo `TAURI_ENV_PLATFORM`).

## Riferimenti

- [Tauri 2 – Create Project](https://v2.tauri.app/start/create-project/)
- [Tauri 2 – SvelteKit](https://v2.tauri.app/start/frontend/sveltekit/)
- [Tauri 2 – Migrate from 1.x](https://v2.tauri.app/start/migrate/from-tauri-1)
- [Tauri 2 – Permissions](https://v2.tauri.app/security/permissions)

## Data Decisione

2026-01-27

## Note

- Per PokeTracker si assume **desktop** (Windows/macOS/Linux); il paragrafo Mobile è per eventuale estensione futura.
- Aggiornare dipendenze con la [guida ufficiale](https://v2.tauri.app/develop/updating-dependencies).
