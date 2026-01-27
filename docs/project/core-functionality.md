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
- L'app non sta monitorando attivamente la cartella
- La cartella non è accessibile temporaneamente
- L'app è stata chiusa e riaperta

I dati devono essere salvati localmente per essere disponibili anche senza accesso ai file salvataggio originali.

## Gestione Percorso Salvataggi

### Percorso Fisso

L'app deve avere un percorso fisso per i salvataggi. Questo permette all'app di sapere:
- Quale gioco Pokemon
- Quale allenatore
- Dove si trova il salvataggio

### Gestione Profili/Allenatori

L'app deve gestire più profili/allenatori separati:
- Creazione di profili multipli (senza sistema di login)
- Ogni profilo è completamente separato
- Ogni profilo può avere una cartella salvataggi diversa
- Dati separati per ogni profilo (Pokedex, progresso, etc.)
- Switch tra profili senza logout/login

### Assegnazione Cartella per Profilo

Ogni profilo può avere una cartella salvataggi assegnata:
- Accesso ai file di sistema per selezionare cartella
- Salvataggio della cartella selezionata come percorso fisso per quel profilo
- Cartelle diverse per profili diversi

### Controllo Automatico

L'app deve controllare i file salvataggio in modo continuo:

**All'Avvio:**
- Controlla i file `.sav` nella cartella assegnata
- Verifica se ci sono aggiornamenti (file modificati)
- Se è la prima volta: estrae e salva tutti i dati disponibili
- Se ci sono aggiornamenti: aggiorna i dati esistenti con le modifiche

**Con App Aperta:**
- Monitora continuamente la cartella assegnata
- Rileva modifiche ai file `.sav` in tempo reale
- Aggiorna automaticamente i dati quando rileva modifiche

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

1. L'app legge file salvataggio Pokemon dal percorso assegnato
2. Riconosce automaticamente gioco e versione
3. Estrapola dati dal file
4. Organizza i dati raccolti associandoli a gioco e allenatore

### Sincronizzazione

L'app mantiene i dati sincronizzati con i file salvataggio:
- All'avvio controlla automaticamente la cartella assegnata
- Durante l'esecuzione monitora continuamente la cartella
- Rileva file nuovi o modificati in tempo reale
- Aggiorna i dati in base alle modifiche rilevate

### Persistenza e Memoria

I dati estratti devono essere:
- Salvati localmente nel dispositivo
- Disponibili anche quando la cartella non è accessibile
- Mantenuti in memoria per accesso rapido
- Sincronizzati con i file salvataggio quando disponibili

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
