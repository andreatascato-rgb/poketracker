# Standard: Stack UI (Tailwind + shadcn-svelte)

## Obiettivo

Definisce lo stack e le best practice per implementare l’UI/UX di PokeTracker: **Tailwind CSS** + **shadcn-svelte** (su **bits-ui**). Senza questo tipo di strumenti, UI/UX di successo richiederebbero scrivere e mantenere tutto a mano (token, componenti accessibili, layout, dark mode), con più tempo, più rischi di incoerenza e più carico di manutenzione.

**Obbligatorietà:** quando si crea o modifica una **parte UI/UX** (componente visibile, layout, pagina, form, pulsante, sidebar, ecc.) si **deve** usare questo stack: Tailwind per stili e token, componenti da `$lib/components/ui` quando disponibili (Button, Input, Card, ecc.), e `docs/standards/design-system-standard.md` per principi visivi. Non introdurre CSS custom o componenti UI da zero al posto di Tailwind/shadcn.

## Perché Tailwind + shadcn (o equivalente)

- **Velocità e coerenza** — Utility-first (Tailwind) e componenti predefiniti (shadcn) riducono tempo di sviluppo e mantengono consistenza visiva senza inventare tutto da zero.
- **Meno CSS custom** — Tailwind sostituisce gran parte dei file CSS custom; i token vivono in config/theme, non sparsi in fogli di stile.
- **Componenti in codebase** — shadcn non è una dependency “scatola nera”: i componenti si copiano nel progetto (`$lib/components/ui`), si modificano in sede, ownership completo.
- **Accessibilità by default** — bits-ui (base di shadcn-svelte) offre primitivi headless con focus, keyboard, ARIA; evita di reinventare a11y per ogni widget.
- **Design system solido** — Token (colori, spaziature, tipografia) centralizzati in Tailwind; theming e dark mode gestibili in un solo posto. Vedi `docs/standards/design-system-standard.md` per principi visivi.

**Senza Tailwind/shadcn:** si possono ottenere ottimi risultati con CSS custom e componenti propri, ma il costo è più alto (design tokens, layout, primitivi accessibili, manutenzione) e va messo in conto esplicitamente.

## Stack scelto (implementato)

| Livello | Tool | Uso |
|--------|------|-----|
| **Utility / token** | **Tailwind CSS** | Classi utility, theme (colori, spacing, font), dark mode via `dark:`; config in Vite (`@tailwindcss/vite`), token in `src/app.css` |
| **Componenti** | **shadcn-svelte** | Button, Input, Card, ecc. in `src/lib/components/ui/`; aggiunti via `npx shadcn-svelte@latest add <nome>` |
| **Primitivi headless** | **bits-ui** | Dependency di shadcn-svelte; accessibilità, comportamento, nessuno stile imposto |

**Componenti UI già presenti:** `$lib/components/ui/button`, `$lib/components/ui/input`, `$lib/components/ui/card`. Per altri usare `npx shadcn-svelte@latest add <nome>`. Helper `cn()` in `$lib/utils.ts`.

- **Svelte 5**: shadcn-svelte e bits-ui supportano Svelte 5; usare le API aggiornate (snippet invece di `let:`, `ref` invece di `el` dove indicato dalla doc).
- **SvelteKit**: setup ufficiale shadcn-svelte per SvelteKit; Tailwind integrato nel build.
- **Design system**: i token visivi (palette, spacing, font, dimensioni layout) definiti in `docs/standards/design-system-standard.md` si implementano tramite variabili CSS in `src/app.css` (sezione `.dark`), in linea con lo **standard Poketrack**. Layout shell: Top Bar 48px, Sidebar 280px; vedi anche `docs/project/poketrack-reference.md`.

## Best practice

### Organizzazione

