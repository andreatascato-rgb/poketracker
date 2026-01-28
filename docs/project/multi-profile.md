# Feature: Gestione Multi-Profilo

## Obiettivo

Documenta la gestione di più profili/allenatori nell'app, ognuno con dati e file salvataggio separati.

## Descrizione

L'app deve supportare più profili/allenatori contemporaneamente, ognuno completamente separato con i propri dati e file salvataggio.

## Funzionalità

### Creazione Profili

- Creazione di profili multipli
- Nessun sistema di login/logout richiesto
- Ogni profilo ha un nome identificativo
- Profili completamente indipendenti

### Separazione Dati

Ogni profilo mantiene dati separati:
- Elenco di percorsi salvataggi assegnati (più path per profilo)
- Pokedex personale
- Dati allenatore
- Progresso gioco
- Tutti i dati estratti dai salvataggi

La **cartella main** (opzionale, globale, impostata una volta) non è dati di profilo: serve solo come comodo per Sfoglia e suggerimenti. Vedi [glossary](./glossary.md), [core-functionality](./core-functionality.md).

### Switch Profili

- Cambio tra profili senza logout/login
- Interfaccia per selezionare profilo attivo
- Dati del profilo selezionato visibili
- Dati altri profili non visibili ma mantenuti

### Monitoraggio per Profilo

Ogni profilo monitora solo i propri percorsi salvataggi:
- Controllo automatico all'avvio per i percorsi del profilo attivo
- Monitoraggio continuo di ogni percorso assegnato al profilo attivo
- Dati aggiornati solo per il profilo attivo
- Altri profili mantengono i loro dati salvati

## Integrazione con Sistema Esistente

Questa feature si integra con:
- Cartella main (preferenza globale) e sistema di assegnazione percorsi per profilo
- Sistema di analisi salvataggi (per profilo)
- Sistema di sincronizzazione (per profilo attivo)
- Persistenza dati (salvataggio separato per profilo)

## Note

Questa feature permette a più utenti di usare l'app sullo stesso dispositivo, o a un utente di gestire più allenatori/giochi separatamente.
