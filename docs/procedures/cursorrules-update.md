# Procedure: Modificare .cursorrules

## Quando Usare Questa Procedure

- Query: "modifica cursorrules", "aggiorna cursorrules", "cambia regole", "aggiorna .cursorrules", "aggiungi regola"
- Se la query corrisponde, questa procedure è OBBLIGATORIA

## Obbligatorietà

Completamento integrale della checklist obbligatorio prima di proporre implementazione. Nessuna eccezione.

## File da Leggere PRIMA

1. **Standard Cursorrules**: `docs/standards/cursorrules-standard.md:1-177`
   - Lunghezza ottimale: righe 9-12 (max 2000 token)
   - Struttura obbligatoria: righe 18-42
   - Principi scrittura: righe 44-68

2. **File Attuale**: `.cursorrules:1-43`
   - Leggi tutto per capire stato attuale
   - Verifica lunghezza corrente

## Checklist Obbligatoria

1. [ ] Leggi `docs/standards/cursorrules-standard.md:9-12` - Verifica lunghezza max 2000 token
2. [ ] Leggi `.cursorrules` completo per capire stato attuale
3. [ ] Verifica struttura gerarchica obbligatoria (`cursorrules-standard.md:18-42`):
   - RUOLI E RESPONSABILITÀ
   - REGOLE OPERATIVE
   - STANDARD APPLICATI
   - WORKFLOW OBBLIGATORIO
   - FORMATO OUTPUT
4. [ ] Se aggiungi regola, verifica non superi 2000 token (`cursorrules-standard.md:117`)
5. [ ] Mantieni linguaggio imperativo (`cursorrules-standard.md:46-48`)
6. [ ] Una regola per punto, non frasi lunghe (`cursorrules-standard.md:54-60`)
7. [ ] Se aggiungi nuovo standard, aggiorna sezione STANDARD APPLICATI
8. [ ] Riferimenti a standard, non duplicazioni (`cursorrules-standard.md:110-113`)

## Riferimenti Standard

- `docs/standards/cursorrules-standard.md:1-177` - Standard completo per .cursorrules
- `docs/standards/markdown-optimization.md:1-163` - Standard markdown (si applica anche a .cursorrules)

## Struttura Obbligatoria

Il file deve avere queste sezioni in ordine:

```markdown
# PokeTracker - Regole AI

## RUOLI E RESPONSABILITÀ
[Max 5-7 punti chiave]

## REGOLE OPERATIVE
[Lista numerata, ordinate per priorità]

## STANDARD APPLICATI
[Riferimenti a docs/standards/, non duplicazioni]

## WORKFLOW OBBLIGATORIO
[Processi da seguire PRIMA di implementare]

## FORMATO OUTPUT
[Come rispondere e comunicare]
```

## Note

- NON procedere senza aver completato la checklist
- Se il file supera 2000 token, sposta dettagli in `docs/standards/` (`cursorrules-standard.md:118`)
- Mantieni il file snello e focalizzato (`cursorrules-standard.md:130-133`)
