# Inventario Parser - PokeTracker

## Obiettivo

Documento di riferimento che elenca **quali e quanti parser** sono in uso, dove vivono, cosa fanno e da dove derivano (PKHeX vs nostri). Serve come base per la distinzione futura (solo gioco/versione, solo Pokedex, ecc.) e per allineare implementazione e documentazione.

## Riepilogo

| Quantità | Dettaglio |
|----------|-----------|
| **Parser usati dall'app** | **2** (detect + pokedex_auto) |
| **Parser Pokedex per gioco** | **1** (FRLG — Rosso Fuoco/Verde Foglia, nostro) |
| **Parser nel sidecar non invocati da Rust** | **1** (`pokedex` generico PKHeX) |
| **Comandi sidecar non parser** | **1** (probe, utility debug) |

- **detect:** riconoscimento gioco/versione/lingua/generazione (PKHeX + helper).
- **pokedex_auto:** invocato da `extract_pokedex_from_save`; in base al tipo di save usa un parser dedicato (es. FRLG) o restituisce vuoto.
- **Parser FRLG:** nostro, legge i byte raw del .sav (Bulbapedia: Section 0, owned 0x28, seen 0x5C). Usato per Rosso Fuoco / Verde Foglia (ITA e altre lingue, stessa struttura save).
- **pokedex** (generico): nel sidecar ma non usato dall'app; strategia futura in `pokedex-parser-strategy.md`.
- **probe:** utility debug, non invocato dall'app.

---

## 1. Parser `detect` (attivo)

