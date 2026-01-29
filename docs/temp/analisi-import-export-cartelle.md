# Analisi: Import/Export e cartelle (dove in app, cartella dedicata, 2026)

## Obiettivo

Definire: (1) dove inserire import/export nell’app (Editor vs Impostazioni); (2) come gestire la cartella dedicata (scelta utente vs creata dall’app all’avvio); (3) approccio consigliato nel 2026 per Tauri desktop.

---

## 1. Wiki in fondo alla sidebar — fatto

- **Ordine attuale:** Allenatore → Statistiche → Pokedex → Editor → Salvataggi → **Wiki** (in fondo).
- Doc: `layout-navigation-change.md`, `glossary.md` aggiornati.

---

## 2. Dove inserire Import/Export nell’app

Riepilogo dalla analisi precedente:

| Contesto | Azioni | Dove |
|----------|--------|------|
| **Editor** | Backup .sav prima di modificare, Salva modifiche, Export/import team da save | **UI Editor** (pulsanti/flusso) |
| **App/Profilo** | Backup DB/profilo, Export Pokedex (CSV/JSON), Restore, Reset profilo | **Impostazioni** (sottosezione Backup e dati / Dati) |

- **Import/Export legati al save (Editor):** apri file .sav, esporta team, importa in altro save → tutto nell’**Editor**.
- **Import/Export dati app (profilo, Pokedex, backup):** backup, export Pokedex, ripristino → **Impostazioni**.

---

## 3. Cartella dedicata: chi la crea e chi la sceglie

### 3.1 Dati app (DB, stato) — la app li crea

- **Oggi:** All’avvio dell’exe Tauri usa `app.path().app_data_dir()` (in `src-tauri/src/lib.rs`). La directory viene creata dal sistema/da Tauri al primo uso (es. Windows: `%AppData%\com.poketracker`).
- **Nessuna scelta utente:** L’utente non deve scegliere dove mettere il DB. È la best practice: dati app in app data dir, gestita dal SO.
- **Conclusione:** Nessun cambiamento. La “cartella dedicata” per dati app **è creata automaticamente** quando l’exe gira; non serve UI per sceglierla.

### 3.2 Export (dove salvare i file esportati) — cartella dedicata sì

**Motivo:** Senza una cartella dedicata, l’utente salva dove capita (dialog “Salva con nome” → Desktop, ultima cartella, ecc.) e **non sa dove ritrovare** export e backup dell’app. Serve un posto prevedibile e visibile.

**Scelta:**

- **Cartella dedicata per export/backup:** la app ne definisce una (e la crea al primo export).
  - **Default:** sottocartella creata dall’app, es. `app_data_dir()/exports` e `app_data_dir()/backups` (o un’unica `exports` per export + backup).
  - **Opzionale in Impostazioni:** “Cartella export” / “Cartella backup” per sceglierne un’altra (es. Documenti/PokeTracker, OneDrive); path salvato in preferenze.
- **In UI:** in Impostazioni → Backup e dati, oltre alle azioni Export/Backup, mostrare **“Apri cartella export”** (o “Apri cartella backup”) che apre nel file manager la cartella dove l’app salva export/backup, così l’utente sa dove trovare i file.
- **Comportamento:** le azioni “Esporta Pokedex”, “Backup ora” scrivono in quella cartella (con nome file proposto); si può comunque offrire “Salva con nome” per salvare altrove in una volta.

### 3.3 Import (da dove leggere i file) — nessuna cartella dedicata

- **Import = cartella del contesto (gioco/save scelto):** Per “Importa backup”, “Ripristina”, “Apri save” si usa il **dialog “Apri file”** (Tauri `open()`). L’utente sceglie il file dalla cartella che preferisce (es. cartella del gioco, dove ha il save, dove ha messo un backup).
- **Conclusione:** Nessuna cartella dedicata per l’import; la “cartella” è quella che l’utente sceglie nel dialog (tipicamente dove ha i save o i backup).

### 3.4 Backup automatici (futuro)

- Se un giorno si aggiungono backup periodici (es. “Backup ogni settimana”), la destinazione può essere:
  - **Default:** `app_data_dir()/backups` (la app la crea).
  - **Opzionale in Impostazioni:** “Cartella backup” (es. OneDrive/Dropbox) scelta con dialog e path salvato in preferenze.
- Stessa logica: default sotto app data; override opzionale in Impostazioni.

---

## 4. Come gestirlo al meglio nel 2026 (Tauri desktop)

### Principi

1. **App data dir (dati interni):** Usare sempre `app.path().app_data_dir()` per DB e stato. La directory è gestita dal SO; non esporre scelta all’utente (tranne eventuale “Apri cartella dati” in Impostazioni per utenti avanzati).
2. **Export: cartella dedicata sì.** La app crea e usa una cartella per export/backup (default `app_data_dir()/exports` o `/backups`) così l’utente sa dove trova i file. In Impostazioni: “Apri cartella export” e opzionale “Cartella export” per sceglierne un’altra.
3. **Import: nessuna cartella dedicata.** Dialog “Apri file”; l’utente sceglie il file dalla cartella del gioco/save o dove ha i backup.
4. **Permessi e sandbox (Tauri 2):** Per scrivere/leggere fuori da app data dir (es. cartella export, file scelti con dialog), usare le capability e gli scope del plugin fs/path secondo la doc Tauri 2.

### Riepilogo operativo

| Cosa | Chi crea la cartella | Dove si mette in UI |
|------|----------------------|---------------------|
| DB app, stato | App (`app_data_dir()` all’avvio) | Nessuna scelta utente |
| **Export / Backup** | **App crea cartella dedicata** (es. `app_data_dir()/exports` e `/backups`); export/backup vanno lì. Opzionale “Cartella export” in Impostazioni per sovrascrivere. | Impostazioni → Backup e dati: “Apri cartella export”, “Cartella export” (scegli altra). Editor: Salva/Export team (path nella cartella dedicata o dialog) |
| Import / Apri save | Nessuna cartella dedicata; utente sceglie file nel dialog (es. cartella del gioco) | Editor: Apri. Impostazioni: Ripristina backup |

---

## 5. Checklist implementativa (quando si fa)

- [ ] **Backend:** Creare cartella dedicata export/backup al primo uso (`app_data_dir()/exports` e/o `backups`). Comandi che scrivono export/backup in quella cartella; preferenza “Cartella export” (path salvato in app_state) per sovrascriverla.
- [ ] **Impostazioni → Backup e dati:** “Export Pokedex”, “Backup ora” → salvataggio nella cartella dedicata (nome file proposto); “Apri cartella export” che apre la cartella nel file manager; opzionale “Cartella export” per sceglierne un’altra. “Ripristina” → dialog open (nessuna cartella dedicata per import).
- [ ] **Editor:** “Backup .sav”, “Salva”, “Esporta team” (path nella cartella dedicata o dialog se si vuole salvare altrove).
- [ ] **Doc:** Aggiornare `self-management.md` con: cartella dedicata export/backup (creata dall’app), “Apri cartella export”, opzione “Cartella export” in Impostazioni; import senza cartella dedicata (dialog, cartella del gioco/save).
