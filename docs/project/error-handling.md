# Gestione Errori - PokeTracker

## Obiettivo

Definisce la strategia di gestione errori per l'applicazione PokeTracker (Tauri + Rust + Svelte).

## Principi Fondamentali

### Rust Error Handling

**Result<T, E> Pattern:**
- Tutte le funzioni che possono fallire ritornano `Result<T, E>`
- Propagazione errori con `?` operator
- Nessun panic in produzione (solo in situazioni irrecuperabili)

**Custom Error Types:**
- Enum di errori specifici del dominio
- Uso di `thiserror` per derivare automaticamente `Error` trait
- Conversione automatica tra tipi di errore

### Tauri IPC Error Handling

**Comandi Tauri:**
- Tutti i comandi ritornano `Result<T, String>` o custom error type
- Errori serializzati e inviati al frontend
- Frontend riceve errori strutturati

### Frontend Error Handling

**Svelte:**
- Cattura errori da comandi Tauri
- Store per gestire stato errori
- Notifiche user-friendly
- Logging errori per debugging

## Categorie Errori

### Errori File System

**Tipi:**
- File non trovato
- Permessi insufficienti
- Spazio disco insufficiente
- File corrotti
- Path invalido

**Gestione:**
- Validazione path prima di operazioni
- Controllo permessi
- Messaggi chiari all'utente
- Suggerimenti di risoluzione

### Errori Parser/Sidecar

**Tipi:**
- Processo sidecar fallito
- Timeout processo
- JSON invalido
- Salvataggio non riconosciuto
- Versione incompatibile

**Gestione:**
- Timeout configurabile
- Retry logic per errori temporanei
- Validazione JSON prima di parsing
- Messaggi specifici per tipo di errore

### Errori Database

**Tipi:**
- Connessione fallita
- Query fallita
- Database corrotto
- Migrazione fallita
- Constraint violation

**Gestione:**
- Retry connessione
- Validazione dati prima di inserimento
- Backup automatico prima di migrazioni
- Recovery da backup se corruzione

### Errori Validazione

**Tipi:**
- Dati invalidi
- Salvataggio corrotto
- Dati inconsistenti
- Formato non supportato

**Gestione:**
- Validazione preventiva
- Messaggi chiari su cosa è invalido
- Suggerimenti per correggere

## Implementazione

### Custom Error Type (Rust)

**Struttura:**
```rust
#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error("File not found: {0}")]
    FileNotFound(String),
    
    #[error("Permission denied: {0}")]
    PermissionDenied(String),
    
    #[error("Parser error: {0}")]
    ParserError(String),
    
    #[error("Database error: {0}")]
    DatabaseError(#[from] rusqlite::Error),
    
    // ... altri errori
}
```

**Uso:**
- Tutti i servizi ritornano `Result<T, AppError>`
- Conversione automatica da errori di librerie
- Propagazione con `?`

### Comandi Tauri

**Pattern:**
```rust
#[tauri::command]
async fn read_save_file(path: String) -> Result<SaveData, String> {
    // Operazione che può fallire
    // Ritorna errore come String per Tauri
}
```

**Conversione:**
- Convertire `AppError` in `String` per Tauri
- Includere messaggio user-friendly
- Logging errore completo internamente

### Frontend Svelte

**Pattern:**
```typescript
try {
  const data = await invoke('read_save_file', { path });
} catch (error) {
  // Gestione errore
  errorStore.set(error);
  showNotification(error);
}
```

**Store Errori:**
- Store Svelte per stato errori globali
- Notifiche toast per errori
- Log errori per debugging

### Registro errori e notifiche (standard prodotto)

Quando un errore deve essere visibile all’utente (es. versione non determinata con languageId raw, sidecar fallito), **non** mostrare dati di debug direttamente nelle schermate operative. Invece:

1. **Avviso (transiente):** toast/snackbar breve che segnala il problema.
2. **Notifica (persistente):** voce nel centro notifiche, apribile da icona dedicata in Top Bar; clic sull’icona apre un componente dedicato alle notifiche.
3. **Archivio errori:** creare una voce in **Archivio → Errori** (sezione sidebar “Archivio”, sottosezione “Errori”) con: **Data**, **Tipo errore**, pulsante **Apri** (apre dettaglio completo copiabile), azione **Elimina**. Scopo: avere un log completo da copiare/incollare (es. per supporto o assistente AI).

Vedi [notifications-and-error-archive.md](./notifications-and-error-archive.md) per scelte UX complete, posizione “dove si vede oggi il fallimento” (es. languageId raw nel dialog Salvataggi) e definizione di Top Bar notifiche, Archivio, tabella Errori.

## Logging

### Strategia Logging

**Livelli:**
- **Error**: Errori critici che richiedono attenzione
- **Warn**: Avvisi (file non trovato, validazione fallita)
- **Info**: Operazioni importanti (sync completato, profilo cambiato)
- **Debug**: Informazioni dettagliate per debugging

**Strumenti:**
- `tracing` o `log` per logging strutturato
- File log nella app data directory
- Rotazione log per gestire spazio

### Logging Errori

**Cosa loggare:**
- Tipo errore
- Contesto (file, operazione, profilo)
- Stack trace (se disponibile)
- Timestamp
- Dati rilevanti (senza dati sensibili)

## Recovery e Fallback

### Strategie Recovery

**File Corrotti:**
- Backup automatico prima di modifiche
- Ripristino da backup se corruzione
- Validazione dopo scrittura

**Database Corrotto:**
- Backup periodici
- Ripristino da backup
- Migrazione automatica se possibile

**Sidecar Fallito:**
- Retry con backoff esponenziale
- Timeout configurabile
- Fallback a operazione alternativa se possibile

**Permessi Insufficienti:**
- Messaggio chiaro all'utente
- Suggerimento per risolvere
- Non bloccare altre operazioni

## Messaggi Utente

### Principi

**User-Friendly:**
- Messaggi chiari e comprensibili
- Niente stack trace o errori tecnici
- Spiegazione del problema
- Suggerimenti per risolvere

**Esempi:**
- ❌ "Error: EACCES: permission denied, open 'C:\...'"
- ✅ "Impossibile accedere al file. Verifica i permessi della cartella."

**Categorie Messaggi:**
- **Info**: Operazione completata con successo
- **Warning**: Avviso ma operazione continuabile
- **Error**: Errore che blocca operazione
- **Critical**: Errore critico che richiede attenzione immediata

## Note

Questa strategia permette:
- Gestione errori robusta e type-safe
- Messaggi utente chiari
- Debugging facilitato con logging
- Recovery automatico quando possibile
- App stabile anche con errori
