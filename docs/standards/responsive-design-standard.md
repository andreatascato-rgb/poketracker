# Standard: Design Responsive

## Obiettivo

Definisce le best practice 2026 per layout e componenti che si adattano a viewport e contesto: mobile-first, breakpoint, unità fluide, container queries quando utile. Garantisce coerenza e manutenibilità. **Implementazione:** Tailwind per breakpoint e utility; token e shell da `docs/standards/design-system-standard.md`.

**Riferimento:** best practice 2025–2026 (mobile-first, container queries, fluid typography, touch target); stack Tailwind in `docs/standards/ui-stack-standard.md`.

## Principi

- **Mobile-first** — Stile base per schermi piccoli; prefissi breakpoint per viewport crescenti. Riduce duplicazione e favorisce performance/SEO.
- **Unità relative** — Preferire `rem`, `%`, `clamp()`; evitare `px` fissi per layout e tipografia dove serve scalabilità.
- **Breakpoint semantici** — Usare range coerenti (mobile / tablet / desktop / large) e allineare Tailwind `sm`/`md`/`lg`/`xl`/`2xl` quando si definiscono layout adattivi.
- **Touch-friendly** — Target interattivi almeno 44×44px su viewport touch; rispettare `prefers-reduced-motion` (vedi `docs/standards/accessibility-standard.md`).
- **Progressive enhancement** — Layout funzionali al minimo; miglioramenti per viewport maggiori o capacità (es. container queries) senza rompere il base.

## Breakpoint (2026)

Range indicativi per scelte di layout; l’implementazione usa i breakpoint Tailwind (mobile-first, min-width).

| Nome       | Range viewport   | Uso tipico                          | Tailwind |
|------------|------------------|-------------------------------------|----------|
| Mobile     | 320px – 767px    | Layout a colonna, nav compatta      | default (nessun prefisso) |
| Tablet     | 768px – 1023px   | 2 colonne, sidebar collassabile     | `md:`    |
| Desktop    | 1024px – 1439px  | Layout pieno (sidebar 220px, area flex) | `lg:`    |
| Large      | 1440px+          | Contenuto max-width, più respiro    | `xl:`, `2xl:` |

**Tailwind default:** `sm` 640px, `md` 768px, `lg` 1024px, `xl` 1280px, `2xl` 1536px. Usare i prefissi Tailwind (`md:`, `lg:`, ecc.) senza ridefinirli, salvo necessità progetto documentata.

## Regole di implementazione

### Layout e spacing

- **Layout shell (Top Bar, Sidebar, Area contenuto):** dimensioni e comportamento da `docs/standards/design-system-standard.md`. Su viewport stretta: Sidebar collassata (48px) o nascosta con toggle; Area contenuto sempre scrollabile.
- **Unità:** `rem` per font e spacing quando deve scalare; `%` o `flex`/`grid` per larghezze; `clamp(min, preferred, max)` per dimensioni fluide (es. tipografia, container).
- **Esempio fluido:**  
  `width: 100%; max-width: min(1200px, 90vw);` oppure in Tailwind `w-full max-w-[min(1200px,90vw)]`.

### Tipografia fluida

- Dove serve scalare con la viewport:  
  `font-size: clamp(1rem, 4vw + 1rem, 1.5rem);` (o equivalenti in rem). Min e max definiti in base alla gerarchia (body, h1, h2) in `design-system-standard.md`.
- Preferire variabili/Token (es. `--font-size-body`) e scale limitata; evitare troppi step fissi per breakpoint.

### Flexbox e Grid

- **Flexbox:** per layout direzionali (colonna su mobile, riga da `md:` in su); `flex-wrap` dove serve wrap.
- **Grid:** per griglie multi-colonna (1 col mobile, 2–3 col da `md:`/`lg:`); usare `grid-template-columns` con `minmax()` o `fr` per colonne adattive.
- **Esempio:**  
  `grid-cols-1 md:grid-cols-2 lg:grid-cols-3` (Tailwind).

### Container queries (quando utile)

- Per componenti che devono adattarsi alla **larghezza del contenitore** (card, sidebar interna, pannelli), non solo alla viewport: `container-type: inline-size` sul contenitore e `@container` / `@container (min-width: …)` negli stili.
- Tailwind v4+: `@container` e `@container/<name>`. Usare dove riduce media query ripetute e migliora riusabilità (es. card in sidebar vs in area principale).
- Non sostituire con container queries le scelte di layout shell (Top Bar / Sidebar / Area contenuto), che restano viewport-based.

### Immagini e media

- **Responsive:** `width: 100%; height: auto;` con `object-fit: cover` o `contain` dove appropriato.
- **Multiple densità:** `srcset` + `sizes` quando si caricano immagini per diverse risoluzioni.
- In Tailwind: `w-full h-auto object-cover` (o `object-contain`).

### Touch e interazione

- **Target minimo:** 44×44px per pulsanti e link su viewport potenzialmente touch (anche in app desktop ridimensionabile). In Tailwind: `min-h-11 min-w-11` (44px) o equivalente.
- **Spacing:** distanza adeguata tra target per evitare tap accidentali; rispettare `prefers-reduced-motion` per animazioni (accessibility-standard).

## Integrazione Tailwind

- **Prefissi breakpoint:** `sm:`, `md:`, `lg:`, `xl:`, `2xl:` per layout, visibility, dimensioni. Base = mobile.
- **Container queries:** se in uso, definire `@container` in theme o in layer `@layer components` e usare le utility Tailwind per container queries quando disponibili.
- **Token:** spacing, font-size, colori da theme/`app.css`; nessun valore “magico” nel markup.

## Anti-pattern da evitare

- **Solo px fissi** per layout e font che devono adattarsi.
- **Desktop-first** con override per “mobile”: rende il codice meno chiaro e più fragile.
- **Breakpoint “a sentimento”** non allineati a Tailwind o allo standard (es. 823px isolato).
- **Target < 44px** per controlli usabili con touch.
- **Container queries per lo shell globale** invece che per componenti modulari.

## Applicazione in PokeTracker

- **Layout e navigazione:** `docs/procedures/workflow/layout-navigation-change.md` e `design-system-standard.md` per Top Bar, Sidebar, Area contenuto; su viewport stretta verificare sidebar collassata e area contenuto usabile.
- **Nuovi componenti/pagine:** applicare mobile-first, breakpoint Tailwind e touch target; usare `docs/procedures/workflow/responsive-design-change.md` quando si introducono o modificano layout adattivi in modo rilevante.
- **Design system:** token e dimensioni fisse (es. Top Bar 48px, Sidebar 220px) restano da design-system; il comportamento “collassa/nasconde” su viewport piccola è definito qui.

## Riferimenti

| Documento | Uso |
|-----------|-----|
| [design-system-standard.md](./design-system-standard.md) | Token, shell, dimensioni, tipografia |
| [ui-stack-standard.md](./ui-stack-standard.md) | Tailwind, shadcn, theme |
| [layout-navigation-change.md](../procedures/workflow/layout-navigation-change.md) | Modifiche shell e nav |
| [responsive-design-change.md](../procedures/workflow/responsive-design-change.md) | Procedure per layout adattivo |
| [accessibility-standard.md](./accessibility-standard.md) | Touch, focus, reduced-motion |

## Data decisione

2026-01-28

## Note

- Lo standard segue best practice 2025–2026 (mobile-first, container queries, fluid typography, touch). Revisione periodica consigliata.
- Per priorità in caso di conflitto: `docs/project/priorities.md`.
