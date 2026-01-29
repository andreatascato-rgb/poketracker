# Feature: Gestione Interna App

## Obiettivo

Documenta la sezione dell'app che permette all'utente di gestire e monitorare l'app autonomamente, senza dover sempre chiedere assistenza.

## Descrizione

L'app deve avere una sezione dedicata per la gestione interna che permette all'utente di monitorare, configurare e modificare aspetti dell'app in modo autonomo.

## Funzionalità

### Monitoraggio Stato

Sezione per monitorare lo stato dell'app:
- Stato sincronizzazione file salvataggio
- Profili attivi e loro stato
- Dati caricati e aggiornati
- Errori o avvisi recenti
- Statistiche uso app

### Gestione Profili

Gestione completa profili dall'interno dell'app:
- Creare nuovi profili
- Eliminare profili
- Rinominare profili
- Gestire cartella main (globale) e percorsi salvataggi per profilo
- Visualizzare dettagli profilo

### Configurazione App

Impostazioni e configurazioni modificabili:
- Preferenze visualizzazione
- Impostazioni sincronizzazione
- Configurazione database
- Gestione risorse (icone, immagini)
- Impostazioni validazione

### Gestione Dati (Backup e dati)

**Sezione:** Impostazioni → Backup e dati (`/impostazioni/backup-dati`).

- **Cartella export:** Cartella dedicata dove l’app salva export e backup (default: `app_data_dir()/exports`). L’utente può aprirla (“Apri cartella export”), scegliere un’altra cartella (“Scegli cartella”) o ripristinare la predefinita (“Usa predefinita”). Vedi [analisi-import-export-cartelle.md](../temp/analisi-import-export-cartelle.md).
- **Export Pokedex:** Esporta i dati Pokedex nella cartella export (in sviluppo).
- **Backup ora:** Crea un backup del profilo/DB nella cartella export (in sviluppo).
- **Ripristina:** Importa un backup precedentemente esportato (dialog “Apri file”; nessuna cartella dedicata per l’import).

Import/Export legati al save (Editor): backup .sav, Salva, Export team → UI Editor. Import/Export dati app → Impostazioni → Backup e dati.

### Gestione Risorse

Gestione risorse dell'app:
- Download/aggiornamento risorse (icone, immagini)
- Verifica integrità risorse
- Pulizia cache risorse
- Gestione spazio disco

### Log e Diagnostica

Informazioni tecniche per troubleshooting:
- Log operazioni app
- Errori e avvisi
- Informazioni sistema
- Diagnostica problemi comuni

## Interfaccia Utente

L'interfaccia deve essere:
- Intuitiva per utenti non tecnici
- Organizzata in sezioni chiare
- Con spiegazioni per ogni opzione
- Con conferme per operazioni critiche
- Con feedback visivo per operazioni

## Note

Questa feature è importante per l'autonomia dell'utente. Deve essere progettata per essere usabile anche da utenti senza conoscenze di programmazione, ma con familiarità con PC e app.
