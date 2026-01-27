# Standard: Sidecar C# (Parser)

## Obiettivo

Definisce target framework, contratto I/O, build e posizionamento del sidecar C# (parser PKHeX) per PokeTracker, in abbinamento a [tauri2-sidecar-standard](./tauri2-sidecar-standard.md).

## Target Framework

- **.NET 8** (`net8.0`): LTS fino a novembre 2026; preferito per stabilità.
- **.NET 9** è STS fino a novembre 2026; usabile se si vogliono le ultime performance/feature.
- Per PokeTracker si adotta **.NET 8** a meno di esigenze specifiche.

## Tipo di Progetto e Publish

- **Tipo:** Console application (`<OutputType>Exe</OutputType>`).
- **Publish single-file** per semplificare il deploy come sidecar:

```xml
<PropertyGroup>
  <TargetFramework>net8.0</TargetFramework>
  <OutputType>Exe</OutputType>
  <RuntimeIdentifier>win-x64</RuntimeIdentifier>
  <PublishSingleFile>true</PublishSingleFile>
  <SelfContained>true</SelfContained>
</PropertyGroup>
```

- **SelfContained:** `true` se il PC di destinazione non deve avere .NET installato; altrimenti `false` e `RuntimeIdentifier` per un exe runtime-specific.
- **RuntimeIdentifier:** `win-x64` per Windows x64; per altri OS usare gli RID appropriati (es. `linux-x64`, `osx-x64`).

## Nome File e Destinazione

- Tauri risolve il sidecar dal nome dichiarato in `externalBin` + **target triple**.
- Per Windows x64 il suffisso è **`-x86_64-pc-windows-msvc`** → il file deve chiamarsi ad es. **`parser-x86_64-pc-windows-msvc.exe`**.
- **Destinazione:** copiare l’exe in `src-tauri/binaries/` (path relativo alla root progetto: `src-tauri/binaries/parser-x86_64-pc-windows-msvc.exe`).
- **Build:** eseguire `dotnet publish -c Release` nel progetto C# e poi copiare l’exe in `src-tauri/binaries/` con il nome richiesto; oppure usare uno script/CI che fa publish + copy.

## Contratto I/O con Rust

Coerente con [parser-architecture](../project/parser-architecture.md): input via **argomenti**, output via **stdout** (JSON).

- **Input:** passare tutto tramite **argomenti** (path del file, tipo di operazione, eventuali opzioni). Evitare stdin per dati critici (il plugin shell Tauri può avere limiti sullo stdin).
- **Output:** un singolo oggetto/array JSON su **stdout**, una riga sola o formato “JSON lines” (una riga = un JSON) se serve streaming.
- **Stderr:** riservato a log/debug; Rust può ignorarlo o inoltrarlo ai log dell’app.
- **Encoding:** usare **UTF-8** su stdout così Rust può decodificare il JSON senza sorprese.

In C# all’avvio:

```csharp
Console.OutputEncoding = System.Text.Encoding.UTF8;
```

E per scrivere la risposta:

```csharp
string json = System.Text.Json.JsonSerializer.Serialize(result);
Console.WriteLine(json);
```

- **Exit code:** `0` = successo; non-zero = errore; Rust può usare l’exit code oltre al corpo della risposta.

## Lettura da stdin (opzionale)

Se in futuro si usasse stdin per comandi aggiuntivi (es. protocollo “comando per riga”):

- `Console.ReadLine()` legge una riga per volta; in un loop blocca e aspetta input (adatto a pipe).
- Per input binario o a chunk usare `Console.OpenStandardInput()` e leggere da quello stream.

Per ora il contratto principale resta **args in, JSON su stdout**.

## Struttura Progetto C#

- Progetto in **`src-sidecar/`** (o nome concordato con [project-structure](../project/project-structure.md)).
- Output di publish: di default in `src-sidecar/bin/Release/net8.0/win-x64/publish/`; l’exe single-file sarà in quella cartella.
- **Post-build / script:**  
  - opzione A: target MSBuild `AfterTargets="Publish"` che copia l’exe in `../src-tauri/binaries/parser-x86_64-pc-windows-msvc.exe` (path relativo alla root del repo);  
  - opzione B: script (PowerShell, Make, ecc.) che lancia `dotnet publish` e poi `Copy-Item`/`cp` verso `src-tauri/binaries/`.

## Riferimenti

- [tauri2-sidecar-standard](./tauri2-sidecar-standard.md) (config Tauri, shell, permessi)
- [parser-architecture](../project/parser-architecture.md) (ruolo del parser e flusso Rust ↔ C#)
- [.NET 8 single-file](https://learn.microsoft.com/en-us/dotnet/core/deploying/single-file/overview)
- [Console.OutputEncoding](https://learn.microsoft.com/en-us/dotnet/api/system.console.outputencoding)

## Data Decisione

2026-01-27

## Note

- Per supportare Windows + Linux + macOS servono build separate e file con i rispettivi suffissi target triple in `src-tauri/binaries/`.
- PKHeX e altre dipendenze C# vanno incluse nel publish (single-file le incorpora se `<SelfContained>true` e se non si usa `PublishTrimmed` in modo da escluderle).
