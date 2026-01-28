# Pattern UI applicati e scelte di stile

## Obiettivo

Documenta **design e stile consolidati** scelti in PokeTracker: empty state, card, CTA, area contenuto. Serve per allineare l’AI e lo sviluppo futuro senza rifare scelte ogni volta.

**Uso:** prima di introdurre nuovi blocchi UI (empty state, card, form, CTA), consultare questo file e `docs/standards/design-system-standard.md`. Riferimento di implementazione: `src/routes/+page.svelte` (Home empty state).

---

## Area contenuto

- **Padding:** `var(--spacing-xl, 24px)` su tutto il perimetro dell’area main.
- **Dove:** layout `src/routes/+layout.svelte`, classe `.main-content-inner` che wrappa `{@render children()}`.
- **Regola:** tutte le pagine ricevono lo stesso padding; non replicare padding ad hoc nelle pagine.

---

## Empty state (stato vuoto)

Pattern usato quando una sezione è vuota (es. nessun profilo) e si vuole guidare l’utente verso un’azione.

### Regola obbligatoria (AI e implementazione)

**Quando implementi una vista che mostra una lista o tabella** (profili, salvataggi, categorie, file aperti, risultati di ricerca, ecc.) **e i dati possono essere vuoti**, devi:

1. **Usare il componente EmptyState** (`$lib/components/ui/empty-state`) invece di una riga di messaggio in tabella o di una tabella vuota.
2. **In empty state mostrare solo il CTA centrato** nel blocco EmptyState; **non** mostrare il pulsante d'azione in header (CardAction) quando la vista è vuota — il pulsante in alto a destra va mostrato solo quando ci sono dati.
3. **Seguire** layout, titolo action-oriented, descrizione e CTA definiti nella sezione sotto.

Riferimento implementativo: `src/routes/profilo/salvataggi/+page.svelte`, `src/routes/impostazioni/profili/+page.svelte`.

### Layout e centratura

- **Wrapper:** `flex min-h-[calc(100vh-96px)] flex-col items-center justify-center`.
  - 96px = 48px (top bar) + 48px (padding verticale area contenuto).
  - Contenuto centrato sia in orizzontale che in verticale nell’area sotto la top bar.
- **Accessibilità:** `role="region"` e `aria-label` descrittivo (es. "Stato vuoto: nessun profilo").

### Struttura del blocco (Card)

1. **Card** shadcn (`Card`, `CardHeader`, `CardTitle`, `CardDescription`, `CardFooter`).
2. **Larghezza:** `max-w-md` sulla Card; `w-full` per adattarsi al wrapper.
3. **Allineamento testo:** `text-center` sulla Card quando il messaggio è centrato.

### Contenuto

- **Icona sopra il titolo:** una sola icona (es. UserPlus), `size-12`, `text-muted-foreground/70`, `strokeWidth={1.5}`. Stile lineare, non infantile (design-system).
- **Titolo:** breve, **action-oriented e positivo** (es. "Crea il tuo primo allenatore" invece di "Nessun profilo"). Best practice: NNGroup, Carbon, Lightning.
- **Descrizione:** `max-w-[66ch] leading-relaxed` su `CardDescription` per leggibilità. Testo che spiega il prossimo passo e il beneficio.

### CTA (call-to-action)

- **Componente:** `Button` shadcn.
- **Variant:** `outline` (bordo, sfondo neutro, hover accent). Stile da tool desktop, meno “blocco pieno”.
- **Size:** `default` (h-9, px-4).
- **Contenuto:** icona + testo (es. UserPlus `size-4` + "Crea nuovo allenatore"). Icona con `aria-hidden="true"`; `aria-label` sul bottone per screen reader.

---

## Riepilogo scelte stilistiche

