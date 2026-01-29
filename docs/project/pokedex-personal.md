# Feature: Pokedex Personale

## Obiettivo

Documenta la feature del Pokedex personale che mostra lo stato di completamento del Pokedex dell'allenatore basato sui dati dei salvataggi.

## Descrizione

Il Pokedex personale è una delle feature principali dell'app. Crea un Pokedex personalizzato per ogni allenatore basato sui dati estratti dai file salvataggio.

## Funzionalità

### Creazione Pokedex

Il Pokedex viene creato automaticamente dai dati dei file `.sav`:
- Analizza i Pokemon incontrati/catturati dall'allenatore
- Aggiorna lo stato di ogni Pokemon nel Pokedex
- Mantiene lo stato anche quando l'app è offline

### Stati Pokemon

Ogni Pokemon nel Pokedex può avere tre stati (come nei giochi originali):

- **Non visto**: Pokemon non ancora incontrato dall'allenatore
- **Visto**: Pokemon visto ma non catturato
- **Catturato**: Pokemon catturato dall'allenatore

### Visualizzazione

Il Pokedex mostra:
- Icone/immagini di tutti i Pokemon
- Stato di ogni Pokemon (non visto/visto/catturato)
- Informazioni sui tipi con icone
- Icona della classica Pokeball per Pokemon catturati

### Struttura UI: empty state vs griglia

- **Empty state (nessun dato dal save):** se il profilo non ha ancora nessuna riga in `pokedex_state` (nessuna sync/watcher mai eseguita), mostrare **empty state** "Popola il Pokedex" con CTA "Vai a Salvataggi" — non mostrare la griglia di 493 tile. Così l’utente che ha creato un profilo ma non ha attivato il watcher non vede "tutti i Pokemon" in stato ???.
- **Griglia (dati presenti):** quando `get_pokedex_state` restituisce almeno una riga (almeno una sync è stata fatta), mostrare la **Card di sezione** "Pokedex" con la griglia di tile.
- **Pagina:** una sola Card "Pokedex" (titolo + descrizione) che contiene la griglia. Non usare una Card shadcn completa per ogni Pokemon (troppo pesante per 493 voci).
- **Tile = singolo Pokemon:** blocco compatto in griglia (stesso linguaggio visivo: bordo, `--bg-tertiary`, hover). Responsive: es. `grid-cols-2 sm:grid-cols-3 md:grid-cols-4 lg:grid-cols-5 xl:grid-cols-6 gap-4` (design-system, responsive-design-standard).

### Contenuto di ogni tile (info in card/tile)

| Info | Obbligatorio | Dove | Note |
|------|--------------|------|------|
| **Sprite** | Sì | Sopra o a sinistra | `src="/pokedex-sprites/{id}.png"`, `alt="{nome}"`. Dimensione fissa (es. 64×64), `object-contain`. |
| **Numero** | Sì | Vicino al nome | Formato `#001` … `#493`. `text-xs text-muted-foreground`. |
| **Nome** | Sì | Sotto lo sprite o a destra | Nome specie (es. Bulbasaur). `text-sm font-medium`. |
| **Stato** | Sì | In evidenza (icona o label) | **Non visto** / **Visto** / **Catturato**. Icona: es. cerchio vuoto / occhio / Pokeball (o CheckCircle). Colori stato da design-system; per "Catturato" opzionale icona Pokeball. |
| **Tipi** | Opzionale (quando ci sono icone) | Sotto nome o in fondo tile | 1–2 tipi (Fuoco, Acqua, …). Badge o icone tipo; rimandare se asset non pronti. |

- **Accessibilità:** ogni tile con `role="article"` o link/section; sprite con `alt` descrittivo; stato non solo a colori (icona + tooltip o label).
- **Hover:** transizione `--transition-default`; superficie rialzata (`--hover-bg`) come da design-system. Click (futuro): aprire dettaglio o Wiki.

### Far assomigliare il Pokedex a quelli dei giochi (mantenendo stile moderno)

Obiettivo: **stesso “senso”** del Pokedex in-game (registro, progressione, completamento) con **estetica da tool desktop** (design-system: moderno, professionale, elegante, sofisticato). Non copiare pixel o palette da gioco.

**Cosa prendiamo dai giochi (semantica e struttura):**

