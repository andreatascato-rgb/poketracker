# Procedure: Creare Store Svelte

## Obiettivo

Definisce come aggiungere un nuovo store (stato condiviso) nel frontend PokeTracker usando Svelte 5 runes o pattern compatibili.

## Quando Usare Questa Procedura

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

1. [ ] **Store in `.ts`:** usare **`writable`/`readable`** da `svelte/store`. **Non** usare `$state` in file `.ts`: le runes sono trasformate solo in `.svelte` e `.svelte.ts`; in `.ts` a runtime `$state` non è definito → `ReferenceError` e schermata bianca. Vedi [versioni-stack-standard](../../standards/versioni-stack-standard.md) § Store in file .ts.
2. [ ] Verifica `docs/project/project-structure.md` — Crea il file in `src/lib/stores/<nome>.ts` (o `.js` se senza tipi); naming kebab-case o camelCase coerente con gli altri store
3. [ ] Esporre stato tramite `writable`/`readable` e funzioni che aggiornano lo store (es. `load()`, `set()`); nei componenti usare la sottoscrizione `$storeName`.
4. [ ] Se lo store chiama `invoke`: import da `@tauri-apps/api/core`, uso `await` e `try/catch` (`typescript-frontend-standard.md:48-62`)
5. [ ] Import da `$lib/stores/...` nei componenti che usano lo store (`typescript-frontend-standard.md:21-34`)

## Riferimenti Standard

- `docs/standards/svelte-sveltekit-standard.md:1-80` — Runes, $state, $derived
- `docs/project/project-structure.md` — Posizione `lib/stores/`
- `docs/standards/typescript-frontend-standard.md:36-62` — Invoke e gestione errori

## Note

- In PokeTracker gli store sono in `lib/stores/*.ts` (file `.ts` non processati da Svelte per le runes). In questi file usare **sempre** `writable`/`readable`; **non** usare `$state` → a runtime non è definito, con rischio di schermata bianca. Nei componenti `.svelte` la sottoscrizione `$storeName` funziona normalmente. Vedi [versioni-stack-standard](../../standards/versioni-stack-standard.md) § "Store in file .ts".
- Se in futuro lo store fosse in un file `.svelte.ts` con trasformazione runes abilitata, si potrebbero usare runes; con la struttura attuale, `writable`/`readable` in `.ts` è la scelta obbligatoria e verificata.
