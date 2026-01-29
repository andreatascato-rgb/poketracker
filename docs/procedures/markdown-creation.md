# Procedure: Creare/Modificare File Markdown

## Quando Usare Questa Procedura

- Query: "crea file md", "modifica file md", "crea standard", "documentazione", "crea guida", "crea procedure"
- Quando devi creare o modificare qualsiasi file `.md` in `docs/`
- Se la query corrisponde, questa procedure è OBBLIGATORIA

## Obbligatorietà

Completamento integrale della checklist obbligatorio prima di proporre implementazione. Nessuna eccezione.

## File da Leggere PRIMA

1. **Standard Markdown**: `docs/standards/markdown-optimization.md:1-163`
   - Sezione Obiettivo obbligatoria: riga 26
   - Paragrafi brevi: righe 17-22
   - Struttura gerarchica: righe 9-15

2. **Organizzazione File**: `docs/standards/file-organization.md:1-37`
   - Dove creare file: righe 9-10
   - Struttura cartelle: righe 13-21

## Checklist Obbligatoria

1. [ ] Leggi `docs/standards/markdown-optimization.md:26` - Sezione "Obiettivo" obbligatoria all'inizio
2. [ ] Verifica `docs/standards/file-organization.md:9` - File deve essere in `docs/` con sottocartelle appropriate
3. [ ] Aggiungi sezione "## Obiettivo" all'inizio del file (max 1-2 frasi)
4. [ ] Verifica paragrafi max 2-3 frasi (`markdown-optimization.md:19`)
5. [ ] Verifica struttura gerarchica H1-H3 (`markdown-optimization.md:11-14`)
6. [ ] Usa elenchi puntati invece di paragrafi lunghi (`markdown-optimization.md:21`)
7. [ ] Se è un nuovo standard, aggiungi a `docs/standards/README.md:13-18`

## Riferimenti Standard

- `docs/standards/markdown-optimization.md:1-163` - Standard completo per markdown
- `docs/standards/file-organization.md:1-37` - Organizzazione file di documentazione

## Esempio Struttura File

```markdown
# Titolo Principale

## Obiettivo

[1-2 frasi che spiegano lo scopo del file]

## Sezione Principale 1

[Contenuto breve, max 2-3 frasi per paragrafo]

### Sottosezione 1.1

[Contenuto specifico]

## Sezione Principale 2

- Punto chiave 1
- Punto chiave 2
- Punto chiave 3
```

## Note

- NON procedere senza aver completato la checklist
- Ogni file deve rispettare TUTTI i punti della checklist
- Se il file è molto lungo (>800 righe), considera di dividerlo (`markdown-optimization.md:57`)