| Elemento            | Scelta                                    | Motivo / riferimento      |
|---------------------|-------------------------------------------|----------------------------|
| Padding area contenuto | `--spacing-xl` (24px)                 | Design-system, ContentArea |
| Empty state centrato | `flex items-center justify-center`, min-h viewport | NNGroup, Carbon, Lightning |
| Titolo empty state  | Action-oriented, positivo                | NNGroup, Carbon            |
| Descrizione         | `max-w-[66ch] leading-relaxed`          | Leggibilità (~66 caratteri) |
| CTA in empty state  | Button `variant="outline"`, size default | Coerenza con stile tool   |
| Icona nel CTA       | Icona + testo                            | Chiarezza azione          |
| Icona sopra titolo  | Muted, size-12, lineare                  | Design-system, Carbon/Primer |
| **Gruppo pulsanti (CardAction, Dialog/AlertDialog footer, EmptyState CTA)** | `gap-3` (12px) | Design-system `--spacing-md`; standard per due o più pulsanti affiancati |
| **Indicatore stato main cartella (solo icona, no box)** | CheckCircle: inattivo = `opacity-40 text-primary` (bianco spento), attivo = `opacity-100 text-[var(--icon-success)]` (verde) | Carbon/Spectrum "status light"; neutro, senza semantica successo/errore; non usare icona identica all’azione |

---

## Riferimenti esterni e standard

- **Empty state:** NNGroup, Carbon Design System, Lightning Design System, Primer — centratura, linguaggio positivo, CTA unico.
- **Loading e sync:** `docs/project/loading-and-sync-ux.md` — quando overlay vs inline, soglie temporali, watcher, più sync contemporanee.
- **Token e principi:** `docs/standards/design-system-standard.md`.
- **Stack UI:** `docs/standards/ui-stack-standard.md` (Tailwind, shadcn-svelte).
- **Riferimento visivo Poketrack:** `docs/project/poketrack-reference.md`.

---

## Dove è applicato oggi

- **Home** (`src/routes/+page.svelte`): empty state "nessun profilo" con card centrata, titolo "Crea il tuo primo allenatore", CTA outline "Crea nuovo allenatore".
- **Layout** (`src/routes/+layout.svelte`): padding area contenuto via `.main-content-inner`; **sidebar** — stati hover/attivo come in [design-system-standard](../standards/design-system-standard.md) (Pattern — Sidebar, Stati voci).

Nuovi empty state (Profilo, Editor, Wiki, ecc.) vanno allineati a questo pattern salvo diversa indicazione.

---

## Form in dialog (creazione entità)

Pattern per form di creazione aperto da un CTA (es. "Crea nuovo allenatore" dall’empty state): **Dialog** con form contenuto, non navigazione a pagina dedicata.

### Best practice 2024–2026 (riferimenti)

- **NNGroup / Adobe / CFPB:** label sopra o a sinistra del campo; campi obbligatori segnati (asterisco o "Obbligatorio"); niente pulsante Reset/Clear; validazione inline sui campi; messaggi di errore **sotto il campo**, chiari e non colpevolizzanti; mai disabilitare il submit — usare validazione + messaggi.
- **Validazione:** errore mostrato **dopo blur** (non mentre si digita) o **al submit**; un messaggio per campo; colore + icona/testo (non solo colore); messaggio che spiega come correggere.
- **Accessibilità:** `<label for="id">` associato all’input; `aria-required` e `required` sui campi obbligatori; autofocus sul primo campo all’apertura; focus trap nel dialog; ordine di tab logico.
- **Dialog (shadcn/Radix):** header (titolo), body (form), footer (azioni); chiusura con Escape e click fuori; binding `open` per aprire/chiudere da codice (es. dopo submit riuscito).

### Allineamento con stile app

- **Dialog:** shadcn-svelte `Dialog.Root`, `Dialog.Content`, `Dialog.Header`, `Dialog.Footer`, `Dialog.Title`, `Dialog.Description` (se serve), `Dialog.Close`.
- **Form:** componenti da `$lib/components/ui`: `Input`, `Label`, `Button`. Tailwind + token design-system; niente CSS custom.
- **Bottoni nel footer:** Azione secondaria (Annulla / Chiudi): `variant="outline"` — coerente con CTA empty state. Azione primaria (Crea / Salva): `variant="default"` per evidenziare il submit.
- **Validazione:** frontend per UX (required, trim); backend già valida (`create_profile` restituisce messaggi chiari). Mostrare errore sotto l’input; per errori da backend (nome duplicato, nome vuoto) riusare il messaggio del command come da error-handling-standard.
- **Campo nome allenatore:** label **sempre** "Nome allenatore" in tutti i dialog (creazione, rinomina); placeholder opzionale; `maxlength`/limit se definiti dal backend; `autofocus` sul primo campo all’apertura.

