# Catalog Componenti UI

## Obiettivo

Elenco dei primitivi e dei compositi UI riutilizzabili in PokeTracker. Fonte di verità per “cosa esiste” e “come riusarlo”. Prima di creare un nuovo pulsante, input, card, ecc. controllare qui; se esiste un primitivo adatto, usarlo. Vedi [ui-primitives-standard](../standards/ui-primitives-standard.md).

## Come usare il catalog

- **Creare elemento classico (primitivo):** controllare la tabella Primitivi; se non c’è, creare con la procedure [ui-primitive-creation](../procedures/svelte/ui-primitive-creation.md) e aggiungere una riga qui.
- **Creare componente personalizzato:** preferire composizione da primitivi elencati sotto; se il componente è riutilizzabile in più videate, aggiungerlo alla tabella Compositi.

## Primitivi

| Nome | Path | Props principali | Varianti | Token / Note |
|------|------|------------------|----------|---------------|
| *(nessuno ancora)* | — | — | — | Aggiungere righe quando si creano primitivi con ui-primitive-creation. |

## Compositi (riutilizzabili)

Componenti domain-specific usati in più contesti. Costruiti da primitivi + logica.

| Nome | Path | Uso | Primitivi usati | Note |
|------|------|-----|-----------------|------|
| *(nessuno ancora)* | — | — | — | Aggiungere quando un componente diventa riutilizzabile. |

## Note

- **Path:** relativo a `src/lib/` (es. `components/ui/Button.svelte`).
- **Varianti:** es. `primary | secondary | danger`, `sm | md | lg`.
- Aggiornare il catalog ogni volta che si aggiunge un primitivo o un composito riutilizzabile.
