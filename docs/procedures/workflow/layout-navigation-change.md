# Procedure: Modifica Navigazione / Sidebar / Layout

## Obiettivo

Definisce come aggiungere, rimuovere o modificare voci e sottovoci della sidebar e come mantenere coerenza con layout Top Bar + Sidebar + Area contenuto. **Layout e dimensioni seguono lo standard Poketrack:** Top Bar 48px, Sidebar 280px (48px collassata), area contenuto flex 1; scroll solo nell'area contenuto. Ordine voci: Profilo → Editor → Wiki → Impostazioni; max due livelli.

Riferimento visivo e token: [poketrack-reference.md](../../project/poketrack-reference.md), [design-system-standard.md](../../standards/design-system-standard.md).

## Quando Usare Questa Procedure

- Query: "aggiungi voce sidebar", "aggiungi voce menu", "modifica navigazione", "aggiungi sezione Profilo", "cambia ordine voci", "modifica layout", "aggiungi sottovoce Wiki", "rimuovi voce menu", "aggiungi voce Impostazioni"
- Quando si deve cambiare la struttura di navigazione (sidebar, voci, sottovoci) o il layout a tre parti
- Se la query corrisponde, questa procedure è OBBLIGATORIA

## Obbligatorietà

Completamento integrale della checklist obbligatorio prima di proporre implementazione. Nessuna eccezione.

## File da Leggere PRIMA

1. **Struttura progetto – Layout e routes**: `docs/project/project-structure.md:119-164`
   - Componenti layout: `lib/components/layout/` (TopBar.svelte, Sidebar.svelte, ContentArea.svelte) — righe 119-122
   - Route SvelteKit: `src/routes/` — righe 153-164

2. **Glossario**: `docs/project/glossary.md` — Termini Sidebar, Top Bar, Area contenuto, Route, Profilo, Editor, Wiki, Impostazioni

3. **Se si aggiunge una nuova pagina**: procedure [page-creation](../svelte/page-creation.md) per route e `+page.svelte`

## Checklist Obbligatoria

1. [ ] **Layout shell standard Poketrack:** Top Bar **48px**, Sidebar **280px** (48px se collassata), area contenuto flex 1; scroll solo in area contenuto. Vedi [design-system-standard.md](../../standards/design-system-standard.md) e [poketrack-reference.md](../../project/poketrack-reference.md).
2. [ ] Sidebar: **ordine fissato** Profilo → Editor → Wiki → Impostazioni; **max due livelli** (voce → sottovoci)
3. [ ] Se **nuova voce** sidebar: rispettare l’ordine; aggiornare il codice di Sidebar (o componente equivalente)
4. [ ] Se **nuova sottovoce**: rispettare max due livelli; allineare a [features.md](../../project/features.md) e ai doc di feature (es. [wiki-offline.md](../../project/wiki-offline.md) per sottosezioni Wiki)
5. [ ] Componenti layout in `src/lib/components/layout/` o layout in `src/routes/+layout.svelte`; modifiche alla nav vanno in Sidebar (e eventuali route)
6. [ ] Se la nuova voce/sottovoce richiede una **nuova pagina**: seguire [page-creation](../svelte/page-creation.md) per `src/routes/.../+page.svelte` e collegare la route alla sidebar
7. [ ] Rispettare [design-system-standard.md](../../standards/design-system-standard.md) per token, tipografia, icone, dimensioni (standard Poketrack)
8. [ ] **Stack UI obbligatorio:** usare Tailwind e, dove possibile, componenti da [ui-stack-standard.md](../../standards/ui-stack-standard.md). Nuovi elementi di layout/sidebar non devono introdurre CSS custom o componenti UI da zero.

## Verifica obbligatoria dopo modifiche al layout

Per evitare che l’app si veda “vuota” (senza sidebar né pulsante profilo), **dopo ogni modifica** a top bar, sidebar o `+layout.svelte`:

1. [ ] **Dev browser:** `npm run dev` → aprire `http://localhost:1420` → verificare che si vedano **Top Bar** (con pulsante profilo a destra), **Sidebar** (con voci e icone), **Area contenuto**.
2. [ ] **Dev Tauri:** `tauri dev` → verificare che nella finestra dell’app si vedano gli stessi elementi (Top Bar, Sidebar, Area contenuto).
3. [ ] **Console:** controllare la console (browser o DevTools Tauri) per errori JavaScript; nessun errore a runtime che blocchi il layout.

Se uno di questi fallisce, non considerare l’implementazione completata. Per analisi casi “layout non visibile”: [analisi-layout-non-visibile.md](../../temp/analisi-layout-non-visibile.md).

**Nota:** l’uso di accessi “sicuri” a `$page` (es. `$page?.url?.pathname ?? ""`) **non** risolve il problema “layout non visibile”; la causa va cercata altrove. Non documentare quella modifica come fix.

## Riferimenti Standard

- [poketrack-reference.md](../../project/poketrack-reference.md) — Cosa adottiamo da Poketrack (layout, token)
- [design-system-standard.md](../../standards/design-system-standard.md) — Token, dimensioni layout, stile sidebar/header
- [ui-stack-standard.md](../../standards/ui-stack-standard.md) — Stack UI (Tailwind + shadcn-svelte)
- [project-structure.md](../../project/project-structure.md) — Componenti layout, routes SvelteKit
- [glossary.md](../../project/glossary.md) — Termini navigazione e layout

## Note

- **Ordine voci:** Profilo → Editor → Wiki → Impostazioni (prima “il mio allenatore”, poi strumenti, riferimento, sistema).
- Per “crea pagina” senza toccare la sidebar usare [page-creation](../svelte/page-creation.md).
- Per “crea componente” (es. nuovo blocco nella Top Bar) usare [component-creation](../svelte/component-creation.md).

