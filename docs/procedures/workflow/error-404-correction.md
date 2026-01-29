# Procedure: Correzione errore 404 (route o risorsa non trovata)

## Obiettivo

Eliminare un 404 ricorrente **correggendo la causa**: individuare chi fa la richiesta al path che non esiste più, rimuovere o aggiornare quel riferimento. Nessun workaround (redirect/route fantasma) se non richiesto esplicitamente come decisione di prodotto.

## Quando usare questa procedura

- Query: "404", "errore 404", "get X non esiste", "route non trovata", "correggi 404", "fix 404", "continua ad apparire 404"
- Quando in console o terminale compare ripetutamente un 404 su un path (es. `GET /profilo/dashboard`) che non esiste più o non è usato

## Obbligatorietà

Completamento della checklist obbligatorio prima di proporre implementazione. Riferimento: `docs/standards/fix-without-workaround-standard.md`.

## File da leggere prima

1. **Standard fix senza workaround**: `docs/standards/fix-without-workaround-standard.md`
2. **Bug fix (causa radice)**: `docs/procedures/workflow/bug-fix.md` (sezioni sintomo, causa radice, modifica minima)

## Checklist obbligatoria

1. [ ] **Identificare il path che restituisce 404** (es. `/profilo/dashboard`) e confermare che la route/risorsa sia stata rimossa o non esista più.
2. [ ] **Cercare nel codice chi può fare quella richiesta**:
   - Link: `href=` che punta al path (cercare il path letterale e varianti, es. `profilo/dashboard`, `/profilo/dashboard`).
   - Navigazione: `goto(...)` o `redirect(...)` verso quel path.
   - Fetch/load: `fetch(...)` o caricamenti che usano quel path.
   - Costruzione dinamica: concatenazione di path (es. `base + '/dashboard'`) che può produrre quel path.
   - Cercare in: `src/` (route, layout, componenti, store). Eventualmente build/manifest solo se si sospetta cache.
3. [ ] **Se si trova un riferimento in codice**: rimuoverlo o aggiornarlo (es. cambiare `href` da `/profilo/dashboard` a `/profilo`). **Questo è il fix.** Non aggiungere redirect dalla vecchia route.
4. [ ] **Se non si trova alcun riferimento in codice**:
   - **Cache di build**: eliminare `.svelte-kit/` e `build/` (se presenti), poi riavviare il dev server. La 404 può essere causata da manifest/chunk obsoleti che riferivano la route rimossa.
   - **Browser**: la richiesta può venire da tab aperta sul vecchio URL, cronologia, prefetch su pagina cached. Documentare che l’utente deve evitare di aprire quel URL (aggiornare preferiti, chiudere tab, eventuale hard refresh / svuotare cache).
5. [ ] **Verificare**: dopo le modifiche, non deve più comparire il 404 per quel path (salvo che l’utente apra esplicitamente quel URL da bookmark/cronologia; in quel caso è comportamento atteso se non si è deciso di supportare backward compatibility).
6. [ ] **Non introdurre workaround**: non creare route “vuote” che fanno solo redirect dalla vecchia URL, a meno che non sia una **decisione esplicita di prodotto** (es. “supportiamo i vecchi link”). In quel caso documentarla.

## Riferimenti standard

- Fix senza workaround: `docs/standards/fix-without-workaround-standard.md`
- Bug fix (causa radice): `docs/procedures/workflow/bug-fix.md`

## Esempio (404 su route rimossa)

- **Sintomo**: in terminale compare `GET /profilo/dashboard` 404; la route `profilo/dashboard` è stata eliminata.
- **Checklist**: cercare `profilo/dashboard`, `href=.*dashboard`, `goto(.*dashboard)`, `redirect(.*dashboard)` in `src/`. Se non si trova nulla → pulire `.svelte-kit/` e `build/`, riavviare dev; indicare all’utente di non usare il vecchio URL da tab/preferiti.
- **Non fare**: creare `src/routes/profilo/dashboard/+page.ts` che fa solo `redirect(302, '/profilo')` per “evitare” il 404.
