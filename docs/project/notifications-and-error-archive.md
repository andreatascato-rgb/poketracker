# Notifiche e Archivio errori

## Obiettivo

Definisce le scelte UX e di prodotto per: (1) dove e come mostrare avvisi/notifiche invece di dati di debug in schermata; (2) standard per errori → notifica + registro in Archivio; (3) Top Bar con icona notifiche e componente dedicato; (4) **Impostazioni → Errori** per log completo da incollare (nessuna voce sidebar «Archivio») (es. ad assistente AI).

**Stato:** scelte definite; implementazione base completata. **Utilità piena** (vedi sotto) estende persistenza, formato detail e caso “Versione non determinata”.

---

## Utilità piena (checklist implementativa)

Definizione operativa per considerare Archivio Errori **completo e utile** come da doc:

1. **Persistenza archivio**
   - Voci persistite in SQLite (tabella `error_archive`); backend source of truth.
   - Comandi: `get_error_archive_entries`, `add_error_archive_entry`, `remove_error_archive_entry`.
   - Pagina Impostazioni → Errori: fetch all’apertura; add/remove tramite invoke.

2. **Caso “Versione non determinata” + languageIdRaw**
   - Nessun “L?” in schermata (dialog Riepilogo salvataggio).
   - Quando il detect va a buon fine ma versione “—” (o vuota) e c’è `languageIdRaw`: toast + voce in Archivio con `detail` strutturato (path, languageIdRaw, message, ecc.) come da “Formato della voce in archivio”.

3. **Detail strutturato**
   - Uso sistematico del formato doc (`type`, `at`, `message`, `path`, campi aggiuntivi) per le voci in archivio.
   - Helper `formatErrorDetail` (o equivalente) e refactor dei `reportSystemError` esistenti dove si hanno dati strutturati (path, languageIdRaw, stderr, ecc.).

4. **Centro notifiche (Top Bar)**
   - Fuori scope per Utilità piena; doc: “quando il componente esiste”.

Riferimento implementativo: store/service `error-archive`, pagina `impostazioni/errori`, procedure `error-handling.md` e `db-migration.md`.

---

## Dove si vede oggi il “fallimento” (es. languageId raw)

Il valore raw di LanguageID restituito dal save (e quindi il “fallimento” nel riconoscere la versione/lingua) è mostrato **solo** in un punto:

- **Pagina:** Allenatore → Salvataggi  
- **Contesto:** Dialog “Riepilogo salvataggio” dopo il detect di un file `.sav`  
- **Condizione:** quando la **Versione** è "—" (o vuota) e il sidecar ha comunque un `languageIdRaw`  
- **UI attuale:** sotto il percorso, una riga “L? &lt;valore&gt;” (es. `L? 0`) con title “LanguageID raw dal save (solo debug)”  
- **File:** `src/routes/profilo/salvataggi/+page.svelte` (blocco condizionale sul dialog)

È l’unico posto in cui l’utente vede quale LanguageID ha restituito il save quando la versione non è stata determinata.

---

## Scelta: niente debug in schermata, ma avviso + notifica + Archivio errori

Invece di mostrare “L?” (e simili) direttamente nel dialog o in altre schermate:

1. **Avviso (transiente):** toast/snackbar breve che segnala che qualcosa non è stato riconosciuto (es. “Versione non determinata; dettagli in Notifiche”).
2. **Notifica (persistente):** la stessa informazione entra nel **centro notifiche** dell’app, apribile da Top Bar.
3. **Se è un errore:** oltre alla notifica, si crea una voce nello **Archivio → Errori** con tutti i dettagli necessari per debug e per copiare/incollare il log (es. ad assistente).

Obiettivo: l’utente non vede stringhe di debug sparse nelle schermate, ma ha un posto unico (notifiche + Archivio Errori) dove trovare e riusare le informazioni quando serve.

---

## Notifiche: Top Bar e componente dedicato

- **Top Bar:** icona dedicata alle notifiche, allineata alle altre (stile coerente con selettore profilo, eventuali altre icone).
- **Comportamento:** al clic sull’icona si apre un **componente dedicato alle notifiche** (pannello/drawer/sheet), non una “mail” esterna. Best practice per app desktop: centro notifiche in-app (come in VS Code, Slack, Notion).
- **Contenuto del componente:** elenco di notifiche (avvisi, errori, info) con possibili azioni (marca come letta, elimina, “apri dettaglio”). Le notifiche di tipo errore possono avere un link/azione “Vedi in Archivio errori” per aprire la voce corrispondente in Archivio → Errori.

