# Procedure: Implementare Nuova Feature

## Obiettivo

Definisce come pianificare e implementare una nuova funzionalità in PokeTracker, identificando i layer coinvolti (frontend, backend, entrambi) e applicando le procedure e gli standard pertinenti.

## Quando Usare Questa Procedure

- Query: "nuova feature", "aggiungi feature", "implementa feature", "nuova funzionalità", "aggiungi funzionalità", "implementa funzionalità", "nuova capacità", "aggiungi capacità"
- Quando l’obiettivo è introdurre una nuova capacità utente (non un singolo file tipo “un componente” o “un comando”) che può toccare più parti dell’app
- Se la query corrisponde, questa procedure è OBBLIGATORIA

## Obbligatorietà

Completamento integrale della checklist obbligatorio prima di proporre implementazione. Nessuna eccezione.

## File da Leggere PRIMA

1. **Struttura progetto**: `docs/project/project-structure.md:1-110`
   - Layer frontend (lib/components, stores, routes, services) e backend (commands, services, models, db)

2. **Organizzazione file**: `docs/standards/file-organization.md:1-44`
   - Dove va documentazione e convenzioni

3. **Standard pertinenti**: in base al layer scelto
   - Frontend UI: `docs/standards/svelte-sveltekit-standard.md`, `docs/standards/typescript-frontend-standard.md`, `docs/standards/ui-stack-standard.md` (Tailwind + shadcn-svelte), `docs/standards/design-system-standard.md` (stile: moderno, professionale, dark, no infantile/banale)
   - Comandi/backend: `docs/standards/rust-tauri-standard.md`, `docs/project/project-structure.md` (commands, services)

## Checklist Obbligatoria

1. [ ] Leggi `docs/project/project-structure.md:1-110` — Identifica quali layer tocca la feature (solo frontend, solo backend, entrambi)
2. [ ] **Piano prima di proporre:** Scrivi un breve piano (pseudo-codice o elenco) delle modifiche previste a Rust, Svelte e file di config (tauri, capabilities, ecc.) prima di proporre il diff. Vedi [coding-best-practices](../../standards/coding-best-practices.md) § Workflow.
3. [ ] Per ogni layer (procedure specifiche), individua la procedure specifica in `docs/procedures/INDEX.md`:
   - Nuovi componenti → `docs/procedures/svelte/component-creation.md`
   - Nuovi store → `docs/procedures/svelte/store-setup.md`
   - Nuove pagine/route → `docs/procedures/svelte/page-creation.md`
   - Nuovi comandi Tauri → `docs/procedures/rust/command-creation.md`
   - Integrazione sidecar → `docs/procedures/workflow/sidecar-setup.md`
4. [ ] Applica le procedure identificate nell’ordine logico (es. prima command, poi componente che invoca)
5. [ ] Verifica tutti gli standard referenziati nelle procedure usate
6. [ ] **Verifica compilazione:** Dopo modifiche a `src-tauri/`, esegui `cargo check` (o `cargo build` / `pnpm tauri build`) e considera il task concluso solo se la build passa. Vedi [coding-best-practices](../../standards/coding-best-practices.md) § Workflow.
7. [ ] Se la feature richiede nuovi file in `docs/` (guide, standard): usare `docs/procedures/markdown-creation.md`

## Riferimenti Standard

- `docs/project/project-structure.md:1-110` — Layer e cartelle
- `docs/project/priorities.md` — Ordine priorità in caso di trade-off
- `docs/procedures/INDEX.md` — Mappa query → procedure (component-creation, store-setup, command-creation, sidecar-setup, page-creation)
- `docs/standards/file-organization.md:1-44` — Documentazione in docs/
- `docs/standards/ui-stack-standard.md` — Se la feature tocca UI: Tailwind + shadcn-svelte, componenti da `$lib/components/ui`
- `docs/standards/design-system-standard.md` — Stile UI: moderno, professionale, dark, no infantile/banale

## Note

- Questa procedure non sostituisce le procedure atomiche (component-creation, command-creation, ecc.): le richiama e le combina.
- Se la richiesta è già molto specifica (“aggiungi un comando X”), usare direttamente `command-creation.md` (o la procedure corrispondente) invece di new-feature.
