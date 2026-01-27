# Piano di sviluppo iniziale - PokeTracker

## Obiettivo

Elenco ordinato delle prime azioni da fare per passare dallo stato attuale (solo documentazione) a un'app apribile e navigabile, con profilo e persistenza base. Le priorità seguono `docs/project/priorities.md`: correttezza e dati > performance > funzionalità.

**Stato avanzamento (aggiornato):** Passi 1–4 completati (Bootstrap, Layout+navigazione, Profilo base, DB e persistenza profili). Prossimo: passo 5 (prima pagina contenuto reale).

---

## Piano (5 passi)

### 1. Bootstrap progetto

**Cosa:** Creare l'app Tauri 2 + SvelteKit da zero (scaffold, struttura cartelle, allineamento standard).

**Perché:** Senza questo non c'è codice eseguibile.

**Come:** Procedura `docs/procedures/workflow/project-bootstrap.md`. Standard: `docs/standards/tauri2-standard.md`, `docs/standards/svelte-sveltekit-standard.md`, `docs/project/project-structure.md`.

**Output atteso:** `src-tauri/`, `src/` (frontend SvelteKit), app che si avvia in dev con `npm run tauri dev`.

---

### 2. Layout e navigazione base

**Cosa:** Shell UI con Top Bar + Sidebar + Area contenuto; route/placeholder per Profilo, Editor, Wiki, Impostazioni.

**Perché:** L'utente deve poter navigare tra le sezioni (Profilo, Editor, Wiki, Impostazioni). Layout e terminologia in `docs/project/glossary.md`.

**Come:** Procedure `docs/procedures/workflow/layout-navigation-change.md` e `docs/procedures/svelte/page-creation.md`. Layout in `lib/components/layout/`, route in `src/routes/`.

**Output atteso:** App con tre zone (Top Bar, Sidebar, Area contenuto) e quattro voci di menu che caricano pagine placeholder.

---

### 3. Profilo base (backend + store + UI)

**Cosa:** Comando Rust che espone "lista profili" e "profilo attivo"; store frontend; selettore profilo in Top Bar (anche con un solo profilo placeholder).

**Perché:** Il profilo è il contesto di tutti i dati (core-functionality, multi-profile). Correttezza dati = modelli e API profilo definiti per primi.

**Come:** Procedure `docs/procedures/rust/command-creation.md`, `docs/procedures/svelte/store-setup.md`, `docs/procedures/svelte/component-creation.md`. Standard: `docs/standards/rust-tauri-standard.md`, `docs/standards/typescript-frontend-standard.md`.

**Output atteso:** Un comando (es. `get_profiles` / `get_active_profile`), store che lo usa, selettore in Top Bar che mostra il profilo attivo.

---

### 4. DB e persistenza profili

**Cosa:** Schema SQLite per i profili, migrazione, comandi che leggono/scrivono profili su DB.

**Perché:** I profili devono persistere tra una chiusura e l'altra (priorità correttezza/dati).

**Come:** Procedure `docs/procedures/workflow/db-migration.md`, `docs/procedures/workflow/command-with-db.md`. Standard: `docs/standards/database-migrations-standard.md`.

**Output atteso:** Tabella `profiles`, migrazione applicata, comandi che usano il DB per creare/leggere/attivare profili.

---

### 5. Prima pagina contenuto reale

**Cosa:** Una pagina (es. Profilo o Impostazioni) che usa il layout, mostra dati del profilo attivo e magari un messaggio di stato (es. "Nessuna cartella salvataggi").

**Perché:** Chiude il loop: si apre l'app, si naviga, si vede un contenuto legato ai dati.

**Come:** Procedure `docs/procedures/svelte/page-creation.md`, `docs/procedures/svelte/component-creation.md`. Eventuale servizio frontend che invoca i comandi profilo.

**Output atteso:** Clic su "Profilo" (o "Impostazioni") mostra una schermata con dati del profilo attivo e uno stato minimo.

---

## Cosa non è in questo piano (dopo)

- Sidecar/parser C# e file `.sav` → dopo passo 5, con `docs/procedures/workflow/sidecar-setup.md`.
- Pokedex, Wiki, Editor → feature vere; da pianificare quando layout + profilo + DB base sono solidi.
- Versioning: alla fine del passo 1 (o al primo push) inizializzare `docs/VERSION-HISTORY.md` e versione `0.1.0` come da `docs/standards/versioning-standard.md`.

---

## Ordine e dipendenze

```
1. Bootstrap  →  2. Layout  →  3. Profilo base  →  4. DB profili  →  5. Prima pagina contenuto
       ↑              ↑                ↑                   ↑
       |              |                |                   |
   niente         usa src/         comando + store     comando + DB
   senza 1         da 1             da 2                da 3
```

Ogni passo usa l'output del precedente. Non saltare passi: ad esempio non ha senso il DB profili senza comando e senza layout che lo usano.

---

## Riferimenti

- Priorità: `docs/project/priorities.md`
- Procedure: `docs/procedures/INDEX.md`
- Architettura: `docs/project/architecture-overview.md`
- Core: `docs/project/core-functionality.md`
- Layout/terminologia: `docs/project/glossary.md`