### Struttura tipica

1. **Dialog.Root** con `bind:open` (stato locale o da props).
2. **Dialog.Content:** `Dialog.Header` (titolo "Nuovo allenatore"), corpo con `<form>`:
   - `<Label for="nome">Nome allenatore</Label>` + `<Input id="nome" bind:value={nome} required aria-required />`
   - messaggio errore sotto l’input (solo se `error` è valorizzato).
3. **Dialog.Footer:** `Button variant="outline"` "Annulla" (chiude dialog), `Button type="submit" variant="default"` "Crea" (submit form).
4. **Submit:** `invoke("create_profile", { name: nome.trim() })`; in successo chiudere dialog e chiamare `loadProfiles()`; in errore mostrare messaggio sotto l’input.

### Riferimenti

- `docs/standards/forms-validation-standard.md` — chi valida cosa, messaggi.
- `docs/standards/error-handling-standard.md` — inline per campi, messaggi user-facing.
- `docs/procedures/svelte/form-creation.md` — checklist form.

### Dove è applicato oggi

- **Home** (`src/routes/+page.svelte`): dialog "Nuovo allenatore" con form (Label + Input nome), validazione frontend (required, trim), messaggio backend sotto il campo, footer Annulla (outline) + Crea (default).

---

## Suddivisione sezioni e uso Card / paragrafi / tabelle

### Identità

- **Tool a blocchi:** ogni concetto è un blocco chiaro (card o panel). Stesso linguaggio visivo ovunque.
- **Card = unità di contenuto standard:** dove non serve un panel in stile editor, il contenuto vive in card.
- **Niente muri di testo:** il testo lungo sta dentro card o sotto titoli di sezione.

### Sotto-sezioni per vista

| Vista        | Sotto-sezioni (blocchi logici)                                                                 |
|--------------|------------------------------------------------------------------------------------------------|
| Home         | Una sola (empty state o panoramica profilo/salvataggi). Nessun sotto-menu.                     |
| Allenatore   | Dati allenatore, Percorsi salvataggi, Statistiche / Pokedex. Card o tab se il contenuto cresce. |
| Editor       | File aperto \| Panello modifica (Pokémon / Allenatore / Inventario / …) \| Validazione. Panel, non solo card. |
| Wiki         | Categorie (Pokémon, Nature, Mosse, …). Liste e scheda dettaglio come card.                    |
| Impostazioni | Profili, App, Risorse, Backup, … — un gruppo = una o più card.                                |

### Dove mettere lista allenatori, Modifica ed Elimina

**Scelta:** la lista degli allenatori (profili), con pulsanti **Modifica** e **Elimina**, vive in **Impostazioni** (`/impostazioni`), dentro una card **"Profili"**.

**Perché Impostazioni e non Allenatore (Profilo):**

- **Allenatore** = vista sui **dati del profilo attivo** (nome, percorsi salvataggi, Pokedex, statistiche). È “il mio allenatore”, lettura/uso.
- **Gestione profili** (crea, elenca, rinomina, elimina) = **configurazione app** e “chi può usare l’app con quali dati”. Rientra in self-management (gestione interna app) → Impostazioni.
- Glossario: *Impostazioni* = “configurazione app e gestione interna”; *Selettore profilo* in Top Bar serve per **cambiare** profilo; la **lista + azioni** sta dove si gestisce l’app → Impostazioni.

**Contenuto card "Profili" (Impostazioni):**

- Tabella: Nome, Stato (Attivo / —), Azioni (Modifica, Elimina).
- In header card: titolo "Profili" + pulsante "Nuovo allenatore" (apre dialog creazione).
- Dialog Rinomina per Modifica; conferma prima di Elimina.
- Riferimento implementativo: `docs/project/self-management.md` (Gestione Profili), `docs/project/features.md` (Gestione Multi-Profilo + Gestione Interna App).

### Card stack vs Tab: quando usare cosa

**Regola base:** una **sola area contenuto** con scroll; le sotto-sezioni sono blocchi nella stessa pagina, non route diverse.