| Aspetto | Nei giochi | Nel nostro Pokedex |
|---------|------------|---------------------|
| **Registro ordinato** | Lista per numero nazional (#001, #002, …) | Griglia in ordine di id; numero in evidenza (#001) su ogni tile. |
| **Tre stati** | Non visto / Visto / Catturato | Stessi tre stati; differenza visiva chiara ma non “giocosa” (icona + colore stato, es. muted / accent / success). |
| **Non visto = mistero** | Spesso silhouette o “???” | Sprite in **grigio/silhouette** (CSS `filter: grayscale(1) brightness(0.5)` o simile) oppure placeholder; nome opzionale “???” se si vuole fedeltà. Mantenere leggibilità (contrasto). |
| **Visto vs Catturato** | Visto = scheda sbloccata, Catturato = Pokéball / check | **Visto:** sprite a colori, icona “visto” (es. occhio o cerchio). **Catturato:** icona Pokéball o CheckCircle (token `--icon-success`), sprite a colori. |
| **Completamento** | “X catturati su Y” | **Header o summary:** contatore tipo “Catturati: 42 / 493” (o “Visti: 120 / 493”) in stile sobrio (text-muted, numeri in evidenza). Una sola riga; niente badge giganti. |
| **Dettaglio su selezione** | In-game: scheda con descrizione, tipo, altezza/peso | Click su tile → pannello laterale o dialog con dettaglio (nome, numero, tipi, stato, eventuale flavour). Stile card/dialog design-system. |

**Cosa non copiamo (estetica da gioco):**

- Niente pixel art UI, bordi “game”, font da gioco, palette vivaci da handheld.
- Niente illustrazioni infantili; niente “Pokédex rosso” come sfondo card.
- Icone: lineari e minimali (Lucide / product icons); Pokéball per “catturato” può essere icona stilizzata o CheckCircle per coerenza con il resto dell’app.

**Riepilogo visivo per stato (mantenendo stile elegante):**

- **Non visto:** tile con sprite in grigio/silhouette (o placeholder), numero #XXX, nome “???” o nome comunque visibile; icona stato “non visto” (cerchio vuoto / minus). Colori muted.
- **Visto:** sprite a colori, numero, nome, icona “visto” (occhio o cerchio tratteggiato). Testo normale.
- **Catturato:** sprite a colori, numero, nome, icona Pokéball o CheckCircle (verde `--icon-success`). Leggera enfasi (es. bordo sottile o sfondo lieve) per “completato”.

Così il Pokedex **assomiglia** a quello dei giochi (registro, numeri, tre stati, completamento, dettaglio) ma resta **moderno, professionale ed elegante** grazie a tipografia, token, icone e palette del design-system.

## Risorse Necessarie (tutto offline)

L'app **non** usa CDN né URL esterni a runtime. Tutte le risorse sono locali (bundle o pre-scaricati).

### Sprite Pokemon (Pokedex)

- **Fonte e path:** vedi [pokedex-sprites](./pokedex-sprites.md). Sprite gen 1–4 (id 1–493) in `static/pokedex-sprites/{id}.png`, inclusi nel bundle.
- **Come ottenerli:** eseguire `npm run pokedex-sprites` (script che scarica da PokeAPI/sprites); i file vanno in `static/` e restano offline.
- **In app:** riferimento solo path locali (es. `/pokedex-sprites/1.png`); nessun fallback su rete.

### Altre risorse (future)

- **Icone tipi**: tutti i tipi Pokemon (Fuoco, Acqua, Erba, etc.) — bundle o cartella locale.
- **Icona Pokeball**: per indicare Pokemon catturati — già in bundle (es. icone app) o asset in `static/`.

## Integrazione con Dati Salvataggio

Il Pokedex si aggiorna automaticamente quando:
- Vengono analizzati nuovi file salvataggio
- I file salvataggio vengono modificati
- L'app rileva cambiamenti nei dati dell'allenatore

## Note

Questa feature è fondamentale per l'esperienza utente. Richiede:
- Database completo (o lista) Pokemon per la fascia in uso (es. gen 1–4)
- Sprite e icone **solo locali** come da [pokedex-sprites](./pokedex-sprites.md)
- Integrazione con il sistema di analisi salvataggi (stato visto/catturato)
