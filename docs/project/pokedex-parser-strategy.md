# Strategia: Parser Pokedex per gioco/versione

## Obiettivo

Definisce la decisione di usare **parser specifici per ogni gioco/versione** solo per il tracciamento del Pokedex, invece di affidarsi a PKHeX per questa parte. I dati estratti vengono salvati in DB e persistono anche a watcher spento; al riavvio del parser i dati vengono aggiornati.

## Contesto e problema

- L’uso di **PKHeX** per tracciare il Pokedex **non sta andando a buon fine** (limiti, bug o incompatibilità con alcuni giochi/versioni).
- La documentazione e le analisi indicano che il **parsing del Pokedex è complesso e molto variabile** tra generazioni e titoli: strutture, offset e formati cambiano spesso.
- Un parser generico (o un unico percorso via PKHeX) rischia di essere fragile e poco preciso.

## Decisione

Creare **parser singoli per ogni gioco/versione**, dedicati **solo** al tracciamento del Pokedex:

- **Scope:** solo estrazione stato Pokedex (visto/catturato per specie), non altri dati del salvataggio.
- **Precisione:** ogni parser conosce il formato del singolo titolo/versione e può essere più affidabile e manutenibile.
- **Riuso architettura:** il sidecar e l’integrazione Tauri restano come in `parser-architecture.md`; per il Pokedex si aggiungono (o sostituiscono) comandi/parser specifici per gioco invece di un unico comando generico basato su PKHeX.

## Come funziona

1. **Attivazione**
   - L’utente attiva il **watcher** su un file di salvataggio di un gioco/versione già presente nella sezione **Salvataggi** (path e contesto già noti all’app).
   - All’attivazione del watcher si avvia il **parser Pokedex** per quel gioco/versione, insieme eventualmente ad altri parser (es. detect, dati allenatore) se necessari.

2. **Esecuzione**
   - Il parser legge il file di salvataggio e estrae solo i dati Pokedex (stato visto/catturato per specie).
   - I dati vengono **salvati nel DB** (stesso modello/contratto usato oggi, es. `pokedex_state` per profilo/salvataggio).

3. **Persistenza**
   - Anche **a watcher spento** i dati restano in DB: l’utente continua a vedere il Pokedex aggiornato all’ultima esecuzione.

4. **Riavvio**
   - Se l’utente **riaccende il watcher** (o riesegue la sync per quel salvataggio), il parser **riparte** e **aggiorna** i dati in DB (sovrascrivendo/aggiornando le righe esistenti per quel contesto).

## Coerenza con il progetto

- **Watcher e Salvataggi:** il watcher opera su file/cartelle già associati al profilo e presenti in Salvataggi; non introduce nuovi flussi, solo quale parser viene invocato.
- **DB e UI:** il modello dati e la UI del Pokedex personale (vedi `pokedex-personal.md`) restano invariati; cambia solo la **fonte** dei dati (parser per gioco invece di PKHeX).
- **Sidecar/Tauri:** l’architettura in `parser-architecture.md` resta valida; i nuovi parser possono essere comandi aggiuntivi nel sidecar (es. `pokedex_bdsp <path>`) o logica in Rust che chiama il sidecar con parametro “gioco/versione” per scegliere il parser.

## Prossimi passi (da pianificare)

- Definire l’elenco **giochi/versioni** da supportare per primi e l’ordine di implementazione.
- Per ogni gioco/versione: specificare formato Pokedex (offset, struttura) e implementare il parser dedicato (sidecar o Rust).
- Decidere se mantenere temporaneamente un fallback PKHeX per alcuni titoli o rimuoverlo del tutto per il Pokedex.

## Riferimenti

- `parser-architecture.md` — Architettura sidecar e integrazione Tauri
- `pokedex-personal.md` — Feature Pokedex personale e uso dati in UI
- `database-storage.md` — Modello dati e tabelle (es. `pokedex_state`)
