# Procedure: Ottimizzazione Performance

## Obiettivo

Definisce come applicare ottimizzazioni performance in linea con [performance-standard](../../standards/performance-standard.md) e [performance](../../project/performance.md).

## Quando Usare Questa Procedura

- Query: "ottimizza", "performance", "lazy load", "virtualizza lista", "indice DB", "ottimizza performance", "lista lenta", "rendering lento"
- Quando si deve ridurre tempo di risposta, uso memoria o carico iniziale
- Se la query corrisponde, questa procedure è OBBLIGATORIA

## Obbligatorietà

Completamento integrale della checklist obbligatorio prima di proporre implementazione. Nessuna eccezione.

## File da Leggere PRIMA

1. **Performance Standard**: `docs/standards/performance-standard.md:1-65`
   - Lazy load, virtualizzazione, DB, IPC: righe 7-38
   - Misurazione: righe 40-48

2. **Strategia performance**: `docs/project/performance.md:1-70`
   - Target e ottimizzazioni per dominio

## Checklist Obbligatoria

1. [ ] Leggi `docs/standards/performance-standard.md:7-18` — Lazy load: route e componenti pesanti con dynamic import; dati paginati o incremental
2. [ ] Liste lunghe: se centinaia/migliaia di righe, usare virtualizzazione (TanStack Virtual o svelte-virtual-list) (`performance-standard.md:20-25`)
3. [ ] DB: verificare indici su colonne in WHERE/JOIN/ORDER BY delle query lente; prepared statements, evitare N+1 (`performance-standard.md:27-32`)
4. [ ] IPC: evitare loop di `invoke`; raggruppare in un command dove possibile; preferire async (`performance-standard.md:34-38`)
5. [ ] Frontend: `$derived` per calcoli costosi; debounce/throttle su input che triggerano fetch; lazy load immagini (`performance-standard.md:40-44`)
6. [ ] Misurare in build di produzione (o production preview); usare Lighthouse/Performance tab (`performance-standard.md:46-48`)

## Riferimenti Standard

- `docs/standards/performance-standard.md:1-65` — Lazy load, virtualizzazione, DB, IPC
- `docs/project/performance.md:1-70` — Strategia e target
- `docs/project/priorities.md` — In trade-off (es. leggibilità vs velocità): correttezza > performance > funzionalità

## Note

- Per “fix bug” di lentezza usare prima [bug-fix](../../workflow/bug-fix.md) per capire la causa; questa procedure si applica quando l’obiettivo è ottimizzare in modo sistematico.
