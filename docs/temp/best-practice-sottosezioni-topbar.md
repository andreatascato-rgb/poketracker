# Best practice: sottosezioni da pulsante in Top Bar

## Contesto

Quando un’azione in Top Bar apre una sezione con **più sottosezioni** (es. Impostazioni → Profili, Errori), servono due cose:

1. **Entry point**: come aprire la sezione (dropdown con voci, link diretto, ecc.).
2. **Navigazione tra sottosezioni**: come passare da una sottosezione all’altra senza tornare alla Top Bar.

## Pattern confrontati

| Pattern | Descrizione | Pro | Contro |
|--------|-------------|-----|--------|
| **Dropdown + tab in pagina** | Icona Top Bar apre menu con link alle sottosezioni; la pagina di destinazione ha tab (o sub-nav) per le altre sottosezioni. | Chiaro, scalabile, utente può passare da una sottosezione all’altra senza riaprire il menu. | Richiede un layout dedicato per la sezione (es. `impostazioni/+layout.svelte`). |
| **Solo scroll** | Un’unica pagina lunga con sezioni; link nella Top Bar portano ad anchor (#profili, #errori) e la pagina fa scroll. | Semplice, nessun cambio route. | Poco adatto a contenuti lunghi (tabelle, liste); scroll può essere confusionario con molto contenuto. |
| **Solo dropdown** | Icona apre menu; ogni voce naviga a una route; nessuna sub-nav nella pagina. | Implementazione minima. | Per cambiare sottosezione l’utente deve riaprire il menu dalla Top Bar. |
| **Drawer/Sheet** | Icona apre un pannello laterale con lista sottosezioni e contenuto inline. | Tutto in un unico contesto. | Occupa spazio; su desktop spesso si preferisce la full page. |

## Scelta in PokeTracker (Impostazioni)

- **Entry**: pulsante Impostazioni in Top Bar → **dropdown** con voci “Profili” e “Errori” che navigano a `/impostazioni/profili` e `/impostazioni/errori`.
- **Sottosezioni**: layout **Impostazioni** (`impostazioni/+layout.svelte`) con **tab orizzontali** (Profili | Errori) sopra il contenuto; tab = link alle stesse route, stato attivo in base al path.

Risultato: un click sulla Top Bar porta a una sottosezione; da lì si può passare all’altra con un click sul tab, senza riaprire il menu.

## Raccomandazione generale

- **Poche sottosezioni (2–5), contenuto per pagina (tabelle, form, liste)** → **Dropdown + tab in pagina** (pattern adottato per Impostazioni).
- **Poche sottosezioni, contenuto breve (solo testo/avvisi)** → **Solo scroll** con anchor e scroll-spy è accettabile.
- **Molte sottosezioni o gerarchia profonda** → Valutare una **pagina dedicata** con sidebar o tab a più livelli invece di un solo pulsante in Top Bar.

## Riferimenti

- Implementazione: `src/routes/+layout.svelte` (dropdown Impostazioni), `src/routes/impostazioni/+layout.svelte` (tab Profili | Errori).
- Design system: `docs/standards/design-system-standard.md`, `docs/standards/interaction-states-standard.md`.
