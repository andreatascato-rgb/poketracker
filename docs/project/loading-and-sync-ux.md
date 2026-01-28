# Loading e sync — Best practice UX

## Obiettivo

Raccoglie le **best practice UX** per stati di caricamento, feedback visivo e aggiornamenti in background (es. watcher su file .sav). Serve per decidere quando usare overlay, inline, toast e come gestire **più operazioni di sync contemporanee** senza degradare l’esperienza. Fonti: Nielsen Norman Group (NN/g), Material Design, Carbon, Primer, Mozilla Acorn, web.dev, Lightning.

**Uso:** prima di introdurre o modificare loading overlay, spinner, indicatori di sync o feedback per `sav-file-changed` / `sync_sav_now`, consultare questo documento e `docs/project/ui-patterns-applied.md`, `docs/standards/design-system-standard.md`.

---

## Principi generali

### Visibilità dello stato di sistema (NN/g)

- **Requisito:** l’utente deve sempre sapere se il sistema sta lavorando o è fermo.
- Senza feedback, operazioni lente sembrano fallite → doppi click, frustrazione.
- Indicatori di caricamento riducono ansia e **perceived wait time**; gli utenti tollerano attese più lunghe se vedono che qualcosa sta succedendo.

### User-initiated vs system-initiated

| Tipo | Esempio | Feedback tipico |
|------|---------|------------------|
| **User-initiated** | Click "Attiva watcher", "Aggiungi salvataggio", submit form | **Risposta immediata** (ack). Poi loading coerente con durata. L’utente si aspetta conferma. |
| **System-initiated** | `sav-file-changed` → sync, refresh dati in background | **Non interrompere**. Feedback **discreto** (es. piccolo indicatore, toast opzionale). L’utente non ha richiesto l’azione. |

Per **aggiornamenti in background** (watcher che triggera sync): evitare overlay fullscreen, modali o blocchi UI; preferire indicatori **locali** (per riga, badge) o **toast** per successo/errore.

---

## Soglie temporali

### Quando mostrare un indicatore di loading

- **&lt; 200 ms:** **non** mostrare spinner/overlay. Il flash confonde e peggiora la perceived performance.
- **≥ 200 ms:** considerare un indicatore se l’operazione può durare abbastanza da generare dubbi ("è partita?").
- **Minimum display time:** se mostri un indicatore, fallo restare visibile **almeno 400 ms** per evitare flicker fastidioso.

Fonte: Mozilla Acorn; allineato a NN/g (evitare feedback inutile per operazioni brevissime).

### Delay prima di mostrare lo spinner

- **~1 s (Primer):** ritardare la comparsa dello spinner di 1 secondo. Se l’operazione finisce prima, non mostrare nulla.
- **Effetto:** operazioni veloci non mostrano flash; solo le attese “lunghe” ottengono feedback.

### Determinate vs indeterminate

- **Indeterminate** (spinner, barra senza %): quando **durata sconosciuta** o variabile (es. sync, chiamate di rete, parsing).
- **Determinate** (progress bar con %): quando la durata è **nota o stimabile** e **≥ 3 s**. Sotto i 3 s l’utente non ha tempo di leggere la percentuale.

---

## Overlay fullscreen vs inline vs skeleton

### Overlay fullscreen

**Quando usarlo:**

- L’operazione **disabilita l’intera app** (l’utente non può fare nulla nel frattempo).
- Operazione **lunga** (ordine di secondi) e **bloccante**.
- Esempi: submit form “pesante”, export globale, operazione che richiede risorse totali.

**Quando evitarlo:**

- Aggiornamenti in **background** (es. watcher).
- Operazioni **brevi** (&lt; 1–2 s) dove un overlay creerebbe solo flash.
- Desktop: overlay fullscreen può risultare **molto invasivo**; preferire feedback localizzato.

### Inline loading

**Quando usarlo:**

- Operazione **limitata** a un componente (es. una riga, un form, un pulsante).
- L’utente **può continuare** a usare il resto dell’UI.
- Operazioni **brevi** o di durata variabile (spinner piccolo, messaggio tipo "Salvataggio in corso…").

**Regola Carbon:** evitare inline loading su **più di un elemento alla volta**, tranne in caso di **caricamento iniziale** della pagina o **refresh** esplicito.

**Posizionamento:** l’indicatore deve stare **nello stesso punto** del contenuto che sta caricando e mantenere allineamento (no salti di layout).

### Skeleton

**Quando usarlo:**

- **Full‑page** o **blocchi grandi** (liste, tabelle) al **primo caricamento**.
- La **struttura** della pagina è nota; si aspetta solo il contenuto.

**Quando evitarlo:**

- **Singoli componenti** (es. una riga, un card) → preferire spinner.
- Modal o aree che si caricano con il resto della pagina già visibile; per accessibilità e chiarezza spesso è meglio uno spinner contestuale.

**Beneficio:** gli skeleton riducono la **perceived wait** (~20–30%) rispetto a spinner a parità di tempo reale, perché mostrano la forma del contenuto in arrivo.

---

## Aggiornamenti in background e sync

### Non interrompere il focus

- Download, sync, refresh in background devono avvenire **senza bloccare** l’UI.
- **Evitare:** modal, overlay fullscreen, messaggi che richiedono click per chiudere.
- **Preferire:** badge, piccoli indicatori, **toast** per esito (successo/errore).

### Toast per esito di operazioni di routine

- **Successo:** toast breve, auto-dismiss. Adatto a “Salvataggio aggiornato”, “Sync completato”.
- **Errore:** toast con messaggio chiaro; può avere azione “Riprova” o link a dettagli se utile.
- **Attenzione:** i toast sono **poco discoverable** (spesso in angolo, scompaiono). Non usare il toast come **unico** canale per feedback critico; per errori gravi valutare banner o messaggio inline persistente.

