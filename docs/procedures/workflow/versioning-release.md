# Procedure: Versionamento e Release (Commit/Push con versione)

## Obiettivo

Prima di ogni commit e push che si considera “rilascio versionato”, si decide insieme la versione, si aggiorna lo storico e si scrive la versione nel messaggio di commit.

## Quando Usare Questa Procedure

- Query: "commit e push", "push con versione", "rilascia versione", "versionamento", "decidere versione", "fare release", "scrivere nel commit la versione", "aggiorna changelog", "aggiungi a VERSION-HISTORY", "preparare push con versione"
- Quando si sta per fare push e si vuole associare una versione al commit
- Se la query corrisponde, questa procedure è OBBLIGATORIA

## Obbligatorietà

Completamento integrale della checklist obbligatorio prima di proporre implementazione. Nessuna eccezione.

## Prerequisiti

1. **Standard versioning**: `docs/standards/versioning-standard.md` — schema SemVer, formato commit, dove aggiornare versione e storico
2. **Storico versioni**: `docs/VERSION-HISTORY.md` — formato delle entrate e ultima versione registrata

## Checklist Obbligatoria

1. [ ] **Decidere la versione insieme** (utente + AI): in base all’ultima in `docs/VERSION-HISTORY.md`, scegliere bump MAJOR / MINOR / PATCH e valore finale (es. `0.2.0`). L’utente conferma la versione da usare.
2. [ ] **Scrivere la nuova entrata** in `docs/VERSION-HISTORY.md` in cima al file: data odierna, titolo `## [X.Y.Z] - YYYY-MM-DD`, sezione **Descrizione** (breve), sezione **Change** con elenco puntato degli change del commit/PR.
3. [ ] **Aggiornare i file di versione** se presenti: `src-tauri/tauri.conf.json` → `version`, `src-tauri/Cargo.toml` → `version`; se usato, `package.json` → `version`. Coerenza con valore scelto (es. `0.2.0`).
4. [ ] **Proporre messaggio di commit** che includa la versione, es. `release: v0.2.0 - breve descrizione` o `v0.2.0 - breve descrizione`. Aspettare conferma utente prima di eseguire commit.
5. [ ] **Eseguire commit e push** solo dopo conferma esplicita dell’utente sulla versione e sul messaggio.

## Riferimenti Standard

- `docs/standards/versioning-standard.md` — schema SemVer, formato commit, file di storico, flusso decisionale

## Esempio Completo

**Contesto**: ultima versione in VERSION-HISTORY è `0.1.0`. Sono stati aggiunti wizard import e fix al caricamento profilo.

**Step 1 – Decisione**: proposti bump MINOR → `0.2.0`; utente conferma.

**Step 2 – Nuova entrata in `docs/VERSION-HISTORY.md`** (in cima):

```markdown
## [0.2.0] - 2026-01-27

### Descrizione
Wizard per importazione save e miglioramenti al caricamento del profilo.

### Change
- Nuovo wizard passo-passo per importazione save
- Fix caricamento profilo quando manca file di configurazione
```

**Step 3 – File di progetto**: se esistono, in `tauri.conf.json` e `Cargo.toml` impostare `"version": "0.2.0"`.

**Step 4 – Messaggio di commit proposto**:  
`release: v0.2.0 - Wizard importazione save e fix caricamento profilo`

**Step 5 – Dopo ok utente**: eseguire `git add …`, `git commit -m "release: v0.2.0 - …"`, `git push`.

## Note

- Per push “di lavoro” senza considerarlo un rilascio versionato, non serve questa procedure; si può fare commit/push senza aggiornare VERSION-HISTORY né mettere la versione nel messaggio.
- Tag Git (es. `git tag v0.2.0`) sono opzionali; si può introdurre in seguito o in una variante della procedura.
