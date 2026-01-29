# Glossario PokeTracker

## Obiettivo

Definisce i termini di dominio e tecnici usati in PokeTracker. Usare sempre questi termini in documentazione e codice per consistenza. Riferimento per AI e sviluppatori.

## App e struttura

| Termine | Significato |
|---------|-------------|
| **Top Bar** | Barra superiore fissa con controlli globali: aggiornamento app, icona notifiche (centro notifiche), selettore profilo. Vedi [notifications-and-error-archive](./notifications-and-error-archive.md) per notifiche. |
| **Sidebar** | Navigazione principale collassabile; voci: Allenatore, Statistiche, Pokedex, Editor, Salvataggi, Wiki (in fondo, con sottosezioni); Impostazioni (Profili, Errori) dal dropdown Top Bar. Max due livelli (voce → sottovoci). Etichetta «Allenatore» per la voce profilo. |
| **Area contenuto** | Zona centrale scrollabile dove compare il contenuto della sezione attiva. Unico contesto con scroll. |
| **Layout** | Struttura a tre parti: Top Bar + Sidebar + Area contenuto. |
| **Componente base** | Componente riutilizzabile di base (Button, Input, Card, Badge, …). Path: `lib/components/ui/`. |
| **Route** | Pagina SvelteKit in `src/routes/` (es. `+page.svelte`). Ogni voce/sottovoce della sidebar corrisponde a una route. |

## Profilo e dati allenatore

| Termine | Significato |
|---------|-------------|
| **Profilo** | Contesto utente/allenatore: dati isolati (Pokedex, percorsi salvataggi, statistiche). Più profili = più “allenatori” sullo stesso PC. |
| **Allenatore** | Sinonimo di profilo in contesto gioco: l’entità i cui dati di gioco (save, Pokedex) sono gestiti dal profilo. |
| **Selettore profilo** | Controllo in Top Bar per cambiare il profilo attivo; i dati in sidebar e area contenuto si aggiornano. |
| **Cartella main** | Preferenza globale (impostata una volta): percorso dove l’utente tiene emulatori e save. Usata solo come comodo (es. apertura Sfoglia, suggerimenti). Opzionale; non è monitorata. |
| **Percorsi salvataggi** | Elenco di percorsi sul filesystem scelti dall’utente e associati al profilo. L’app monitora solo questi path per cercare file `.sav` per quel profilo. Ogni profilo può avere più percorsi. |

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
| **Impostazioni** | Sezione accessibile dal dropdown Top Bar: sottosezioni Profili, Errori, Backup e dati. Configurazione app e gestione interna (backup, risorse, log, updater). Vedi [self-management](./self-management.md). |
| **Self-management** | Gestione interna app: backup, risorse, log, aggiornamenti. Documentata in [self-management](./self-management.md). |
| **Avvisi** | Messaggio transiente (toast/snackbar) per segnalare subito un problema o un risultato. |
| **Notifiche** | Centro notifiche in-app: icona in Top Bar; clic apre componente dedicato con elenco notifiche (avvisi, errori, info). Le notifiche di tipo errore generano anche una voce in Impostazioni → Errori. Vedi [notifications-and-error-archive](./notifications-and-error-archive.md). |
| **Registro errori** | Insieme delle voci in Impostazioni → Errori; ogni voce ha data, tipo, dettaglio completo copiabile, azione elimina. Standard: quando una notifica riguarda un errore, si crea anche una voce nel registro. Vedi [notifications-and-error-archive](./notifications-and-error-archive.md). |
| **Multi-profilo** | Gestione di più profili/allenatori separati sull’app. Vedi [multi-profile](./multi-profile.md). |

## Tecnico (architettura, codice)

| Termine | Significato |
|---------|-------------|
| **Parser** | Componente che legge file `.sav` e estrae dati (gioco, versione, Pokemon, ecc.). In PokeTracker: sidecar C#. |
| **Sidecar** | Eseguibile esterno (es. parser C#) invocato dal backend Tauri. Vedi [tauri2-sidecar-standard](../standards/tauri2-sidecar-standard.md), [parser-architecture](./parser-architecture.md). |
| **Knowledge DB / Database conoscenza** | Base di dati (es. SQLite) con informazioni statiche su Pokemon, mosse, nature, ecc. Usata da wiki e UI. Vedi [knowledge-database](./knowledge-database.md). |
| **Invoke** | Chiamata da frontend a comando Tauri (`invoke('command_name', { ... })`). |
| **Comando (Tauri)** | Funzione Rust esposta al frontend tramite Tauri; entry point per logica backend. |
| **Catalog** | Elenco componenti riutilizzabili (primitivi e compositi). Path: `lib/components/ui/`. |
| **Reuse-first** | Regola: prima controllare il catalog; se esiste un primitivo/composito adatto, usarlo; creare nuovo solo se manca. |

## Note

- Per struttura progetto: [project-structure](./project-structure.md).
