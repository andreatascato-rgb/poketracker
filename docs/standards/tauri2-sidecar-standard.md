# Standard: Sidecar in Tauri 2

## Obiettivo

Definisce come configurare, invocare e gestire un **sidecar** (eseguibile esterno, es. parser C#) in Tauri 2 per PokeTracker.

## Configurazione in `tauri.conf.json`

- Aggiungere i binari in **`bundle > externalBin`** (array di stringhe).
- I path **relativi** sono risolti a partire da **`src-tauri`**.

Esempio:

```json
{
  "bundle": {
    "externalBin": ["binaries/parser"]
  }
}
```

Per PokeTracker il sidecar del parser può essere `binaries/parser` → i file dovranno stare in `src-tauri/binaries/` con il suffisso della target triple (vedi sotto).

## Naming: Target Triple

Ogni binario deve avere **nome + suffisso della target triple** della piattaforma.

| Piattaforma | Suffisso esempio | File richiesto |
|-------------|------------------|----------------|
| Windows x64 | `-x86_64-pc-windows-msvc` | `parser-x86_64-pc-windows-msvc.exe` |
| Linux x64   | `-x86_64-unknown-linux-gnu` | `parser-x86_64-unknown-linux-gnu` |
| macOS ARM   | `-aarch64-apple-darwin` | `parser-aarch64-apple-darwin` |

- Per conoscere la propria target: `rustc --print host-tuple` (Rust 1.84+).
- Il valore in `externalBin` è **senza** suffisso (es. `binaries/parser`); Tauri aggiunge il suffisso in base al target.

## Dove mettere i binari

- **Path:** `src-tauri/binaries/<nome>-<target-triple>[.exe]`
- Esempio Windows: `src-tauri/binaries/parser-x86_64-pc-windows-msvc.exe`
- In **dev** e in **build** Tauri risolve il path a partire da `src-tauri`; i binari vanno copiati/buildati lì prima di `tauri dev` / `tauri build`.
- Per un sidecar C#: configurare il build .NET in modo che l’output sia in quella cartella con il nome corretto (o copiarlo con uno script post-build).

## Shell Plugin e Invocazione da Rust

- **Dipendenze:** `tauri-plugin-shell` (Cargo e, se usato, `@tauri-apps/plugin-shell` per il frontend).
- **Setup:** `tauri::Builder::default().plugin(tauri_plugin_shell::init())`.
- **Invoco sidecar:**

```rust
use tauri_plugin_shell::ShellExt;

let sidecar = app.shell().sidecar("parser").unwrap();
let (mut rx, mut child) = sidecar.args(&["--arg1", "val"]).spawn().expect("spawn failed");

// Eventi stdout/stderr via rx
while let Some(event) = rx.recv().await {
    // gestire CommandEvent::Stdout(line), Stderr(line), ecc.
}
```

- `sidecar("parser")` si riferisce a `externalBin: ["binaries/parser"]` (usa il nome “parser”, non il path completo).
- **Argomenti:** `.args(&["a", "b"])` prima di `.spawn()`.
- **Stdin:** il plugin shell può esporre modi per scrivere sullo stdin del processo; in alcuni casi è stato segnalato che l’invio via `write` non è affidabile. Preferire, quando possibile, **argomenti** o **comunicazione via stdout** (es. il sidecar legge da args e risponde su stdout in JSON).

## Permessi

- Di default **nessun** comando shell è consentito.
- Servono **permission** con identifier **`shell:allow-execute`** e uno **scope** che includa il sidecar.

Esempio in un file capability (es. `capabilities/default.json` o simile, secondo la struttura del progetto):

```json
{
  "permissions": [
    {
      "identifier": "shell:allow-execute",
      "allow": [
        {
          "name": "parser",
          "sidecar": true,
          "args": true
        }
      ]
    }
  ]
}
```

- **`name`:** deve coincidere con il nome usato in `externalBin` (es. `"parser"` per `binaries/parser`).
- **`sidecar`:** `true` per binari dichiarati in `externalBin`.
- **`args`:** `true` (qualsiasi), `false` (nessuno) oppure array/validator per restringere gli argomenti consentiti.

## Sidecar C# (PokeTracker)

- **Target:** .NET (es. `net8.0`) come console app; output = eseguibile `.exe` su Windows.
- **Nome file:** deve rispettare la target triple, es. `parser-x86_64-pc-windows-msvc.exe` su Windows x64.
- **Contratto consigliato:** input via **argomenti** (path file, opzioni); output via **stdout** (es. JSON); stderr per log/errori. Evitare di dipendere da stdin per dati critici.
- **Build:** post-build o script che copia l’exe da `src-sidecar/bin/Release/net8.0/` (o simile) in `src-tauri/binaries/parser-<target-triple>.exe`.

## Working Directory

- In esecuzione, la working directory del processo Tauri può essere diversa da quella del sidecar; se il sidecar apre file con path relativi, passare path assoluti come argomenti o verificare la cwd quando si fa `spawn`.

## Riferimenti

- [Tauri 2 – Sidecar](https://v2.tauri.app/develop/sidecar/)
- [Tauri 2 – Shell plugin](https://v2.tauri.app/plugin/shell)
- [Tauri 2 – Command scope](https://v2.tauri.app/security/scope/)

## Data Decisione

2026-01-27

## Note

- Per PokeTracker il sidecar è il **parser C#** (PKHeX). Coordinare naming e path con [parser-architecture](../project/parser-architecture.md) e con lo standard C# sidecar quando definito.
- Cross-compilation: in genere si builda il sidecar sulla stessa piattaforma target; per supportare Windows/Mac/Linux servono build separati e file con i rispettivi suffissi target triple.
