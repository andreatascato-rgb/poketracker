# Feature: Pokedex Personale

## Obiettivo

Documenta la feature del Pokedex personale che mostra lo stato di completamento del Pokedex dell'allenatore basato sui dati dei salvataggi.

## Descrizione

Il Pokedex personale è una delle feature principali dell'app. Crea un Pokedex personalizzato per ogni allenatore basato sui dati estratti dai file salvataggio.

## Funzionalità

### Creazione Pokedex

Il Pokedex viene creato automaticamente dai dati dei file `.sav`:
- Analizza i Pokemon incontrati/catturati dall'allenatore
- Aggiorna lo stato di ogni Pokemon nel Pokedex
- Mantiene lo stato anche quando l'app è offline

### Stati Pokemon

Ogni Pokemon nel Pokedex può avere tre stati (come nei giochi originali):

- **Non visto**: Pokemon non ancora incontrato dall'allenatore
- **Visto**: Pokemon visto ma non catturato
- **Catturato**: Pokemon catturato dall'allenatore

### Visualizzazione

Il Pokedex mostra:
- Icone/immagini di tutti i Pokemon
- Stato di ogni Pokemon (non visto/visto/catturato)
- Informazioni sui tipi con icone
- Icona della classica Pokeball per Pokemon catturati

## Risorse Necessarie

### Download Risorse

L'app deve scaricare e gestire localmente:

- **Icone/immagini Pokemon**: Per ogni Pokemon nel database
- **Icone tipi**: Per tutti i tipi Pokemon (Fuoco, Acqua, Erba, etc.)
- **Icona Pokeball**: La classica Pokeball per indicare Pokemon catturati

### Gestione Risorse

- Download iniziale di tutte le risorse necessarie
- Salvataggio locale per funzionamento offline
- Aggiornamento risorse quando necessario

## Integrazione con Dati Salvataggio

Il Pokedex si aggiorna automaticamente quando:
- Vengono analizzati nuovi file salvataggio
- I file salvataggio vengono modificati
- L'app rileva cambiamenti nei dati dell'allenatore

## Note

Questa feature è fondamentale per l'esperienza utente. Richiede:
- Database completo di tutti i Pokemon
- Sistema di download e gestione risorse
- Integrazione con il sistema di analisi salvataggi
