# Standard: Organizzazione File

## Obiettivo

Definisce dove e come organizzare tutti i file markdown di documentazione del progetto.

## Decisione

Tutti i file markdown di documentazione devono essere organizzati nella cartella `docs/` utilizzando sottocartelle appropriate.

## Struttura

```
docs/
├── standards/     # Standard e convenzioni del progetto (README.md)
├── procedures/   # Procedure operative (INDEX.md)
├── project/      # Informazioni su progetto e applicazione (README.md)
├── temp/         # File temporanei: valutazioni, confronti, bozze (README.md)
├── ideas/        # Idee, brainstorming, feature requests
├── architecture/ # Documentazione architetturale
├── api/          # Documentazione API
└── ...           # Altre sottocartelle secondo necessità
```

## Regole

- **NON** creare cartelle di documentazione nella root del progetto
- **SEMPRE** usare `docs/` come base per tutta la documentazione
- Creare sottocartelle quando necessario per organizzare meglio
- Mantenere la root del progetto pulita

## README e indici

Ogni sottocartella principale di `docs/` deve avere un file di orientamento, così chi apre la cartella sa cosa contiene.

| Cartella | File | Uso |
|----------|------|-----|
| **standards/** | `README.md` | Obiettivo, come funzionano gli standard, lista standard |
| **project/** | `README.md` | Obiettivo, entry point, elenco/raggruppamento documenti |
| **procedures/** | `INDEX.md` | Mappa query utente → procedure corrispondenti |
| **temp/** | `README.md` | File temporanei: valutazioni, confronti, bozze (eliminabili) |

- **README.md**: descrive la cartella, elenca i documenti, indica entry point o gruppi.
- **INDEX.md**: usato dove serve una mappa per tipo di uso (es. query → procedure).

Quando si aggiunge una nuova sottocartella principale sotto `docs/`, creare il relativo `README.md` (o `INDEX.md` se è una mappa).

## Data Decisione

2026-01-27

## Note

Questa decisione è stata presa per mantenere il progetto organizzato e professionale.
