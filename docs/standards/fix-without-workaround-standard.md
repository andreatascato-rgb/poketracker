# Standard: Fix senza workaround

## Obiettivo

Quando si corregge un errore o un bug, **si corregge sempre la causa radice**. Non si aggiungono workaround (redirect, catch-all, eccezioni) per “nascondere” il sintomo, a meno che non sia una decisione esplicita di prodotto.

## Regola

- **Fix = rimuovere o correggere la sorgente del problema**, non aggirare il problema.
- **Workaround** (da evitare se non richiesti): redirect da URL obsoleti, route “fantasma” che reindirizzano, catch-all che restituiscono 200, silenziamento di errori senza correzione della causa.
- **Eccezione**: un redirect (o simile) è ammesso **solo** se documentato come requisito di prodotto (es. “supportare vecchi URL per backward compatibility”) e non come sostituto di una correzione.

## Quando si applica

- Correzione di errori 404 o altri errori ricorrenti in console/terminale.
- Bug fix in generale: v. `docs/procedures/workflow/bug-fix.md` e `docs/procedures/workflow/error-404-correction.md`.

## Riferimenti

- Procedure correzione 404: `docs/procedures/workflow/error-404-correction.md`
- Bug fix (causa radice, modifica minima): `docs/procedures/workflow/bug-fix.md`
- Best practice coding (no TODO/stub, verifica): `docs/standards/coding-best-practices.md`
