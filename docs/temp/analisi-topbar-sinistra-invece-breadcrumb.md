# Analisi: cosa mettere in alto a sinistra al posto del breadcrumb

## Contesto

Con **solo sezione/sottosezione** (es. Profilo > Allenatore > Dashboard, o Profilo > Editor) il breadcrumb è **completamente inutile**: ripete ciò che la sidebar già evidenzia e non aggiunge livelli di profondità reali. Serve capire cosa mostrare in quella zona (top bar sinistra) per essere utili e in linea con lo stile.

## Situazione attuale in PokeTracker

- **Top bar sinistra:** breadcrumb (nome profilo + sezione + eventuale sottosezione).
- **Sidebar:** sezioni e sottosezioni con voce attiva evidenziata.
- **Area contenuto:** titolo di sezione in CardHeader (es. "Salvataggi", "Profili", "Errori").

Se mettiamo **titoli di sezione** in top bar sinistra, li avremmo **3 volte** (top bar, sidebar, titolo sezione nel contenuto) → **inutile**.

## Best practice (Nielsen Norman, UX pattern, app moderne)

### Quando il breadcrumb non serve

- **Gerarchia piatta o poco profonda** — poche livelli, navigazione già chiara.
- **La gerarchia è già evidente** — es. sidebar con sezioni/sottosezioni evidenziate.
- **Sito piccolo o pochi livelli** — il breadcrumb non aggiunge orientamento.

Fonte: [NNG – Breadcrumbs](https://www.nngroup.com/articles/breadcrumbs/), [UX Patterns – Breadcrumb](https://uxpatterns.dev/en/patterns/navigation/breadcrumb).

### Cosa mettere nell’header / top bar sinistra

Principi ricorrenti:

1. **Titolo di pagina (page title)** — “Dove sono ora” in modo diretto. Usato come indicatore principale di contesto quando non serve trail.
2. **Nome app + titolo pagina** — es. “PokeTracker · Dashboard”: branding + contesto in poco spazio.
3. **Contesto corrente** — es. “Profilo: Default”, “Allenatore · Dashboard”; una sola riga che riassume il contesto senza ripetere la sidebar.

**VS Code:** nella title bar non c’è breadcrumb; c’è il titolo del documento/editor e la sidebar fornisce il contesto di vista.

**App tipo Slack / Linear / Figma:** di solito a sinistra c’è logo/nome + titolo del workspace/canale/vista corrente, non un trail di breadcrumb a meno che non ci siano molti livelli (es. cartelle/file).

### Regola pratica

- **Pochi livelli (1–2) o gerarchia già chiara dalla nav:** in alto a sinistra **solo titolo pagina** (o “App · Titolo”).
- **Molti livelli (3+) e possibili atterraggi profondi (link esterni, search):** breadcrumb ha senso; può stare sopra il titolo o nella top bar a seconda del layout.

In PokeTracker abbiamo al massimo 3 livelli; la sidebar e il titolo di sezione nel contenuto comunicano già dove siamo. **Non** mettere titoli di sezione in top bar: sarebbero ripetuti **3 volte** (top bar, sidebar, CardHeader) → **inutile**. 
## Cosa NON mettere in top bar sinistra

- **Titoli di sezione / titolo pagina** — già in sidebar (voce attiva) e in area contenuto (CardHeader). Ripeterli in top bar = 3 volte = inutile.
- **Breadcrumb** (profilo + sezione + sottosezione) — stesso contenuto di sidebar + titolo sezione; ridondante.

## Opzioni per la top bar sinistra

| Opzione | Descrizione | Pro | Contro |
|--------|-------------|-----|--------|
| **A. Solo nome app** | Es. “Dashboard”, “Allenatore · Salvataggi”, “Impostazioni · Profili” | Chiaro, minimale, coerente con best practice | Niente nome app in barra (resta in sidebar/altrove) |
| **B. Nome app · titolo** | Es. “PokeTracker · Dashboard” | Branding + contesto, pattern comune | Più lungo; “PokeTracker” può essere ridondante se già in sidebar |
| **C. Profilo · titolo** | Es. “Default · Dashboard”, “Default · Impostazioni · Profili” | Contesto profilo sempre visibile | Può diventare lungo; profilo già in sidebar |
| **D. Titolo contestuale breve** | Es. “Dashboard”, “Salvataggi”, “Profili” (solo ultimo segmento) | Molto pulito | Perde contesto sezione (es. “Profili” senza “Impostazioni”) |

**Raccomandazione:** **A (solo nome app)** o **B (vuoto)**. La top bar sinistra non ripete sezione/titolo; il contesto resta in sidebar + titolo sezione nel contenuto. “Allenatore · Dashboard”, “Impostazioni · Profili”). Coerente con NNG e con app moderne: un solo blocco “dove sono” senza ripetere la struttura della sidebar.

## Allineamento con lo stile attuale

- **Token:** `--text-primary` per il titolo; eventuale parte secondaria con `--text-secondary` se si vuole gerarchia visiva (es. “Allenatore” spento, “Dashboard” pieno).
- **Tipografia:** `--font-primary`, `--font-size-body` o `--font-size-ui-primary`; titolo principale in medium se serve.
- **Spazio:** zona `top-bar-left` invariata (padding, allineamento); niente breadcrumb = più spazio per titolo lungo senza wrap eccessivo.

Nessun cambiamento di layout shell o token; solo contenuto della zona sinistra.

## Cosa fare in pratica

1. **Togliere il breadcrumb** dalla top bar sinistra (non sostituirlo con titolo sezione: sarebbe 3× ridondante).
2. **Mettere in top bar sinistra** solo nome app ("PokeTracker") oppure lasciare vuoto; nessun titolo di sezione.
3. **Aggiornare** `docs/standards/breadcrumb-standard.md`: indicare che il breadcrumb in top bar non è usato quando la gerarchia è solo sezione/sottosezione; prevedere uso futuro solo in caso di gerarchia più profonda (es. sottopagine Wiki, categorie multiple).
4. **Opzionale:** riusare il componente `Breadcrumb` in altre zone (es. sopra il contenuto di una vista “profonda”) se in futuro servisse un trail reale.

## Riferimenti

- [NNG – Breadcrumbs: 11 Design Guidelines](https://www.nngroup.com/articles/breadcrumbs/)
- [UX Stack Exchange – Breadcrumbs and Page Titles](https://ux.stackexchange.com/questions/90074/breadcrumbs-and-page-titles)
- [UX Stack Exchange – App title or page title in title bar](https://ux.stackexchange.com/questions/98250/app-title-or-page-title-in-the-title-bar-of-an-app)
- Design system e Top Bar: `docs/standards/design-system-standard.md`, `docs/project/poketrack-reference.md`

## Data

2026-01-28
