# Architettura Parser - PokeTracker

## Obiettivo

Definisce l'architettura per il parsing dei file salvataggio Pokemon (`.sav`, `.dsv`) nell'applicazione PokeTracker.

## Problema

Il parsing dei file salvataggio Pokemon in Rust è estremamente complesso perché:
- Ogni generazione ha formato diverso
- Formati non documentati ufficialmente
- Checksum e validazione complessi
- Edge cases numerosi

PKHeX (C#) è la libreria standard che risolve tutti questi problemi, ma integrare C# direttamente in Rust/Tauri è complesso.

## Soluzione: Sidecar Pattern

### Architettura Sidecar

**App C# separata** che:
- Usa PKHeX per leggere/scrivere file salvataggio
- Espone API semplice (JSON in/out)
- Viene chiamata da Rust/Tauri come processo esterno

**Comunicazione:**
- Rust spawna processo C# con parametri (path file, operazione)
- C# processa e ritorna JSON con dati
- Rust riceve JSON e lo usa nell'app

### Vantaggi

✅ **Riusa codice maturo**: PKHeX è testato e supporta tutte le generazioni
✅ **Separazione responsabilità**: Parser isolato, facile da aggiornare
✅ **Semplice da implementare**: Tauri può spawnare processi facilmente
✅ **Mantenibilità**: Aggiornare PKHeX non richiede modifiche Rust
✅ **Debugging**: Puoi testare parser C# indipendentemente

### Svantaggi

⚠️ **Overhead processo**: Spawnare processo ha costo (accettabile per operazioni non frequenti)
⚠️ **Dipendenze**: Richiede .NET runtime installato (o bundle con app)
⚠️ **Error handling**: Gestione errori tra processi richiede attenzione

## Implementazione

### App C# Sidecar

**Struttura:**
- Console app C# minimale
- Usa PKHeX come libreria
- Accetta comandi via CLI args o stdin
- Output JSON su stdout

**Operazioni:**
- `read <filepath>`: Legge salvataggio e ritorna JSON
- `write <filepath> <json>`: Scrive modifiche al salvataggio
- `validate <filepath>`: Valida salvataggio

### Integrazione Tauri

**Rust chiama sidecar:**
```rust
// Esempio pseudocodice
let output = Command::new("pkhex-sidecar.exe")
    .arg("read")
    .arg(save_file_path)
    .output()?;
    
let data: SaveData = serde_json::from_slice(&output.stdout)?;
```

**Gestione errori:**
- Controllo exit code processo
- Parsing JSON con validazione
- Timeout per operazioni lunghe

## Distribuzione

### Opzioni

**1. Bundle con app:**
- Includi `.exe` sidecar nel bundle Tauri
- Path relativo all'app
- Nessuna dipendenza esterna

**2. Download automatico:**
- App scarica sidecar al primo avvio
- Cache locale
- Aggiornamenti sidecar indipendenti

**3. Richiesta installazione:**
- Utente installa sidecar separatamente
- App verifica presenza all'avvio

### Consigliato: Bundle

Bundle con app è la soluzione più semplice per utente finale.

## Alternative Considerate

### ❌ FFI C# → Rust
- Troppo complesso
- Richiede binding layer
- Difficile debugging

### ❌ Parser Rust nativo
- Estremamente difficile (già provato)
- Richiede reverse engineering completo
- Tempo di sviluppo enorme

### ❌ Photino invece di Tauri
- Cambia stack tecnologico completo
- Perdi vantaggi Tauri + Rust

## Note

Questa architettura è un compromesso pragmatico che:
- Sfrutta PKHeX maturo e testato
- Mantiene Tauri + Rust + Svelte come stack principale
- Isola complessità parsing in componente separato
- Permette sviluppo incrementale
