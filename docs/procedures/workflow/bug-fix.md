# Procedure: Correggere Bug (Bug Fix)

## Obiettivo

Definisce come individuare, correggere e verificare un bug in modo mirato, senza alterare comportamento non richiesto.

## Quando Usare Questa Procedure

- Query: "fix bug", "correggi bug", "risolvi bug", "bug fix", "fix", "correggi", "risolvi", "non funziona", "errore", "fix error", "correggi errore"
- Quando l’utente segnala un comportamento errato o un errore da correggere
- Se la query corrisponde, questa procedure è OBBLIGATORIA

## Obbligatorietà

Completamento integrale della checklist obbligatorio prima di proporre implementazione. Nessuna eccezione.

## File da Leggere PRIMA

1. **Struttura progetto**: `docs/project/project-structure.md:1-50`
   - Dove si colloca il codice probabilmente coinvolto (frontend/backend)

2. **Standard pertinenti**: in base all’area del bug
   - Frontend: `docs/standards/svelte-sveltekit-standard.md`, `docs/standards/typescript-frontend-standard.md`
   - Backend: `docs/standards/rust-tauri-standard.md`

## Checklist Obbligatoria

1. [ ] Identifica il **sintomo** descritto dall’utente (messaggio di errore, comportamento atteso vs attuale)
2. [ ] Individua **file e funzioni** plausibilmente coinvolti (stack trace, log, nomi citati nella query)
3. [ ] Leggi il codice rilevante e gli standard applicabili (`docs/standards/` per quel layer)
4. [ ] Individua la **causa radice** (non solo il sintomo); se non univoca, proporre ipotesi e passi di verifica
5. [ ] Proponi una **modifica minima** che risolve il bug senza cambiare contratto/API o comportamento non richiesto
6. [ ] Verifica che la modifica rispetti gli standard del progetto (error handling, tipi, runes, ecc.)
7. [ ] Indica come l’utente può **verificare** il fix (passi riproducibili o test manuali)

## Riferimenti Standard

- `docs/project/project-structure.md:1-50` — Dove cercare il codice
- `docs/standards/` — Standard per frontend e backend (applicabili in base all’area del bug)

## Note

- Non estendere scope (“approfittiamo per refactorare”): limitare la proposta al fix del bug salvo richiesta esplicita.
- Se il “bug” è in realtà una richiesta di nuova funzionalità o cambio di specifica, proporre di trattarla come feature o conferma esplicita prima di procedere.
