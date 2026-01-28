# Analisi: icona sync TopBar mai verde

## Obiettivo

Individuare la **causa radice** per cui l’icona sync in TopBar non diventa mai verde (CheckCircle verde con watcher attivi), nonostante la logica prevista e i 23 tentativi di fix. **Nessuna modifica al codice** fino a conferma della/e causa/e.

---

## Comportamento atteso

| Condizione | Icona |
|------------|--------|
| Nessun watcher attivo | CheckCircle bianca |
| ≥ 1 watcher attivo, nessun sync in corso | **CheckCircle verde** |
| ≥ 1 sync in corso | RefreshCw verde spinning |

Problema osservato: l’icona **non diventa mai verde** quando ci sono watcher attivi.

---

## Flusso dati (riepilogo)

1. **Store** `sync.ts`: `watchedCount` (writable), `syncingPaths` (Set), `setWatchedCount`, `addSyncing` / `removeSyncing`.
2. **Layout** `+layout.svelte`: legge `watchedCount` e `syncingPaths` via `$effect` + `subscribe`, copia in `watchedNum` e `syncingSize`; in template usa `watchedNum > 0` per CheckCircle verde e `syncingSize > 0` per RefreshCw spinning.
3. **Chi aggiorna `watchedCount`**:
   - **Layout `onMount`**: `invoke("get_sav_watched_paths")` → `setWatchedCount(paths?.length ?? 0)` (una tantum, fire‑and‑forget).
   - **Salvataggi `loadSaves()`**: `get_sav_entries` + `get_sav_watched_paths` → `setWatchedCount(watchedPaths.length)` (su mount, dopo toggle watcher, dopo `sav-file-changed`, dopo add/remove save).
4. **Backend**: `get_sav_watched_paths` legge da DB; tutto l’accesso DB passa da **un’unica connessione SQLite** in `Mutex` (`DbState`). Ogni command fa `lock` → query → `unlock`. Le invoke sono quindi **serializzate** sul DB.

---

## Causa 1 (principale): race tra layout e Salvataggi su `watchedCount`

### Meccanismo

- **Layout** al mount fa `get_sav_watched_paths` e in `.then(...)` chiama `setWatchedCount(paths?.length ?? 0)`. La invoke è asincrona e **non** viene mai annullata.
- **Salvataggi** aggiorna `watchedCount` solo tramite `loadSaves()` (mount, toggle, `sav-file-changed`, add/remove). Dopo un toggle “attiva watcher”, si fa `sync_sav_now` → `set_sav_watched` → `loadSaves` → `setWatchedCount(watchedPaths.length)` con il valore **aggiornato** (es. 1).

Quando l’utente aggiunge un watcher:

1. `loadSaves` post‑toggle imposta correttamente `watchedCount(1)` → icona verde.
2. La **invoke** `get_sav_watched_paths` lanciata dal **layout** in `onMount` può essere ancora in coda o in esecuzione (DB serializzato). Quando **finisce dopo** il `loadSaves` del toggle, il layout esegue comunque `setWatchedCount(paths?.length ?? 0)` con il risultato della **sua** richiesta.
3. Quella richiesta era partita **prima** dell’aggiunta del watcher (o comunque in uno stato “vecchio”). Il backend può aver restituito `[]` (0 watcher) o una lista non aggiornata.
4. Si **sovrascrive** così il valore corretto (1) con 0 (o con un valore obsoleto). L’icona torna bianca e non resta mai verde.

### Perché è plausibile che la invoke del layout arrivi “in ritardo”

- **Un solo DB** in `Mutex`: tutte le invoke che toccano il DB (profili, `get_sav_entries`, `get_sav_watched_paths`, `set_sav_watched`, `sync_sav_now`, …) sono serializzate.
- Al mount del layout partono subito `loadProfiles` (più invoke) e `get_sav_watched_paths`. Poi l’utente naviga su Salvataggi → `loadSaves` (almeno 2 invoke), toggle watcher → `sync_sav_now`, `set_sav_watched`, di nuovo `loadSaves`. La `get_sav_watched_paths` del layout può **restare in coda** dietro a queste e risolversi **dopo** che Salvataggi ha già impostato `watchedCount` a 1.
- In quel caso il layout **sovrascrive** con il risultato “vecchio” della propria invoke. Succede in modo deterministico ogni volta che la coda DB fa finire la invoke del layout dopo il `loadSaves` del toggle.

