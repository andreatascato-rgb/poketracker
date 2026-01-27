# Procedure: Applicare Stile / Design Tokens / Tema

## Obiettivo

Definisce come applicare stile, token e tema in linea con [ui-implementation-standard](../../standards/ui-implementation-standard.md) e [ui-ux-design](../../project/ui-ux-design.md).

## Quando Usare Questa Procedure

- Query: "applica stile", "design tokens", "aggiungi tema", "tema dark", "design system", "token", "stile componente", "CSS variables"
- Quando si deve introdurre o modificare token, tema o stili globali/component in linea con il design
- Se la query corrisponde, questa procedure è OBBLIGATORIA

## Obbligatorietà

Completamento integrale della checklist obbligatorio prima di proporre implementazione. Nessuna eccezione.

## File da Leggere PRIMA

1. **UI Implementation**: `docs/standards/ui-implementation-standard.md:1-60`
   - Token (CSS vars), naming, gerarchia: righe 7-20
   - Layout, scroll, scrollbar: righe 22-30

2. **Design UI/UX**: `docs/project/ui-ux-design.md:29-131`
   - Colori, tipografia, layout, scroll

## Checklist Obbligatoria

1. [ ] Leggi `docs/standards/ui-implementation-standard.md:7-20` — Token su `:root`; naming kebab-case e semantico; categorie `--color-*`, `--spacing-*`, `--font-*`, ecc.
2. [ ] Verifica `docs/project/ui-ux-design.md` — Allineare token e stili a palette, tipografia e sensazione “in game” / VS Code
3. [ ] Per nuovi componenti: usare stili scoped; opzionale BEM per leggibilità (`ui-implementation-standard.md:22-25`)
4. [ ] Layout: Top Bar e Sidebar fuori dal contenitore scrollabile; Area contenuto con `overflow: auto` (`ui-implementation-standard.md:26-28`)
5. [ ] Scrollbar: nascoste di default (`scrollbar-width: none`; `::-webkit-scrollbar { display: none }`) (`ui-implementation-standard.md:29-30`)
6. [ ] Se si aggiunge tema (light/dark): variabili semantiche che cambiano per tema; class/attribute su `html` o `body` (`ui-implementation-standard.md:32-36`)

## Riferimenti Standard

- `docs/standards/ui-implementation-standard.md:1-60` — Token, layout, tema
- `docs/project/ui-ux-design.md:29-76` — Design e riferimento visivo

## Note

- Per “crea componente” usare `docs/procedures/svelte/component-creation.md`; questa procedure si applica quando si lavora su stile/token/tema in modo esplicito.