- **Componenti UI riutilizzabili** in `src/lib/components/ui/` (o sotto-cartelle tipo `ui/button`, `ui/dialog`): qui finiscono i componenti aggiunti via CLI shadcn-svelte e quelli costruiti su bits-ui.
- **Layout** (TopBar, Sidebar, ContentArea) in `src/lib/components/layout/`; possono usare componenti da `ui/` e classi Tailwind.
- **Un solo posto per il tema**: Tailwind config + eventuale `app.css` per variabili globali; evitare valori magici ripetuti nei componenti.
- **`cn()` (classnames)** per concatenare classi Tailwind e varianti; usare l’helper fornito da shadcn-svelte dove presente.

### Token e theme

- **Colori e spacing** nel theme Tailwind (o in CSS vars referenziati dal theme); naming semantico (`--background`, `--foreground`, `--muted`, ecc.) in linea con il design-system.
- **Dark mode**: preferire `class` strategy (es. `dark` su `html`) e classi `dark:...`; palette dark come default coerente con `docs/standards/design-system-standard.md`.

### Componenti

- **Preferire componenti shadcn-svelte** quando coprono il caso (button, input, select, dialog, dropdown, sidebar, ecc.); aggiungerli via CLI e adattarli allo stile progetto.
- **Estendere, non modificare alla cieca**: partire dal componente shadcn, estendere con classi/varianti; evitare fork pesanti che poi non si riescono a aggiornare.
- **Composizione** preferita a ereditarietà o wrapper molto “spessi”; tenere i componenti UI presentazionali e la logica nei livelli superiori (pagine, layout).

### Accessibilità

- **Non disattivare comportamenti bits-ui** (focus, keyboard, ARIA) senza motivo; per personalizzazioni che toccano a11y, verificare `docs/standards/accessibility-standard.md`.
- **Contrasto e focus** restano obbligatori; il tema Tailwind/shadcn va tarato sui valori richiesti (WCAG AA).

### Performance e bundle

- **Import solo ciò che serve**: i componenti shadcn-svelte sono nel codebase, quindi tree-shaking dipende da come li importi; evitare barrel file che importano tutto l’`ui/`.
- **Lazy load** per route o modali pesanti solo dove serve; non è sostitutivo di una struttura componenti snella.

## Anti-pattern da evitare

- **Modificare a mano i file node_modules** — I componenti UI devono stare in `src/lib/components/ui/`, non in dipendenze.
- **CSS custom quando Tailwind basta** — Preferire utility Tailwind; se serve ripetizione, usare `@apply` in layer dedicati o estrarre un componente.
- **Ignorare a11y** — bits-ui e shadcn danno base solida; non rimuovere ruoli, focus trap o label per “semplificare”.
- **Token sparsi** — Nessun colore o spacing “magico” nei componenti; usare sempre token del theme o variabili del design-system.
- **Mix di convenzioni** — Se si usa Tailwind per layout/colori, non introdurre un altro sistema di utility (es. altro framework CSS) nello stesso layer UI.

## Riferimenti

- **Design system (token, dimensioni, standard Poketrack)**: `docs/standards/design-system-standard.md`
- **Cosa adottiamo da Poketrack**: `docs/project/poketrack-reference.md`
- **Accessibilità**: `docs/standards/accessibility-standard.md`
- **Creazione componenti**: `docs/procedures/svelte/component-creation.md`
- **Layout / navigazione**: `docs/procedures/workflow/layout-navigation-change.md`
- **shadcn-svelte**: [shadcn-svelte.com](https://shadcn-svelte.com) — installazione SvelteKit, CLI, componenti
- **bits-ui**: [bits-ui.com](https://bits-ui.com) — primitivi, Svelte 5 migration
- **Tailwind**: [tailwindcss.com](https://tailwindcss.com) — config, theme, dark mode

## Data decisione

2026-01-27

## Note

- Lo stack va **implementato** (Tailwind + shadcn-svelte aggiunti al progetto) prima di usarlo in tutti i nuovi componenti UI; fino ad allora, i componenti esistenti restano così come sono e le nuove UI vanno allineate allo stack appena pronto.
- In caso di conflitto con altri standard (es. performance, bundle size), applicare `docs/project/priorities.md`; lo stack UI serve a **ridurre** rischio e costi, non a introdurre complessità inutile.
