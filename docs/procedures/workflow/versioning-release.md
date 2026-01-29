# Procedure: Versionamento e Release (Commit/Push con versione)

## Obiettivo

Prima di ogni commit e push che si considera “rilascio versionato”, l’AI determina da sola la versione (ultima in VERSION-HISTORY + modifiche → SemVer), aggiorna lo storico, esegue commit e push. Conferma utente solo se bump MAJOR o tipo di cambio ambiguo.

## Quando Usare Questa Procedura

- Query: "commit e push", "push con versione", "rilascia versione", "versionamento", "decidere versione", "fare release", "scrivere nel commit la versione", "aggiorna changelog", "aggiungi a VERSION-HISTORY", "preparare push con versione"
- Quando si sta per fare push e si vuole associare una versione al commit
- Se la query corrisponde, questa procedure è OBBLIGATORIA

## Obbligatorietà

Completamento integrale della checklist obbligatorio prima di proporre implementazione. Nessuna eccezione.

## Prerequisiti

1. **Standard versioning**: `docs/standards/versioning-standard.md` — schema SemVer, formato commit, dove aggiornare versione e storico
2. **Storico versioni**: `docs/VERSION-HISTORY.md` — formato delle entrate e ultima versione registrata

## Checklist Obbligatoria

1. [ ] **Leggere ultima versione** in `docs/VERSION-HISTORY.md` e **modifiche** (file da includere nel push, diff o contenuto rilevante).
2. [ ] **Decidere bump** (MAJOR / MINOR / PATCH) in base a SemVer: breaking → MAJOR, nuove feature → MINOR, fix/refactor/doc → PATCH. Se MAJOR o ambiguo → proporre versione e chiedere conferma utente; altrimenti procedere in autonomia.
3. [ ] **Scrivere la nuova entrata** in `docs/VERSION-HISTORY.md` in cima a "## Versioni": `### [X.Y.Z] - YYYY-MM-DD`, **Descrizione** (breve), **Change** (elenco puntato).
4. [ ] **Aggiornare i file di versione** se presenti: `src-tauri/tauri.conf.json`, `src-tauri/Cargo.toml`, `package.json` → stesso valore X.Y.Z.
5. [ ] **Eseguire commit e push**: `git add …`, `git commit -m "release: vX.Y.Z - breve descrizione"`, `git push`. Senza chiedere conferma se bump non è MAJOR e non è ambiguo.

## Riferimenti Standard

- `docs/standards/versioning-standard.md` — schema SemVer, formato commit, file di storico, flusso decisionale

## Esempio Completo (autonomo)

**Contesto**: ultima versione in VERSION-HISTORY è `0.1.0`. Modifiche: wizard import + fix caricamento profilo → MINOR (nuova feature + fix retrocompatibili).

**Step 1–2**: bump MINOR → `0.2.0`; non MAJOR né ambiguo → procedere senza chiedere conferma.

**Step 3 – Nuova entrata in `docs/VERSION-HISTORY.md`** (in cima a "## Versioni"):

```markdown
### [0.2.0] - 2026-01-27

#### Descrizione
Wizard per importazione save e miglioramenti al caricamento del profilo.

#### Change
- Nuovo wizard passo-passo per importazione save
- Fix caricamento profilo quando manca file di configurazione
```

**Step 4**: se esistono, aggiornare `tauri.conf.json` e `Cargo.toml` con `"version": "0.2.0"`.

**Step 5**: `git add …`, `git commit -m "release: v0.2.0 - Wizard importazione save e fix caricamento profilo"`, `git push`.

## Note

- Per push “di lavoro” senza considerarlo un rilascio versionato, non serve questa procedure; si può fare commit/push senza aggiornare VERSION-HISTORY né mettere la versione nel messaggio.
- Tag Git (es. `git tag v0.2.0`) sono opzionali; si può introdurre in seguito o in una variante della procedura.
