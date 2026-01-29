# Standard: Design System

## Obiettivo

Definisce lo stile visivo e i pattern UI di PokeTracker: moderno, professionale, non infantile né banale. Base per coerenza in tutta l'app desktop 2026 rivolta a player competitivi Pokémon che usano attivamente il PC e conoscono gli strumenti.

**Riferimento visivo:** l'aspetto dell'app deve essere **uguale all'app Poketrack** (progetto sibling di riferimento). Token, dimensioni del layout e scelte visive adottate da Poketrack sono il nostro standard. Elenco adozioni e roadmap: `docs/project/poketrack-reference.md`.

**Implementazione:** token in `src/app.css` (sezione `.dark`), layout shell in `src/routes/+layout.svelte`; componenti con **Tailwind CSS** e **shadcn-svelte**. Vedi `docs/standards/ui-stack-standard.md` per stack e best practice.

## Principi

- **Moderno e professionale** — Aspetto da tool desktop, non da gioco casuale.
- **Niente infantile/banale** — Niente illustrazioni "carine", font giocosi o palette da videogioco; priorità a leggibilità e densità informativa.
- **Suddivisione pulita e chiara** — Sezioni visivamente distinte (header, sidebar, area contenuto), spaziature coerenti, gerarchia evidente.
- **Dark mode primario** — Tema scuro come default; palette pensata per uso prolungato e riduzione affaticamento.
- **Allineamento a Poketrack** — Per layout, colori e dimensioni si segue l'app Poketrack; in dubbio, verificare lì.

## Riferimenti visivi

- **App Poketrack** — Riferimento primario: stessi colori (VS Code style), stessa Top Bar (48px), stessa Sidebar (280px), stessi token, scrollbar nascosta. Vedi `docs/project/poketrack-reference.md`.
- **VS Code** (solo ciò che serve): Activity Bar + Sidebar + Editor + Panel; icone per vista; toolbar essenziali; gerarchia container → item.

## Token e palette (dark mode — standard Poketrack)

I token sotto sono quelli **adottati da Poketrack** (stile VS Code). Vivono in `src/app.css` nella sezione `.dark` e vanno usati per tutta l'UI.

### Sfondi e bordi

| Token | Valore | Uso |
|-------|--------|-----|
| `--bg-primary` | `#1e1e1e` | Sfondo principale (Top Bar, area contenuto, root) |
| `--bg-secondary` | `#252526` | Sidebar |
| `--bg-tertiary` | `#2d2d30` | Card, panel, hover surface |
| `--border-primary` | `#3e3e42` | Bordi, separatori |
| `--text-primary` | `#cccccc` | Testo principale |
| `--text-secondary` | `#858585` | Testo secondario / muted |
| `--font-primary` | `'Segoe UI', 'SF Pro Display', -apple-system, …` | Font UI |
| `--font-size-body` | `13px` | Corpo e voci menu |
| `--spacing-xs` … `--spacing-xl` | 4px, 8px, 12px, 16px, 24px | Spaziature |
| `--hover-bg` | `#2a2d2e` | Hover su voci/pulsanti; sfondo lieve voce corrente sidebar |
| `--pressed-bg` | `#1f2224` | Sfondo `:active` (pressione); istantaneo, no transizione. Vedi `interaction-states-standard.md` |
| `--active-bg` | `#094771` | Stati "selezionato" in liste/dropdown (es. opzione Profilo); **non** usato per voce corrente sidebar |
| `--focus-ring` | `#007acc` | Focus, accent, barra indicatore voce corrente sidebar |
| `--transition-default` | `200ms ease-out` | Transizioni UI (hover, focus, aperture/chiusure dialog, sidebar, voci) |

Mappatura per Tailwind/shadcn: in `.dark` risultano `--background: var(--bg-primary)`, `--foreground: var(--text-primary)`; `--sidebar` e varianti per la sidebar.

### Icone stato

| Token | Valore | Uso |
|-------|--------|-----|
| `--icon-success` | `#4ade80` | Icone stato valido/successo (es. sync attivo, watcher attivo, toast successo) |
| `--icon-edit` | `#7ec8ff` | Icone azioni modifica/primarie (più chiaro del focus-ring per leggibilità su dark) |
| `--icon-destructive` | `#f87171` | Icone azioni distruttive (es. elimina); vivido e riconoscibile su dark |

