# Deployment - PokeTracker

## Obiettivo

Definisce la strategia di deployment e distribuzione per l'applicazione PokeTracker.

## Best Practice 2026

### Build Tauri

**Release Build:**
- Build ottimizzato (release mode)
- Code splitting e tree shaking
- Minificazione frontend
- Bundle sidecar C# incluso

**Target Platforms:**
- Windows (x64)
- macOS (x64, ARM64)
- Linux (x64)

### Distribuzione

**Installer:**
- Windows: MSI o NSIS installer
- macOS: DMG con app bundle
- Linux: AppImage o DEB/RPM

**Auto-Updater:**
- Tauri updater integrato
- Controllo versioni automatico
- Download e installazione aggiornamenti
- Rollback se aggiornamento fallisce

### Bundle

**Contenuto Bundle:**
- App Tauri compilata
- Sidecar C# (.exe o binario)
- Database conoscenza iniziale (SQLite)
- Risorse base (sprite, icone)
- Certificati code signing (se disponibili)

**Dimensioni:**
- App base: ~50-100 MB
- Con risorse base: ~100-150 MB
- Con risorse complete: ~500 MB - 1.5 GB (opzionale)

### Code Signing

**Windows:**
- Certificato code signing
- Firma .exe e installer
- Evita warning Windows Defender

**macOS:**
- Certificato Apple Developer
- Notarizzazione app
- Gatekeeper compatibility

**Linux:**
- GPG signature (opzionale)
- Repository signing

### Aggiornamenti

**Strategia:**
- Server per hosting aggiornamenti
- Versioning semantico
- Changelog per ogni versione
- Download incrementali quando possibile

**Notifiche:**
- Notifica utente quando disponibile aggiornamento
- Opzione installazione immediata o differita
- Progress bar durante download

### Distribuzione Canali

**Stable:**
- Release pubbliche
- Testate e stabili
- Aggiornamenti automatici

**Beta:**
- Release pre-stable
- Testing utenti volontari
- Feedback prima di stable

**Dev:**
- Build sviluppo
- Testing interno
- Non distribuite pubblicamente

## Note

Questa strategia permette:
- Distribuzione semplice per utenti
- Aggiornamenti automatici
- Supporto multipli piattaforme
- Professionalità e sicurezza
