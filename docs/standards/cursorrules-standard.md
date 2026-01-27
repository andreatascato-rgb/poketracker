# Standard: Compilazione .cursorrules

## Obiettivo

Il file `.cursorrules` deve essere ottimizzato per massimizzare l'efficacia dell'AI mantenendo performance elevate. Questo standard definisce come compilare e mantenere il file.

## Lunghezza Ottimale

- **Range consigliato**: 500-2000 token (~400-1600 parole)
- **Massimo assoluto**: 3000 token per evitare perdita di focus
- **Minimo**: 200 token per garantire contesto sufficiente
- **Target ideale**: 800-1200 token per bilanciare completezza e performance

**Razionale**: 
- File troppo lunghi riducono l'attenzione dell'AI sulle regole chiave
- File troppo corti mancano di contesto necessario

## Struttura Consigliata

Il file deve seguire questa struttura gerarchica:

```
# TITOLO PROGETTO

## RUOLI E RESPONSABILITÀ
[Definizione chiara di chi fa cosa]

## REGOLE OPERATIVE
[Regole che non si infrangono mai, ordinate per priorità]

## STANDARD APPLICATI
[Riferimenti a docs/standards/ per dettagli]

## STACK TECNOLOGICO
[Specifiche tecnologiche del progetto]

## WORKFLOW
[Processi e procedure da seguire]

## FORMATO OUTPUT
[Come l'AI deve rispondere e comunicare]
```

## Principi di Scrittura

### 1. Linguaggio Imperativo
- ✅ **Corretto**: "Non creare file senza permesso esplicito"
- ❌ **Sbagliato**: "Potresti considerare di non creare file senza permesso"

### 2. Specificità
- ✅ **Corretto**: "Tutti i file markdown vanno in `docs/` con sottocartelle"
- ❌ **Sbagliato**: "Organizza i file in modo logico"

### 3. Una Regola per Punto
- ✅ **Corretto**: 
  ```
  - Non creare file senza permesso
  - Consulta docs/standards/ prima di implementare
  ```
- ❌ **Sbagliato**: "Non creare file senza permesso e consulta sempre docs/standards/ prima di implementare qualsiasi cosa"

### 4. Priorità Decrescente
- Regole più importanti in alto
- Regole meno critiche in basso

### 5. Evitare Ambiguità
- ✅ **Corretto**: "SEMPRE usa snake_case per funzioni Rust"
- ❌ **Sbagliato**: "Usa convenzioni appropriate per Rust"

## Contenuto Ottimale

### Sezione: RUOLI E RESPONSABILITÀ

- Definisce chiaramente cosa l'AI può/non può fare
- Stabilisce i confini di azione
- Massimo 5-7 punti chiave

### Sezione: REGOLE OPERATIVE

- Regole che non hanno eccezioni
- Ordinate per importanza
- Formato: lista puntata con linguaggio diretto

### Sezione: STANDARD APPLICATI

- Riferimenti a `docs/standards/[file].md`
- NON ripetere dettagli già documentati
- Solo riferimenti e regole di applicazione

### Sezione: STACK TECNOLOGICO

- Tecnologie principali (es. Tauri, Rust, Svelte)
- Versioni se rilevanti
- Convenzioni specifiche dello stack

### Sezione: WORKFLOW

- Processi da seguire (es. "Proponi → Aspetta approvazione → Implementa")
- Ordine di operazioni
- Checkpoint importanti

### Sezione: FORMATO OUTPUT

- Come strutturare le risposte
- Quando usare code blocks, liste, etc.
- Stile di comunicazione preferito

## Best Practice per Performance

### Riferimenti, Non Duplicazioni

- Se uno standard è documentato in `docs/standards/`, fai riferimento al file
- NON copiare il contenuto completo in `.cursorrules`

### Aggiornamenti Incrementali

- Quando aggiungi una regola, verifica che il file non superi i 2000 token
- Se necessario, sposta dettagli in `docs/standards/` e lascia solo il riferimento

### Versioning

- Aggiungi un commento con data/versione quando fai modifiche significative
- Esempio: `# Ultimo aggiornamento: 2026-01-27`

### Test Periodici

- Verifica periodicamente che le regole siano ancora rilevanti
- Rimuovi regole obsolete per mantenere il file snello

### Separazione Concerns

- `.cursorrules` = regole operative immediate
- `docs/standards/` = documentazione dettagliata e contesto

## Esempio di Struttura Ottimale

```markdown
# PokeTracker - Regole AI

## RUOLI E RESPONSABILITÀ

- Creare/modificare file quando esplicitamente richiesto nella query
- Proporre soluzioni per azioni non esplicitamente richieste
- Applicare sempre gli standard documentati in `docs/standards/`
- Chiedere conferma per modifiche che cambiano comportamento esistente

## REGOLE OPERATIVE

1. Tutti i file markdown di documentazione vanno in `docs/` con sottocartelle appropriate
2. NON creare cartelle di documentazione nella root del progetto
3. Consulta `docs/standards/` per gli standard dettagliati prima di implementare
4. Procedi direttamente quando la query richiede esplicitamente l'azione
5. Chiedi conferma per modifiche che cambiano comportamento esistente o non sono esplicitamente richieste

## STANDARD APPLICATI

- Vedi: `docs/standards/file-organization.md` per organizzazione file
- Vedi: `docs/standards/cursorrules-standard.md` per come compilare questo file
- Vedi: `docs/standards/guide-structure.md` per struttura procedure

## WORKFLOW OBBLIGATORIO

PRIMA di qualsiasi implementazione:

1. Identifica pattern nella query utente
2. Consulta `docs/procedures/INDEX.md` per trovare procedure corrispondente
3. Leggi OBBLIGATORIAMENTE la procedure identificata
4. Completa la checklist nella procedure punto per punto
5. Verifica tutti gli standard referenziati nella procedure
6. SOLO DOPO aver letto tutto, proponi implementazione
7. Aspetta approvazione esplicita dell'utente
8. Implementa solo dopo conferma

Nessuna eccezione: se esiste una procedure corrispondente, va sempre applicata. Vedi `docs/procedures/INDEX.md` per "Quando nessuna procedure corrisponde".

## FORMATO OUTPUT

- Sii conciso e strutturato
- Usa code blocks per esempi di codice
- Cita file con path quando rilevante
- Proponi, non imporre
```

## Data Decisione

2026-01-27

## Note

Questo standard si basa su principi consolidati di prompt engineering e ottimizzazione per AI. Il file `.cursorrules` deve essere considerato un "system prompt" ottimizzato per il progetto specifico.
