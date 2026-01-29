# Analisi: Home, Statistiche, Export/Backup

## Obiettivo

Decidere: (1) rimozione route Archivio; (2) se serve Home dedicata o landing in Allenatore; (3) se statistiche stanno tutte in Allenatore o servono sezione/top-level; (4) se export/backup vanno inseriti, manuali, e per cosa.

---

## 1. Archivio (route) — fatto

- **Stato:** Route `/archivio` e `/archivio/errori` rimosse; erano solo redirect a `/impostazioni/errori`.
- **Nessun link** in app puntava a `/archivio`; registro errori è **Impostazioni → Errori**.
- **Doc aggiornate:** `notifications-and-error-archive.md`, `layout-navigation-change.md` (nessuna voce sidebar «Archivio», route solo `/impostazioni/errori`).

---

## 2. Home

- **Ora:** `/` fa redirect 302 a `/profilo`; `+page.svelte` per `/` è vuoto (solo commento). La “home” è già **Allenatore**.
- **Conclusione:** Non serve sezione Home dedicata. La landing è **Allenatore** (`/profilo`). Se in futuro si vuole una dashboard con riepilogo (ultimi save, notifiche, link rapidi), si può:
  - arricchire la pagina **Allenatore** (`/profilo`) con quel contenuto, oppure
  - introdurre una route `/dashboard` o simile solo quando il contenuto lo giustifica.
- **Raccomandazione:** Lasciare `/` → `/profilo` e considerare “home” = pagina Allenatore; eventuale “dashboard” = contenuto della stessa pagina o sotto-route sotto `/profilo` (es. `/profilo` = overview, senza nuova voce sidebar “Home”).

---

## 3. Statistiche

- **Ora:** Route `/profilo/statistiche` esiste; pagina placeholder (“contenuto in fasi successive”). **Non** c’è voce sidebar “Statistiche” e **nessun link** dalla UI a `/profilo/statistiche` (route al momento irraggiungibile dalla nav).
- **Opzioni:**
  - **A – Tutto sotto Allenatore:** Statistiche come **sottopagina di profilo**: accesso da landing Allenatore (tab/link/card) o come **sottovoce sidebar** sotto “Allenatore” (se in futuro la sidebar avrà children per Profilo: es. Allenatore → Dashboard, Statistiche, …). Statistiche complesse (grafici, tabelle) possono comunque stare in `/profilo/statistiche` senza bisogno di una sezione top-level.
  - **B – Sezione dedicata top-level:** Voce sidebar “Statistiche” separata (come Pokedex/Salvataggi). Ha senso solo se le statistiche diventano un uso primario e si vogliono allo stesso livello di Pokedex/Editor.
- **Raccomandazione:** Tenere le statistiche **sotto Allenatore** (stessa “sezione” profilo): route `/profilo/statistiche`, contenuto anche complesso lì. Quando si implementa la pagina Allenatore, aggiungere **accesso a Statistiche** (link/tab/card da `/profilo`). Valutare sottovoci sidebar “Allenatore” (Dashboard, Statistiche) solo se la nav diventa troppo affollata o se Statistiche diventa uso primario.

---

## 4. Export / Backup

- **Riferimento:** `docs/project/self-management.md` — “Gestione Dati”: Backup/ripristino dati, Esportazione dati, Reset dati profilo.
- **Domande da chiarire:**
  - **Manuale o automatico?** Backup/export tipicamente **avviati dall’utente** (pulsante “Esporta”, “Backup ora”) o schedulati (cron); per un’app desktop come PokeTracker, **manuale** è la scelta più semplice e prevedibile.
  - **Per cosa?** Es. export Pokedex (CSV/JSON), backup DB profilo (file da ripristinare), export lista salvataggi; “backup” può essere “copia sicura dati app/profilo” per ripristino su altro PC o dopo reinstall.
  - **Dove inserirli?** Sotto **Impostazioni** (es. Impostazioni → Backup/Export o Impostazioni → Dati). Non serve voce sidebar dedicata “Backup”; una sottosezione in Impostazioni è sufficiente.
- **Raccomandazione:**
  - **Sì, inserirli** quando si implementa la gestione dati (self-management): come **sottosezione Impostazioni** (es. “Backup e export” o “Dati” con Backup + Export + eventuale Reset).
  - **Manuale:** azioni esplicite ( “Crea backup”, “Esporta Pokedex”, …) con conferma per operazioni distruttive.
  - **Scope:** definire in fase di design (solo DB profilo? solo Pokedex? anche percorsi/preferenze?) e documentare in `self-management.md` o in un doc di specifica backup/export.

---

## Riepilogo azioni

| Tema        | Azione                                                                 |
|------------|-------------------------------------------------------------------------|
| Archivio   | Fatto: route rimosse, doc aggiornate.                                  |
| Home       | Nessun cambiamento: landing = Allenatore (`/` → `/profilo`).           |
| Statistiche| **Voce sidebar aggiunta** dopo Allenatore; route `/profilo/statistiche`; per 8 giochi, statistiche singole/sommate, per ogni variabile. |
| Export/Backup | Vedi sezione sotto: Editor vs Impostazioni. |

---

## 5. Statistiche — voce in sidebar (fatto)

- **Posizione:** Dopo **Allenatore**, prima di **Pokedex**. Ordine: Allenatore → Statistiche → Pokedex → Wiki → Editor → Salvataggi.
- **Motivo:** Blocco “profilo/dati allenatore” (Allenatore + Statistiche) prima di Pokedex; statistiche per 8 giochi, singole/sommate, per variabile richiedono sezione dedicata.
- **Route:** `/profilo/statistiche`; titolo e breadcrumb già in `+layout.svelte`.

---

## 6. Backup / Export / Salva — casi d’uso e dove metterli

### Quando serve

1. **Editor (modifica Pokemon/oggetti ecc.):**
   - **Backup .sav prima di modificare:** copia del file prima di aprire in editor (evitare perdite).
   - **Salva modifiche:** scrivere il .sav modificato su disco dopo edit (azione “Salva” in Editor).
   - **Export/import dal save:** esportare Pokemon/team da un save, importare in un altro (workflow avanzato).
   → **Dove:** azioni **nell’Editor** (es. “Backup prima di modificare”, “Salva”, “Esporta team”). Non in Impostazioni.

2. **App / profilo:**
   - **Backup dati app/profilo:** DB profilo, Pokedex estratto, preferenze — per ripristino dopo reinstall o su altro PC.
   - **Export dati (es. Pokedex in CSV/JSON):** per analisi esterna o backup “leggero”.
   - **Import/restore:** ripristinare un backup profilo.
   - **Reset dati profilo:** azione distruttiva; utile avere backup prima.
   → **Dove:** **Impostazioni** (sottosezione “Backup e dati” o “Dati”: backup/ripristino, export Pokedex, reset profilo).

### Conclusione

| Contesto   | Cosa                        | Dove metterlo        |
|-----------|-----------------------------|----------------------|
| Editor    | Backup .sav, Salva, Export team | UI **Editor** (pulsanti/flusso) |
| App/Profilo | Backup DB/profilo, Export Pokedex, Restore, Reset | **Impostazioni** (sottosezione dedicata) |

- **Backup .sav / Salva** = parte del flusso Editor, non Impostazioni.
- **Backup app, export dati profilo, restore, reset** = Impostazioni. Quindi **sì, una sottosezione in Impostazioni** (Backup e dati / Dati) per tutto ciò che riguarda dati app e profilo; Editor gestisce solo il ciclo “apri save → modifica → backup/salva”.
