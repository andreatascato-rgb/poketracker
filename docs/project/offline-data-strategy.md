# Strategia Offline e Dati Locali

## Obiettivo

Riassume la strategia di dati offline e persistenza per PokeTracker: tutto locale, nessun sync per ora, dove vivono i dati, cache-first implicito. Fonte di requisiti: [core-functionality](./core-functionality.md); vista tecnica: [architecture-overview](./architecture-overview.md).

## Principio: Offline-First

- L'app funziona **solo** su dati locali; non dipende da rete per core e wiki.
- Dati estratti dai save, knowledge DB, profili e Pokedex sono **sempre** su disco (SQLite, file, risorse bundle).
- Nessun “sync con cloud” né conflitti multi-device per ora; strategia “local-only”.

## Dove Vivono i Dati

| Dato | Dove | Fonte di verità |
|------|------|------------------|
| **Profili, cartelle save, Pokedex per profilo** | DB SQLite (Rust), path da `app.path().app_data_dir()` | [database-storage](./database-storage.md), [architecture-overview](./architecture-overview.md) |
| **Knowledge (Pokemon, mosse, nature, wiki)** | DB o bundle locale; lettura via comandi Rust | [knowledge-database](./knowledge-database.md) |
| **File `.sav`** | Filesystem; lettura/parsing via sidecar C# | [parser-architecture](./parser-architecture.md), [core-functionality](./core-functionality.md) |
| **Risorse (icone, sprite)** | Bundle app o download locale; path da app | [features](./features.md), [pokedex-personal](./pokedex-personal.md) |

## Cache-First (Implicito)

- **Tutto è già “cache”:** non c’è distinzione “rete vs locale”; l’unica fonte è locale.
- **Aggiornamento dati:** (a) all’avvio: controllo file `.sav` in cartella profilo, estrazione/aggiornamento; (b) con app aperta: monitoraggio cartella, rilevazione modifiche, aggiornamento incrementale. Vedi [core-functionality](./core-functionality.md) (Controllo automatico, Riconoscimento automatico).
- **Persistenza:** dati estratti salvati in DB locale per restare disponibili anche senza accesso ai file `.sav` originali.

## Sync Futuro (Opzionale)

- Se in futuro si introducono sync o “cloud backup”, definire in un doc dedicato: conflitti, last-writer-wins, merge, risoluzione manuale.
- Per ora non è in scope; questo doc non descrive sync.

## Riferimenti

- [core-functionality](./core-functionality.md) — Offline, persistenza, profili, cartella save, controllo automatico
- [architecture-overview](./architecture-overview.md) — Stack, flusso dati, dove vivono profilo/Pokedex/wiki
- [database-storage](./database-storage.md) — DB e storage
- [knowledge-database](./knowledge-database.md) — Knowledge DB e wiki