Quindi: **la fetch del layout in `onMount` può sovrascrivere `watchedCount` con un valore obsoleto (tipicamente 0) dopo che l’utente ha attivato watcher e Salvataggi ha aggiornato correttamente lo store.** Questo spiega “icona mai verde”.

---

## Causa 2 (secondaria): `loadSaves` in errore azzera `watchedCount`

In `loadSaves`:

```ts
try {
  const [list, watched] = await Promise.all([
    invoke("get_sav_entries"),
    invoke("get_sav_watched_paths"),
  ]);
  // ...
  setWatchedCount(watchedPaths.length);
} catch {
  saves = [];
  watchedPaths = [];
  setWatchedCount(0);  // ← sovrascrive anche con 0 in errore
}
```

Se **una sola** delle due invoke fallisce (o `Promise.all` va in catch), si esegue comunque `setWatchedCount(0)`. Si può quindi azzerare il conteggio watcher **anche quando i watcher sono effettivamente attivi** (es. `get_sav_entries` fallisce, `get_sav_watched_paths` sarebbe ok). In quei casi l’icona torna bianca. È un effetto secondario ma coerente con “mai verde” se gli errori sono ricorrenti.

---

## Cosa è stato verificato (e non è la causa)

- **CSS / `--icon-success`**: definito in `.dark`; `app.html` ha `class="dark"`. Il token è usato per `.status-active` e `.status-syncing`. Nessun dubbio sul colore verde.
- **Template layout**: usa `watchedNum` e `syncingSize` con `{#if syncingSize > 0}` … `{:else if watchedNum > 0}` … `{:else}`. La logica è corretta.
- **Store e `setWatchedCount`**: un solo store, un solo punto di scrittura logico (`setWatchedCount`). Nessuna duplicazione di istanze.
- **Salvataggi dopo toggle**: `set_sav_watched` viene chiamato prima di `loadSaves`; `loadSaves` usa quindi dati aggiornati. Il valore “1” viene effettivamente scritto in store prima che il layout possa sovrascrivere.
- **`$effect` + `subscribe`**: gli effect sottoscrivono a `watchedCount` e `syncingPaths` e aggiornano `watchedNum` / `syncingSize`. Non c’è lettura reattiva di `watchedNum`/`syncingSize` nell’effect, quindi non si innesca un re‑run che faccia da solo perdere gli update. Il problema osservato è meglio spiegato dalla sovrascrittura da layout.
- **Layout vs Salvataggi**: un solo root layout; la TopBar con l’icona sync è lì. Salvataggi è pagina figlia. Nessun dubbio su quale componente mostri l’icona.

---

## Conclusione

- **Causa principale**: la `get_sav_watched_paths` lanciata in **layout `onMount`** può risolversi **dopo** che l’utente ha attivato un watcher e Salvataggi ha impostato `watchedCount` correttamente. Il layout sovrascrive allora con il risultato della propria invoke (spesso 0 watcher), azzerando di fatto lo stato “watcher attivi” e impedendo che l’icona resti verde.
- **Causa secondaria**: in caso di errore in `loadSaves`, `setWatchedCount(0)` azzera sempre il conteggio, anche quando ci sono watcher attivi.

Entrambe le cause sono **sovrascritture inappropriate** di `watchedCount`: o da una fetch “ritardata” del layout, o dal branch `catch` di `loadSaves`.

---

## Implementazione (fix applicato)

- **Store `sync`**: introdotti `watchedCountFromSalvataggi` (flag), `setWatchedCountFromSalvataggi(n)` e `canLayoutSetWatchedCount()`. Salvataggi aggiorna sempre tramite `setWatchedCountFromSalvataggi`; il layout usa `setWatchedCount` solo se `canLayoutSetWatchedCount()` è true.
- **Layout**: `get_sav_watched_paths` in `onMount` resta; in `.then` / `.catch` si chiama `setWatchedCount` **solo** se `canLayoutSetWatchedCount()` è true. Evita la sovrascrittura quando Salvataggi ha già aggiornato.
- **Salvataggi `loadSaves`**: successo → `setWatchedCountFromSalvataggi(watchedPaths.length)`; in `catch` **non** si tocca `watchedCount` (niente più `setWatchedCount(0)`).

Risultato: un solo writer “authoritative” dopo la prima `loadSaves`; valore iniziale da layout finché Salvataggi non ha aggiornato; niente race né azzeramento in errore.
