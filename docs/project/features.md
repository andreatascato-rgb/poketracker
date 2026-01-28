# Features - PokeTracker

## Obiettivo

Elenco e documentazione di tutte le features dell'applicazione PokeTracker.

## Features

### Pokedex Personale

Feature principale che crea un Pokedex personale dell'allenatore basato sui dati estratti dai salvataggi.

**Funzionalità:**
- Crea Pokedex personale dai dati dei file `.sav`
- Stati Pokemon: non visto / visto / catturato (come nei giochi originali)
- Visualizzazione con icone e immagini Pokemon

**Risorse Necessarie:**
- Icone/immagini di tutti i Pokemon
- Icone/immagini dei tipi
- Icona della classica Pokeball
- Download e gestione locale di tutte le risorse

**Stati Pokemon:**
- **Non visto**: Pokemon non incontrato
- **Visto**: Pokemon visto ma non catturato
- **Catturato**: Pokemon catturato dall'allenatore

Vedi [pokedex-personal.md](./pokedex-personal.md) per dettagli completi.

### Wiki Offline

Wiki consultabile offline con informazioni complete su Pokemon, nature, mosse e altro, simile a Pokewiki o Bulbapedia.

**Funzionalità:**
- Consultazione offline completa
- Divisa per categorie: Pokemon, natura, mosse, etc.
- Strategie per ogni gioco (best Pokemon, leggendari, etc.)
- Informazioni approfondite su ogni elemento

**Categorie:**
- Pokemon (con dettagli completi)
- Nature
- Mosse
- Altri elementi (da definire)

**Strategie:**
- Best Pokemon per ogni gioco
- Leggendari disponibili
- Strategie di gioco specifiche per versione

Vedi [wiki-offline.md](./wiki-offline.md) per dettagli completi.

### Editor Salvataggi (PKHeX-like)

Editor completo per modificare file salvataggio Pokemon con funzionalità simili a PKHeX.

**Funzionalità:**
- Modifica Pokemon (statistiche, mosse, livello, natura, IV/EV)
- Modifica allenatore (nome, ID, badge, denaro)
- Modifica inventario (oggetti, Pokeball, strumenti)
- Modifica Pokedex (stati visto/catturato)
- Modifica eventi e progresso gioco
- Validazione dati per evitare salvataggi corrotti

**Integrazione:**
- Utilizza il parser per leggere salvataggi
- Si integra con database conoscenza per validazione
- Sincronizza modifiche con sistema esistente

Vedi [save-editor.md](./save-editor.md) per dettagli completi.

### Gestione Multi-Profilo

Sistema di gestione di più profili/allenatori separati, ognuno con dati e file salvataggio indipendenti.

**Funzionalità:**
- Creazione profili multipli (senza login)
- Dati completamente separati per ogni profilo
- Cartella main opzionale (globale, una volta); percorsi salvataggi per ogni profilo (più path per profilo)
- Switch tra profili senza logout/login
- Monitoraggio separato per ogni profilo (solo sui percorsi assegnati)

Vedi [multi-profile.md](./multi-profile.md) per dettagli completi.

### Gestione Interna App

Sezione dell'app per gestire e monitorare l'app autonomamente, senza assistenza esterna.

**Funzionalità:**
- Monitoraggio stato app e sincronizzazione
- Gestione profili completa
- Configurazione app e impostazioni
- Gestione dati (backup, reset, esportazione)
- Gestione risorse (download, aggiornamento)
- Log e diagnostica per troubleshooting

**Interfaccia:**
- Intuitiva per utenti non tecnici
- Spiegazioni chiare per ogni opzione
- Feedback visivo per operazioni

Vedi [self-management.md](./self-management.md) per dettagli completi.

## Altre Features

Altre features verranno aggiunte qui man mano che vengono definite.

## Organizzazione

Ogni feature può essere documentata in dettaglio in file separati se necessario:
- `docs/project/features/feature-name.md` per feature complesse

## Note

Questa è un'app complessa con molte features. La documentazione verrà costruita incrementally durante lo sviluppo.
