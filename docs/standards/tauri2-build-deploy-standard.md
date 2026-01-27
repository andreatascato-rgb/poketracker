# Standard: Build, Deploy e Release Tauri 2

## Obiettivo

Definisce come buildare, bundle e distribuire l'app Tauri 2 (PokeTracker): comandi, formati installer, pipeline di release e code signing.

## Build

- **Comando base:** `npm run tauri build` (o `pnpm`, `yarn`, `cargo tauri build`).
- **Build senza bundle:** `npm run tauri build -- --no-bundle` — compila app e frontend ma non crea installer; utile per bundling successivo con opzioni specifiche.
- **Bundle separato:** `npm run tauri build -- --bundles msi` (es. solo MSI su Windows); formati: `msi`, `nsis`, `app`, `dmg`, `appimage`, `deb`, `rpm`.
- **Versione:** definire in `tauri.conf.json > version`; se assente, Tauri usa `package.version` da `src-tauri/Cargo.toml`.

## Formati e Piattaforme

- **Windows:** MSI (WiX) o NSIS; MSI solo su Windows (no cross-compile). Default: `.msi` o `.exe` (NSIS).
- **macOS:** `.app` bundle, `.dmg` per distribuzione; per App Store o notarizzazione servono certificati Apple.
- **Linux:** AppImage, DEB, RPM; signing AppImage opzionale (GPG, variabili `SIGN`, `SIGN_KEY`, `APPIMAGETOOL_SIGN_PASSPHRASE`).

## Pipeline di Release

- **tauri-action** (GitHub Actions): checkout, setup Node/Rust, build frontend, `tauri build`, creazione GitHub Release e artifact.
- Trigger tipici: push su branch `release` o tag `app-v*` / `v*`.
- L'action crea tag e release usando la versione dell'app; configurare `tauri.conf.json > plugins > updater` e `bundle > createUpdaterArtifacts` per updater.

## Code Signing

- **Windows:** certificato code signing per .exe e installer; riduce warning Defender/SmartScreen.
- **macOS:** certificato Apple Developer, notarizzazione; richiesto per distribuzione fuori da Mac App Store.
- **Linux:** firma GPG su AppImage/repo opzionale.

## Updater

- Abilitare in `tauri.conf.json`: `plugins > updater`; per generare artifact updater: `bundle > createUpdaterArtifacts: true`.
- Hosting: server o GitHub Release per i file di update; versioning semantico e changelog per release.

## Riferimenti

- [Tauri 2 – Distribute](https://v2.tauri.app/distribute/)
- [Tauri 2 – GitHub pipeline](https://v2.tauri.app/distribute/pipelines/github)
- [Tauri 2 – Linux code signing](https://v2.tauri.app/distribute/sign/linux)
- [Deployment PokeTracker](../project/deployment.md)

## Data Decisione

2026-01-27

## Note

- Per PokeTracker: sidecar C# va incluso nel bundle; path binari in `src-tauri/binaries/` come da [tauri2-sidecar-standard](./tauri2-sidecar-standard.md).
- Dimensioni e contenuto bundle: app + sidecar + DB conoscenza iniziale + risorse base; vedere [deployment.md](../project/deployment.md).
