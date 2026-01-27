# Standard: CI/CD e Pipeline

## Obiettivo

Definisce regole per pipeline di integrazione e release (trigger, quality gates, release flow). Build e bundle Tauri: vedi [tauri2-build-deploy-standard](./tauri2-build-deploy-standard.md); strategia distribuzione: [deployment](../project/deployment.md).

## Pipeline di Release

- **Piattaforma:** GitHub Actions con [tauri-action](https://github.com/tauri-apps/tauri-action) (o equivalente) per build Tauri.
- **Trigger:** push su branch `release` o tag `v*` / `app-v*`; evitare trigger su ogni push a `main` per release pubbliche.
- **Steps tipici:** checkout, setup Node e Rust, install dipendenze, build frontend, `tauri build`, upload artifact, creazione GitHub Release.
- **Versione:** letta da `tauri.conf.json > version`; usare versioning semantico (semver) per release.

## Quality Gates

- **Prima del build di release:** lint (ESLint, rustfmt/clippy, dotnet format per sidecar) e test (unit/integration dove definiti).
- **Fallire su errori:** la pipeline deve fallire se lint o test falliscono; non pubblicare artifact da build rossa.
- **Opzionale:** branch protection su `main`/`release` con required status check per la workflow di release.

## Artifact e Release

- **Artifact:** installer e bundle come da [tauri2-build-deploy-standard](./tauri2-build-deploy-standard.md); sidecar incluso in `src-tauri/binaries/` prima del build.
- **GitHub Release:** creare release con tag e changelog; allegare file di installazione come asset.
- **Updater:** se abilitato, configurare `plugins > updater` e `bundle > createUpdaterArtifacts`; hosting su server o GitHub Release.

## Canali e ambiente

- **Stable:** release pubbliche da tag; testate; vedi [deployment](../project/deployment.md) per canali Stable/Beta/Dev.
- **Secrets:** certificati e token (code signing, notarizzazione) in GitHub Secrets; mai in repo.
- **Multi-OS:** build Windows su runner Windows, macOS su runner macOS, Linux su runner Linux; per matrix usare strategy matrix della workflow.

## Riferimenti

- [tauri2-build-deploy-standard](./tauri2-build-deploy-standard.md) — comandi build, bundle, pipeline Tauri
- [deployment](../project/deployment.md) — strategia, canali, code signing
- [Tauri 2 – Pipelines GitHub](https://v2.tauri.app/distribute/pipelines/github)

## Data Decisione

2026-01-27

## Note

- Per “build”, “deploy”, “release”, “configura CI”, “pipeline” usare [deploy-release](../procedures/workflow/deploy-release.md) e questo standard.