| Criterio | Card in stack (verticale) | Tab orizzontali |
|----------|---------------------------|------------------|
| **Numero gruppi** | 2–5 gruppi (es. Profili, App, Risorse) | 5+ gruppi o quando una card diventa molto lunga |
| **Uso tipico** | Ogni gruppo è visibile scrollando; utente scorre per cercare | Utente sceglie “dove andare” (Profili | App | Risorse | Backup | Log) |
| **Best practice 2026** | VS Code Settings “flat”, Carbon / GitHub Settings a blocchi; pochi blocchi = tutto in una colonna | Molte voci = tab o nav secondaria per ridurre rumore e lunghezza pagina |
| **Impostazioni oggi** | Profili (+ App, Risorse, Backup quando ci sono) = **card in stack** con `gap-6` | Quando arrivano Log, Diagnostica, Avanzate, ecc. → valutare **Tab** (shadcn Tabs) con Profili | App | Risorse | Backup | Log |

**Variabili e scalabilità:**

- **Poche impostazioni:** una sola colonna di card (Profili, poi App, poi Risorse…). Nessun tab.
- **Tante impostazioni:** introdurre **Tab** in cima all’area Impostazioni: "Profili" | "App" | "Risorse" | "Backup" | "Log". Ogni tab mostra solo le card di quel gruppo.
- **Componente:** quando serve, usare **Tabs** da shadcn-svelte (`npx shadcn-svelte@latest add tabs`); stile allineato a design-system.
- **Allenatore (Profilo):** stessa logica: poche sotto-sezioni (Dati, Percorsi salvataggi, Pokedex) → card in stack; se crescono (statistiche, grafici, export) → card o tab in base a lunghezza e chiarezza.

**Riferimenti:** VS Code Settings (categorie a sinistra quando sono molte); Carbon Design System (tabs for secondary nav); NNGroup — “Tabs used for secondary navigation”.

### Quando usare Card, paragrafi, tabelle

| Elemento   | Uso                                                                 | Esempi |
|------------|---------------------------------------------------------------------|--------|
| **Card**   | Blocco logico con titolo (e opzionale descrizione): unità di informazione o azione. Contenitore standard per quasi tutta l’UI che non è editor o testo lungo. | Empty state, "Dati allenatore", "Profili", "Scheda Pokémon", "Opzione notifiche", voci Wiki. |
| **Paragrafi** | Testo continuo da leggere: descrizioni, flavour, note, help. **Dentro** una card o sotto titolo di sezione. | Descrizione mossa, testo "Come usare", note in Impostazioni. |
| **Tabelle**   | Dati tabellari: molte righe, stesse colonne. Di solito **dentro** una card ("Statistiche base", "Mosse apprese"). Per **stile, padding, allineamento e proporzioni** di ogni tabella → sezione **"Standard tabelle"** sotto. | Lista mosse (nome, tipo, PP, potenza), stats base, confronto nature. |

---

## Standard tabelle (2026 e scelte PokeTracker)

Riferimento unico per **tutte** le tabelle dell’app (Impostazioni, Wiki, Allenatore, ecc.). Garantisce aspetto uguale, accessibilità e proporzioni coerenti. Allineato a WCAG, Carbon Design System, W3C Design System e preferenze di progetto (celle centrate, padding uniforme, colonne proporzionali al contenuto).

### Best practice 2026 (riferimenti esterni)

- **WCAG / W3C:** usare markup semantico (`<table>`, `<thead>`, `<tbody>`, `<th>`, `<td>`); `<th scope="col">` per le intestazioni di colonna, `scope="row"` per eventuali intestazioni di riga; `<caption>` o `aria-label` sulla tabella per nome/descrizione; evitare celle vuote solo per layout; per molte colonne usare contenitore scrollabile (`overflow-x-auto`).
- **Carbon Design System:** padding celle coerente con token (es. 16px orizzontale = spacing-05); altezza riga leggibile (default 48px, short 32px, compact 24px); stati visibili per hover/focus/selected; stessa scala di spaziatura per tutte le tabelle.
- **W3C Design System:** tabella dentro wrapper con `role="region"` e `tabindex="0"` se scroll orizzontale; conservare struttura a griglia.
- **Leggibilità:** allineamento coerente (testo e numeri); padding sufficiente per non dare sensazione di affollamento; proporzioni colonne che riflettono il tipo di contenuto (colonna con più testo/info = più larga).

