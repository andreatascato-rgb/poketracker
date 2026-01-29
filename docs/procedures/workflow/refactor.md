# Procedure: Refactoring

## Obiettivo

Definisce come pianificare e eseguire un refactoring senza cambiare il comportamento osservabile dell’app, rispettando standard e struttura.

## Quando Usare Questa Procedura

- Query: "refactor", "refactoring", "ristruttura", "ristrutturazione", "migra codice", "pulizia codice", "estrai componente", "estrai funzione", "rinomina", "sposta file"
- Quando l’obiettivo è migliorare struttura/leggibilità senza aggiungere feature né correggere bug specifici
- Se la query corrisponde, questa procedure è OBBLIGATORIA

## Obbligatorietà

Completamento integrale della checklist obbligatorio prima di proporre implementazione. Nessuna eccezione.

## File da Leggere PRIMA

1. **Struttura progetto**: `docs/project/project-structure.md:1-110`
   - Dove devono stare moduli, componenti, comandi, services

2. **Standard pertinenti** al codice toccato:
   - Frontend: `docs/standards/svelte-sveltekit-standard.md`, `docs/standards/typescript-frontend-standard.md`
   - Backend: `docs/standards/rust-tauri-standard.md`
   - Organizzazione: `docs/standards/file-organization.md`

## Checklist Obbligatoria

1. [ ] Delimita **scope** del refactoring (solo frontend, solo backend, solo un modulo/feature)
2. [ ] Leggi `docs/project/project-structure.md` — Verifica che spostamenti/rinominazioni rispettino cartelle e convenzioni
3. [ ] Per ogni file toccato, applica gli standard indicati in `docs/standards/` (runes, Result, naming, path)
4. [ ] **Non** cambiare API pubbliche (signature comandi, props esposte, contratti invoke) salvo richiesta esplicita
5. [ ] Proponi refactoring in **step incrementali** (es. prima rinominare, poi estrarre) per ridurre rischio e facilitare review
6. [ ] Indica come l’utente può **verificare** che il comportamento non sia cambiato (stessi flussi, stessi test)

## Riferimenti Standard

- `docs/project/project-structure.md:1-110` — Struttura e convenzioni
- `docs/standards/` — Standard per frontend e backend
- `docs/standards/file-organization.md` — Dove va documentazione e codice in docs/

## Note

- Refactoring “grande” (es. migrazione Svelte 4 → 5, cambio architettura) va spezzato in step e possibilmente documentato in una procedure dedicata o in docs/project/.
- Se la richiesta mescola refactoring e nuova feature (o bug fix), applicare prima la procedura corrispondente (new-feature, bug-fix) e trattare il refactoring come passo aggiuntivo esplicito.
