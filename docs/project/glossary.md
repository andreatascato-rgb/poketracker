# Glossario PokeTracker

## Obiettivo

Definisce i termini di dominio e tecnici usati in PokeTracker. Usare sempre questi termini in documentazione e codice per consistenza. Riferimento per AI e sviluppatori.

## App e UI

| Termine | Significato |
|---------|-------------|
| **Top Bar** | Barra superiore fissa con controlli globali: aggiornamento app, avvisi, selettore profilo. |
| **Sidebar** | Navigazione principale collassabile; voci: Profilo, Editor, Wiki, Impostazioni. Max due livelli (voce → sottovoci). |
| **Area contenuto** | Zona centrale scrollabile dove compare il contenuto della sezione attiva. Unico contesto con scroll. |
| **Layout** | Struttura a tre parti: Top Bar + Sidebar + Area contenuto. Fonte: [ui-ux-design](./ui-ux-design.md). |
| **Primitivo UI** | Componente riutilizzabile di base (Button, Input, Card, Badge, …). Path: `lib/components/ui/`. Vedi [ui-primitives-standard](../standards/ui-primitives-standard.md), [ui-component-catalog](./ui-component-catalog.md). |
| **Token** | Design token: variabili CSS (es. `--color-primary`, `--spacing-md`) usate per stile coerente. Fonte: [ui-implementation-standard](../standards/ui-implementation-standard.md). |
| **Route** | Pagina SvelteKit in `src/routes/` (es. `+page.svelte`). Ogni voce/sottovoce della sidebar corrisponde a una route. |

## Profilo e dati allenatore

| Termine | Significato |
|---------|-------------|
| **Profilo** | Contesto utente/allenatore: dati isolati (Pokedex, percorso salvataggi, statistiche). Più profili = più “allenatori” sullo stesso PC. |
| **Allenatore** | Sinonimo di profilo in contesto gioco: l’entità i cui dati di gioco (save, Pokedex) sono gestiti dal profilo. |
| **Selettore profilo** | Controllo in Top Bar per cambiare il profilo attivo; i dati in sidebar e area contenuto si aggiornano. |
| **Cartella salvataggi** | Percorso sul filesystem dove l’app cerca i file `.sav` per il profilo attivo. Assegnato per profilo. |

## Salvataggio e editor

| Termine | Significato |
|---------|-------------|
| **Save / Salvataggio** | File `.sav` del gioco Pokemon; contiene dati di progresso dell’allenatore. |
| **Editor** | Editor salvataggi in stile PKHeX: lettura e modifica di file `.sav` tramite librerie PKHeX. Voce sidebar. |
| **PKHeX** | Libreria/strumento di riferimento per formato e logica dei save Pokemon. L’editor si ispira a PKHeX. |

## Pokedex

| Termine | Significato |
|---------|-------------|
| **Pokedex** | Registro Pokemon dell’allenatore; stati estratti dai save. In-app: Pokedex personale come da [pokedex-personal](./pokedex-personal.md). |
| **Pokedex personale** | Feature che mostra lo stato di completamento del Pokedex per il profilo attivo (non visto / visto / catturato). |
| **Non visto** | Stato Pokemon: non ancora incontrato dall’allenatore. |
| **Visto** | Stato Pokemon: incontrato ma non catturato. |
| **Catturato** | Stato Pokemon: catturato dall’allenatore. |

## Wiki

| Termine | Significato |
|---------|-------------|
| **Wiki** | Contenuti consultabili offline (Pokemon, nature, mosse, walkthrough, strategie). Voce sidebar; sottosezioni in [wiki-offline](./wiki-offline.md). |
| **Wiki offline** | Feature: consultazione completa senza rete; categorie e strategie come da [wiki-offline](./wiki-offline.md). |

## Impostazioni e gestione app

| Termine | Significato |
|---------|-------------|
| **Impostazioni** | Voce sidebar: configurazione app e gestione interna (backup, risorse, log, updater). Vedi [self-management](./self-management.md). |
| **Self-management** | Gestione interna app: backup, risorse, log, aggiornamenti. Documentata in [self-management](./self-management.md). |
| **Avvisi** | Pannello da Top Bar: errori/azioni da correggere e ultimi aggiornamenti. |
| **Multi-profilo** | Gestione di più profili/allenatori separati sull’app. Vedi [multi-profile](./multi-profile.md). |

## Tecnico (architettura, codice)

| Termine | Significato |
|---------|-------------|
| **Parser** | Componente che legge file `.sav` e estrae dati (gioco, versione, Pokemon, ecc.). In PokeTracker: sidecar C#. |
| **Sidecar** | Eseguibile esterno (es. parser C#) invocato dal backend Tauri. Vedi [tauri2-sidecar-standard](../standards/tauri2-sidecar-standard.md), [parser-architecture](./parser-architecture.md). |
| **Knowledge DB / Database conoscenza** | Base di dati (es. SQLite) con informazioni statiche su Pokemon, mosse, nature, ecc. Usata da wiki e UI. Vedi [knowledge-database](./knowledge-database.md). |
| **Invoke** | Chiamata da frontend a comando Tauri (`invoke('command_name', { ... })`). |
| **Comando (Tauri)** | Funzione Rust esposta al frontend tramite Tauri; entry point per logica backend. |
| **Catalog (UI)** | Elenco primitivi e compositi UI riutilizzabili. Fonte: [ui-component-catalog](./ui-component-catalog.md). |
| **Reuse-first** | Regola: prima controllare il catalog; se esiste un primitivo/composito adatto, usarlo; creare nuovo solo se manca. |

## Note

- Per layout e navigazione: [ui-ux-design](./ui-ux-design.md).
- Per struttura progetto: [project-structure](./project-structure.md).
- Per primitivi e catalog: [ui-primitives-standard](../standards/ui-primitives-standard.md), [ui-component-catalog](./ui-component-catalog.md).
