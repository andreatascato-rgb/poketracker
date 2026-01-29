# Analisi: layout non visibile (sidebar e pulsante profilo mancanti)

## Contesto

L’utente ha segnalato che l’app si vede come finestra quasi vuota: top bar con titolo “PokeTracker”, nessuna sidebar, nessun pulsante profilo, area grande e scura.

## Possibili cause

### 1. Errore a runtime nel layout

Se qualcosa nel layout va in errore prima o durante il render, Svelte può non disegnare nulla e restare su sfondo/blank.

- **`$page` non pronto**: in `isSidebarActive(href)` si usava `$page.url.pathname`. Se al primo render (o in qualche percorso) `$page` non è ancora definito, l’accesso a `$page.url` può lanciare e bloccare tutto.
- **Componenti figli**: un errore in `ProfileSelector` o nelle icone Lucide può far fallire l’intero layout se non è contenuto da un error boundary.

### 2. Build / ambiente sbagliato

- **Build vecchia**: se si avvia l’eseguibile ottenuto con `tauri build` fatto **prima** delle ultime modifiche al layout, si vede ancora la vecchia UI (o una versione minimale).
- **Progetto sbagliato**: avviare l’app da un’altra cartella o un altro repo “PokeTracker” mostra un’altra UI.
- **Tauri vs browser**: in `tauri dev` il frontend è su `http://localhost:1420`; se il dev server non è partito o non è quello di questo progetto, la webview può restare vuota o su “connection refused”.

### 3. Cosa si vede davvero

La “top bar” con icona e “PokeTracker” può essere la **title bar nativa** di Tauri, non il nostro header HTML. In quel caso:
- La barra in alto è quella della finestra OS.
- La “grande area scura” è tutta la **webview**.
- Se sidebar e pulsante profilo non compaiono, il nostro layout o non viene renderizzato (crash) o viene renderizzato con dimensioni/visibilità zero (bug CSS/struttura).

Quindi: “sidebar e profilo mancanti” = o il layout non gira, o è presente ma invisibile.

## Interventi tentati (non risolutivi)

1. **Uso sicuro di `$page`** in `+layout.svelte` (`$page?.url?.pathname ?? ""`): **non ha risolto** il problema. La causa reale del layout non visibile resta da identificare; non va documentata come soluzione.

2. **Checklist di verifica** nella procedura di layout: dopo ogni modifica, controllare che top bar, sidebar e area contenuto si vedano in dev e in Tauri. Utile per non dare per “fatto” un layout che in realtà non si vede.

## Come non farlo risuccedere

- **Dopo ogni modifica al layout** (top bar, sidebar, `+layout.svelte`, `+layout.ts`):
  1. Eseguire `npm run dev`, aprire `http://localhost:1420` nel browser e verificare che si vedano top bar, sidebar e area contenuto.
  2. Eseguire `tauri dev` e verificare che nella finestra dell’app si vedano top bar, sidebar e area contenuto.
  3. Controllare la console (browser o DevTools Tauri) per errori JavaScript.
- **Prima di un build da dare all’utente**: rifare il punto 2 e, se possibile, avviare anche l’eseguibile prodotto da `tauri build` e controllare ancora la visibilità del layout.
- **Non** indicare come “fix per layout non visibile” l’accesso sicuro a `$page` o altre modifiche simili, finché non si è verificato che risolvono davvero il problema.

## Riferimenti

- Procedure layout: `docs/procedures/workflow/layout-navigation-change.md`
- Analisi presente: `docs/temp/analisi-layout-non-visibile.md` (temporanea)

## Data

2026-01-27
