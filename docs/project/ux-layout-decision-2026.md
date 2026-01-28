# Valutazione UX: layout e navigazione per PokeTracker (2026)

## Obiettivo

Verificare se la struttura attuale (Top Bar + Sidebar + Area contenuto, voci: Home → Allenatore → Editor → Wiki → Impostazioni) è la **soluzione migliore per l’esperienza utente nel 2026**, date le feature e i job-to-be-done dell’app.

---

## Cosa fa l’app (in sintesi)

- **Tracker / dashboard**: vedere dati del proprio allenatore (Pokedex, salvataggi, cartella) estratti dai `.sav`.
- **Editor**: modificare file salvataggio in stile PKHeX (Pokémon, allenatore, inventario, eventi).
- **Riferimento**: Wiki offline (Pokémon, nature, mosse, strategie).
- **Gestione**: multi-profilo, cartella main (opzionale) + percorsi salvataggi per profilo, backup, risorse, impostazioni.
- **Utente tipo**: chi gioca a Pokémon e vuole tracciare/modificare i save, spesso offline; può essere un singolo utente con più “allenatori” o più persone sullo stesso PC.

---

## Job-to-be-done principali

| Job | Dove oggi | Frequenza tipica |
|-----|-----------|-------------------|
| “Vedere il mio Pokedex / i miei save” | Home (se con profilo) o Allenatore | Alta |
| “Modificare un save” | Editor | Media–alta |
| “Cercare info su un Pokémon/mossa/natura” | Wiki | Media |
| “Aggiungere profilo, gestire cartella main / percorsi, backup” | Impostazioni | Bassa |
| “Prima volta: creare il primo allenatore” | Home (empty state) | Una tantum |

L’utente spesso **alterna** contesti: guarda Allenatore → apre Editor per un save → consulta Wiki → torna ad Allenatore. Il profilo (chi sono “io” in questa sessione) deve restare sempre evidente.

---

## Alternative considerate

### 1. Tab orizzontali in alto (solo tab, niente sidebar)

- **Pro**: poche distrazioni, tutto lo spazio al contenuto.
- **Contro**: con 5 voci (Home, Allenatore, Editor, Wiki, Impostazioni) le etichette si affollano; su finestre strette si degrada. Le tab funzionano meglio con 3–4 voci o con “More”.
- **Verdict**: non ideale per 5 sezioni di pari livello con nomi lunghi. Best practice 2024–2025 (NNGroup, SAP): limitare tab a 5–7 item e preferire overflow; qui la sidebar gestisce meglio 5 voci.

### 2. Hub centrale (una sola “Home” da cui si va agli strumenti)

- **Pro**: un solo punto di partenza, buono per first-time.
- **Contro**: ogni passaggio Editor ↔ Wiki ↔ Allenatore richiederebbe “torna all’hub → scegli destinazione”. Aumenta i click per chi lavora spesso in più contesti.
- **Verdict**: utile **dentro** Home (dashboard con link rapidi), non come unica forma di navigazione principale.

### 3. Wizard / task-based

- **Pro**: ottimo per flussi lineari (es. “configura profilo in 3 step”).
- **Contro**: le sessioni tipiche sono lunghe (modifica save, consultazione wiki), non “completa 3 step e esci”. Il wizard non sostituisce la nav strutturale.
- **Verdict**: da usare per sotto-flussi (onboarding, configurazione guidata), non per la shell globale.

### 4. Sidebar persistente (scelta attuale)

- **Pro**: orientamento chiaro, switch tra Home / Allenatore / Editor / Wiki / Impostazioni in un click; profilo in Top Bar sempre visibile; pattern familiare (VS Code, Slack, Notion, Discord). Scalabile a sottovoci (es. Wiki → Pokémon / Nature / Mosse) con max 2 livelli.
- **Contro**: occupa ~220px; su schermi piccoli si può collassare (già previsto).
- **Verdict**: **allineato alle best practice 2024–2026** per app desktop con 4–6 aree funzionali distinte e utilizzo che alterna contesti.

---

## Perché questa struttura è una buona soluzione per il 2026

