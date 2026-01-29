# Procedure: Estrai Componente

## Obiettivo

Definisce come estrarre un blocco di markup/logica da un componente Svelte esistente in un nuovo file componente, rispettando runes, cartelle, catalog se diventa primitivo. Più specifica di [refactor](./refactor.md) per il caso “estrai componente”.

## Quando Usare Questa Procedura

- Query: "estrai componente", "estrai in nuovo file", "spezza componente", "estrai blocco in componente", "separa in componente", "estrai in .svelte"
- Quando si deve spostare una parte di un componente Svelte in un nuovo file `.svelte` riutilizzabile
- Se la query corrisponde, questa procedure è OBBLIGATORIA

## Obbligatorietà

Completamento integrale della checklist obbligatorio prima di proporre implementazione. Nessuna eccezione.

## File da Leggere PRIMA

1. **Creazione componente**: `docs/procedures/svelte/component-creation.md` — Dove creare il file, runes, props, convenzioni

2. **Refactoring**: `docs/procedures/workflow/refactor.md` — Scope, step incrementali, non cambiare API pubbliche

3. **Struttura progetto**: `docs/project/project-structure.md:117-135`
   - Cartelle `lib/components/` (dominio, **ui** per primitivi)

## Checklist Obbligatoria

1. [ ] **Individua il blocco** da estrarre (markup, stato, logica) e le **dipendenze** (props da passare, eventi da esporre)
2. [ ] **Destinazione:** usare [component-creation](../svelte/component-creation.md) e la cartella di dominio appropriata (`layout/`, `pokedex/`, `ui/` per primitivi, …). Se il componente estratto è UI visibile, rispettare `docs/standards/ui-stack-standard.md` (Tailwind, `$lib/components/ui`).
3. [ ] **Nuovo file:** creare il file con `$props()` per gli input; spostare stato locale con `$state` / `$derived` / `$effect` come da [svelte-sveltekit-standard](../../standards/svelte-sveltekit-standard.md)
4. [ ] **Sostituire** nel componente originale il blocco con l’uso del nuovo componente (import, passaggio props, binding eventi)
5. [ ] **Import:** aggiornare import nel file di origine; usare alias `$lib` o configurati
6. [ ] **Verifica:** nessun cambio di comportamento osservabile; stessi flussi e dati

## Riferimenti Standard

- `docs/procedures/svelte/component-creation.md` — Dove e come creare componenti
- `docs/standards/ui-stack-standard.md` — Se il componente estratto è UI: Tailwind + shadcn-svelte, componenti da `$lib/components/ui`
- `docs/procedures/workflow/refactor.md` — Refactoring generale
- `docs/project/project-structure.md:117-135` — Cartelle componenti
- `docs/standards/svelte-sveltekit-standard.md` — Runes, props

## Note

- Per refactoring generico (rinomina, sposta file, estrai funzione) usare [refactor](./refactor.md).
- Per “crea componente” da zero (non estrazione) usare [component-creation](../svelte/component-creation.md).
