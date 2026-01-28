# Standard: Stati interattivi (hover, active, focus)

## Obiettivo

Definisce come **tutti** i componenti interattivi (pulsanti, voci nav, icon-button, link, opzioni) devono reagire a hover e click. Standard unico per tutta l'app, allineato a best practice 2025–2026.

**Riferimenti:** `docs/standards/design-system-standard.md` (token, `--transition-default`), `docs/standards/accessibility-standard.md` (focus, `prefers-reduced-motion`).

## Principi

- **Hover:** feedback **sottile ma chiaro**. Transizione fluida su colore di sfondo (e testo se serve).
- **Active (pressione):** feedback **istantaneo**. Nessuna transizione su `:active`; sfondo più scuro che “scatta” al mousedown.
- **Niente `transform` né `scale`** su elementi con testo/icone: causano sfocatura visibile.
- **Focus:** anello visibile per tastiera; invariato rispetto al design system.

## Token

| Token | Valore | Uso |
|-------|--------|-----|
| `--hover-bg` | `#2a2d2e` | Sfondo hover (voci, icon-btn, pulsanti ghost) |
| `--pressed-bg` | `#1f2224` | Sfondo `:active` (più scuro di hover); **istantaneo** |
| `--transition-default` | `200ms ease-out` | Solo per **hover** (e focus dove si anima) |

`--pressed-bg` è definito in `src/app.css` (sezione `.dark`). Per varianti con colore (es. primary, destructive) si usano tonalità più scure (es. `primary/80`) invece di `--pressed-bg`.

## Regole tecniche

### 1. Transizioni

- **Base:** `transition: background-color var(--transition-default)` (e `color` se cambia su hover).
- **`:active`:** `transition: none` per lo stesso elemento, così il passaggio a pressed è **istantaneo**.

### 2. Hover

- Cambio **solo** di `background-color` (e eventualmente `color`).
- Durata `--transition-default` (200ms).

### 3. Active

- Sfondo **più scuro** (es. `--pressed-bg` o variante tipo `primary/80`).
- **Sempre** `transition: none` su `:active` per “snap” immediato.

### 4. Cosa non usare

- `transform`, `scale`, `translate` su elementi interattivi con testo/icone.
- `transition` su `:active` (feedback deve essere immediato).

### 5. Riduzione movimento

- In `prefers-reduced-motion: reduce` le transizioni sono già ridotte/disattivate globalmente in `src/app.css`. Non servono override specifici per gli stati interattivi.

## Dove si applica

- **Button** (shadcn): default, destructive, outline, secondary, ghost, ghost-muted, link.
- **Top bar:** icon-button (sync, cartella, ricarica).
- **Sidebar:** toggle expand/collapse, voci nav, sottovoci.
- **ProfileSelector:** trigger, opzioni dropdown.
- **Dialog:** pulsante chiudi.
- **Altri** controlli cliccabili (link, opzioni liste, ecc.): stesso schema hover + active.

## Riferimenti

- Design system (token, transizioni): `docs/standards/design-system-standard.md`
- Stack UI (Tailwind, shadcn): `docs/standards/ui-stack-standard.md`
- Accessibilità: `docs/standards/accessibility-standard.md`
