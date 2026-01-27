# Design UI/UX - PokeTracker

## Obiettivo

Definisce lo stile, l'interfaccia utente e l'esperienza utente dell'applicazione PokeTracker.

## Target Utenti

- Età: 20-30 anni
- Competenza: Usa tecnologia attivamente
- Gaming: Gioca a Pokemon in maniera competitiva
- Aspettative: Interfaccia professionale, moderna, non banale

## Principi di Design

### NO

- ❌ Stile banale
- ❌ Stile infantile
- ❌ Interfaccia formale/rigida
- ❌ Design datato

### SI

- ✅ Stile moderno 2026
- ✅ Interfaccia professionale ma diretta
- ✅ Design pulito e chiaro
- ✅ Riferimenti al gioco Pokemon

## Stile Base

### Riferimento VS Code

Prendere ispirazione da VS Code per:
- Layout pulito e chiaro
- Organizzazione spaziale efficiente
- Tipografia leggibile
- Palette colori sobria ma efficace
- Interfaccia funzionale e professionale

### Sensazione "In Game"

Integrare elementi visivi Pokemon per dare sensazione di prodotto "in game":
- Layout simili ai giochi Pokemon
- Colori ispirati ai giochi Pokemon
- Badge e icone ufficiali Pokemon
- Immagini ufficiali Pokemon quando appropriato
- Elementi UI che richiamano l'estetica dei giochi

## Elementi Design

### Colori

- Palette moderna e professionale
- Colori ispirati ai giochi Pokemon (non copiati, ma ispirati)
- Contrasto adeguato per leggibilità
- Coerenza cromatica in tutta l'app

### Tipografia

- Font moderni e leggibili
- Gerarchia tipografica chiara
- Dimensioni appropriate per leggibilità
- Stile professionale ma accessibile

### Icone e Immagini

- Icone ufficiali Pokemon quando possibile
- Immagini ufficiali Pokemon per elementi chiave
- Badge e simboli riconoscibili dai giochi
- Stile coerente in tutta l'app

### Layout Generale

L'app ha una struttura a tre parti: **Top Bar**, **Sidebar collassabile**, **Area contenuto**.
Layout ispirato a VS Code: organizzazione spaziale chiara, gerarchia massimo due livelli in sidebar.

**Scroll:** solo l’**Area contenuto** ha contesto di scroll; Top Bar e Sidebar restano **fissi** e non si muovono quando l’utente scrolla (vedi [Comportamento scroll](#comportamento-scroll)).

### Top Bar

Controlli globali e stato dell'app. Da sinistra a destra (o secondo ordine visivo adottato):

| Elemento | Funzione |
|----------|----------|
| **Pulsante aggiornamento app** | Avvia il check e l’installazione di aggiornamenti (gestione interna app). |
| **Pulsante avvisi** | Apre il pannello avvisi: errori/azioni da correggere e ultimi aggiornamenti. |
| **Selettore profilo** | Cambia il profilo/allenatore attivo; i dati in sidebar e contenuto si aggiornano di conseguenza. |

**Avvisi (best practice):**
- Separare visivamente **errori / cose da correggere** (priorità alta, badge/colore dedicato) da **ultimi aggiornamenti** (informativo).
- Un unico pannello con due gruppi o tab è sufficiente.
- Evitare troppe notifiche non critiche per ridurre affaticamento.

### Sidebar

Navigazione principale, **collassabile** per ridurre l’uso di spazio. Voci e sottovoci:

| Voce | Sottosezioni | Note |
|------|--------------|------|
| **Profilo** | Dashboard, Statistiche, Pokedex | Dati dell’allenatore selezionato dal selettore in Top Bar. |
| **Editor** | — | Editor salvataggi in stile PKHeX; usa librerie PKHeX per leggere/modificare `.sav`. |
| **Wiki** | Walkthrough, Pokemon, Nature, Mosse | Wiki offline; sottosezioni allineate a [features.md](./features.md) e [wiki-offline.md](./wiki-offline.md). |
| **Impostazioni** | — | Configurazione app e gestione interna (backup, risorse, log, ecc.). |

**Ordine voci:** Profilo → Editor → Wiki → Impostazioni (prima “il mio allenatore”, poi strumenti, riferimento, sistema).

### Area Contenuto

- Area principale per il contenuto della sezione attiva.
- Layout adattabile in base alla voce selezionata (dashboard, editor, wiki, ecc.).
- Comportamento responsive dove applicabile.

### Comportamento scroll

Lo scroll è **limitato alla sola Area contenuto**. Top Bar e Sidebar non partecipano allo scroll.

| Zona | Scroll | Comportamento |
|------|--------|----------------|
| **Top Bar** | No | Sempre visibile in alto, posizione fissa. |
| **Sidebar** | No | Sempre visibile (aperta) o nascosta quando collassata; posizione fissa, non scorre con il contenuto. |
| **Area contenuto** | Sì | Unico contesto scrollabile; overflow verticale (e orizzontale se necessario) gestito qui. |

**Scrollbar non visibili:**
- Le scrollbar **non devono essere visibili** in condizioni normali. L'utente scrolla con rotella, touch, trackpad o drag senza vedere la barra.
- Comportamento desiderato: scrollbar nascoste di default; eventuale comparsa **solo al passaggio/hover** sull'area scrollabile, se si vuole dare un hint di scrollabilità, oppure sempre nascoste.
- In ogni caso evitare scrollbar sempre visibili che occupano spazio e impattano l'estetica.

**Implementazione (riferimento):**
- Top Bar e Sidebar fuori dal contenitore scrollabile; area contenuto in un contenitore con `overflow: auto` (o equivalente) che occupa lo spazio restante.
- Evitare scroll sulla viewport principale (`body`/root): lo scroll avviene solo dentro l’area contenuto, così barra e sidebar restano fisse.
- Nascondere le scrollbar via CSS: `scrollbar-width: none` (Firefox), `-ms-overflow-style: none` (IE/Edge), `::-webkit-scrollbar { display: none }` (Chrome/Safari/Electron), o equivalente nel framework UI scelto. Lo scroll resta attivo con rotella/touch.

### Navigazione

- **Sidebar**: gerarchia chiara (voce → sottovoci), al massimo due livelli.
- **Top Bar**: azioni rapide e stato globale (aggiorna, avvisi, profilo).
- Transizioni intuitive tra sezioni; stato di navigazione persistente dove utile.

## Obiettivo Finale

Creare un prodotto che sia:
- **Bello**: Esteticamente piacevole e moderno
- **Moderno**: Design 2026, non datato
- **Professionale**: Adatto a pubblico tecnico e competitivo
- **In Game**: Con riferimenti visivi ai giochi Pokemon
- **Funzionale**: Interfaccia che funziona bene

## Note

Ricerche approfondite su UI/UX verranno fatte successivamente. Questo documento definisce la direzione generale del design.
