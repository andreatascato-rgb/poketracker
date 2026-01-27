# Procedure: Rimuovere Componente, Comando o Risorsa

## Obiettivo

Definisce come rimuovere in modo coerente un componente, un comando, un servizio, una pagina o altra risorsa senza lasciare riferimenti orfani.

## Quando Usare Questa Procedure

- Query: "rimuovi", "elimina", "delete", "togli", "rimuovi componente", "elimina comando", "rimuovi feature", "delete file"
- Quando l'utente chiede esplicitamente di eliminare una parte del codice o una funzionalità
- Se la query corrisponde, questa procedure è OBBLIGATORIA

## Obbligatorietà

Completamento integrale della checklist obbligatorio prima di proporre implementazione. Nessuna eccezione.

## File da Leggere PRIMA

1. **Struttura progetto**: `docs/project/project-structure.md:1-155`
   - Dove sono componenti, store, servizi, comandi, route

2. **INDEX procedure**: `docs/procedures/INDEX.md` — Per capire quali procedure “inverse” possono applicarsi (es. command-creation → rimuovi anche registrazione e invoke)

## Checklist Obbligatoria

1. [ ] Identifica la **risorsa da rimuovere** (file, command, route, servizio, store) e ogni suo **punto di ingresso** (import, registrazione, route, menu)
2. [ ] Cerca **tutti i riferimenti** alla risorsa: grep/ricerca per nome file, nome command, path, export
3. [ ] Rimuovi o aggiorna i riferimenti **prima** di eliminare il file/entry point (evitare build rotti a metà)
4. [ ] Per un **comando Tauri**: rimuovere la funzione, la registrazione in `invoke_handler![]` e tutti i frontend che usano `invoke('nome_command')` o servizi che lo incapsulano
5. [ ] Per un **componente**: rimuovere il file; aggiornare import e utilizzi in layout/pagine/altri componenti
6. [ ] Per una **pagina/route**: rimuovere `+page.svelte` (e `+page.ts` se esiste); aggiornare link e redirect
7. [ ] Per **permission/capability**: rimuovere o restringere l'identifier in `src-tauri/permissions/` e `capabilities/` se la risorsa era l'unica a usarlo
8. [ ] Proporre la rimozione in **un solo step** (lista file da eliminare + modifiche ai riferimenti) per permettere review unitaria

## Riferimenti Standard

- `docs/project/project-structure.md:1-155` — Posizione di componenti, store, servizi, comandi
- Procedure di creazione correlate (component-creation, command-creation, page-creation, service-creation) per sapere cosa “invertire”

## Note

- Non rimuovere codice “a sorpresa”: la richiesta deve essere esplicita (“rimuovi X”, “elimina Y”). In caso di dubbio chiedere conferma.
- Per “refactoring che sposta/rinomina” usare la procedure `refactor.md`; per “rimuovi e basta” usare questa.