### Scelte PokeTracker (obbligatorie per ogni tabella)

1. **Allineamento orizzontale:** celle e intestazioni **centrate orizzontalmente** (`text-center` su `<th>` e `<td>`). Coerenza visiva in tutta l’app; eccezione: colonne solo numeriche dove ha senso allineare a destra (`text-right`), da usare solo quando la tabella è chiaramente “dati numerici”.
2. **Padding uniforme:** stesso padding per tutte le celle in tutte le tabelle. Valori standard:
   - **Orizzontale:** `px-4` (16px, allineato a `--spacing-lg`).
   - **Verticale:** `py-3` (12px, allineato a `--spacing-md`).
   - Classi Tailwind da usare: `px-4 py-3` su ogni `<th>` e `<td>` (o su `thead th` e `tbody td` via un’unica classe sul `<table>` tramite `th px-4 py-3`, `td px-4 py-3`).
3. **Proporzioni colonne:** la colonna che “prospetta più caratteri” (contenuto tipico più lungo) è **più larga**; le altre restano proporzionate. Regole:
   - Usare `table-layout: auto` (default) oppure, se serve controllo, larghezze/min-width in **`ch`** (caratteri) per coerenza indipendente dal font.
   - Schema ripetibile: colonna “contenuto principale” (es. Nome, Titolo) → `min-width` più grande (es. `min-w-[20ch]` o `min-w-[24ch]`); colonna “stato/secondario” (es. Stato, Tipo) → `min-w-[10ch]` o simile; colonna “Azioni” (solo pulsanti) → larghezza determinata dai pulsanti, es. `min-w-[8rem]` o `w-auto` con `whitespace-nowrap`.
   - **Coerenza tra proporzioni:** per tabelle dello stesso tipo (es. “lista entità + azioni”) usare lo stesso schema di min-width: stessa colonna “principale”, stessa “stato”, stessa “azioni” in tutta l’app.
4. **Contenitore:** tabella sempre dentro un wrapper scrollabile su schermi stretti: `overflow-x-auto rounded-md border`; il bordo usa il token del design-system (`border-border` o `border` da Tailwind/shadcn).

### Markup e struttura (obbligatori)

- `<table role="grid" aria-label="…">` oppure `<table aria-labelledby="id-caption">` con `<caption id="id-caption">` come primo figlio; preferibile `aria-label` se la tabella è in un contesto già etichettato (es. card “Profili”).
- `<thead>` con `<tr>` e `<th scope="col">` per ogni colonna. Nessun `<th>` senza `scope`.
- `<tbody>` con `<tr>` e `<td>`; per righe cliccabili/selezionabili usare gli attributi e gli stati (hover/focus) definiti dall’accessibility-standard.
- Evitare `colspan`/`rowspan` salvo necessità; preferire tabelle semplici.

### Stile visivo (classi standard)

- **Tabella:** `w-full min-w-[…] text-sm` (min-width solo se serve garantire una larghezza minima complessiva).
- **Header:** `<thead>` con `border-b border-[var(--border-primary)] bg-[var(--bg-tertiary)]` (superficie rialzata, bordo netto); ogni `<th>` con `px-4 py-3 text-center text-muted-foreground text-xs font-semibold uppercase tracking-wider` per differenziare chiaramente l’intestazione dalle righe (stile data-table moderno, Carbon/Material).
- **Righe:** ogni `<tr>` con `border-b border-border last:border-b-0`; nessun evidenziamento al hover.
- **Wrapper:** `overflow-x-auto rounded-md border border-[var(--border-primary)]` intorno al `<table>`; eventuale `role="region" aria-label="Tabella …"` sul wrapper se la tabella non ha già un nome accessibile.

### Schema tipo “lista con azioni” (es. Profili, liste Impostazioni)

| Colonna      | Contenuto tipico  | Allineamento | Larghezza / min-width    |
|--------------|-------------------|--------------|---------------------------|
| Principale   | Nome, Titolo, …   | `text-center`| `min-w-[20ch]` o maggiore|
| Secondaria   | Stato, Tipo, …    | `text-center`| `min-w-[10ch]`           |
| Azioni       | Pulsanti icona    | `text-center`| `min-w-[8rem]` o auto    |

