# Standard: Permissions e Capabilities Tauri 2

## Obiettivo

Definisce come configurare permessi e capabilities in Tauri 2: identifier, scope, comandi, e dove collocare i file di configurazione.

## Modello Permissions

- I **permissions** descrivono privilegi espliciti per command e risorse.
- Controllano quali command sono accessibili dal frontend e possono abilitare, negare o vincolare l'accesso.
- Gli **scope** mappano l'accesso a risorse (path, URL) ai command.

## Struttura di un Permission

- **Identifier:** nome univoco (ASCII lowercase, max 116 caratteri).
- **Description:** documento impatto e scopo (per audit).
- **Commands:** `allow` o `deny` per command specifici.
- **Scopes:** path o risorse con regole `allow`/`deny` (es. `$HOME/*`, `$RESOURCE/*`).

Esempio TOML:

```toml
[[permission]]
identifier = "my-permission"
description = "Access to user files in HOME"
[permission.commands]
allow = ["read_file", "write_file"]
[[permission.scope.allow]]
path = "$HOME/*"
[[permission.scope.deny]]
path = "$HOME/.secret"
```

- File permissions: in **`src-tauri/permissions/`** come `.json` o `.toml`.

## Capabilities

- Le **capabilities** assegnano quali permissions sono attive per quali window/webview.
- File in **`src-tauri/capabilities/`** (es. `default.json` o `default.toml`).
- Struttura tipica:
  - **identifier:** nome della capability.
  - **windows:** lista di window id (es. `["main"]`).
  - **permissions:** lista di identifier di permission da concedere.

Esempio JSON:

```json
{
  "identifier": "main-capability",
  "windows": ["main"],
  "permissions": ["core:path:default", "core:window:allow-set-title", "shell:allow-execute"]
}
```

- I file in `capabilities/` sono referenziati da `tauri.conf.json` (o inclusi per default dal template).

## Convenzioni Naming

- **Core:** prefisso `core:` (es. `core:path:default`, `core:window:allow-set-title`).
- **Plugin:** `nome-plugin:command-name` oppure `nome-plugin:default`; il prefisso `tauri-plugin-` non va scritto nell'identifier.
- **Sidecar:** in genere si usa `shell:allow-execute` con scope che include `name`, `sidecar: true`, `args: true` per il binario sidecar.

## Priorità e Sicurezza

- **Deny** ha priorità su **allow** quando entrambi potrebbero applicarsi.
- Scope di risorse: evitare glob troppo ampi; preferire path sotto `app_data_dir` o cartelle utente esplicite.

## Riferimenti

- [Tauri 2 – Permissions](https://v2.tauri.app/security/permissions)
- [Tauri 2 – Capabilities](https://v2.tauri.app/security/capabilities/)
- [Tauri 2 – Using plugin permissions](https://v2.tauri.app/learn/security/using-plugin-permissions)
- [tauri2-sidecar-standard](./tauri2-sidecar-standard.md) (permessi shell per sidecar)

## Data Decisione

2026-01-27

## Note

- Per PokeTracker: usare capabilities per la main window e per eventuali altre finestre; per il sidecar C# vedi esempio in [tauri2-sidecar-standard](./tauri2-sidecar-standard.md) (shell:allow-execute + scope sidecar).
