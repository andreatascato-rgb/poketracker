# Procedure: Modifica Navigazione / Sidebar / Layout

## Obiettivo

Definisce come aggiungere, rimuovere o modificare voci e sottovoci della sidebar e come mantenere coerenza con layout Top Bar + Sidebar + Area contenuto. **Layout e dimensioni:** Top Bar 48px, Sidebar 220px in PokeTracker (standard Poketrack 280px; scelta compatta), 48px collassata, area contenuto flex 1; scroll solo nell'area contenuto. Ordine voci: **Home → Allenatore (route /profilo) → Editor → Wiki → Impostazioni**; max due livelli. **Pianificata:** voce **Archivio** (sottosezione Errori) per registro errori e log da incollare; vedi [notifications-and-error-archive.md](../../project/notifications-and-error-archive.md). **Implementazione:** nav e layout shell sono in `src/routes/+layout.svelte` (markup inline); i componenti in `lib/components/layout/` non sono usati dal layout principale.

Riferimento visivo e token: [poketrack-reference.md](../../project/poketrack-reference.md), [design-system-standard.md](../../standards/design-system-standard.md). Per breakpoint e comportamento su viewport stretta: [responsive-design-standard.md](../../standards/responsive-design-standard.md).

## Quando Usare Questa Procedure

- Query: "aggiungi voce sidebar", "aggiungi voce menu", "modifica navigazione", "aggiungi sezione Profilo", "cambia ordine voci", "modifica layout", "aggiungi sottovoce Wiki", "rimuovi voce menu", "aggiungi voce Impostazioni"
- Quando si deve cambiare la struttura di navigazione (sidebar, voci, sottovoci) o il layout a tre parti
- Se la query corrisponde, questa procedure è OBBLIGATORIA

## Obbligatorietà

Completamento integrale della checklist obbligatorio prima di proporre implementazione. Nessuna eccezione.

## File da Leggere PRIMA

1. **Struttura progetto – Layout e routes**: `docs/project/project-structure.md:119-164`
   - Layout shell e nav: implementati in `src/routes/+layout.svelte` (markup inline). Componenti in `lib/components/layout/` esistono ma non sono usati dal layout principale.
   - Route SvelteKit: `src/routes/` — righe 153-164

2. **Glossario**: `docs/project/glossary.md` — Termini Sidebar, Top Bar, Area contenuto, Route, Allenatore/Profilo, Editor, Wiki, Impostazioni

3. **Se si aggiunge una nuova pagina**: procedure [page-creation](../svelte/page-creation.md) per route e `+page.svelte`

## Checklist Obbligatoria

1. [ ] **Layout shell:** Top Bar **48px**, Sidebar **220px** in PokeTracker (48px se collassata), area contenuto flex 1; scroll solo in area contenuto. Vedi [design-system-standard.md](../../standards/design-system-standard.md) e [poketrack-reference.md](../../project/poketrack-reference.md).
2. [ ] Sidebar: **ordine fissato** Home → Allenatore (/profilo) → Editor → Wiki → Impostazioni; **max due livelli** (voce → sottovoci)
3. [ ] Se **nuova voce** sidebar: rispettare l’ordine; aggiornare la nav in `src/routes/+layout.svelte` (array `sidebarItems`)
4. [ ] Se **nuova sottovoce**: rispettare max due livelli; allineare a [features.md](../../project/features.md) e ai doc di feature (es. [wiki-offline.md](../../project/wiki-offline.md) per sottosezioni Wiki)
5. [ ] Layout e nav: implementazione in `src/routes/+layout.svelte` (markup inline). Modifiche alla nav vanno in `+layout.svelte` (array `sidebarItems` e voci); nuove route in `src/routes/`
6. [ ] Se la nuova voce/sottovoce richiede una **nuova pagina**: seguire [page-creation](../svelte/page-creation.md) per `src/routes/.../+page.svelte` e collegare la route alla sidebar
7. [ ] Rispettare [design-system-standard.md](../../standards/design-system-standard.md) per token, tipografia, icone, dimensioni (standard Poketrack)
8. [ ] **Stack UI obbligatorio:** usare Tailwind e, dove possibile, componenti da [ui-stack-standard.md](../../standards/ui-stack-standard.md). Nuovi elementi di layout/sidebar non devono introdurre CSS custom o componenti UI da zero.
9. [ ] **Responsive:** su viewport stretta Sidebar collassata (48px) o nascosta con toggle; area contenuto sempre usabile. Breakpoint e regole in [responsive-design-standard.md](../../standards/responsive-design-standard.md). Se la modifica introduce layout adattivo nuovo, seguire anche [responsive-design-change.md](./responsive-design-change.md).

## Verifica obbligatoria dopo modifiche al layout

Per evitare che l’app si veda “vuota” (senza sidebar né pulsante profilo), **dopo ogni modifica** a top bar, sidebar o `+layout.svelte`:

1. [ ] **Dev browser:** `npm run dev` → aprire `http://localhost:1420` → verificare che si vedano **Top Bar** (con pulsante profilo a destra), **Sidebar** (con voci e icone), **Area contenuto**.
2. [ ] **Dev Tauri:** `tauri dev` → verificare che nella finestra dell’app si vedano gli stessi elementi (Top Bar, Sidebar, Area contenuto).
3. [ ] **Console:** controllare la console (browser o DevTools Tauri) per errori JavaScript; nessun errore a runtime che blocchi il layout.
4. [ ] **Viewport stretta (opzionale se si è toccato shell/nav):** ridimensionare sotto 768px e verificare sidebar collassata/nascosta e area contenuto utilizzabile; vedi [responsive-design-standard.md](../../standards/responsive-design-standard.md).

Se uno di questi fallisce, non considerare l’implementazione completata. Per analisi casi “layout non visibile”: [analisi-layout-non-visibile.md](../../temp/analisi-layout-non-visibile.md).

**Nota:** l’uso di accessi “sicuri” a `$page` (es. `$page?.url?.pathname ?? ""`) **non** risolve il problema “layout non visibile”; la causa va cercata altrove. Non documentare quella modifica come fix.

## Riferimenti Standard

- [poketrack-reference.md](../../project/poketrack-reference.md) — Cosa adottiamo da Poketrack (layout, token)
- [design-system-standard.md](../../standards/design-system-standard.md) — Token, dimensioni layout, stile sidebar/header
- [responsive-design-standard.md](../../standards/responsive-design-standard.md) — Breakpoint, viewport stretta, shell adattivo
- [ui-stack-standard.md](../../standards/ui-stack-standard.md) — Stack UI (Tailwind + shadcn-svelte)
- [project-structure.md](../../project/project-structure.md) — Componenti layout, routes SvelteKit
- [glossary.md](../../project/glossary.md) — Termini navigazione e layout

## Regole per voci con sottovoci (obbligatorie)

Quando una voce della sidebar ha sottovoci (es. Allenatore → Profilo, Wiki → Categorie, Impostazioni → Profili):

1. **Voce padre = pulsante che espande, non link:** il padre (es. "Impostazioni") è un **`<button>`** che **solo** apre/chiude le sottovoci. **Non** è un link: cliccarlo **non** deve navigare né portare su una sottosezione. Solo le sottovoci (Profili, ecc.) sono `<a>` e navigano.
2. **Voce padre mai evidenziata:** il padre non deve mai mostrare lo stato attivo; solo le sottovoci possono essere evidenziate (barra a sinistra + sfondo lieve) quando sei sulla loro pagina. Stati visivi: hover = `--bg-tertiary`, voce corrente = barra 3px `--focus-ring` + `--hover-bg`; vedi [design-system-standard.md](../../standards/design-system-standard.md) (Pattern — Sidebar, Stati voci).
3. **Sottovoci in blocco indentato:** le sottovoci stanno in un contenitore `.sidebar-children` con **margin-inline-start** (es. 28px), **bordo verticale a sinistra** (`border-inline-start`) e **padding**; ogni sottovoce ha **icona + label** (classe `sidebar-subitem`).
4. **Visibilità sottovoci:** visibili solo quando il padre è espanso (stato locale) **oppure** quando si è già in quella sezione. **"In sezione"** = il path corrente è l'href del padre, un href di una sottovoce, o un sotto-path di uno di essi (es. Allenatore è "in sezione" su `/profilo/dashboard` e `/profilo/salvataggi`). Es.: `isExpanded = expandedSection === item.href || (expandedSection === null && inThisSection)`.
5. **Standard espansione (obbligatorio):** lo stato espanso/chiuso delle sezioni è **solo utente**. La navigazione (click su sottovoce, back/forward, link esterni) **non** deve mai chiudere o riaprire le sezioni. Alla prima visita si apre la sezione che contiene la route corrente; da quel momento solo il toggle sul padre cambia lo stato.

**Riferimento:** `src/routes/+layout.svelte` — padre = `<button class="sidebar-item sidebar-parent">` con `onclick` che toggle lo stato; sottovoci in `<div class="sidebar-children">`; solo le sottovoci sono `<a>`. Helper `isInSection(path, item)` e inizializzazione una tantum in `$effect` (nessun reset su cambio path).

## Note

- **Ordine voci:** Home → Allenatore → Editor → Wiki → Impostazioni (entry, poi “il mio allenatore”, strumenti, riferimento, sistema).
- Per “crea pagina” senza toccare la sidebar usare [page-creation](../svelte/page-creation.md).
- Per “crea componente” (es. nuovo blocco nella Top Bar) usare [component-creation](../svelte/component-creation.md).