Questo schema va riusato per ogni tabella “lista entità + stato + azioni” così proporzioni e look restano uguali in tutta l’app.

### Riepilogo standard tabelle

| Elemento           | Scelta                                                           |
|--------------------|------------------------------------------------------------------|
| Allineamento celle | Centrato orizzontale (`text-center`) per th e td                 |
| Padding celle      | `px-4 py-3` (16px orizz., 12px vert.) — token --spacing-lg / md  |
| Proporzioni        | Colonna “più caratteri” = più larga; min-width in `ch`; schema coerente tra tabelle simili |
| Header             | `bg-[var(--bg-tertiary)]`, `border-b border-[var(--border-primary)]`, `px-4 py-3`, `text-muted-foreground text-xs font-semibold uppercase tracking-wider` |
| Righe              | `border-b border-border` tra righe; nessun evidenziamento al hover |
| Contenitore        | `overflow-x-auto rounded-md border border-[var(--border-primary)]`                             |
| Accessibilità      | `aria-label` o `<caption>` su table; `<th scope="col">`         |

### Riferimenti

- **Design-system (token):** `docs/standards/design-system-standard.md` — `--spacing-lg`, `--spacing-md`.
- **Accessibilità:** `docs/standards/accessibility-standard.md`; WCAG tecniche H51, W3C “Creating Accessible Tables”.
- **Card Impostazioni:** sezione “Stile, design e layout delle card in Impostazioni” per contesto d’uso dentro le card.

---

## Card di sezione (stile professionale)

Pattern per card usate come **blocchi di sezione** in una pagina (es. Impostazioni, Wiki, Allenatore): titolo, eventuale descrizione, corpo con contenuto o azioni.

### Obiettivo

Card moderne e professionali, coerenti con design-system e best practice 2026: leggibilità, gerarchia chiara, densità adatta a tool desktop, accessibilità.

### Struttura

1. **Card** shadcn (`Card` come wrapper).
2. **CardHeader**: `CardTitle` (obbligatorio) + `CardDescription` (opzionale, quando serve contesto).
3. **CardContent** (o contenuto diretto nel corpo della card): contenuto principale.
4. **CardFooter** (opzionale): azioni secondarie, link, CTA.

### Stile e token

- **Card:** usare componenti da `$lib/components/ui/card`. Stile default shadcn (rounded-xl, border, shadow-sm) già allineato al design-system; non sovrascrivere con valori magici.
- **Titolo:** `CardTitle`, `text-base` o `text-lg`, `font-semibold`. Title case. Nessun font "display" o giocoso.
- **Titolo con azione sulla riga:** quando CardTitle e un Button size="sm" condividono la stessa riga (es. "Profili" + "Nuovo allenatore"), il titolo deve essere **text-lg**, **min-h-8**, **flex items-center**, **font-semibold** — così il titolo resta visivamente uguale o più grande del bottone. Non ridurre il titolo a text-sm per “allinearlo” al bottone.
- **Descrizione:** `CardDescription`, `text-muted-foreground`, `text-sm`; `max-w-[66ch]` se il blocco è largo. Leggibilità prima di tutto.
- **Spaziatura:** rispettare il gap interno dei componenti card (es. gap-6 sul Card); per il corpo usare `space-y-3` o `gap-3` tra elementi.

### Layout pagina con più card

- **Container:** `space-y-6` o `flex flex-col gap-6` tra una card e l’altra.
- **Larghezza:** card a tutta larghezza nell’area contenuto (entro il padding `--spacing-xl`); non obbligatorio `max-w` per card di sezione.
- **Ordine:** titolo di pagina (h1) opzionale in cima; poi le card in ordine logico.

### Best practice 2026

- **Gerarchia:** un solo titolo principale per card (CardTitle); sottotitoli solo se necessari, con stile minore (font-size, muted).
- **Accessibilità:** card con `role="region"` e `aria-labelledby` puntato all’id del CardTitle se la card è una landmark; oppure `aria-label` sul Card quando non c’è titolo visibile.
- **Azioni:** CTA e link nel corpo o nel footer; Button `variant="outline"` per azioni secondarie, `variant="default"` per azione primaria della card (coerente con empty state e form).
- **Contenuto:** evitare card vuote o solo decorative; ogni card deve avere uno scopo chiaro (informazione o azione).