### Più elementi che si aggiornano insieme

- **Carbon:** non usare inline loading su più elementi contemporaneamente, **eccetto** load/refresh iniziale.
- **Alternativa 1:** un **unico** indicatore globale discreto, es. “Sincronizzazione 2 salvataggi…” (testo + spinner piccolo), fuori dal flusso principale (es. barra sopra la tabella, piccolo banner).
- **Alternativa 2:** indicatori **per riga** (spinner/icona nella cella “Sync”) solo per le righe in sync. Più granulare ma più complesso; va gestito bene con N righe contemporaneamente.
- **Evitare:** N overlay o N dialog; **evitare** fullscreen overlay per sync avviata dal watcher.

---

## Debounce e concorrenza

### Debounce su eventi rapidi

- Se un file .sav può generare **più eventi** in pochi secondi (es. save multipli dell’emulatore), **raggruppare** gli eventi (debounce) e lanciare **una** sync per path.
- Riduce: carico su sidecar, scritture DB, flicker della UI, troppi toast.

### Serializzare per path (opzionale)

- Se per lo **stesso path** parte una nuova sync mentre la precedente è ancora in corso, si può **ignorare** la nuova o **accodare**.
- Evita race e duplicazione di lavoro; comportamento da documentare (es. “ultima modifica vince” o “queue per path”).

---

## Applicazione al watcher PokeTracker

### Contesto

- **User-initiated:** attivare watcher (toggle) → si fa `sync_sav_now` poi `set_sav_watched`. L’utente si aspetta feedback.
- **System-initiated:** `sav-file-changed` → `sync_sav_now` + `loadSaves`. Nessuna azione diretta dell’utente.

### Raccomandazioni

1. **Toggle “Attiva watcher” (user-initiated)**  
   - Overlay fullscreen **accettabile** solo se la sync può durare **diversi secondi** e blocca concettualmente l’uso della lista.  
   - **Preferibile:** **inline** nella riga (es. spinner al posto dell’icona Sync) oppure piccolo overlay **limitato alla card/tabella** (non tutta l’app), con messaggio “Sincronizzazione in corso…”.  
   - Mantenere **minimum display time** ~400 ms e **delay** ~1 s prima di mostrare lo spinner (se l’operazione finisce prima, niente spinner).

2. **`sav-file-changed` (system-initiated)**  
   - **No** overlay fullscreen.  
   - **Sì** uno dei seguenti:
     - **Indicatore per riga:** spinner/icona nella cella Sync della riga in corso di aggiornamento; alternativa “check” quando idle.
     - **Indicatore globale discreto:** piccola barra/banner “Sincronizzazione N salvataggi…” sopra la tabella, visibile solo quando almeno un sync è attivo.
   - **Successo:** toast **opzionale** e non troppo frequente (es. solo se l’utente è sulla vista Salvataggi, e magari debounce: max un toast ogni X secondi per path).
   - **Errore:** **sempre** toast (o messaggio persistente) con testo chiaro e possibilmente “Riprova”.

3. **Più watcher attivi contemporaneamente**  
   - Evitare **più overlay** o **più modal**.  
   - Usare **un solo** indicatore globale (“Sincronizzazione 2 salvataggi…”) **oppure** indicatori **per riga** per ogni path in sync.  
   - Aggiornare lo stato (rimozione spinner / aggiornamento “N”) non appena ogni sync termina.

4. **Debounce**  
   - Implementare **debounce** su `sav-file-changed` per path (es. 1–2 s). Una sola `sync_sav_now` per path per finestra temporale.

5. **Analisi file (detect) / Aggiungi salvataggio**  
   - Durante `detect_save_game_version` o “Aggiungi salvataggio”: overlay **limitato al dialog** o alla tabella è preferibile a fullscreen. Stesse regole su delay (~1 s) e minimum display (400 ms).

---

## Riepilogo decisioni

| Scenario | Overlay fullscreen | Inline / per riga | Indicatore globale | Toast |
|----------|--------------------|-------------------|--------------------|-------|
| Toggle “Attiva watcher” | Solo se sync molto lenta e bloccante; preferibile no | **Sì** (riga o area tabella) | Alternativa | Successo/errore |
| `sav-file-changed` | **No** | **Sì** (per riga) | **Sì** (alternativa) | Errore sempre; successo opzionale, debounced |
| N sync contemporanee | **No** | Per riga **oppure** | Un solo “N salvataggi…” | Come sopra |
| Aggiungi / Detect | Preferibile no | Dialog / area | — | Successo/errore |

---

## Riferimenti

- [Progress Indicators Make a Slow System Less Insufferable](https://www.nngroup.com/articles/progress-indicators/) (NN/g)
- [Skeleton Screens 101](https://www.nngroup.com/articles/skeleton-screens/) (NN/g)
- [Loading \| Primer](https://primer.style/ui-patterns/loading/)
- [Progress indicators – Material Design 3](https://m3.material.io/components/progress-indicators/guidelines)
- [Inline loading \| Carbon](https://carbondesignsystem.com/components/inline-loading/usage)
- [Loading indicators \| Acorn (Mozilla)](https://acorn.firefox.com/latest/patterns/loading-indicators-SKyqf3Lj) (200 ms / 400 ms)
- [Offline UX design guidelines](https://web.dev/articles/offline-ux-design-guides) (web.dev)
- [Electron Guide – auto-updates](https://www.electronjs.org/docs/latest/api/auto-updater) (background updates, non-interruptive)
- `docs/project/ui-patterns-applied.md` — Empty state, card, CTA
- `docs/standards/design-system-standard.md` — Token, feedback (toast, inline)
- `docs/project/core-functionality.md` — Watcher, sync, persistenza
