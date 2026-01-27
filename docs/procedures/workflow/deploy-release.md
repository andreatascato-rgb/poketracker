# Procedure: Build, Deploy e Release

## Obiettivo

Definisce come preparare build di release, bundle installer e pipeline di distribuzione per PokeTracker (Tauri 2).

## Quando Usare Questa Procedure

- Query: "build", "deploy", "release", "distribuisci", "pacchetto", "installer", "tauri build", "crea installer", "release build", "bundle"
- Quando si deve generare una build di release, un installer o configurare/aggiornare la pipeline di release
- Se la query corrisponde, questa procedure è OBBLIGATORIA

## Obbligatorietà

Completamento integrale della checklist obbligatorio prima di proporre implementazione. Nessuna eccezione.

## File da Leggere PRIMA

1. **Build/Deploy Tauri 2**: `docs/standards/tauri2-build-deploy-standard.md:1-80`
   - Comandi build, --no-bundle, formati installer: righe 7-25
   - Pipeline, code signing, updater: righe 26-55

2. **CI/CD**: `docs/standards/ci-cd-standard.md:1-55` — Trigger, quality gates, release flow (se si configura pipeline)

3. **Deployment PokeTracker**: `docs/project/deployment.md:1-100`
   - Target, installer, code signing, canali

4. **Sidecar**: `docs/standards/tauri2-sidecar-standard.md` — Sidecar C# incluso nel bundle

## Checklist Obbligatoria

1. [ ] Leggi `docs/standards/tauri2-build-deploy-standard.md:7-25` — Usa `npm run tauri build`; per bundle specifici `--bundles msi` (Windows) o equivalente
2. [ ] Verifica versione in `tauri.conf.json > version` (o `Cargo.toml`) (`tauri2-build-deploy-standard.md:15-16`)
3. [ ] Se pipeline CI/CD: verificare esempio tauri-action e trigger (branch/tag) (`tauri2-build-deploy-standard.md:26-32`)
4. [ ] Per code signing: riferirsi a `docs/project/deployment.md` e doc Tauri (Windows certificato, macOS notarizzazione, Linux GPG)
5. [ ] Verificare che sidecar C# sia in `src-tauri/binaries/` con nome target-triple prima del build (`tauri2-sidecar-standard.md`)
6. [ ] Se abilitare updater: `plugins > updater` e `bundle > createUpdaterArtifacts` (`tauri2-build-deploy-standard.md:48-52`)

## Riferimenti Standard

- `docs/standards/tauri2-build-deploy-standard.md:1-80` — Build, bundle, release, signing
- `docs/standards/ci-cd-standard.md:1-55` — Pipeline, trigger, quality gates
- `docs/project/deployment.md:1-100` — Strategia PokeTracker
- `docs/standards/tauri2-sidecar-standard.md` — Binari sidecar nel bundle

## Note

- Build MSI solo su Windows; per macOS/Linux usare rispettivamente build su macOS/Linux o CI multi-OS.
- Per "aggiungi dipendenza" usare `dependency-add.md`; per "configura pipeline GitHub" questa procedure + tauri2-build-deploy-standard.
