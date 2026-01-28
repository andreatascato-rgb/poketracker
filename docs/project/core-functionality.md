# Funzionalità Core - PokeTracker

## Obiettivo

Documenta le funzionalità fondamentali dell'applicazione PokeTracker.

## Requisiti Fondamentali

### Funzionamento Offline

L'app funziona completamente offline. Non richiede connessione internet per:
- Analisi file salvataggio
- Riconoscimento gioco/versione
- Estrazione dati
- Visualizzazione dati

### Persistenza Dati

I dati estratti dai salvataggi devono rimanere in memoria anche quando:
- L'app non sta monitorando attivamente i percorsi
- I percorsi non sono accessibili temporaneamente
- L'app è stata chiusa e riaperta

I dati devono essere salvati localmente per essere disponibili anche senza accesso ai file salvataggio originali.

## Gestione Percorsi Salvataggi

L'app permette all'utente di configurare sia una **cartella main** (globale, opzionale) sia **percorsi specifici** per profilo. Il monitoraggio reale avviene solo sui percorsi specifici associati a ogni profilo.

### Cartella Main (globale, opzionale)

- Impostata **una volta sola**, a livello app (non per profilo).
- Percorso dove l'utente tiene di solito emulatori e save (es. `C:\Emulatori`).
- **Non è monitorata**: non viene usata per cercare o osservare file `.sav`.
- Uso: comodo per **Sfoglia** (il dialog si apre in cartella main quando si aggiunge un percorso) e per eventuali **suggerimenti** (sottocartelle proposte).

### Percorsi Specifici per Profilo

- Ogni profilo ha un **elenco di percorsi** scelti dall'utente (es. tramite Sfoglia).
- Ogni percorso è associato esplicitamente al profilo → associazione sav ↔ allenatore chiara.
- L'app **monitora solo** questi percorsi per quel profilo; nessuna scansione generica su cartella main.

### Gestione Profili/Allenatori

L'app deve gestire più profili/allenatori separati:
- Creazione di profili multipli (senza sistema di login)
- Ogni profilo è completamente separato
- Ogni profilo ha i propri percorsi salvataggi (più path per profilo)
- Dati separati per ogni profilo (Pokedex, progresso, etc.)
- Switch tra profili senza logout/login

### Aggiunta Percorsi per Profilo

- Accesso ai file di sistema (Sfoglia) per selezionare una cartella da aggiungere al profilo
- Salvataggio del percorso nell'elenco del profilo
- Possibilità di più percorsi per profilo (es. una cartella per emulatore/gioco)
- La cartella main, se impostata, può essere usata come punto di partenza per Sfoglia

### Controllo Automatico

L'app controlla i file salvataggio solo nei **percorsi assegnati** al profilo:

**All'Avvio:**
- Controlla i file `.sav` in tutti i percorsi assegnati al profilo attivo
- Verifica se ci sono aggiornamenti (file modificati)
- Se è la prima volta: estrae e salva tutti i dati disponibili
- Se ci sono aggiornamenti: aggiorna i dati esistenti con le modifiche

**Con App Aperta:**
- Monitora continuamente ogni percorso assegnato al profilo attivo
- Rileva modifiche ai file `.sav` in tempo reale
- Aggiorna automaticamente i dati quando rileva modifiche

**Implementazione:** il monitoraggio continuo usa **file system watcher** (eventi del SO: es. `ReadDirectoryChangesW` su Windows, `inotify` su Linux, FSEvents su macOS), non polling. Un watcher per ogni percorso monitorato; struttura prevista in `project-structure.md` → `monitoring/watcher.rs`, `services/save_monitor.rs`.

### Riconoscimento Automatico

L'app deve riconoscere automaticamente dai file salvataggio:
- **Gioco**: Quale gioco Pokemon (es. Pokemon Rosso, Pokemon Smeraldo, etc.)
- **Versione**: Versione del gioco (italiana, USA, etc.)

Questo riconoscimento avviene analizzando il file salvataggio stesso.

## Analisi File Salvataggio

### Formati Supportati

- `.sav` - File salvataggio standard
- `.dsv` - File salvataggio alternativo

### Processo

1. L'app legge file salvataggio Pokemon dai percorsi assegnati al profilo
2. Riconosce automaticamente gioco e versione
3. Estrapola dati dal file
4. Organizza i dati raccolti associandoli a gioco e allenatore

### Sincronizzazione

L'app mantiene i dati sincronizzati con i file salvataggio:
- All'avvio controlla automaticamente tutti i percorsi assegnati al profilo attivo
- Durante l'esecuzione monitora continuamente quei percorsi
- Rileva file nuovi o modificati in tempo reale
- Aggiorna i dati in base alle modifiche rilevate

### Persistenza e Memoria

I dati estratti devono essere:
- Salvati localmente nel dispositivo
- Disponibili anche quando i percorsi non sono accessibili
- Mantenuti in memoria per accesso rapido
- Sincronizzati con i file salvataggio quando i percorsi sono disponibili

Questo permette all'app di funzionare offline e mostrare dati anche senza accesso immediato ai file originali.

### Parser

Il parsing dei file salvataggio viene gestito tramite architettura sidecar che utilizza PKHeX (C#).

**Nota**: Vedi [parser-architecture.md](./parser-architecture.md) per dettagli completi sull'architettura.

## Database Conoscenza

L'app mantiene un database completo con:

- Tutti i giochi Pokemon
- Tutti i Pokemon
- Tutti i dati correlati (mosse, abilità, tipi, etc.)

Questo database permette all'app di interpretare correttamente i dati estratti dai salvataggi.

**Nota**: Vedi [knowledge-database.md](./knowledge-database.md) per dettagli completi su dimensioni, architettura e gestione.

## Raccolta Dati Allenatore

Dai file salvataggio analizzati, l'app raccoglie:

- Dati sull'allenatore
- Pokemon posseduti
- Progresso nel gioco
- Altri dati rilevanti

## Note

Questa è la funzionalità base. Altre features verranno aggiunte successivamente.
