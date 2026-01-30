# Allenatore — Dashboard di atterraggio

## Obiettivo

Definisce **scopo, dati e layout** della sezione Allenatore: dashboard di atterraggio dell'app e controllo veloce sul profilo attivo. Il punto di partenza è la **card Allenatore**, ispirata alla **Trainer Card** dei giochi Pokémon (layout e info). Design coerente con il resto dell'app (professionale, moderno, elegante, raffinato).

**Riferimenti:** [glossary.md](./glossary.md) (Allenatore = vista dati profilo attivo), [ui-patterns-applied.md](./ui-patterns-applied.md) (sotto-sezioni Allenatore, card in stack), [design-system-standard.md](../standards/design-system-standard.md). **Fonti esterne:** Bulbapedia [Trainer Card (game)](https://bulbapedia.bulbagarden.net/wiki/Trainer_Card_(game)), Serebii Diamond/Pearl Trainer Card.

---

## Come viene visualizzato nei giochi Pokémon (Trainer Card)

Nei giochi, la **Trainer Card** (Scheda Allenatore) è la schermata che mostra l'identità e i progressi dell'allenatore. Layout e informazioni sono coerenti tra le generazioni (con variazioni per Gen).

### Informazioni mostrate (Bulbapedia — "All generations")

- **Nome del giocatore** (OT)
- **Denaro** (soldi in tasca)
- **Tempo di gioco** (ore/minuti totali)
- **Badge** (numero e/o icone; in Gen I–II volti capopalestra poi sostituiti dai badge)
- Da Gen II: **Trainer ID** (numero a 5 cifre)
- Da Gen II: **Pokédex** — in Gen II/III/HGSS numero **catturati**; in Diamante/Perla/Platinum e Gen V/VI numero **visti**

### Layout tipico nei giochi

- **Identità in evidenza:** immagine del personaggio giocante (sprite) + nome. Nei titoli DS (Gen IV) la card occupa uno schermo con ritratto e dati affiancati o su due livelli.
- **Blocco statistiche:** denaro, tempo, badge, Pokédex (e in Gen IV: data inizio avventura, data sconfitta Elite Four; "Trainer Case" con badge separati sull'altro schermo).
- **Gen IV (D/P/Pt):** card con stelle (0–5) in base ai traguardi; badge mostrati in una sezione dedicata (Trainer Case); retro della card con firma/grafica.
- **Aspetto:** la card è un "documento" riconoscibile: titolo "Trainer Card" / "Scheda Allenatore", bordi/forma da tessera, informazioni ordinate (non un semplice elenco).

In sintesi: **identità (nome + ritratto) + denaro + tempo + badge + Pokédex**; layout a "tessera" con zona identità e zona dati.

---

## Card Allenatore (punto di partenza)

La **card Allenatore** in PokeTracker riprende **layout e tipi di info** della Trainer Card dei giochi, adattati al design system (dark, token, tipografia professionale) e ai dati che abbiamo o avremo.

---

## Allineamento stile Trainer Card ↔ stile app

Obiettivo: mantenere il **concetto** e la **struttura** della Trainer Card (identità + blocco dati, tipi di info), ma **aspetto visivo** 100% coerente con il design system (tool desktop, dark, professionale). Nessuna copia pixel dei giochi; riconoscibilità tramite struttura e contenuto, non tramite colori o illustrazioni da gioco.

### Cosa prendiamo dalla Trainer Card (giochi)

| Aspetto | Nei giochi | In PokeTracker (concetto) |
|--------|------------|---------------------------|
| **Struttura** | Zona identità (sprite + nome) + blocco dati (denaro, tempo, badge, Pokédex) | Stessa suddivisione: **identità in alto** (nome + avatar), **blocco dati sotto** (label + valore per ogni campo). |
| **Tipi di info** | Nome OT, denaro, tempo, badge, Pokédex, Trainer ID | Stessi campi (nome, denaro, tempo, badge, Pokédex; TID opzionale). Ordine e label allineati al gioco dove ha senso. |
| **Idea "tessera"** | Card come documento riconoscibile, due aree visive distinte | Una sola **Card** shadcn con due blocchi interni (identità vs dati), senza decorazioni da gioco. |
| **Gerarchia** | Nome/ritratto in evidenza, numeri ordinati | Nome in evidenza (titolo o text-lg), numeri in blocco compatto (label muted + valore in primo piano, come PokedexStats). |

### Cosa adattiamo allo stile app (design system)

| Aspetto | Nei giochi | In PokeTracker (regola) |
|--------|------------|--------------------------|
| **Colori** | Palette chiara/colorata da videogioco, sfondi illustrati | Solo token dark: `--bg-primary`, `--bg-tertiary`, `--border-primary`, `--text-primary`, `--text-secondary` (muted). Nessun colore "da gioco". |
| **Tipografia** | Font da gioco, dimensioni variabili | `--font-primary` (Segoe UI, SF Pro, system-ui), `--font-size-body` (13px); titoli `text-lg font-semibold`. Title case. Niente font display o giocosi. |
| **Card / contenitore** | Forma "tessera" illustrata, bordi colorati, stelle/badge decorative | Componente **Card** shadcn: `rounded-xl`, `border`, `shadow-sm`, sfondo da token (`bg-card` / `--bg-tertiary`). Stesso stile delle card Impostazioni e di PokedexStats. |
| **Icone / illustrazioni** | Sprite personaggio, icone badge, stelle, firma | Avatar: sprite da `profile-sprites` (se `avatar_id`) o placeholder neutro. Icone: solo Lucide, lineari, minimali; colore da token (`--icon-success` per stato ok, nessuna icona "da gioco"). |
| **Numeri / KPI** | Stile variabile per gioco | Stile come **PokedexStats**: label `text-[10px] font-medium uppercase tracking-wider text-muted-foreground`, valore `text-4xl font-medium tabular-nums`; divisori sottili, barra progresso ultra-sottile se serve. |
| **Spaziature** | Variabili | Solo `--spacing-*` (xs … xl); `gap-6` tra sezioni nella card, `space-y-3` / `gap-3` tra elementi nel blocco dati. |
| **Transizioni** | Animazioni gioco | Solo `--transition-default` (200ms ease-out) per hover/focus; rispettare `prefers-reduced-motion`. |

### Riepilogo regole per la card Allenatore

1. **Struttura:** identità (nome + avatar) in alto; blocco dati sotto con righe label + valore (Denaro, Tempo, Badge, Pokédex; campi assenti = "—" o omessi).
2. **Token:** sfondo card e blocchi = `--bg-tertiary` o `bg-card`; bordi = `--border-primary`; testo = `--text-primary` / `--text-secondary`; nessun colore fuori palette.
3. **Tipografia:** corpo 13px, titolo nome `text-lg font-semibold`, label dati `text-sm text-muted-foreground`, numeri in evidenza come in PokedexStats.
4. **Componenti:** Card, CardHeader, CardContent da `$lib/components/ui/card`; nessun div custom per "tessera"; eventuale riuso di PokedexStats per la riga Pokédex.
5. **Niente "game":** niente illustrazioni da gioco, niente palette colorata da Trainer Card originale, niente stelle/badge decorative; riconoscibilità solo da struttura e testi.

Riferimenti: [design-system-standard.md](../standards/design-system-standard.md), [ui-patterns-applied.md](./ui-patterns-applied.md) (Card di sezione), `src/lib/components/pokedex/PokedexStats.svelte` (stile numeri e barra).

---

### Layout proposto per la card

1. **Zona identità (in evidenza)**  
   - **Nome:** oggi nome **profilo** (`activeProfile.name`); in futuro, quando estratto dal save, nome **allenatore in-game (OT)** con fallback al nome profilo.  
   - **Ritratto/avatar:** oggi `avatar_id` profilo se impostato (sprite da `profile-sprites`); in futuro opzionale sprite in-game se esposto dal parser.  
   - Posizionamento: in alto nella card, ben leggibile (titolo o sottotitolo di sezione).

2. **Blocco dati (stile Trainer Card)**  
   Ordinato come nei giochi; solo i campi per cui abbiamo fonte (oggi o dopo estrazione save):

   | Campo        | Nei giochi | In PokeTracker (oggi) | In PokeTracker (futuro) |
   |-------------|------------|------------------------|-------------------------|
   | Nome        | OT         | Nome profilo           | Nome OT da save (fallback profilo) |
   | Denaro      | Money      | —                      | Se parser espone `sav.Money` |
   | Tempo gioco | Played     | —                      | Se parser espone `PlayedHours`/`Minutes` |
   | Badge       | Badges     | —                      | Se parser espone numero badge |
   | Pokédex     | Seen/Caught| Sì (da `getPokedexState`) | Stesso + eventuale "visti" da save (DP/Pt) |
   | Trainer ID  | TID        | —                      | Opzionale (di solito non in primo piano in UI) |

   Oggi nella card mostriamo: **nome profilo**, **avatar** (se presente), **Pokédex** (catturati / visti / totale, es. con PokedexStats o riga compatta). I campi denaro, tempo, badge restano **placeholder o assenti** fino a quando il parser non espone dati allenatore dal save (vedi [analisi-dati-estraibili-pkhex.md](../temp/analisi-dati-estraibili-pkhex.md)).

3. **Aspetto "tessera"**  
   - Card unica (shadcn Card) con titolo tipo "Allenatore" o il nome come titolo.  
   - Contenuto diviso in due aree visive: identità (nome + avatar) e blocco dati (numeri/etichette).  
   - Bordi e sfondo con token design-system; nessun effetto "giocoso", ma struttura riconoscibile (come una scheda riassuntiva).

### Cosa implementare per la card Allenatore (v1)

- **Sempre:** nome profilo, eventuale avatar (`avatar_id`), stato Pokedex (catturati, visti, totale) — dati già disponibili.
- **Placeholder opzionali:** "Denaro", "Tempo", "Badge" con trattino o "—" e label chiare, per riservare spazio e ricordare il modello Trainer Card; oppure omessi fino all'estrazione dal save.
- **Layout:** zona identità in alto (nome + avatar); sotto, blocco compatto con Pokedex (e, se si vogliono placeholder, righe Denaro / Tempo / Badge con "—").

### Dati da mostrare (fonti attuali)

| Blocco        | Dati                                      | Fonte |
|---------------|-------------------------------------------|--------|
| **Identità**  | Nome profilo; avatar                      | `activeProfile` (name, avatar_id) |
| **Pokédex**  | Catturati, visti, totale (493)            | `getPokedexState(profileId)` → conteggi; totale = `POKEDEX_GEN4_MAX`. Riuso **PokedexStats** o riga compatta. |
| **Denaro / Tempo / Badge** | Opzionale placeholder "—" | Quando il parser espone dati allenatore dal save (analisi-dati-estraibili-pkhex). |

---

## Ruolo della sezione

- **Allenatore** = prima schermata dopo l’ingresso (Home `/` reindirizza a `/profilo`).
- È la **dashboard** del profilo attivo: panoramica e accesso rapido, non gestione (crea/rinomina/elimina profili → Impostazioni).
- L’utente vede subito: chi è l’allenatore, quanti salvataggi, quanti osservati, avanzamento Pokedex, e può andare in profondità con un click.

---

## Dati da mostrare (fonti attuali)

Tutti i dati sono già disponibili da servizi e store; nessun nuovo comando backend per la sola dashboard.

| Blocco logico      | Dati                                      | Fonte |
|--------------------|-------------------------------------------|--------|
| **Profilo / identità** | Nome profilo attivo                       | `activeProfile` (store) → `name` |
| **Salvataggi**     | Numero salvataggi totali; numero percorsi “osservati” (watcher attivo); eventuale lista compatta (gioco, versione, ultimo aggiornamento) | `getSavEntries()`, `getSavWatchedPaths()` |
| **Pokedex**       | Catturati, visti, totale (es. 493)        | `getPokedexState(profileId)` → conteggi per status; totale = `POKEDEX_GEN4_MAX` |
| **Statistiche**   | Placeholder “In arrivo” o link            | Pagina `/profilo/statistiche` ancora vuota |

**Dettaglio Pokedex:** da `PokedexStateEntry[]` (species_id, status) si derivano `caught` (status caught), `seen` (seen + caught), `total` = 493. Il componente **PokedexStats** esiste già e mostra catturati/visti/total con barra di progresso sottile — riutilizzabile in dashboard.

---

## Resto della dashboard (card in stack)

La **prima** card è sempre la **Card Allenatore** (descritta sopra: identità + Pokedex + eventuali Denaro/Tempo/Badge). Poi, in ordine: Salvataggi, Pokedex, Statistiche.

Coerente con [ui-patterns-applied.md](./ui-patterns-applied.md): poche sotto-sezioni → **card in stack** (nessun tab). Una sola area contenuto con scroll; padding da `.main-content-inner` (`--spacing-xl`).

Ordine dall’alto:

1. **Card “Profilo”**  
   - Titolo: Profilo (o il nome dell’allenatore come titolo).  
   - Contenuto: nome profilo (evidenziato), eventuale placeholder avatar se in futuro `avatar_id` è usato.  
   - Descrizione breve: “Profilo attivo per salvataggi e Pokedex.”  
   - Nessuna azione primaria qui (gestione profili in Impostazioni).

2. **Card “Salvataggi”**  
   - Titolo: Salvataggi.  
   - Contenuto:  
     - **Numeri in primo piano:** X salvataggi, Y osservati (es. “3 salvataggi · 2 osservati”).  
     - Opzionale: lista compatta (una riga per save: gioco, versione, `updated_at`) oppure solo i numeri per mantenere la dashboard leggera.  
   - Azione: link/button “Gestisci salvataggi” → `/profilo/salvataggi`.  
   - Descrizione: “Percorsi monitorati per questo profilo.”

3. **Card “Pokedex”**  
   - Titolo: Pokedex.  
   - Contenuto: **PokedexStats** (caught, seen, total) — stesso componente usato in `/profilo/pokedex`.  
   - Azione: link/button “Apri Pokedex” → `/profilo/pokedex`.  
   - Descrizione opzionale: “Stato completamento Gen 1–4.”

4. **Card “Statistiche”**  
   - Titolo: Statistiche.  
   - Contenuto: messaggio “In arrivo” o breve testo; link “Vai a Statistiche” → `/profilo/statistiche`.  
   - Quando la pagina Statistiche avrà contenuto reale, qui si potranno mostrare 1–2 KPI in anteprima.

**Responsive:** su viewport stretta le card restano in colonna singola; stessi token e spaziature (`gap-6` tra card).

---

## Design e coerenza

- **Card:** componente shadcn `Card`, `CardHeader`, `CardTitle`, `CardDescription`, `CardContent`, `CardFooter` (dove serve CTA). Stile “Card di sezione” come Impostazioni e [ui-patterns-applied.md](./ui-patterns-applied.md).
- **Tipografia:** titoli `text-lg font-semibold`, descrizioni `text-sm text-muted-foreground`, numeri in evidenza come in PokedexStats (es. `text-4xl font-medium tabular-nums` per i KPI).
- **Token:** solo token design-system (`--bg-tertiary`, `--border-primary`, `--text-primary`, `--muted-foreground`, `--spacing-*`). Nessun colore o spacing “a mano”.
- **CTA:** `Button variant="outline" size="sm"` per “Gestisci salvataggi”, “Apri Pokedex”, “Vai a Statistiche” (coerente con empty state e card Impostazioni).
- **Icone:** lineari, minimali (Lucide), stile product; eventuali icone per Profilo (User), Salvataggi (FolderOpen), Pokedex (Search o PawPrint), Statistiche (BarChart3) già usate in sidebar.
- **Stato vuoto:** se il profilo non ha salvataggi, la card Salvataggi mostra “0 salvataggi · 0 osservati” e il CTA “Gestisci salvataggi” invita ad aggiungere percorsi. Stesso approccio per Pokedex (0/0/493).

---

## Dati “in arrivo” (non per la prima implementazione)

- Dati estratti dal save in-game (nome allenatore in-game, badge, ora di gioco, ecc.): quando il parser/backend li espone, si potranno aggiungere nella card Profilo o in una card “Gioco” senza stravolgere il layout.
- Statistiche gioco (ore, badge, squadra, ecc.): quando la pagina Statistiche sarà definita, la card Statistiche potrà mostrare un riepilogo (es. “X badge, Y ore”).

---

## Riepilogo dati dashboard (checklist)

| Elemento           | Mostrato in dashboard        | Fonte / nota                          |
|-------------------|------------------------------|----------------------------------------|
| Nome profilo      | Sì, card Allenatore           | `activeProfile.name`                   |
| N° salvataggi     | Sì, card Salvataggi          | `getSavEntries().length`               |
| N° osservati      | Sì, card Salvataggi          | `getSavWatchedPaths().length`          |
| Lista save compatta | Opzionale                   | `getSavEntries()` (game, version, updated_at) |
| Pokedex caught/seen/total | Sì, card Pokedex      | `getPokedexState(profileId)` + PokedexStats |
| Statistiche       | Placeholder + link           | Pagina futuro                         |

---

## Procedura e standard

- Prima di implementare: [docs/procedures/INDEX.md](../procedures/INDEX.md) per eventuale procedura “nuova pagina/sezione”.
- Componenti: riuso `PokedexStats`, Card e Button da `$lib/components/ui`; nessun nuovo componente complesso per la v1.
- Accessibilità: landmark/regioni con `aria-labelledby` sulle card; CTA con testo chiaro e `aria-label` dove serve.

---

## Data creazione

2026-01-30
