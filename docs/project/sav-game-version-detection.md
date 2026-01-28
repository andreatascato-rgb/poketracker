# Riconoscimento gioco e versione da file .sav

## Obiettivo

Definisce l'implementazione del **riconoscimento automatico di gioco e versione** tramite analisi del file salvataggio (`.sav`/`.dsv`). Serve come spec unica quando si procede con questa feature: requisiti, libreria PKHeX, procedure e checklist.

Requisito di dominio: [core-functionality.md](./core-functionality.md) (sezione Riconoscimento automatico). Architettura: [parser-architecture.md](./parser-architecture.md).

---

## Cosa si deve ottenere

L'app deve ricavare da ogni file `.sav` (o `.dsv`):

- **Gioco**: quale titolo Pokémon (es. Rosso, Smeraldo, Spada/Scudo, Scarlatto/Violetto).
- **Versione**: variante/lingua (es. italiana, USA).

Il riconoscimento avviene **solo leggendo il file salvataggio**, senza input utente.

---

## Dipendenza: PKHeX

- **Libreria:** `PKHeX.Core` (NuGet). Referenza per formati save Pokémon e per determinare gioco/versione.
- **Dove si usa:** solo nel **sidecar C#** (`src-sidecar/`). Non in Rust né nel frontend.
- **Come si integra:** progetto C# in `src-sidecar/` che referenzia PKHeX; comando (es. `detect <path>`) carica il save con PKHeX, estrae gioco e versione, scrive JSON su stdout.

PKHeX espone tipi come `GameVersion` e API per caricare un save; il sidecar mappa questi valori in un JSON chiaro per Rust: `game` = nome gioco in italiano (es. "Scarlatto/Violetto", "Spada/Scudo"), `version` = etichetta lingua (ITA, USA, JPN, FRA, GER, SPA, KOR, CHS, CHT). Le mappe GameVersion/tipo → italiano e LanguageID → lingua sono nel sidecar (`SaveDetectHelper`).

---

## Architettura

- **Sidecar C#** (nuovo): console app .NET 8 in `src-sidecar/`, output in `src-tauri/binaries/` con nome per target triple (es. `parser-x86_64-pc-windows-msvc.exe`).
- **Contratto I/O:** input via argomenti (path file, operazione), output JSON su **stdout**, UTF-8. Standard: [csharp-sidecar-standard.md](../standards/csharp-sidecar-standard.md).
- **Rust:** comando Tauri che invoca il sidecar (es. `detect <path>`), legge stdout, deserializza e restituisce al frontend `{ game, version }`. Integrazione sidecar: [tauri2-sidecar-standard.md](../standards/tauri2-sidecar-standard.md).

Flusso: frontend → `invoke('detect_save_game_version', { path })` → Rust → sidecar `detect <path>` → PKHeX legge .sav → JSON su stdout → Rust → frontend.

---

## Procedure da applicare (in ordine)

1. **Sidecar:** [sidecar-setup.md](../procedures/workflow/sidecar-setup.md) — externalBin, shell plugin, permessi, naming binario.
2. **Dipendenze C#:** [dependency-add.md](../procedures/workflow/dependency-add.md) — `dotnet add package PKHeX.Core` nel progetto `src-sidecar/`.
3. **Integrazione libreria:** [library-integration.md](../procedures/workflow/library-integration.md) — primo uso di PKHeX nel sidecar (punto di ingresso, contratto JSON).
4. **Comando Rust:** [command-creation.md](../procedures/rust/command-creation.md) — comando che invoca il sidecar e espone il risultato al frontend.
5. (Opzionale) **Feature end-to-end:** [new-feature.md](../procedures/workflow/new-feature.md) se si considera questa implementazione come una “nuova feature” che tocca UI/flusso utente.

---

## Checklist implementazione

- [x] Creato progetto C# in `src-sidecar/` (.NET 8, Console, PublishSingleFile).
- [x] Aggiunto pacchetto `PKHeX.Core` 24.5.5 (.NET 8); target MSBuild copia exe in `src-tauri/binaries/` dopo publish.
- [x] Implementato comando sidecar `detect <path>`: carica save con PKHeX, ricava gioco e versione, serializza JSON su stdout; in errore scrive `{"error":"..."}` su stdout.
- [x] Sidecar restituisce `game` = nome italiano (mappa GameVersion/tipo) e `version` = etichetta lingua ITA/USA/… (da proprietà Language/LanguageID del save).
- [x] **"Versione" in UI = lingua/regione** (ITA, USA, JPN, FRA, GER, SPA, KOR, CHS, CHT).
- [x] Schema lingua **dalla Gen3 in poi** (nel save): 1=JPN, 2=USA, 3=FRA, 4=ITA, 5=GER, 6=salto, 7=SPA (0x01–0x07, 0x06 non usato).
- [x] **Fallback da path**: per Gen1–3 PKHeX spesso non espone la lingua dal save; il sidecar inferisce dal path (cerca ITA, USA, JPN, English, Italian, ecc. nel percorso).
- [x] Sidecar restituisce anche `generation` (1–9) e opzionale `languageIdRaw` (per debug).
- [x] Binario pubblicato in `src-tauri/binaries/` con nome `parser-x86_64-pc-windows-msvc.exe` (Windows x64).
- [x] Configurati in Tauri: `externalBin`, `tauri-plugin-shell`, permessi `shell:allow-execute` per il sidecar.
- [x] Comando Rust `detect_save_game_version(path)` invoca sidecar, legge stdout, gestisce JSON di errore, restituisce `SaveGameVersion { game, version }`.
- [ ] Contratto esposto al frontend: `invoke('detect_save_game_version', { path })` → `{ game, version }` o errore; coerente con api-contract se definito.

---

## Riferimenti

- Requisito: [core-functionality.md](./core-functionality.md) (Riconoscimento automatico)
- Architettura parser: [parser-architecture.md](./parser-architecture.md)
- Struttura: [project-structure.md](./project-structure.md) (`src-sidecar/`, `src-tauri/binaries/`)
- Standard sidecar C#: [csharp-sidecar-standard.md](../standards/csharp-sidecar-standard.md)
- Standard sidecar Tauri: [tauri2-sidecar-standard.md](../standards/tauri2-sidecar-standard.md)
- Glossario: [glossary.md](./glossary.md) (Save, PKHeX, Parser)
