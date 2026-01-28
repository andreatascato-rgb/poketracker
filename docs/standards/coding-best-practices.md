# Standard: Best practice di coding (Tauri + Svelte 2026)

## Obiettivo

Raccolta sintetica delle regole di coding che massimizzano correttezza e manutenibilità nel passaggio Rust ↔ Svelte e nel workflow AI. Ogni punto rimanda allo standard di dettaglio; **non** sostituisce gli standard, ma è il checkpoint unico per l’AI prima di implementare.

## 1. Bridge Rust ↔ Svelte (tipi e invoke)

- **Rust:** Ogni tipo usato come parametro o ritorno di un command deve avere `#[derive(serde::Serialize, serde::Deserialize)]`. Dettaglio: [rust-tauri-standard](./rust-tauri-standard.md).
- **TypeScript:** Interfacce/tipi che rispecchiano le struct Rust; **non** usare `any` per argomenti o ritorno di `invoke`. Dettaglio: [typescript-frontend-standard](./typescript-frontend-standard.md).
- **Invoke:** Sempre `import { invoke } from '@tauri-apps/api/core'` e pattern `invoke<'nome_comando', { args }>`. Dettaglio: [versioni-stack-standard](./versioni-stack-standard.md).

## 2. Panic-free (Rust)

- Nei **command** e nel **codice da essi chiamato**: **non** usare `unwrap()` né `expect()`; usare `?` e `.map_err(|e| e.to_string())` (o altro `E: Serialize`) e far risalire l’errore al frontend.
- I command restituiscono **sempre** `Result<T, E>` con `E: Serialize`. Dettaglio: [rust-tauri-standard](./rust-tauri-standard.md).

## 3. Svelte 5: solo UI + invocazioni

- **Runes** per reattività: `$state`, `$derived`, `$effect`; **non** `$:` né `let` reattivo senza runes. Dettaglio: [svelte-sveltekit-standard](./svelte-sveltekit-standard.md), [versioni-stack-standard](./versioni-stack-standard.md).
- **Separazione:** I file `.svelte` contengono solo UI, stato locale (runes) e chiamate a servizi/store; la **logica di business** resta in Rust ed è esposta tramite command. I componenti usano i servizi, non `invoke` sparso. Dettaglio: [architecture-overview](../project/architecture-overview.md), [typescript-frontend-standard](./typescript-frontend-standard.md).

## 4. Least privilege e sicurezza

- **Permissions:** Modello **permissions** (Tauri 2), **non** allowlist; configurare solo i permessi necessari in `src-tauri/capabilities/`. Dettaglio: [tauri2-permissions-standard](./tauri2-permissions-standard.md).
- **No shell dal frontend:** Non esporre al frontend command che eseguono shell/comandi arbitrari; le operazioni di sistema vanno incapsulate in command Rust con parametri validati. Dettaglio: [security-standard](./security-standard.md).

## 5. Workflow operativo (per AI e procedure)

- **Piano prima:** Per feature o modifiche multi-layer, descrivere brevemente le modifiche previste a Rust, Svelte e file di config **prima** di proporre il diff. Dettaglio: procedure [new-feature](../procedures/workflow/new-feature.md).
- **Verifica compilazione:** Dopo modifiche a `src-tauri/`, eseguire `cargo check` (o `cargo build` / `pnpm tauri build`) e considerare il task concluso solo se la build passa. Dettaglio: procedure [project-bootstrap](../procedures/workflow/project-bootstrap.md), [command-creation](../procedures/rust/command-creation.md).
- **No TODO/stub come deliverable:** Non lasciare commenti `// TODO` o funzioni/stub vuoti come risultato consegnato; implementare la logica (anche minimale) o tracciare i task in backlog/issue, non nel codice.
- **Bug fix: cap iterazioni e verifica catena:** Per bug "UI non riflette lo stato" (es. icona che non cambia aspetto), verificare la **catena completa**: (1) stato alla fonte, (2) reattività (sottoscrizione/store), (3) **applicabilità CSS** (se la classe è su un child component, usare `:global()` — v. [bug-fix](../procedures/workflow/bug-fix.md), [design-system](design-system-standard.md) Icone, [ui-stack](ui-stack-standard.md)). **Dopo 2–3 tentativi falliti**: fermarsi, formulare ipotesi, strumentare (log) per validare, correggere solo in base a evidenza. Dettaglio: procedure [bug-fix](../procedures/workflow/bug-fix.md) — sezione "Bug UI non riflette lo stato".

## Riferimenti rapidi

| Tema | Standard |
|------|----------|
| Tipi IPC Rust, Result, no panic | [rust-tauri-standard](./rust-tauri-standard.md) |
| Invoke, tipi TS, servizi | [typescript-frontend-standard](./typescript-frontend-standard.md) |
| Runes, Svelte 5 | [svelte-sveltekit-standard](./svelte-sveltekit-standard.md) |
| Versioni, import, anti-pattern | [versioni-stack-standard](./versioni-stack-standard.md) |
| Permessi, capabilities | [tauri2-permissions-standard](./tauri2-permissions-standard.md) |
| Sicurezza, no shell | [security-standard](./security-standard.md) |

## Data e revisione

- **Data:** 2026-01-27
- **Revisione:** Allineata a standard 2026; aggiornare quando si aggiungono nuove regole o si modificano gli standard referenziati.