### Transizioni

- **Token:** `--transition-default: 200ms ease-out` in `src/app.css` (sezione `.dark`). Valore adottato da Poketrack.
- **Regola:** per hover, focus, aperture/chiusure (dialog, sidebar, voci) usare `var(--transition-default)` in stili custom/inline, oppure durata equivalente: Tailwind `duration-200` = 200ms. I componenti UI e il layout devono allinearsi a questo valore per un mood coerente.
- **Stati interattivi (hover, active):** hover con transizione fluida; `:active` **senza** transizione (feedback istantaneo). Dettaglio in `docs/standards/interaction-states-standard.md`.
- **Riduzione movimento:** quando l'utente preferisce meno movimento (`prefers-reduced-motion: reduce`), le transizioni e le animazioni vanno ridotte o disattivate. Implementazione globale in `src/app.css`; dettagli in `docs/standards/accessibility-standard.md`.

### Layout shell (dimensioni — da Poketrack)

| Elemento | Valore | Note |
|----------|--------|------|
| Top Bar | altezza **48px**, min-height 48px | Padding 0 16px (`--spacing-lg`), bordo sotto `--border-primary` |
| Top Bar icon (toolbar) | **32×32 px**, gap **2 px** tra icone | Target ≥24×24 (WCAG 2.2 SC 2.5.8); nessun margin sui figli della toolbar |
| Sidebar | larghezza **220px** in PokeTracker (espansa), **48px** (collassata); standard Poketrack 280px | bg `--bg-secondary`, bordo destro `--border-primary`; header min-height 40px, padding 8px |
| Area contenuto | flex 1, overflow-y auto | bg `--bg-primary`; solo questa zona scrolla |
| Scrollbar | nascosta | In tutta l'app (classe `.poketrack-layout` o equivalente) |

Top Bar e Sidebar **non** scrollano; lo scroll è solo nell'area contenuto.

## Tipografia

- **Font**: `--font-primary` (Segoe UI, SF Pro, system-ui). Evitare font "display" o giocosi.
- **Dimensione corpo**: `--font-size-body` (13px).
- **Gerarchia**: titolo app/sezione in grassetto e dimensione maggiore; voci di menu e corpo in regular.
- **Title case** per titoli e voci di menu (es. "Profilo", "Editor", "Wiki", "Impostazioni").

## Spaziature e layout

- **Padding**: sufficiente intorno a titoli e voci (12–16px per voci di menu); usare `--spacing-*`.
- **Layout a tre parti**: Top Bar | Sidebar | Area contenuto; dimensioni come tabella "Layout shell" sopra. Vedi `docs/procedures/workflow/layout-navigation-change.md`.
- **Allineamento**: elementi di navigazione e liste allineati a sinistra; spaziature verticali uniformi tra voci dello stesso livello.

## Icone

