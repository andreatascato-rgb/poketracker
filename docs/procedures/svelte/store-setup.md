# Procedure: Creare Store Svelte

## Obiettivo

Definisce come aggiungere un nuovo store (stato condiviso) nel frontend PokeTracker usando Svelte 5 runes o pattern compatibili.

## Quando Usare Questa Procedure

- Query: "crea store", "aggiungi store", "nuovo store", "svelte store", "stato globale", "shared state", "aggiungi stato condiviso"
- Quando si deve introdurre stato condiviso tra componenti (es. stato app, preferenze, cache)
- Se la query corrisponde, questa procedure è OBBLIGATORIA

## Obbligatorietà

Completamento integrale della checklist obbligatorio prima di proporre implementazione. Nessuna eccezione.

## File da Leggere PRIMA

1. **Svelte 5 / SvelteKit**: `docs/standards/svelte-sveltekit-standard.md:1-80`
   - `$state` per stato reattivo: righe 17-18
   - `$derived` per valori calcolati: righe 19-20
   - `$effect` per side effect: righe 21-30

2. **Struttura progetto**: `docs/project/project-structure.md`
   - Cartella `lib/stores/`: cercare "stores" nella struttura frontend

3. **TypeScript/Frontend**: `docs/standards/typescript-frontend-standard.md:36-62`
   - Se lo store chiama comandi Tauri: `invoke`, `try/catch`, tipi

## Checklist Obbligatoria

1. [ ] Leggi `docs/standards/svelte-sveltekit-standard.md:13-31` — Preferire runes: per stato condiviso usare `$state()` (o oggetto/class con `$state` interno) esportato da un modulo
2. [ ] Verifica `docs/project/project-structure.md` — Crea il file in `src/lib/stores/<nome>.ts` (o `.js` se senza tipi); naming kebab-case o camelCase coerente con gli altri store
3. [ ] Esporre stato tramite variabile reattiva (`$state`) o getter; evitare store Svelte 4 (`writable`/`readable`) nel nuovo codice se il progetto è migrato a Svelte 5
4. [ ] Se lo store chiama `invoke`: import da `@tauri-apps/api/core`, uso `await` e `try/catch` (`typescript-frontend-standard.md:48-62`)
5. [ ] Import da `$lib/stores/...` nei componenti che usano lo store (`typescript-frontend-standard.md:21-34`)

## Riferimenti Standard

- `docs/standards/svelte-sveltekit-standard.md:1-80` — Runes, $state, $derived
- `docs/project/project-structure.md` — Posizione `lib/stores/`
- `docs/standards/typescript-frontend-standard.md:36-62` — Invoke e gestione errori

## Note

- In Svelte 5, uno “store” può essere un modulo che esporta `$state` o un oggetto con proprietà reattive; non è obbligatorio usare l’API `writable`/`readable`.
- Se il progetto usa ancora Svelte 4 store, preferire comunque runes per i nuovi store quando si è su Svelte 5.
