# Standard: Internazionalizzazione (i18n)

## Obiettivo

Definisce come gestire traduzioni e locale in PokeTracker (frontend Svelte; eventuale condivisione con Rust via file/risorse). Per app desktop Tauri si può partire da i18n solo frontend e estendere in seguito.

## Scelta approccio

- **Solo frontend (minimo):** file di traduzione in `src/lib/i18n/` (es. `it.json`, `en.json`); locale da preferences o da primo avvio; nessun backend i18n.
- **Frontend + risorse condivise:** stesso formato (JSON) in cartella `lang/` (o simile) inclusa nel bundle; Rust legge via `PathResolver` se servono stringhe in notifiche/errori backend; frontend legge da API o da file locale.

## Struttura chiavi

- **Namespacing:** chiavi gerarchiche per sezione (es. `common.save`, `profile.dashboard.title`, `editor.toolbar.undo`). Evitare chiavi piatte lunghe.
- **Naming:** snake_case o dot.path; coerente in tutta l’app. Esempio: `errors.file_not_found`, `buttons.confirm`.
- **Fallback:** lingua di default (es. `en`); se manca una chiave, usare chiave default o la chiave stessa come placeholder temporaneo.

## Librerie frontend (Svelte/SvelteKit)

- **sveltekit-i18n:** leggero, moduli per route, loaders per locale; adatto a SvelteKit. Config in `lib`, load in `+layout`, uso `$t()` in componenti.
- **typesafe-i18n:** type-safe, output in `src/lib/i18n/`, integrazione SvelteKit; chiavi validate a compile-time.
- Scegliere una e usarla in modo coerente: loaders, rilevamento locale (cookie, storage, system), cambio lingua senza reload quando possibile.

## Dove mettere le stringhe

- **Frontend:** `src/lib/i18n/locales/<locale>.json` (o per-namespace: `src/lib/i18n/locales/<locale>/common.json`, `profile.json`).
- **Risorsa condivisa (opzionale):** `lang/<locale>.json` in bundle; accesso da Rust via `resolve_resource` e da frontend via fetch o asset.

## Locale e persistenza

- Salvare la lingua scelta in preferences (tauri-store o file config); all’avvio leggere e applicare prima del primo render.
- Rilevamento iniziale: dalla config salvata, altrimenti da `navigator.language` o equivalente; fallback a lingua di default.

## Riferimenti

- [sveltekit-i18n](https://github.com/sveltekit-i18n)
- [typesafe-i18n](https://github.com/ivanhofer/typesafe-i18n)
- [Tauri – Embedding resources](https://v2.tauri.app/develop/resources/) (per file in bundle)
- [i18n-embed](https://lib.rs/crates/i18n-embed) (Rust, opzionale)

## Data Decisione

2026-01-27

## Note

- Per “aggiungi traduzione”, “i18n”, “nuova lingua” usare questo standard e la procedure i18n-setup o aggiungi-traduzione.