- **Stile**: lineari, minimali, stile "product icons" (VS Code / Material); evitare 3D, cartoon o decorative.
- **Coerenza**: stesso peso (stroke), stessa dimensione base per voci dello stesso tipo.
- **Riuso**: preferire set esistente o libreria coerente; evitare mix di stili.
- **Colore stato**: per icone che indicano stato (successo, modifica, distruttivo) usare i token "Icone stato" sopra: `--icon-success`, `--icon-edit`, `--icon-destructive`.
- **Stili su icone da libreria (es. Lucide)**: le classi sono applicate al **root del component child** (es. l'elemento SVG). Il **CSS scoped** del file che usa l'icona **non** si applica a quell'elemento (lo scope è solo sugli elementi del componente corrente). Per colore, dimensioni o varianti (es. `.status-active`): usare **`:global(.classe)`** nel foglio di stile del parent, oppure wrappare l'icona in un elemento (es. `<span class="...">`) definito nel parent così lo stile scoped si applica. In caso di bug "icona non cambia aspetto pur con stato corretto", verificare per prima cosa l'applicabilità del selettore (v. `docs/procedures/workflow/bug-fix.md` — Bug "UI non riflette lo stato").

## Responsive (layout adattivo)

- **Breakpoint e comportamento:** `docs/standards/responsive-design-standard.md` (mobile-first, breakpoint, unità fluide, touch target).
- **Shell su viewport stretta:** Sidebar collassata (48px) o nascosta con toggle; Area contenuto sempre utilizzabile; Top Bar invariata (48px).
- **Dimensioni fisse** (Top Bar 48px, Sidebar 220px espansa) restano da questo standard; il “quando” collassare o adattare è nello standard responsive.

## Pattern da VS Code / Poketrack

- **Sidebar**: voci raggruppate per funzione; icone + label; al più due livelli (voce → sottovoci); ordine fissato: Home → Allenatore (/profilo) → Editor → Wiki → Impostazioni. **Stati voci:** hover = superficie rialzata (`--bg-tertiary`); voce corrente = barra verticale a sinistra 3px (`--focus-ring`) + sfondo lieve (`--hover-bg`), testo `--text-primary`. Non usare sfondo pieno colorato per la voce corrente (best practice: indicatore chiaro e non distraente).
- **Top Bar**: titolo/azioni essenziali; niente clutter.
- **Area contenuto**: sfondo `--bg-primary`, ben separata dalla sidebar.
- **Stato e feedback**: messaggi chiari (toast, inline); non affidarsi solo al colore (vedi `docs/standards/accessibility-standard.md` e `docs/standards/error-handling-standard.md`).

## Applicazione

- **Nuovi componenti UI**: usare i token sopra (colori, spaziature, transizioni), tipografia e stile icone. Creazione: `docs/procedures/svelte/component-creation.md`.
- **Transizioni:** usare `var(--transition-default)` per hover, focus e aperture/chiusure; rispettare `prefers-reduced-motion` (vedi sotto e `docs/standards/accessibility-standard.md`).
- **Modifiche a layout/navigazione**: rispettare struttura a tre parti, dimensioni Top Bar 48px e Sidebar 220px (PokeTracker), ordine voci. Vedi `docs/procedures/workflow/layout-navigation-change.md`.
- **Accessibilità**: contrasto, focus, semantica e `prefers-reduced-motion` obbligatori; `docs/standards/accessibility-standard.md`.

## Pattern applicati (design e stile consolidati)

Per **empty state, card, CTA, padding area contenuto**, **suddivisione sezioni** (Card/paragrafi/tabelle) e **card di sezione** (stile professionale, Impostazioni/Wiki): `docs/project/ui-patterns-applied.md`. Da consultare prima di introdurre nuovi blocchi UI per restare allineati.

## Procedure e standard collegati

| Azione | Riferimento |
|--------|-------------|
| **Cosa adottiamo da Poketrack** | `docs/project/poketrack-reference.md` |
| **Pattern UI applicati (empty state, CTA, card)** | `docs/project/ui-patterns-applied.md` |
| **Stack UI (Tailwind, shadcn-svelte)** | `docs/standards/ui-stack-standard.md` |
| **Stati interattivi (hover, active)** | `docs/standards/interaction-states-standard.md` |
| **Loading, sync, watcher (UX)** | `docs/project/loading-and-sync-ux.md` |
| Modifica sidebar / layout / navigazione | `docs/procedures/workflow/layout-navigation-change.md` |
| Layout adattivo / breakpoint / responsive | `docs/standards/responsive-design-standard.md`, `docs/procedures/workflow/responsive-design-change.md` |
| Nuovo componente Svelte | `docs/procedures/svelte/component-creation.md` |
| Accessibilità | `docs/standards/accessibility-standard.md` |
| Toast, errori user-facing | `docs/standards/error-handling-standard.md` |

## Data decisione

2026-01-27 (rev. 2026-01-27: adozione standard visivo Poketrack)

## Note

- Il design system va applicato in modo uniforme; in dubbio privilegiare chiarezza e allineamento a Poketrack.
- Per priorità in trade-off si usa `docs/project/priorities.md`; lo stile è parte della funzionalità fruibile (livello 3).
