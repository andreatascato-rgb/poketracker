# Standard: Performance

## Obiettivo

Definisce quando e come ottimizzare le performance in PokeTracker, in linea con [performance](../project/performance.md): lazy load, virtualizzazione, DB, IPC.

## Quando fare lazy load

- **Route:** SvelteKit fa code-splitting per route; evitare import pesanti in `+layout` condivisi; dynamic import per moduli grandi.
- **Componenti pesanti:** editor, viewer complessi, grafici: caricare solo quando la sezione è attiva (dynamic import o condizionale).
- **Dati:** non caricare in un’unica volta liste molto grandi; paginazione o infinite scroll con fetch incrementale.

## Liste lunghe (virtualizzazione)

- **Quando:** liste con centinaia/migliaia di righe (es. Pokedex, log, tabelle).
- **Come:** usare **TanStack Virtual** (o `svelte-virtual-list`) per renderizzare solo gli item visibili; altezza fissa o stimate per scroll fluido.
- Evitare di rendere migliaia di nodi DOM; virtualizzare non appena lo scroll diventa lento.

## Database

- **Indici:** su colonne in WHERE, JOIN, ORDER BY delle query frequenti; indici compositi per query composte.
- **Query:** prepared statements; evitare N+1; batch dove possibile; connection pooling se applicabile.
- **Analisi:** identificare query lente e ottimizzare prima di aggiungere indici a caso.

## IPC (Tauri invoke)

- **Batch:** raggruppare operazioni correlate in un solo command quando ha senso; evitare centinaia di `invoke` in loop.
- **Async:** preferire command async per operazioni I/O; evitare sync bloccanti sul main thread.
- **Payload:** inviare solo i dati necessari; evitare serializzare strutture enormi se basta un subset.

## Frontend (Svelte)

- **Memoizzazione:** `$derived` per valori costosi; evitare ricalcoli inutili in `$effect`.
- **Input:** debounce/throttle su campi che scatenano fetch o calcoli pesanti.
- **Assets:** lazy load immagini; preferire formati moderni (WebP) dove supportato; dimensioni adeguate.

## Misurazione

- **Dev vs prod:** misurare in build di produzione (o production preview); il dev mode è più lento.
- **Strumenti:** Lighthouse, Performance/Network tab; per Rust `criterion` o profiler per hotspot.
- **Target:** LCP, tempo al primo frame, tempo a “list ready” per liste lunghe; documentare in [performance](../project/performance.md) se esistono target numerici.

## Riferimenti

- [performance](../project/performance.md) — Strategia e target
- [SvelteKit – Performance](https://kit.svelte.dev/docs/performance)
- [TanStack Virtual – Svelte](https://tanstack.com/virtual/latest/docs/framework/svelte)

## Data Decisione

2026-01-27

## Note

- Per “ottimizza performance”, “lazy load”, “virtualizza lista”, “indice DB” usare questo standard e la procedura performance-optimization.