Quindi: **avviso** = messaggio transiente (toast); **notifica** = voce persistente nel centro notifiche, apribile dalla Top Bar.

---

## Standard “quando una notifica riguarda un errore”

Quando si genera una notifica di tipo **errore** (es. versione non determinata + languageId raw, sidecar fallito, timeout, ecc.):

1. **Avviso:** mostrare un breve messaggio transiente (toast) per segnalare subito che c’è stato un problema.
2. **Notifica:** aggiungere una voce nel centro notifiche, con titolo/riepilogo e possibilità di aprire il dettaglio.
3. **Archivio errori:** creare una voce in **Archivio → Errori** con:
   - **Data** (e possibilmente ora)
   - **Tipo errore** (es. “Versione non determinata”, “Sidecar timeout”, “File non riconosciuto”)
   - **Icona/pulsante “Apri”** per aprire un componente di **dettaglio errore** con il contenuto completo (messaggio, contesto, stack/backtrace se presenti, languageIdRaw, path, ecc.) in forma copiabile
   - **Azioni:** almeno **Elimina** (icona elimina) per rimuovere la voce dall’archivio

Scopo principale dell’Archivio errori: avere un **log completo** da poter copiare e incollare (es. quando si chiede supporto o si incolla il log all’assistente AI), senza dover cercare dati di debug nelle schermate operative.

---

## Errori in Impostazioni

- **Sezione sidebar:** **Impostazioni** (sottosezioni: Profili, Errori).
- **Sottosezione:** **Errori** (registro errori di sistema; altre sottosezioni Impostazioni possibili in futuro).
- **Route:** `/impostazioni/errori`.
- **Contenuto pagina Errori:**
  - Tabella con colonne: **Data** | **Tipo errore** | **Apri (icona)** | **Azioni (Elimina)**.
  - Clic su “Apri” apre il **componente di visualizzazione dettaglio errore** (drawer/dialog/sheet) con testo completo copiabile.

---

## Esempio: “Versione non determinata” + languageId raw

- **Oggi:** nel dialog Riepilogo salvataggio, se version è "—" e c’è `languageIdRaw`, si mostra “L? &lt;valore&gt;” sotto il percorso.
- **Dopo le scelte:**
  - Non si mostra più “L?” nel dialog.
  - Si mostra un toast tipo: “Versione non determinata per questo salvataggio. Controlla le notifiche.”
  - Si aggiunge una notifica nel centro notifiche: “Versione non determinata – LanguageID raw: &lt;valore&gt;” con azione “Vedi in Archivio errori”.
  - Si crea una voce in Archivio → Errori: data, tipo “Versione non determinata”, pulsante Apri che apre il dettaglio con messaggio completo, path, languageIdRaw, eventuale stack se applicabile; pulsante Elimina.

L’utente può in qualunque momento aprire Archivio → Errori, ordinare per data, aprire il dettaglio e copiare l’intero blocco per incollarlo dove serve.

---

## Riepilogo scelte (per aggiornare altri doc)

| Aspetto | Scelta |
|--------|--------|
| Dove si vede il “fallimento” oggi | Solo nel dialog Riepilogo salvataggio, riga “L? &lt;valore&gt;” quando Versione è "—" e c’è languageIdRaw |
| Dove mostrarlo in futuro | Non in schermata; tramite avviso (toast) + notifica (centro notifiche) + Archivio Errori |
| Notifiche | Icona in Top Bar; clic apre componente dedicato (centro notifiche in-app) |
| Errori | Standard: toast + notifica + voce in Archivio → Errori con Data, Tipo, Apri (dettaglio copiabile), Elimina |
| Errori (posizione) | Impostazioni → Errori; tabella e componente “dettaglio errore” |
| Scopo Archivio Errori | Log completo da copiare/incollare (es. per supporto o assistente AI) |

---

## Standard operativo: collegare un caso reale

Questa sezione è il **riferimento unico** per tutte le operazioni di collegamento di un nuovo caso reale a toast + notifica + Archivio Errori. Ogni nuovo “errore di sistema” deve seguirla così da avere un unico modo di fare le cose.

### Quando conta come “errore di sistema”

