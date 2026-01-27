# Procedure: Configurare Permissions e Capabilities Tauri

## Obiettivo

Definisce come aggiungere o modificare permissions e capabilities in Tauri 2 per PokeTracker (command, scope, sidecar).

## Quando Usare Questa Procedure

- Query: "aggiungi permesso", "configura permessi", "capability", "permissions", "tauri permissions", "configura capability", "scope", "allow command"
- Quando si deve abilitare un nuovo command, restringere scope o aggiungere capability per una window
- Se la query corrisponde, questa procedure è OBBLIGATORIA

## Obbligatorietà

Completamento integrale della checklist obbligatorio prima di proporre implementazione. Nessuna eccezione.

## File da Leggere PRIMA

1. **Permissions Tauri 2**: `docs/standards/tauri2-permissions-standard.md:1-70`
   - Struttura permission (identifier, commands, scope): righe 7-30
   - Capabilities (windows, permissions): righe 32-50
   - Convenzioni naming: righe 52-58

2. **Sidecar**: `docs/standards/tauri2-sidecar-standard.md:66-92`
   - Permessi shell per sidecar (shell:allow-execute, scope)

## Checklist Obbligatoria

1. [ ] Leggi `docs/standards/tauri2-permissions-standard.md:7-30` — File in `src-tauri/permissions/` (JSON o TOML); identifier, commands, scope
2. [ ] Leggi `docs/standards/tauri2-permissions-standard.md:32-50` — File in `src-tauri/capabilities/`; associare permissions a window (es. `["main"]`)
3. [ ] Per **nuovo command**: aggiungere l'identifier del permission che lo allowa; usare `core:*` o permission custom con `commands.allow = ["nome_command"]`
4. [ ] Per **sidecar**: usare `shell:allow-execute` con scope che includa `name`, `sidecar: true`, `args: true` come in `tauri2-sidecar-standard.md:66-92`
5. [ ] Evitare scope troppo ampi; per path usare `$APPDATA/*`, `$HOME/.config/*` o simile (`tauri2-permissions-standard.md:60-62`)
6. [ ] Verificare che i file in `permissions/` e `capabilities/` siano referenziati o inclusi dal template/tauri.conf

## Riferimenti Standard

- `docs/standards/tauri2-permissions-standard.md:1-70` — Permissions, capabilities, naming
- `docs/standards/tauri2-sidecar-standard.md:66-92` — Permessi shell e sidecar

## Note

- Deny ha priorità su allow; per revocare un command aggiungere un permission con `commands.deny` o non includerlo nelle capabilities della window.
