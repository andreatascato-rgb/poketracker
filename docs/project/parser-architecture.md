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
- Console app C# in `src-sidecar/` (Program.cs)
- Usa PKHeX.Core come libreria
- Input: comando + argomenti via CLI args
- Output: JSON su stdout (UTF-8); errori su stderr e exit code non-zero

**Operazioni (contratto attuale):**
- `detect <filepath>`: Riconoscimento gioco/versione/lingua/generazione. Ritorna JSON `{ game, version, generation, languageIdRaw }`.
- `pokedex <filepath>`: Estrazione stato Pokedex (visto/catturato) per le specie valide per la generazione del save. Usa **solo** l’API tipata PKHeX: `SaveFile.GetSeen(speciesId)` e `SaveFile.GetCaught(speciesId)`. Range specie dipendente da `SaveFile.Generation` (nessuna reflection, nessun range hardcoded). Ritorna JSON `{ entries: [{ species_id, status }] }`.

**Nessuna operazione di scrittura:** il sidecar è read-only (lettura salvataggio e dati Pokedex).

### Integrazione Tauri

**Rust chiama sidecar:**
- `src-tauri/src/commands/save_detect.rs`: invoca il sidecar con `detect <path>` o `pokedex <path>`, legge stdout, deserializza JSON, restituisce `Result` al frontend.
- Validazione path (non vuoto, file esistente dove richiesto) lato Rust prima dello spawn.

**Gestione errori:**
- Exit code processo: 0 = successo, non-zero = errore
- JSON su stdout con eventuale campo `error` per messaggio user-facing
- Rust: parsing JSON con `map_err`; nessun `unwrap_or_default` che nasconda errori di parsing

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