| Campo | Valore |
|-------|--------|
| **Comando sidecar** | `detect <path>` |
| **Fonte** | **PKHeX** + helper custom (C#) |
| **Cosa fa** | Legge il file `.sav`, riconosce formato tramite PKHeX `SaveUtil.GetVariantSAV`, poi estrae: nome gioco in italiano, etichetta lingua/versione (ITA, USA, …), generazione (1–9), `languageIdRaw`. Lingua può essere inferita dal path se assente nel save (Gen1–3). |
| **Dove è implementato** | Sidecar: `src-sidecar/Program.cs` (comando `detect`), `SaveDetectHelper`. Rust: `src-tauri/src/commands/save_detect.rs` → `detect_save_game_version`. |
| **Dove è usato** | `profile.rs` (sync/watcher), frontend tramite `invoke('detect_save_game_version', { path })` (es. `src/lib/services/sav.ts`, pagina Salvataggi). |
| **Output** | JSON: `{ game, version, generation, languageIdRaw }`. Tipo Rust: `SaveGameVersion`. |
| **Stato** | **In uso.** |

---

## 2. Parser `pokedex` (implementato nel sidecar, disabilitato in Rust)

| Campo | Valore |
|-------|--------|
| **Comando sidecar** | `pokedex <path>` |
| **Fonte** | **PKHeX** (API tipate `SaveFile.GetSeen`, `SaveFile.GetCaught`) + helper custom `PokedexHelper` (range specie per generazione, propagazione caught alle pre-evoluzioni). |
| **Cosa fa** | Legge il file `.sav`, per ogni specie valida per la generazione del save restituisce stato: `unseen` / `seen` / `caught`. Usa indice dex 0-based dove richiesto; propaga "caught" alle pre-evoluzioni. |
| **Dove è implementato** | Sidecar: `src-sidecar/Program.cs` (comando `pokedex`), `PokedexHelper.GetAllSpeciesStatus`. Rust: `save_detect.rs` → `extract_pokedex_from_save` **non** invoca il sidecar e restituisce sempre `Ok(vec![])`. |
| **Dove sarebbe usato** | `profile.rs` chiama `extract_pokedex_from_save` durante sync/watcher; oggi non riceve dati dal sidecar. |
| **Output** | JSON: `{ entries: [{ species_id, status }] }`. Tipo Rust: `Vec<PokedexExtractEntry>`. |
| **Stato** | **Disabilitato in app.** Strategia futura: parser per gioco/versione (nostri o PKHeX selettivo), vedi `pokedex-parser-strategy.md`. |

---

## 3. Comando `pokedex_auto` e parser FRLG (attivo per Rosso Fuoco/Verde Foglia)

| Campo | Valore |
|-------|--------|
| **Comando sidecar** | `pokedex_auto <path>` |
| **Fonte** | **Nostro** (parser raw byte per FRLG); dispatcher in base al tipo di save (PKHeX `GetType().Name`). |
| **Cosa fa** | Carica il save con PKHeX; se è `SAV3FRLG` (Rosso Fuoco/Verde Foglia) usa `FrLgPokedexParser` che legge i byte raw (blocco A/B, Section 0, owned a 0x28 e seen a 0x5C, 49 byte ciascuno). Altrimenti restituisce `entries` vuoto. Log dettagliati su stderr (blocco usato, save_index, hex primi 16 byte owned/seen, conteggi seen/caught, campione specie). |
| **Dove è implementato** | Sidecar: `Program.cs` (comando `pokedex_auto`), `FrLgPokedexParser` (raw bytes, Bulbapedia). Rust: `save_detect.rs` → `extract_pokedex_from_save` invoca sidecar `pokedex_auto`, inoltra stderr a `eprintln`, log "invoco sidecar" e "ricevute N entries (seen=X, caught=Y)". |
| **Dove è usato** | `profile.rs` (sync_sav_now, sync_pokedex_from_watched_savs_now, sync_all_sav_now) tramite `extract_pokedex_from_save`. |
| **Output** | JSON: `{ entries: [{ species_id, status }] }`. Tipo Rust: `Vec<PokedexExtractEntry>`. |
| **Stato** | **In uso** per FRLG; per altri giochi restituisce vuoto. |
| **Log (watcher)** | In terminale: `[PokeTracker] extract_pokedex_from_save: invoco sidecar...`; `[parser stderr] [parser] FRLG: ...` (blocco, hex, conteggi); `[PokeTracker] extract_pokedex_from_save: ricevute N entries...`. |

---

## 4. Comando `probe` (non è un parser)

| Campo | Valore |
|-------|--------|
| **Comando sidecar** | `probe` (nessun path) |
| **Fonte** | **Nostro** (script di ispezione su PKHeX). |
| **Cosa fa** | Usa reflection su `PKHeX.Core.SaveFile` per elencare interfacce/proprietà/metodi relativi a Zukan/Pokedex/Seen/Caught. Utility per sviluppo e analisi API PKHeX. |
| **Dove è implementato** | `src-sidecar/PkHexPokedexProbe.cs`, invocato da `Program.cs` quando `args[0] == "probe"`. |
| **Usato dall'app** | No. Eseguibile a mano (es. `dotnet run -- probe`). |
| **Stato** | Utility debug, fuori dal flusso app. |

---

## Come aggiungere un nuovo parser Pokedex

1. **Ricerca formato:** trovare offset e struttura Pokedex per il gioco (es. Bulbapedia, Data Crystal, PKHeX source). Decidere se parser raw byte (nostro) o riuso API PKHeX selettivo.
2. **Sidecar — classe parser:** in `src-sidecar/Program.cs` creare una classe `XxxPokedexParser` con metodo statico che riceve `byte[] data` (e opzionale `path` per log), restituisce `List<PokedexHelper.PokedexEntry>`. Log su stderr (blocco/sezione, hex campione, conteggi) per debug.
3. **Sidecar — dispatcher:** nello stesso file, nel blocco `if (command == "pokedex_auto")` aggiungere una condizione sul `typeName` (es. `sav.GetType().Name` come `SAV3E`, `SAV4`, …) e in quel caso chiamare il nuovo parser; altrimenti restare su “nessun parser dedicato, restituisco vuoto”.
4. **Rust:** nessuna modifica necessaria: `extract_pokedex_from_save` invoca già `pokedex_auto` e gestisce il JSON; il contratto `{ entries: [{ species_id, status }] }` è unico.
5. **Documentazione:** aggiornare questo inventario (riepilogo + eventuale nuova sezione per il parser) e, se utile, `pokedex-parser-strategy.md` (ordine implementazione / giochi supportati).

---

## Riferimenti

- **Architettura generale:** [parser-architecture.md](./parser-architecture.md)
- **Strategia Pokedex (parser per gioco/versione):** [pokedex-parser-strategy.md](./pokedex-parser-strategy.md)
- **Riconoscimento gioco/versione:** [sav-game-version-detection.md](./sav-game-version-detection.md)
- **Sidecar:** `src-sidecar/Program.cs`; comandi Rust: `src-tauri/src/commands/save_detect.rs`, invocazioni in `profile.rs`
