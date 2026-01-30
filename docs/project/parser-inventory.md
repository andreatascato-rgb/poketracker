# Inventario Parser - PokeTracker

## Obiettivo

Documento di riferimento che elenca **quali e quanti parser** sono in uso, dove vivono, cosa fanno e da dove derivano (PKHeX vs nostri). Serve come base per la distinzione futura (solo gioco/versione, solo Pokedex, ecc.) e per allineare implementazione e documentazione.

## Riepilogo

| Quantità | Dettaglio |
|----------|-----------|
| **Parser usati dall'app** | **3** (detect + pokedex_auto + trainer) |
| **Parser Pokedex per gioco/lingua** | **FRLG** e **Smeraldo (Emerald)**; ciascuno differenziato per lingua (ITA, USA, JPN, FRA, GER, SPA + generico) |
| **Parser nel sidecar non invocati da Rust** | **1** (`pokedex` generico PKHeX) |
| **Comandi sidecar non parser** | **1** (probe, utility debug) |

- **Un solo parser per (gioco + versione):** ogni coppia tipo save + lingua è associata a **un unico** parser Pokedex (es. SAV3FRLG + ITA → solo FrLgPokedexParser; SAV3E + ITA → solo EmeraldPokedexParser). Non esistono due parser per la stessa combinazione.
- **detect:** riconoscimento gioco/versione/lingua/generazione (PKHeX + helper).
- **pokedex_auto:** invocato da `extract_pokedex_from_save`; in base a **tipo di save + lingua** (da `SaveDetectHelper.GetLanguageLabel`) usa il parser dedicato (es. FRLG ITA, Smeraldo ITA, …) o restituisce vuoto. L’associazione è **automatica**: inserendo il salvataggio corretto (gioco/lingua riconosciuti da detect), il watcher/sync invoca `pokedex_auto` e il sidecar sceglie il parser senza input utente.
- **Parser FRLG:** nostro, per Rosso Fuoco/Verde Foglia (`SAV3FRLG`/`SAV3FR`/`SAV3LG`); stessa struttura save per tutte le lingue (Section 0, owned 0x28, seen 0x5C). Lingua non mappata → parser FRLG generico.
- **Parser Smeraldo:** nostro, per Smeraldo (`SAV3E`); Section 0 come FRLG (owned 0x28, seen 0x5C, 49 byte, 386 specie Gen3). Fonte: Bulbapedia Save data structure (Generation III). Lingua non mappata → parser Smeraldo generico.
- **Parser HGSS:** per Oro Heart Gold / Argento Soul Silver (`SAV4HGSS`) per tutte le lingue; usa **API PKHeX** (`GetSeen`/`GetCaught`), specie 1–493, propagazione caught in `PokedexHelper`. PKHeX Gen4 può non esporre LanguageID: il ramo HGSS viene eseguito sempre quando il tipo save è SAV4HGSS.
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

## 2. Parser `trainer` (attivo)

| Campo | Valore |
|-------|--------|
| **Comando sidecar** | `trainer <path>` |
| **Fonte** | **PKHeX** (proprietà `Money`/`Currency`, `PlayedHours`, `PlayedMinutes`) + reflection custom |
| **Cosa fa** | Legge il file `.sav`, estrae dati allenatore: denaro (`Money` o `Currency` per Gen9), ore di gioco (`PlayedHours`), minuti di gioco (`PlayedMinutes`). Usa reflection per accedere alle proprietà; restituisce `null` per campi non disponibili. |
| **Dove è implementato** | Sidecar: `src-sidecar/Program.cs` (comando `trainer`), `TrainerHelper.ExtractTrainerData`. |
| **Dove sarà usato** | Dashboard allenatore (denaro, tempo), statistiche. |
| **Output** | JSON: `{ money, playedHours, playedMinutes }` (tutti nullable uint/int). |
| **Stato** | **Implementato, da integrare in Rust/frontend.** |

---

## 3. Parser `pokedex` (implementato nel sidecar, disabilitato in Rust)

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

## 4. Comando `pokedex_auto` e parser per gioco/lingua (attivo per FRLG, Smeraldo e HGSS ITA)