1. **Corrispondenza con i job**: ogni voce ha un job chiaro (entry/dashboard, dati allenatore, editing, riferimento, gestione). Niente sovrapposizioni confuse.
2. **Profilo sempre in evidenza**: selettore in Top Bar → switch contesto senza entrare in Impostazioni. Fondamentale per multi-profilo.
3. **Ordine sensato**: Home (entry/dashboard) → Allenatore (“il mio”) → strumenti (Editor, Wiki) → sistema (Impostazioni). Coerente con “identity first, then tools, then system”.
4. **Pattern riconoscibile**: stessa shell ovunque, cambia solo il contenuto → basso cognitive load. Standard per tool desktop complessi.
5. **Scalabilità**: sottovoci (es. Wiki) gestibili con gruppi espandibili; procedure e standard già prevedono max 2 livelli.
6. **Accessibilità e densità**: sidebar collassabile, focus e aria su nav; 220px è un buon compromesso tra leggibilità e spazio per il contenuto.

Rispetto a “tab solo” o “solo hub”, la **sidebar + Top Bar** è la soluzione più adatta a un’app con questi domini e questi utilizzi.

---

## Miglioramenti concreti per “più 2026”

Questi interventi rendono l’esperienza ancora più coerente con le aspettative 2026, **senza cambiare** la scelta “sidebar + Top Bar”.

### 1. Home come dashboard quando esiste un profilo

- **Ora**: Home è empty state (nessun profilo) o contenuto generico.
- **2026**: quando c’è un profilo, Home è una **dashboard** (hub della sessione): sintesi Pokedex, ultimo save / cartella, link rapidi “Apri editor”, “Cerca in Wiki”. Così Home diventa il “centro” e le altre voci sono approfondimenti/strumenti. Riduce la sensazione “ma dove guardo per avere il quadro?”.

### 2. Ricorda ultima sezione (opzionale, preferenza)

- All’avvio, aprire l’ultima sezione usata (es. Editor se l’utente chiude spesso da lì) oppure sempre Home; eventualmente rendere configurabile in Impostazioni (“All’avvio apri: Ultima sezione / Sempre Home”).

### 3. Scorciatoie da tastiera

- **Alt+1 … Alt+5** (o **Cmd/Ctrl+1 … 5**) per le 5 voci. Standard per tool desktop 2026; utenti power-user ne beneficiano subito.

### 4. Sottovoci Wiki senza inflazione sidebar

- Quando arriveranno (Pokémon, Nature, Mosse, ecc.): **un’unica voce “Wiki”** con sottovoci espandibili (max 2 livelli, come da procedure), oppure Wiki come pagina unica con tab/filtri interni. Evitare 10 voci in sidebar.

### 5. Coerenza “Allenatore” ovunque

- Etichetta “Allenatore” in sidebar e in tutti i testi UI quando si parla del profilo/giornata di gioco; “Profilo” resta il termine tecnico (selettore, Impostazioni, doc). Già allineato in glossary; mantenerlo in copy e messaggi.

---

## Riepilogo

- **Domanda**: per un’app come PokeTracker (tracker + editor + wiki + multi-profilo + gestione), è questa la soluzione migliore per l’esperienza utente nel 2026?
- **Risposta**: **sì**. La struttura Top Bar + Sidebar + Area contenuto, con le 5 voci Home → Allenatore → Editor → Wiki → Impostazioni, è adatta ai job-to-be-done, all’uso reale (molti switch di contesto) e alle best practice 2024–2026 per app desktop complesse. I miglioramenti sopra non stravolgono la struttura, la rendono più “dashboard-oriented” e più comoda per uso ripetuto.

---

## Riferimenti

- `docs/project/glossary.md` — Termini layout e nav
- `docs/project/priorities.md` — Priorità (UX in “funzionalità”)
- `docs/project/ui-patterns-applied.md` — Pattern UI e best practice
- `docs/standards/design-system-standard.md` — Dimensioni e token layout
- NNGroup: Tabs vs vertical nav; SAP Design System: tab/sidebar usage; Baymard (desktop productivity).

## Data

2026-01-28
