# Standard: Struttura Procedure

## Obiettivo

Definisce come strutturare le procedure in `docs/procedures/` per garantire che l'AI consulti sempre le informazioni corrette prima di agire. Le procedure sono obbligatorie: nessuna approssimazione, nessun indovinare, nessuna implementazione non documentata. L'AI deve leggere la procedura per intero e completare la checklist prima di proporre o implementare.

## Struttura Obbligatoria

Ogni procedure deve seguire questa struttura:

```markdown
# Procedure: [Nome Azione]

## Quando Usare Questa Procedura

[Pattern di query che attivano questa procedura. Se la query corrisponde, la procedura è OBBLIGATORIA.]

## Obbligatorietà

Completamento integrale della checklist obbligatorio prima di proporre implementazione. Nessuna eccezione.

## Prerequisiti

[File/standard che devono essere letti prima]

## Checklist Obbligatoria

1. [ ] Step 1 con riferimento preciso
2. [ ] Step 2 con riferimento preciso
3. [ ] ...

## Riferimenti Standard

- Standard X: `docs/standards/file.md:riga`
- Standard Y: `docs/standards/file.md:riga`

## Esempio Completo

[Esempio concreto e funzionante]
```

## Sezioni Dettagliate

### Quando Usare Questa Procedura

- Elenca pattern di query che attivano la procedura
- Esempio: "crea componente", "aggiungi componente", "nuovo componente svelte"
- Sii specifico ma inclusivo

### Prerequisiti

- File che devono essere letti prima
- Standard che devono essere verificati
- Dipendenze da altre procedure

### Checklist Obbligatoria

- Step numerati e verificabili, da eseguire **in ordine 1, 2, 3…**; non saltare step
- Ogni step deve avere riferimento preciso a `file.md:riga`; aprire il file indicato, non basarsi solo sul riassunto nella procedura
- Non procedere se checklist non completata

### Riferimenti Standard

- Lista di standard applicabili con righe precise
- Formato: `file.md:riga-inizio:riga-fine`
- Solo riferimenti, non duplicazioni

### Esempio Completo

- Codice funzionante, non pseudocodice
- Mostra input e output atteso
- Include tutti i file coinvolti

## Naming File

- Formato: `kebab-case.md`
- Nome descrittivo dell'azione
- Esempi:
  - `component-creation.md`
  - `command-creation.md`
  - `new-feature.md`

## Organizzazione

```
docs/procedures/
├── INDEX.md              # Mappa query → procedure
├── svelte/
│   ├── component-creation.md
│   └── store-setup.md
├── rust/
│   └── command-creation.md
└── workflow/
    └── new-feature.md
```

## Ottimizzazione per uso da parte dell'AI

- **Trigger obbligatori**: in "Quando Usare" indicare che, se la query corrisponde ai pattern, la procedura è obbligatoria e non si propone implementazione senza averla completata
- **Ordine checklist**: eseguire gli step 1, 2, 3… in sequenza; non saltare
- **Riferimenti file:riga**: per ogni step che cita un file, aprire quel file (e le righe se indicate); non duplicare contenuti lunghi nella procedura
- **Brevità**: preferire "Leggi X", "Applica Y" con riferimento a `docs/...`; evitare incollare interi blocchi già presenti negli standard
- **Blocco Obbligatorietà**: ogni procedure deve contenere la sezione "## Obbligatorietà" con la frase: *Completamento integrale della checklist obbligatorio prima di proporre implementazione. Nessuna eccezione.*

## Data Decisione

2026-01-27

## Note

Le procedure devono essere consultate OBBLIGATORIAMENTE prima di implementare. Questo standard garantisce che l'AI abbia sempre il contesto completo.