### Dove è applicato oggi

- **Impostazioni** (`src/routes/impostazioni/+page.svelte`): card "Profili" con titolo e pulsante "Nuovo allenatore" sulla stessa riga a destra (`CardAction`); descrizione sotto; tabella profili (Nome, Stato, Azioni) con pulsanti Modifica (icona Pencil) ed Elimina (icona Trash2); dialog creazione e dialog rinomina allineati al pattern form-in-dialog. Comandi backend: `rename_profile`, `delete_profile`.

### Dove si usa (altre card future)

- **Impostazioni:** card "App", "Risorse", "Backup", ecc.
- **Wiki / Allenatore:** blocchi informativi e dettaglio come card.

---

## Stile, design e layout delle card in Impostazioni

Riferimento unico per **tutte** le sottosezioni Impostazioni (Profili, App, Risorse, Backup, Log, …). Allineato a design-system, ui-stack e best practice 2025–2026 (VS Code Settings, Carbon, Material, NNGroup). Le card in Impostazioni condividono stesso stile e stesso layout pattern così l’utente capisce subito “sono in una sottosezione impostazioni”.

### Principi (best practice 2026)

- **Un blocco = una card:** ogni sottosezione (Profili, App, Risorse, …) mostra **una sola card** come contenitore principale. Niente più card annidate o stack di card nella stessa sottosezione; se servono sotto-raggruppamenti, usarli **dentro** il corpo della card (es. sotto-titoli, tabelle, gruppi di controlli).
- **Gerarchia chiara:** titolo di sezione (CardTitle) → eventuale descrizione (CardDescription) → contenuto (tabella, form, lista). Un solo titolo principale per card; il resto è corpo o sottotitoli in stile muted.
- **Azioni primarie in header:** l’azione principale della card (es. "Nuovo allenatore" in Profili) sta **nella stessa riga del titolo**, a destra, tramite `CardAction`. Best practice: toolbar/azioni contestuali in cima al blocco, non disperse nel corpo (Material Settings, VS Code).
- **Contenuto prevedibile:** tabelle, form e liste vivono **dentro** CardContent; stessi token (bordi, spacing, font) in tutta Impostazioni per coerenza.
- **Niente muri di testo:** descrizioni brevi; testo lungo solo in CardDescription o in paragrafi dedicati dentro la card, con `max-w-[66ch]` e `text-muted-foreground` dove serve.

### Stack e componenti (allineamento standard)

- **Card:** sempre da `$lib/components/ui/card` — `Card`, `CardHeader`, `CardTitle`, `CardDescription`, `CardContent`, `CardAction` (quando titolo + azione sulla stessa riga). Niente div/custom-card; stile default shadcn (rounded-xl, border, shadow-sm, `bg-card`) è quello da usare.
- **Token:** colori e spaziature da design-system (`docs/standards/design-system-standard.md`). In pratica: `--bg-primary` per area contenuto, `--card` (bg-overlay in dark) per card, `--border-primary` / `--border-subtle` per bordi, `--spacing-xl` per padding area, `--font-size-body` 13px, `--text-primary` / `--text-secondary`. Tailwind/shadcn usano già `--background`, `--card`, `--muted-foreground` mappati in `app.css` alla palette Poketrack.
- **Tipografia:** titolo card `text-lg font-semibold`, descrizione `text-sm text-muted-foreground`, corpo e tabelle `text-sm`. Title case per CardTitle; sentence case per CardDescription. Font da `--font-primary` (Segoe UI, SF Pro, system-ui).

### Layout di una card Impostazioni (struttura)

1. **Card** (wrapper unico).
2. **CardHeader** (grid con titolo + azione):
   - **CardTitle** (id per `aria-labelledby`): testo della sottosezione (es. "Profili"). Classi: `text-lg font-semibold min-h-8 flex items-center`.
   - **CardAction** (opzionale): pulsante principale (es. "Nuovo allenatore") — `Button variant="outline" size="sm"` o `variant="default"` se è l’azione primaria della pagina.
   - **Riga 2 — CardDescription** (sottotitolo): sempre sulla seconda riga, **a tutta larghezza** sotto titolo e pulsante. Il componente `CardHeader` applica la regola: quando ha sia `CardAction` sia `CardDescription`, il sottotitolo ha `grid-row: 2` e `grid-column: 1 / -1` (riga 2 a tutta larghezza). Nessun taglio di testo: il sottotitolo può essere lungo e va a capo nella riga 2. Stile: `text-sm text-muted-foreground`.
