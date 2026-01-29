# Procedure: Modifica Layout Responsive / Breakpoint / Adattivo

## Obiettivo

Definisce come introdurre o modificare layout adattivi (breakpoint, comportamento su viewport stretta/larga, tipografia fluida, container queries) in modo coerente con le best practice 2026. **Standard di riferimento:** `docs/standards/responsive-design-standard.md`. **Stack:** Tailwind + shadcn; shell e token da `docs/standards/design-system-standard.md` e `docs/standards/ui-stack-standard.md`.

## Quando Usare Questa Procedura

- Query: "rendi responsive", "design responsive", "layout mobile", "breakpoint", "layout adattivo", "adatta a viewport", "mobile-first", "sidebar su schermo piccolo", "tipografia fluida", "container query"
- Quando si aggiungono o cambiano comportamenti in base alla larghezza viewport o al contesto contenitore
- Se la query corrisponde, questa procedure è OBBLIGATORIA

## Obbligatorietà

Completamento integrale della checklist obbligatorio prima di proporre implementazione. Nessuna eccezione.

## File da Leggere PRIMA

1. **Standard responsive:** `docs/standards/responsive-design-standard.md` — breakpoint, mobile-first, unità fluide, touch target, container queries
2. **Design system e shell:** `docs/standards/design-system-standard.md` — token, Top Bar 48px, Sidebar 220px/48px collassata, area contenuto
3. **Stack UI:** `docs/standards/ui-stack-standard.md` — Tailwind, prefissi sm/md/lg/xl/2xl
4. **Se si modificano shell/nav:** `docs/procedures/workflow/layout-navigation-change.md`

## Checklist Obbligatoria

1. [ ] **Mobile-first:** stile base per viewport piccola; prefissi `md:`, `lg:`, `xl:`, `2xl:` per viewport maggiori. Vedi [responsive-design-standard.md](../../standards/responsive-design-standard.md).
2. [ ] **Breakpoint Tailwind:** usare i default (sm 640px, md 768px, lg 1024px, xl 1280px, 2xl 1536px) senza introdurre breakpoint custom non documentati.
3. [ ] **Unità:** preferire `rem`, `%`, `clamp()` per dimensioni che devono scalare; evitare `px` fissi per layout fluido. Token da design-system per spacing e font.
4. [ ] **Touch target:** pulsanti/link usabili con touch almeno 44×44px dove richiesto (es. nav, CTA); in Tailwind `min-h-11 min-w-11` o equivalente. Vedi [accessibility-standard.md](../../standards/accessibility-standard.md).
5. [ ] **Shell (Top Bar / Sidebar / Area):** su viewport stretta Sidebar collassata (48px) o nascosta con toggle; area contenuto sempre utilizzabile. Dimensioni da [design-system-standard.md](../../standards/design-system-standard.md). Se si toccano shell/nav, seguire [layout-navigation-change.md](./layout-navigation-change.md).
6. [ ] **Container queries:** usare solo per componenti che si adattano al contenitore (card, pannelli); non per lo shell globale. `container-type: inline-size` e `@container` come in [responsive-design-standard.md](../../standards/responsive-design-standard.md).
7. [ ] **Tipografia fluida:** se si scala con viewport, usare `clamp()` o variabili; allineare a gerarchia e token in design-system.
8. [ ] **Stack UI:** Tailwind e, dove possibile, componenti da `$lib/components/ui`; nessun CSS custom superfluo. Vedi [ui-stack-standard.md](../../standards/ui-stack-standard.md).

## Verifica dopo le modifiche

1. [ ] **Viewport stretta:** ridimensionare finestra (o DevTools device) sotto 768px → verificare sidebar collassata/nascosta e area contenuto leggibile e scrollabile.
2. [ ] **Viewport desktop:** da 1024px in su → layout pieno (Sidebar 220px se espansa, Top Bar 48px, area contenuto flex 1).
3. [ ] **Touch target:** controlli critici (nav, CTA) almeno 44×44px dove previsto.
4. [ ] **Console:** nessun errore JavaScript che alteri layout o visibilità.

## Riferimenti Standard

- [responsive-design-standard.md](../../standards/responsive-design-standard.md) — Breakpoint, mobile-first, unità, container queries, touch
- [design-system-standard.md](../../standards/design-system-standard.md) — Token, shell, dimensioni
- [ui-stack-standard.md](../../standards/ui-stack-standard.md) — Tailwind, shadcn
- [layout-navigation-change.md](./layout-navigation-change.md) — Modifiche a sidebar/nav/shell
- [accessibility-standard.md](../../standards/accessibility-standard.md) — Touch, focus, reduced-motion

## Note

- Per sole modifiche a voci/ordine sidebar o layout shell senza cambio breakpoint/responsive, usare [layout-navigation-change.md](./layout-navigation-change.md).
- Per nuovo componente UI generico usare [component-creation.md](../svelte/component-creation.md); qui si applica quando il componente o la pagina richiedono **comportamento adattivo** per viewport/contenitore.
