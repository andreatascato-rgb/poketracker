# Standard: Markdown Ottimizzato per AI

## Obiettivo

Tutti i file markdown in `docs/` devono essere ottimizzati per la lettura e comprensione da parte di AI. Questo standard definisce come strutturare e formattare la documentazione.

## Principi Fondamentali

### 1. Struttura Gerarchica Chiara

- Usa heading H1 (`#`) solo per il titolo principale
- Usa H2 (`##`) per sezioni principali
- Usa H3 (`###`) per sottosezioni
- Massimo 3-4 livelli di profondità (H1-H3 o H4 se necessario)
- Ogni sezione deve avere uno scopo chiaro e unico

### 2. Paragrafi Brevi

- **Massimo 2-3 frasi per paragrafo**
- Un concetto per paragrafo
- Usa elenchi puntati invece di paragrafi lunghi quando possibile
- Evita blocchi di testo continui

### 3. Contesto Esplicito

- Inizia ogni file con una sezione "Obiettivo" o "Scopo" che spiega cosa contiene
- Definisci acronimi e termini tecnici alla prima occorrenza
- Usa esempi concreti invece di descrizioni astratte
- Specifica "chi, cosa, quando, perché" quando rilevante

### 4. Scannerabilità

- Usa elenchi puntati per informazioni multiple
- Usa elenchi numerati per processi/step
- Evidenzia concetti chiave con **grassetto**
- Usa `codice inline` per terminologia tecnica, nomi file, comandi

### 5. Formattazione Codice

- Usa blocchi di codice con linguaggio specificato:
  ```rust
  // esempio
  ```
- Usa codice inline per riferimenti a file: `docs/standards/file.md`
- Usa codice inline per comandi: `npm install`
- Separa chiaramente esempi dal testo esplicativo

### 6. Link e Riferimenti

- Usa link relativi per file interni: `[testo](./file.md)`
- Usa link assoluti per risorse esterne
- Mantieni i link aggiornati
- Evita link rotti

### 7. Lunghezza File

- **Target**: 200-800 righe per file
- Se un file supera 1000 righe, considera di dividerlo
- Ogni file deve avere uno scopo chiaro e focalizzato

## Struttura Standard di un File

```markdown
# Titolo Principale

## Obiettivo

[1-2 frasi che spiegano lo scopo del file]

## Sezione Principale 1

[Contenuto breve e focalizzato]

### Sottosezione 1.1

[Contenuto specifico]

## Sezione Principale 2

- Punto chiave 1
- Punto chiave 2
- Punto chiave 3

## Esempi

[Esempi concreti quando rilevanti]

## Note

[Informazioni aggiuntive, limitazioni, TODO]
```

## Regole di Scrittura

### DO (Fare)

✅ Usa linguaggio diretto e imperativo quando appropriato
✅ Inizia con il concetto più importante
✅ Usa esempi concreti
✅ Separa concetti diversi in sezioni diverse
✅ Usa tabelle per dati strutturati
✅ Mantieni coerenza terminologica

### DON'T (Non Fare)

❌ Non usare paragrafi di 5+ frasi
❌ Non mescolare argomenti diversi nella stessa sezione
❌ Non usare abbreviazioni senza definirle
❌ Non lasciare sezioni vuote o incomplete
❌ Non usare formattazione HTML complessa
❌ Non creare gerarchie troppo profonde (max H4)

## Esempio di File Ottimizzato

```markdown
# Standard: Naming Convention

## Obiettivo

Definisce le convenzioni di naming per file, variabili e funzioni nel progetto.

## File

- File Rust: `snake_case.rs`
- File Svelte: `PascalCase.svelte`
- File Markdown: `kebab-case.md`

## Variabili

- Rust: `snake_case`
- JavaScript: `camelCase`
- Costanti: `UPPER_SNAKE_CASE`

## Funzioni

- Rust: `snake_case()`
- JavaScript: `camelCase()`

## Note

Le eccezioni devono essere documentate in questo file.
```

## Verifica Qualità

Prima di considerare un file completo, verifica:

- [ ] Ha una sezione "Obiettivo" o "Scopo" all'inizio?
- [ ] I paragrafi sono brevi (max 2-3 frasi)?
- [ ] La gerarchia degli heading è chiara?
- [ ] Ci sono esempi concreti quando necessario?
- [ ] I concetti chiave sono evidenziati?
- [ ] Il file è focalizzato su un argomento principale?
- [ ] I link sono funzionanti?

## Data Decisione

2026-01-27

## Note

Questo standard si applica a tutti i file markdown in `docs/` e deve essere rispettato per garantire la massima leggibilità da parte di AI.
