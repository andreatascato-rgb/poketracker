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

---

## Riferimenti esterni e standard

- **Empty state:** NNGroup, Carbon Design System, Lightning Design System, Primer — centratura, linguaggio positivo, CTA unico.
- **Token e principi:** `docs/standards/design-system-standard.md`.
- **Stack UI:** `docs/standards/ui-stack-standard.md` (Tailwind, shadcn-svelte).
- **Riferimento visivo Poketrack:** `docs/project/poketrack-reference.md`.

---

## Dove è applicato oggi

- **Home** (`src/routes/+page.svelte`): empty state "nessun profilo" con card centrata, titolo "Crea il tuo primo allenatore", CTA outline "Crea nuovo allenatore".
- **Layout** (`src/routes/+layout.svelte`): padding area contenuto via `.main-content-inner`.

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
| Allenatore   | Dati allenatore, Cartella salvataggi, Statistiche / Pokedex. Card o tab se il contenuto cresce. |
| Editor       | File aperto \| Panello modifica (Pokémon / Allenatore / Inventario / …) \| Validazione. Panel, non solo card. |
| Wiki         | Categorie (Pokémon, Nature, Mosse, …). Liste e scheda dettaglio come card.                    |
| Impostazioni | Profili, App, Risorse, Backup, … — un gruppo = una o più card.                                |

### Quando usare Card, paragrafi, tabelle

| Elemento   | Uso                                                                 | Esempi |
|------------|---------------------------------------------------------------------|--------|
| **Card**   | Blocco logico con titolo (e opzionale descrizione): unità di informazione o azione. Contenitore standard per quasi tutta l’UI che non è editor o testo lungo. | Empty state, "Dati allenatore", "Profili", "Scheda Pokémon", "Opzione notifiche", voci Wiki. |
| **Paragrafi** | Testo continuo da leggere: descrizioni, flavour, note, help. **Dentro** una card o sotto titolo di sezione. | Descrizione mossa, testo "Come usare", note in Impostazioni. |
| **Tabelle**   | Dati tabellari: molte righe, stesse colonne. Di solito **dentro** una card ("Statistiche base", "Mosse apprese"). | Lista mosse (nome, tipo, PP, potenza), stats base, confronto nature. |

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
