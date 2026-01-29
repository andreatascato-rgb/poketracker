# Database e Storage - PokeTracker

## Obiettivo

Definisce come gestire database e storage locale per l'applicazione PokeTracker in Tauri + Rust.

## Best Practice Tauri + Rust

### SQLite per Dati Strutturati

**SQLite** è la scelta standard per app Tauri che richiedono:
- Funzionamento offline completo
- Dati strutturati complessi
- Query efficienti
- Transazioni ACID
- Nessuna dipendenza esterna

**Vantaggi:**
- Database embedded (file singolo)
- Zero configurazione
- Performance eccellenti per app desktop
- Supporto completo in Rust (rusqlite)
- Compatibile con Tauri

### File JSON per Configurazione

**File JSON** per dati semplici e configurazione:
- Impostazioni app
- Configurazione profili
- Preferenze utente

**Vantaggi:**
- Leggibile e modificabile manualmente
- Semplice da gestire
- Perfetto per dati non relazionali

## Architettura Storage

### Database SQLite

**Database principale** per:
- Dati profili (allenatori, cartelle, etc.)
- Dati estratti dai salvataggi (Pokemon, progresso, etc.)
- Pokedex personale (stati visto/catturato)
- Database conoscenza (Pokemon, giochi, mosse, etc.)
- Storico modifiche salvataggi

**Struttura:**
- Un database SQLite per app
- Tabelle separate per ogni entità: `profiles`, `app_state`, `pokedex_state` (stato visto/catturato per specie e profilo; vedi [pokedex-personal](./pokedex-personal.md)), salvataggi, ecc.
- Relazioni tra tabelle quando necessario (es. `pokedex_state.profile_id` → `profiles.id` ON DELETE CASCADE)

### File JSON

**File JSON** per:
- Configurazione app generale
- Impostazioni profili
- Cache e dati temporanei

**Struttura:**
- File nella cartella app data di Tauri
- Un file per tipo di configurazione

## Posizione File

### Tauri App Data Directory

Tauri fornisce directory dedicate per app data:
- **Windows**: `%APPDATA%\poketracker\`
- **macOS**: `~/Library/Application Support/poketracker/`
- **Linux**: `~/.config/poketracker/`

**File storage:**
- Database SQLite: `data.db` nella app data directory
- File JSON: `config.json`, `profiles.json`, etc. nella app data directory
- Risorse (icone, immagini): `resources/` nella app data directory

## Librerie Rust Consigliate

### SQLite

**rusqlite** - Binding Rust per SQLite:
- Standard de facto per SQLite in Rust
- Type-safe queries
- Supporto completo funzionalità SQLite
- Compatibile con Tauri

**Alternativa: sqlx** (se serve async):
- Async support
- Type-safe compile-time checked queries
- Più complesso ma più potente

### Serializzazione

**serde** + **serde_json**:
- Serializzazione/deserializzazione Rust
- Standard per JSON in Rust
- Type-safe
- Efficiente

## Migrazioni Database

### Sistema Migrazioni

Implementare sistema di migrazioni per:
- Aggiornare schema database tra versioni app
- Mantenere compatibilità con dati esistenti
- Gestire versioni database

**Approccio:**
- File migrazioni SQL numerati
- Sistema di versioning database
- Applicazione migrazioni all'avvio se necessario

## Backup e Ripristino

### Backup Automatico

- Backup database prima di modifiche importanti
- Backup periodici automatici
- Gestione spazio disco (limitare numero backup)

### Ripristino

- Ripristino da backup
- Esportazione/importazione dati
- Reset completo database

## Note

Questa architettura permette:
- Funzionamento completamente offline
- Persistenza dati robusta
- Performance eccellenti
- Scalabilità per dati complessi
- Facilità di backup e ripristino