| Campo | Valore |
|-------|--------|
| **Comando sidecar** | `pokedex_auto <path>` |
| **Fonte** | **Nostro** (parser raw byte per FRLG e Smeraldo); **PKHeX API** per HGSS ITA; dispatcher in base a **tipo di save + lingua** (`sav.GetType().Name` e `SaveDetectHelper.GetLanguageLabel(sav, path)`). |
| **Cosa fa** | Carica il save con PKHeX; ottiene tipo e lingua. Se **FRLG** → `FrLgPokedexParser.Parse(...)`; se **Smeraldo** → `EmeraldPokedexParser.Parse(...)`; se **HGSS** → `PokedexHelper.GetAllSpeciesStatus(sav)` (specie 1–493, propagazione caught). Altri giochi → `entries` vuoto. HGSS eseguito per tutte le lingue (struttura Pokedex identica; PKHeX Gen4 può non esporre lingua). **Un solo parser per (tipo save + lingua)** per FRLG/Emerald; HGSS unico per tipo. |
| **Dove è implementato** | Sidecar: `Program.cs` (comando `pokedex_auto`, rami `isFrLg` / `isEmerald` / `isHgSs` e switch su lingua), `FrLgPokedexParser`, `EmeraldPokedexParser`, `PokedexHelper.GetAllSpeciesStatus` per HGSS. Rust: `save_detect.rs` → `extract_pokedex_from_save` invoca sidecar `pokedex_auto`. |
| **Dove è usato** | `profile.rs` (sync_sav_now, sync_pokedex_from_watched_savs_now, sync_all_sav_now) tramite `extract_pokedex_from_save`. |
| **Output** | JSON: `{ entries: [{ species_id, status }] }`. Tipo Rust: `Vec<PokedexExtractEntry>`. |
| **Stato** | **In uso** per FRLG, Smeraldo (differenziati per lingua) e HGSS (tutte le lingue); per altri giochi restituisce vuoto. |
| **Log (watcher)** | In terminale: `[parser] pokedex_auto: type=SAV4HGSS lang=... → HGSS`; `[parser] pokedex_auto: N entries`; `[PokeTracker] extract_pokedex_from_save: ricevute N entries...`. In caso di errore HGSS: `[parser] pokedex_auto HGSS: GetAllSpeciesStatus failed: ...`. |

### Parser FRLG (Rosso Fuoco / Verde Foglia)

- Tipo save: `SAV3FRLG`, `SAV3FR`, `SAV3LG`. Classe: `FrLgPokedexParser`. Section 0: owned 0x28, seen 0x5C, 49 byte, 386 specie Gen3.

### Parser Smeraldo (Emerald)

- Tipo save: `SAV3E`. Classe: `EmeraldPokedexParser`. Section 0 come FRLG (owned 0x28, seen 0x5C, 49 byte, 386 specie). Fonte: Bulbapedia Save data structure (Generation III). Si associa automaticamente quando detect restituisce gioco "Smeraldo" e versione/lingua (es. ITA); il watcher/sync invoca `pokedex_auto` e il sidecar seleziona `EmeraldPokedexParser` senza configurazione aggiuntiva.

### Parser Heart Gold / Soul Silver (HGSS)

- Tipo save: `SAV4HGSS` (Oro Heart Gold e Argento Soul Silver, NDS Gen 4). **Tutte le lingue.** Usa API PKHeX (`SaveFile.GetSeen`, `SaveFile.GetCaught`) tramite `PokedexHelper.GetAllSpeciesStatus(sav)`: specie 1–493, propagazione "caught" alle pre-evoluzioni (EvolvesFrom Gen 1–4 in `PokedexHelper`). Nessun parser raw byte (save Gen 4 NDS con struttura/checksum diversa da Gen 3). Il ramo HGSS viene eseguito sempre quando il tipo save è SAV4HGSS (PKHeX Gen4 può non esporre LanguageID; struttura Pokedex identica per ogni lingua).

---

## 5. Comando `probe` (non è un parser)

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

1. **Ricerca formato:** trovare offset e struttura Pokedex per il gioco (e per la lingua, se diversa). Decidere se parser raw byte (nostro) o riuso API PKHeX selettivo.
2. **Sidecar — classe parser:** in `src-sidecar/Program.cs` creare classe (o metodo con parametro lingua) che riceve `byte[] data`, `path`, opzionale `language`; restituisce `List<PokedexHelper.PokedexEntry>`. Log su stderr includono lingua (es. `[parser] FRLG ITA: ...`).
3. **Sidecar — dispatcher:** nel blocco `if (command == "pokedex_auto")` ottenere `languageLabel = SaveDetectHelper.GetLanguageLabel(sav, path)`. Aggiungere condizione su **tipo + lingua** (es. `typeName == "SAV3E"` → solo `EmeraldPokedexParser`). Un solo parser per (tipo + lingua). Lingua non mappata → parser generico per quel gioco o vuoto.
4. **Rust:** nessuna modifica necessaria: `extract_pokedex_from_save` invoca `pokedex_auto <path>`; la lingua è ricavata dal sidecar dal file.
5. **Documentazione:** aggiornare questo inventario e, se utile, `pokedex-parser-strategy.md`.

---

## Riferimenti

- **Architettura generale:** [parser-architecture.md](./parser-architecture.md)
- **Strategia Pokedex (parser per gioco/versione):** [pokedex-parser-strategy.md](./pokedex-parser-strategy.md)
- **Riconoscimento gioco/versione:** [sav-game-version-detection.md](./sav-game-version-detection.md)
- **Sidecar:** `src-sidecar/Program.cs`; comandi Rust: `src-tauri/src/commands/save_detect.rs`, invocazioni in `profile.rs`
