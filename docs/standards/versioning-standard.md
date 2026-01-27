# Standard: Versionamento

## Obiettivo

Definisce come assegnare e registrare le versioni a ogni push: schema di numerazione, dove aggiornare la versione, formato del messaggio di commit e uso del file di storico.

## Schema di Numerazione (SemVer)

Usare **Semantic Versioning** `MAJOR.MINOR.PATCH`:

| Tipo | Quando aumentare | Esempio |
|------|------------------|---------|
| **MAJOR** | Breaking change, incompatibilità con versioni precedenti | 1.0.0 → 2.0.0 |
| **MINOR** | Nuove funzionalità retrocompatibili | 1.2.0 → 1.3.0 |
| **PATCH** | Bug fix, piccoli fix, refactor senza cambio contratto | 1.2.3 → 1.2.4 |

- All’avvio progetto: `0.1.0` (o `0.0.1` se solo doc/setup).
- Versione `1.0.0` quando l’app è considerata stabile per uso reale.

## Dove Aggiornare la Versione

Aggiornare la versione in tutti i punti usati da build e release:

- **Tauri**: `src-tauri/tauri.conf.json` → `version`
- **Rust**: `src-tauri/Cargo.toml` → `version` (deve coincidere con tauri.conf dove usato)
- **Frontend** (se usato da CI/package): `package.json` → `version` (opzionale, allineare se si pubblica npm)

Se un file non esiste ancora (es. progetto solo in docs), aggiornare solo i file presenti e annotare nello storico che la versione è “decisa” per il commit.

## File di Storico Versioni

- **Path**: `docs/VERSION-HISTORY.md`
- **Contenuto**: una sezione per ogni versione pushata, con data, descrizione breve e elenco di change.
- **Ordinamento**: dalla più recente in alto (top-down).
- **Quando aggiornare**: prima di commit/push, come step della procedura di versionamento.

Formato di ogni entrata (dettaglio in procedura):

```markdown
## [X.Y.Z] - YYYY-MM-DD

### Descrizione
Breve testo che riassume la release.

### Change
- Change 1
- Change 2
```

## Formato Messaggio di Commit con Versione

Per commit che corrispondono a un “rilascio” versionato:

- Includere la versione nel messaggio.
- Formato consigliato:  
  `release: vX.Y.Z - breve descrizione`  
  oppure  
  `vX.Y.Z - breve descrizione`

Esempi:

- `release: v0.2.0 - Wizard importazione save`
- `v0.1.1 - Fix caricamento profilo`

Per commit intermedi (WIP, fix minori non ancora “rilasciati”), non è obbligatorio mettere la versione nel messaggio; la versione si assegna al commit di push concordato.

## Flusso Decisionale

1. **Decisore (utente)**: decide la versione da dare al prossimo push (MAJOR/MINOR/PATCH rispetto all’ultima).
2. **Esecuzione (AI/developer)**: aggiorna `docs/VERSION-HISTORY.md`, eventualmente `tauri.conf.json` / `Cargo.toml` / `package.json`, e propone messaggio di commit con versione.
3. **Commit e push**: si fa con la versione concordata e scritta nel messaggio.

Non effettuare push “versionati” senza aver aggiornato `docs/VERSION-HISTORY.md` e senza aver indicato la versione nel commit.

## Riferimenti

- Procedura operativa: `docs/procedures/workflow/versioning-release.md`
- Storico versioni: `docs/VERSION-HISTORY.md`
- Build/Release (quando applicabile): `docs/standards/tauri2-build-deploy-standard.md`, `docs/procedures/workflow/deploy-release.md`

## Data Decisione

2026-01-27

## Note

- Lo storico in `docs/VERSION-HISTORY.md` è la fonte di verità per “cosa conteneva ogni versione pushata”.
- Il tag Git (es. `v0.1.0`) è opzionale ma consigliato per rilasci importanti; può essere aggiunto dalla procedura o in un secondo momento.