- Fallimento di un’**operazione** che l’app ha tentato (sidecar, DB, I/O, parse, comando Tauri fallito).
- **Eccezione / Result::Err** gestita nel backend o nel frontend dopo `invoke`.
- Condizione **anomala** che impedisce il flusso normale (es. versione non determinata, timeout, file non riconosciuto).

**Non** sono errori di sistema: validazione form (nome vuoto, duplicato), scelte utente (annulla, chiudi), stati “normali” (nessun salvataggio, cartella vuota).

### Flusso obbligatorio

Per ogni errore di sistema fare **sempre** nell’ordine:

1. **Toast** — messaggio breve user-facing (es. “Versione non determinata. Controlla le notifiche.”). Usare `toast.error(...)` dal componente sonner.
2. **Archivio** — creare una voce in Archivio → Errori tramite **`reportSystemError({ type, detail, toastMessage? })`** (helper in `$lib/stores/error-archive.ts`). Lo helper fa toast + persistenza via `add_error_archive_entry`; se si passa `toastMessage` quello viene usato per il toast, altrimenti `type`. Per `detail` usare **`formatErrorDetail`** quando si hanno dati strutturati (path, languageIdRaw, ecc.).
3. **Notifica** — quando il centro notifiche (Top Bar) sarà implementato, aggiungere una voce con titolo/riepilogo e azione “Vedi in Archivio errori” (link a `/impostazioni/errori` o alla voce se esiste id).

Oggi: obbligatori **toast + archivio**; notifica quando il componente esiste.

### Formato della voce in archivio

- **type** — etichetta breve user-facing (es. “Versione non determinata”, “Sidecar timeout”, “File non riconosciuto”).
- **detail** — blocco di testo **completo e copiabile** per supporto/assistente AI. Formato suggerito (testo strutturato, non JSON):

  ```
  type: <type>
  at: <ISO 8601>
  message: <messaggio breve>
  path: <path se applicabile>
  [altri campi: languageIdRaw, stack, backtrace, ecc.]
  ```

- **id** — generato dall’helper con `crypto.randomUUID()`.
- **at** — impostato dall’helper con `new Date().toISOString()`.

### Dove chiamare

- **Frontend:** nei `catch` di `invoke(...)` quando l’errore non è di validazione utente; in handler di eventi che ricevono errori dal backend.
- **Pattern:** una sola chiamata a **`reportSystemError({ type, detail, toastMessage? })`** per caso. Lo helper centralizza toast + addErrorEntry.

### Esempi di type e detail per casi reali

| Caso | type | detail (esempio) |
|------|------|------------------|
| Versione non determinata | `Versione non determinata` | `type: Versione non determinata\nat: ...\nmessage: Impossibile determinare la versione del salvataggio.\npath: C:\...\save.sav\nlanguageIdRaw: 0` |
| Sidecar timeout/fallito | `Sidecar timeout` o `Sidecar fallito` | `type: ...\nmessage: ...\npath: ...\nstderr: ...` |
| File non riconosciuto | `File non riconosciuto` | `type: ...\nmessage: ...\npath: ...` |

### Riferimento implementativo

- Store e helper: `src/lib/stores/error-archive.ts` — `reportSystemError`, `formatErrorDetail`, `loadErrorArchiveEntries`, `removeErrorEntry`, `errorArchiveEntries`. Persistenza via servizio `$lib/services/error-archive` (invoke `get_error_archive_entries`, `add_error_archive_entry`, `remove_error_archive_entry`).
- Pagina Impostazioni · Errori: `src/routes/impostazioni/errori/+page.svelte` (fetch su mount, remove via command).
- Backend: tabella `error_archive`, comandi in `src-tauri/src/commands/error_archive.rs`.
- Procedure per “aggiungi gestione errore”: `docs/procedures/workflow/error-handling.md` (rimanda qui per errori che vanno in Archivio).

---

## Riferimenti

- [error-handling.md](./error-handling.md) — Strategia gestione errori; qui si estende con “registro errori e notifiche”
- [ux-layout-decision-2026.md](./ux-layout-decision-2026.md) — Layout Top Bar + Sidebar
- [glossary.md](./glossary.md) — Termini Top Bar, Avvisi; da estendere con Archivio, Notifiche, Registro errori
- [sav-game-version-detection.md](./sav-game-version-detection.md) — Contesto languageIdRaw e detect

## Data

2026-01-28
