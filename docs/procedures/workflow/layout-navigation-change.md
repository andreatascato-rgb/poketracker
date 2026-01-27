# Procedure: Modifica Navigazione / Sidebar / Layout

## Obiettivo

Definisce come aggiungere, rimuovere o modificare voci e sottovoci della sidebar e come mantenere coerenza con layout Top Bar + Sidebar + Area contenuto. Rispetta [ui-ux-design](../../project/ui-ux-design.md) (ordine voci, max due livelli).

## Quando Usare Questa Procedure

- Query: "aggiungi voce sidebar", "aggiungi voce menu", "modifica navigazione", "aggiungi sezione Profilo", "cambia ordine voci", "modifica layout", "aggiungi sottovoce Wiki", "rimuovi voce menu", "aggiungi voce Impostazioni"
- Quando si deve cambiare la struttura di navigazione (sidebar, voci, sottovoci) o il layout a tre parti
- Se la query corrisponde, questa procedure è OBBLIGATORIA

## Obbligatorietà

Completamento integrale della checklist obbligatorio prima di proporre implementazione. Nessuna eccezione.

## File da Leggere PRIMA

1. **UI/UX Design – Sidebar e layout**: `docs/project/ui-ux-design.md:73-111`
   - Layout tre parti (Top Bar, Sidebar, Area contenuto): righe 73-78
   - Sidebar: voci, sottovoci, ordine (Profilo → Editor → Wiki → Impostazioni): righe 95-106
   - Area contenuto: righe 108-111

2. **Struttura progetto – Layout**: `docs/project/project-structure.md:117-122`
   - Componenti layout: `lib/components/layout/` (TopBar.svelte, Sidebar.svelte, ContentArea.svelte)
   - Route e contenuto per ogni voce

3. **Glossario**: `docs/project/glossary.md` — Termini Sidebar, Top Bar, Area contenuto, Route, Profilo, Editor, Wiki, Impostazioni

4. **Se si aggiunge una nuova pagina**: procedure [page-creation](../svelte/page-creation.md) per route e `+page.svelte`

## Checklist Obbligatoria

1. [ ] Leggi `docs/project/ui-ux-design.md:95-106` — Sidebar: voci e sottovoci attuali; **ordine fissato** Profilo → Editor → Wiki → Impostazioni; **max due livelli** (voce → sottovoci)
2. [ ] Se **nuova voce** sidebar: valutare se rispetta l’ordine e le note in ui-ux-design; aggiungere la voce nella tabella Sidebar di ui-ux-design e aggiornare il codice di `Sidebar.svelte` (o componente equivalente)
3. [ ] Se **nuova sottovoce**: rispettare max due livelli; allineare a [features.md](../../project/features.md) e ai doc di feature (es. [wiki-offline.md](../../project/wiki-offline.md) per sottosezioni Wiki)
4. [ ] Componenti layout in `src/lib/components/layout/`: TopBar, Sidebar, ContentArea; modifiche alla nav vanno in Sidebar (e eventuali route)
5. [ ] Se la nuova voce/sottovoce richiede una **nuova pagina**: seguire [page-creation](../svelte/page-creation.md) per `src/routes/.../+page.svelte` e collegare la route alla sidebar
6. [ ] Layout: Top Bar e Sidebar **fuori** dal contenitore scrollabile; scroll solo in Area contenuto (`ui-ux-design.md:114-122`)

## Riferimenti Standard

- `docs/project/ui-ux-design.md:73-122` — Layout, Sidebar, ordine voci, scroll
- `docs/project/project-structure.md:117-135` — Componenti layout, routes
- `docs/project/glossary.md` — Termini navigazione e layout
- `docs/standards/ui-implementation-standard.md:21-23` — Layout e scroll

## Note

- **Ordine voci:** Profilo → Editor → Wiki → Impostazioni (prima “il mio allenatore”, poi strumenti, riferimento, sistema). Non invertire senza aggiornare ui-ux-design.
- Per “crea pagina” senza toccare la sidebar usare [page-creation](../svelte/page-creation.md).
- Per “crea componente” (es. nuovo blocco nella Top Bar) usare [component-creation](../svelte/component-creation.md) e [ui-primitive-creation](../svelte/ui-primitive-creation.md) se è un elemento UI classico.
