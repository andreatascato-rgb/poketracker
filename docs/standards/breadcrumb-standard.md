# Standard: Breadcrumb

## Obiettivo

Definisce come implementare il breadcrumb in PokeTracker: posizione (alto a sinistra), stile (moderno, professionale, elegante), interazione (toccabile, tastiera, focus) e accessibilità. Coerente con design system e shell (Top Bar 48px, token Poketrack).

**Riferimenti:** `docs/standards/design-system-standard.md`, `docs/standards/interaction-states-standard.md`, `docs/standards/responsive-design-standard.md`, `docs/standards/accessibility-standard.md`.

## Principi

- **Alto a sinistra** — Breadcrumb in Top Bar, lato sinistro, come primo elemento di contesto (dove oggi c’è titolo pagina). Alternativa: prima riga area contenuto, allineato a sinistra.
- **Stile sobrio** — Lineare, minimal, niente decorazioni; separatori discreti; testo e link con token esistenti.
- **Toccabile e usabile** — Ogni voce cliccabile con area minima 44×44px su viewport touch; hover e active coerenti con stati interattivi.
- **Accessibile** — Navigazione semantica (`nav`, `aria-label`), voce corrente (`aria-current="page"`), focus visibile, ordine tab logico.

## Posizione

| Collocazione | Uso | Note |
|---------------|-----|------|
| **Top Bar sinistra** (predefinito) | Sostituisce o affianca il titolo pagina nella zona `top-bar-left` | Coerente con "in alto a sinistra"; altezza Top Bar 48px, padding 0 16px |
| Prima riga area contenuto | Opzionale: prima riga di `main-content-inner`, allineata a sinistra | Usare se si vuole lasciare la Top Bar solo a titolo sintetico + azioni |

Il breadcrumb **non** invade la zona azioni (toolbar) a destra della Top Bar.

## Struttura e markup

- **Contenitore:** `<nav aria-label="Breadcrumb">` (o equivalente i18n).
- **Elenco:** lista di link/span; voce corrente **non** link, con `aria-current="page"`.
- **Separatore:** carattere o icona discreta (es. chevron, slash) tra le voci; `aria-hidden="true"` sul separatore.
- **Gerarchia:** primo elemento = nome profilo attivo ("l'inizio"), **mai link** (non porta da nessuna parte); livelli intermedi = link; ultimo = pagina corrente, non link.

Esempio semantico (PokeTracker):

```html
<nav aria-label="Breadcrumb">
  <ol>
    <li>Default</li>
    <li><span aria-hidden="true">/</span></li>
    <li aria-current="page">Allenatore</li>
  </ol>
</nav>
```

## Token e stile visivo

Allineamento a `docs/standards/design-system-standard.md`:

| Elemento | Token / valore | Uso |
|----------|----------------|-----|
| Testo link (sezioni non correnti) | `--text-secondary` | Voci navigabili; "bianco spento" per gerarchia visibile |
| Testo link hover | `--text-primary` | Enfasi al passaggio |
| Testo voce corrente | `--text-primary` + font-medium | Solo ultima voce; non link |
| Voce non corrente senza link (es. nome profilo) | `--text-secondary` | Quando ci sono altre voci dopo |
| Separatore | `--text-secondary` | Discreto, piccolo (es. 12px o icona 14px) |
| Font | `--font-primary`, `--font-size-body` (13px) | Coerenza con Top Bar e sidebar |
| Hover link | `--hover-bg` | Sfondo sottile su voce cliccabile; transizione `--transition-default` |
| Active link | `--pressed-bg` | Sfondo `:active`; nessuna transizione (istantaneo) |
| Focus | `--focus-ring` | Anello focus visibile per tastiera |

Niente colori vivaci o font display; aspetto da tool desktop, non da gioco.

## Interazione

- **Link:** ogni voce intermedia è un `<a href="...">` (navigazione SvelteKit).
- **Hover:** solo su link; sfondo `--hover-bg`, transizione `--transition-default` (vedi `docs/standards/interaction-states-standard.md`).
- **Active:** sfondo `--pressed-bg`, `transition: none` su `:active`.
- **Focus:** anello focus visibile; ordine tab coerente con lettura (da sinistra a destra).
- **Niente `transform`/`scale`** su voci con testo (evitare sfocatura).

## Touch e target minimo

Da `docs/standards/responsive-design-standard.md`:

- **Area cliccabile minima:** 44×44px per ogni voce interattiva (link).
- In Tailwind: `min-h-11 min-w-11` (44px) sul link, oppure padding sufficiente (es. `py-2 px-3` con altezza totale ≥ 44px).
- Distanza adeguata tra voci per evitare tap accidentali; separatore non cliccabile non deve ridurre l’area del link adiacente.

## Accessibilità

- **Landmark:** `nav` con `aria-label="Breadcrumb"` (o chiave i18n).
- **Voce corrente:** `aria-current="page"` sull’ultimo elemento (non link).
- **Screen reader:** separatori nascosti con `aria-hidden="true"`; il percorso è annunciato dalla struttura (link + pagina corrente).
- **Tastiera:** tutti i link raggiungibili con Tab; attivazione con Enter.
- **Riduzione movimento:** rispettare `prefers-reduced-motion` (transizioni già gestite in `src/app.css`).

Vedi `docs/standards/accessibility-standard.md` per contrasto e focus.

## Implementazione consigliata

- **Componente riutilizzabile:** es. `Breadcrumb.svelte` in `$lib/components/ui/breadcrumb/` (o `layout/` se legato alla shell).
- **Props:** `items: { label: string; href?: string }[]`; primo = nome profilo (senza `href`, mai link); ultimo senza `href` = pagina corrente.
- **Layout Top Bar:** il componente va inserito in `top-bar-left`; eventuale titolo sintetico (solo ultima voce) può restare come fallback per pagine senza breadcrumb.
- **Stile:** classi Tailwind con token da `app.css` (es. `text-[var(--text-primary)]`, `hover:bg-[var(--hover-bg)]`); usare `cn()` per varianti.

## Riferimenti

| Documento | Uso |
|-----------|-----|
| [design-system-standard.md](./design-system-standard.md) | Token, Top Bar 48px, tipografia, palette |
| [interaction-states-standard.md](./interaction-states-standard.md) | Hover (transizione), active (istantaneo), no scale |
| [responsive-design-standard.md](./responsive-design-standard.md) | Touch target 44×44px, breakpoint |
| [accessibility-standard.md](./accessibility-standard.md) | Focus, ARIA, contrasto, reduced-motion |
| [ui-stack-standard.md](./ui-stack-standard.md) | Tailwind, shadcn, `cn()` |

## Data decisione

2026-01-28

## Note

- Il breadcrumb va usato dove la gerarchia di navigazione (es. Home > Allenatore > Dashboard) aggiunge valore; su pagine a livello singolo (es. Editor, Wiki) si può mostrare una sola voce o il titolo pagina esistente.
- Per priorità in caso di conflitto: `docs/project/priorities.md`.
