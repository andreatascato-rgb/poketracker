# Procedure: Modificare .cursorrules

## Quando Usare Questa Procedura

- Query: "modifica cursorrules", "aggiorna cursorrules", "cambia regole", "aggiorna .cursorrules", "aggiungi regola"
- Se la query corrisponde, questa procedure è OBBLIGATORIA

## Obbligatorietà

Completamento integrale della checklist obbligatorio prima di proporre implementazione. Nessuna eccezione.

## File da Leggere PRIMA

1. **Standard Cursorrules**: `docs/standards/cursorrules-standard.md` (completo)
   - Lunghezza ottimale: righe 9-12 (max 2000 token)
   - Struttura obbligatoria: righe 18-42
   - Principi scrittura (incluso principio 0 Regole ferree): righe 44-77

2. **File Attuale**: `.cursorrules` (completo)
   - Leggi tutto per capire stato attuale
   - Verifica lunghezza corrente

## Checklist Obbligatoria

1. [ ] Leggi `docs/standards/cursorrules-standard.md` (inclusa sezione "Regole ferree" / principio 0).
2. [ ] Verifica che `.cursorrules` contenga **divieti assoluti** espliciti: non approssimare, non indovinare, non inventare, procedure/standard prima di tutto (`cursorrules-standard.md` principio 0).
3. [ ] Leggi `docs/standards/cursorrules-standard.md:9-12` - Verifica lunghezza max 2000 token.
4. [ ] Leggi `.cursorrules` completo per capire stato attuale.
5. [ ] Verifica struttura gerarchica obbligatoria (`cursorrules-standard.md:18-42`):
   - RUOLI E RESPONSABILITÀ
   - REGOLE OPERATIVE (con divieti assoluti in evidenza)
   - STANDARD APPLICATI
   - WORKFLOW OBBLIGATORIO
   - FORMATO OUTPUT
6. [ ] Se aggiungi regola, verifica non superi 2000 token (`cursorrules-standard.md` sezione "Aggiornamenti Incrementali", righe 124-126).
7. [ ] Mantieni linguaggio imperativo (`cursorrules-standard.md` principio 1, righe 56-58).
8. [ ] Una regola per punto, non frasi lunghe (`cursorrules-standard.md` principio 3, righe 64-70).
9. [ ] Se aggiungi nuovo standard, aggiorna sezione STANDARD APPLICATI.
10. [ ] Riferimenti a standard, non duplicazioni (`cursorrules-standard.md` "Riferimenti, Non Duplicazioni", righe 119-122).

## Riferimenti Standard

- `docs/standards/cursorrules-standard.md` - Standard completo per .cursorrules
- `docs/standards/markdown-optimization.md` - Standard markdown (si applica anche a .cursorrules)

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
- Se il file supera 2000 token, sposta dettagli in `docs/standards/` (`cursorrules-standard.md` sezione "Aggiornamenti Incrementali")
- Mantieni il file snello e focalizzato (`cursorrules-standard.md` sezione "Test Periodici")
