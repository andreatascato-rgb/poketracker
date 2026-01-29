# Procedure: Configurare/Aggiungere Sidecar (es. C# Parser)

## Obiettivo

Definisce come configurare e integrare un sidecar (eseguibile esterno, es. parser C#) in Tauri 2 per PokeTracker.

## Quando Usare Questa Procedura

- Query: "configura sidecar", "aggiungi sidecar", "sidecar C#", "sidecar parser", "configura parser C#", "aggiungi eseguibile esterno", "externalBin", "tauri sidecar"
- Quando si deve aggiungere o modificare un binario esterno invocato da Rust (es. parser PKHeX)
- Se la query corrisponde, questa procedure è OBBLIGATORIA

## Obbligatorietà

Completamento integrale della checklist obbligatorio prima di proporre implementazione. Nessuna eccezione.

## File da Leggere PRIMA

1. **Sidecar Tauri 2**: `docs/standards/tauri2-sidecar-standard.md:1-119`
   - externalBin, target triple, path binari: righe 7-41
   - Shell plugin, invocazione da Rust: righe 42-63
   - Permessi shell:allow-execute: righe 66-92

2. **C# sidecar**: `docs/standards/csharp-sidecar-standard.md:1-80`
   - Nome exe con target triple, path `src-tauri/binaries/`, contratto args/stdout

3. **Struttura progetto**: `docs/project/project-structure.md:156-169`
   - Cartella `src-sidecar/`, output in `src-tauri/binaries/`

## Checklist Obbligatoria

1. [ ] Leggi `docs/standards/tauri2-sidecar-standard.md:7-41` — Aggiungi binario in `bundle > externalBin` (es. `["binaries/parser"]`); path `src-tauri/binaries/<nome>-<target-triple>[.exe]`
2. [ ] Verifica naming target triple: es. Windows x64 = `parser-x86_64-pc-windows-msvc.exe` (`tauri2-sidecar-standard.md:26-35`)
3. [ ] Aggiungi plugin: `tauri-plugin-shell` in Cargo e in `tauri::Builder` (`tauri2-sidecar-standard.md:42-62`)
4. [ ] Configura permessi: `shell:allow-execute` con scope che include il sidecar (`tauri2-sidecar-standard.md:66-92`)
5. [ ] Se sidecar è C#: verifica `docs/standards/csharp-sidecar-standard.md` — .NET 8, PublishSingleFile, output in `src-tauri/binaries/`, nome con target triple; contratto args in / JSON stdout
6. [ ] Invocazione da Rust: `app.shell().sidecar("parser")`, `.args(&["..."])`, `.spawn()`; preferire args/stdout rispetto a stdin (`tauri2-sidecar-standard.md:62-63`)

## Riferimenti Standard

- `docs/standards/tauri2-sidecar-standard.md:1-119` — externalBin, shell plugin, permessi
- `docs/standards/csharp-sidecar-standard.md:1-80` — Build C#, path, contratto
- `docs/project/project-structure.md:156-169` — Struttura src-sidecar

## Note

- Per PokeTracker il sidecar è il parser C# (PKHeX). Coordinare con `docs/project/parser-architecture.md`.
- Cross-compilation: build separati per Windows/Linux/macOS e file con i rispettivi suffissi target triple.
