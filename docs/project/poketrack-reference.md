# Riferimento: app Poketrack

## Obiettivo

Documenta **cosa PokeTracker adotta dall'app Poketrack** (progetto sibling, stesso workspace). Poketrack è il riferimento visivo e tecnico per layout, token e scelte UI; il risultato finale dev'essere coerente (e dove deciso, uguale) a Poketrack.

**Uso:** quando si introducono nuovi elementi UI, layout o stili, verificare qui e in Poketrack se esiste già uno standard da riusare. Per token e layout shell dettagliati: `docs/standards/design-system-standard.md`.

## Adottato (in uso)

| Elemento | Descrizione | Dove in PokeTracker |
|----------|-------------|----------------------|
| **Layout shell** | Top Bar 48px + Sidebar 220px in PokeTracker (48px collassata; Poketrack 280px) + area contenuto flex 1; scroll solo nell'area contenuto | `src/routes/+layout.svelte` |
| **Token colore/sfondo** | `--bg-primary` #1e1e1e, `--bg-secondary` #252526, `--border-primary` #3e3e42, `--text-primary` #cccccc, ecc. (stile VS Code) | `src/app.css` (sezione `.dark`) |
| **Token spaziatura** | `--spacing-xs` … `--spacing-xl` (4px, 8px, 12px, 16px, 24px) | `src/app.css` |
| **Token tipografia** | `--font-primary` (Segoe UI, SF Pro, …), `--font-size-body` 13px | `src/app.css` |
| **Scrollbar nascosta** | Scrollbar nascosta in tutta l'app, scroll funzionante | `src/app.css` (classe `.poketrack-layout`) |
| **Transizioni** | `--transition-default` 200ms ease-out, `--hover-bg`, `--focus-ring` | `src/app.css` |
| **Sidebar hover/attivo** | Hover = superficie rialzata (`--bg-tertiary`); voce corrente = barra sinistra 3px (`--focus-ring`) + sfondo lieve (`--hover-bg`), testo `--text-primary` | `src/routes/+layout.svelte` |

## Da adottare in seguito (roadmap)

Elementi di Poketrack che possiamo introdurre quando serviranno; da verificare in Poketrack prima di implementare.

- **TopBar**: breadcrumb, selettore profilo, struttura header (quando aggiungiamo quelle funzionalità).
- **Sidebar**: eventuali estensioni (nuove voci/sottovoci); stile hover/attivo già adottato (v. tabella "Adottato").
- **Componenti UI**: Button, Card, input, dropdown — stile e token (es. bordi 6px, padding) da allineare a Poketrack dove già usiamo shadcn.
- **Toast / messaggi**: stile e posizionamento se Poketrack ne definisce uno.
- **Empty state / loading**: testo, colori, spaziature.
- **Altro**: qualsiasi pattern visivo o token usato in Poketrack e non ancora in PokeTracker va valutato qui prima di introdurlo.

## Dove guardare in Poketrack

- **Token e global styles**: `src/styles/global.css`
- **Layout principale**: `src/App.svelte` (struttura .app, .app-body, .main-content)
- **TopBar**: `src/lib/components/TopBar.svelte`
- **Sidebar**: `src/lib/components/Sidebar.svelte`, `SidebarGroup.svelte`
- **Button / Card / altri**: `src/lib/components/` (Button.svelte, Card.svelte, …)

(Path tipico: progetto sibling nella stessa workspace, es. `../poketrack/` rispetto a PokeTracker.)

## Riferimenti incrociati

- **Design system (token e dimensioni)** — `docs/standards/design-system-standard.md`
- **Layout e navigazione** — `docs/procedures/workflow/layout-navigation-change.md`
- **Stack UI** — `docs/standards/ui-stack-standard.md`

## Data creazione

2026-01-27