3. **CardContent:** contenuto (tabella, form, lista di opzioni). Padding già gestito da CardContent (`px-6`); verticale con `space-y-3` o `gap-3` tra blocchi interni.

La card usa il layout a griglia del CardHeader: con `CardAction` si attiva `grid-cols-[1fr_auto]`, titolo a sinistra e azione allineata a destra.

### Tabelle dentro le card (es. Profili)

Le tabelle in Impostazioni seguono **per intero** la sezione **"Standard tabelle (2026 e scelte PokeTracker)"** in questo file. In sintesi:

- **Contenitore:** tabella dentro CardContent; wrapper `overflow-x-auto rounded-md border` come da standard tabelle.
- **Stile e allineamento:** celle e intestazioni **centrate orizzontalmente** (`text-center`); padding **uniforme** `px-4 py-3` su ogni `<th>` e `<td>`; proporzioni colonne come da schema “lista con azioni” (colonna principale più larga, Stato/secondaria, Azioni con min-width coerente). Dettaglio: sezione "Standard tabelle" sopra.
- **Header e righe:** `thead` con `bg-[var(--bg-tertiary)]`, `border-b border-[var(--border-primary)]`; `<th>` con `text-muted-foreground text-xs font-semibold uppercase tracking-wider`; celle con `px-4 py-3 text-center border-b`; righe senza evidenziamento al hover.
- **Azioni per riga:** solo icon-button (Pencil, Trash2) con `variant="ghost"` e `size="icon-sm"`; `title` e `aria-label` per accessibilità. Colonna Azioni centrata e di larghezza coerente con lo standard tabelle.

### Spaziatura e allineamento

- **Tra titolo e contenuto:** il gap è già dato dal Card (gap-6). Non aggiungere padding extra tra CardHeader e CardContent.
- **Nella pagina:** la sottosezione Impostazioni è da sola nell’area contenuto (una route = una card). Se in futuro la stessa route mostrasse più di una card, usare `flex flex-col gap-6` tra le card.
- **Padding area contenuto:** gestito dal layout (`--spacing-xl` su `.main-content-inner`); le pagine Impostazioni non aggiungono altro padding intorno alla card.

### Accessibilità

- **Landmark:** card con `role="region"` e `aria-labelledby="{id del CardTitle}"` (es. `id="impostazioni-profili-title"` sul CardTitle).
- **Tabelle:** `<table>` con `aria-label` o `<caption>`; `<th scope="col">` per le intestazioni; per "Azioni" usare intestazione chiara (es. "Azioni") e pulsanti con `aria-label` espliciti (es. "Modifica profilo Rosso", "Elimina profilo Rosso").
- **Focus e contrasto:** rispettare `docs/standards/accessibility-standard.md`; colore non come unico indicatore (stato attivo, errori, disabled).

### Riepilogo visivo

| Elemento            | Scelta                                                                 |
|---------------------|-----------------------------------------------------------------------|
| Contenitore         | Una card per sottosezione; componente `Card` shadcn                   |
| Titolo              | CardTitle, `text-lg font-semibold`, Title case                        |
| Azione in header    | CardAction con Button (outline o default) size="sm"                   |
| Descrizione         | CardDescription, `text-sm text-muted-foreground`, opzionale            |
| Corpo               | CardContent; tabelle con `rounded-md border`, celle `px-4 py-3`       |
| Pulsanti per riga   | Icon-only (Pencil, Trash2) ghost, con aria-label e title               |
| Token               | Design-system (app.css .dark); nessun valore magico nei componenti    |

### Riferimenti incrociati

- **Design system e token:** `docs/standards/design-system-standard.md`
- **Stack UI:** `docs/standards/ui-stack-standard.md`
- **Card di sezione (generale):** sezione "Card di sezione (stile professionale)" sopra
- **Form in dialog (creazione/rinomina):** sezione "Form in dialog" in questo file
- **Accessibilità:** `docs/standards/accessibility-standard.md`
