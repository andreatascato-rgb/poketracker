# Procedure: Integrazione Libreria (Primo Uso)

## Obiettivo

Definisce come integrare una nuova libreria dopo averla aggiunta: dove usarla, convenzioni di import/uso, eventuale wrapper. Estende [dependency-add](./dependency-add.md) con “primo utilizzo” nel codice.

## Quando Usare Questa Procedure

- Query: "integrazione libreria", "integrazione libreria X", "primo uso libreria", "setup libreria", "come usare libreria", "configura libreria", "integra pacchetto"
- Quando si è aggiunta (o si sta per aggiungere) una dipendenza e si deve definire **dove e come** usarla nel progetto
- Se la query corrisponde, questa procedure è OBBLIGATORIA

## Obbligatorietà

Completamento integrale della checklist obbligatorio prima di proporre implementazione. Nessuna eccezione.

## File da Leggere PRIMA

1. **Aggiunta dipendenza**: `docs/procedures/workflow/dependency-add.md` — Layer (npm/Cargo/.NET), file da modificare, versioni

2. **Struttura progetto**: `docs/project/project-structure.md:1-110`
   - Dove vive il codice per frontend (lib/), backend (src-tauri/), sidecar (src-sidecar/)

3. **Architettura**: `docs/project/architecture-overview.md` — Flusso dati, chi chiama chi
   - Frontend non espone logica sensibile; backend per DB/sidecar

4. **Standard del layer**: in base a dove si usa la libreria
   - Frontend: `docs/standards/svelte-sveltekit-standard.md`, `docs/standards/typescript-frontend-standard.md`
   - Backend: `docs/standards/rust-tauri-standard.md`
   - Sidecar: `docs/standards/csharp-sidecar-standard.md`

## Checklist Obbligatoria

1. [ ] **Dipendenze:** se la libreria non è già nel progetto, completare [dependency-add](./dependency-add.md) (npm install / cargo add / dotnet add)
2. [ ] **Layer:** decide **dove** usare la libreria (solo frontend, solo backend, solo sidecar) in base a [architecture-overview](../../project/architecture-overview.md) e [project-structure](../../project/project-structure.md)
3. [ ] **Punto di ingresso:** primo file che importa/usa la libreria (es. componente Svelte, command Rust, servizio C#); rispettare cartelle e convenzioni del layer
4. [ ] **Import / uso:** convenzione di import (path alias `$lib`, crate, namespace) come da standard del layer; evitare import sporchi (es. da node_modules con path lunghi) salvo necessità
5. [ ] **Wrapper (se utile):** se la libreria va riusata in molti punti o va isolata (es. per mock/test), incapsulare in un modulo/servizio (es. `lib/utils/foo.ts`, `services/foo_service.rs`) e usare solo quello
6. [ ] **Config:** se la libreria richiede config (env, file, inizializzazione), metterla dove il progetto già gestisce config (es. build, env, `tauri.conf`) e documentare in commento o in docs
7. [ ] **Standard:** rispettare [svelte-sveltekit-standard](../../standards/svelte-sveltekit-standard.md), [typescript-frontend-standard](../../standards/typescript-frontend-standard.md), [rust-tauri-standard](../../standards/rust-tauri-standard.md) o [csharp-sidecar-standard](../../standards/csharp-sidecar-standard.md) a seconda del layer

## Riferimenti Standard

- `docs/procedures/workflow/dependency-add.md` — Aggiunta dipendenza
- `docs/project/project-structure.md` — Dove mettere codice
- `docs/project/architecture-overview.md` — Flusso e layer
- `docs/standards/` — Standard per frontend, backend, sidecar

## Note

- Per “aggiungi dipendenza” senza integrare nel codice usare solo [dependency-add](./dependency-add.md).
- Per “nuova feature” che richiede una libreria usare [new-feature](./new-feature.md) e poi questa procedure (o dependency-add + questa) come step.
