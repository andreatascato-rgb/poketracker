# Sprite Pokedex (offline)

## Obiettivo

Definisce come l'app ottiene e usa gli sprite Pokémon per la sezione Pokedex: **solo locale**, nessuna dipendenza da rete a runtime. Tutto funziona offline.

## Principio: Solo locale

- Gli sprite usati dalla Pokedex **non** vengono mai caricati da CDN o URL esterni a runtime.
- Sono **inclusi nel bundle** (cartella `static/`) oppure pre-scaricati in fase di build/setup.
- Fonte: repository [PokeAPI/sprites](https://github.com/PokeAPI/sprites) (PNG ufficiali). Download eseguito **una volta** tramite script; i file restano in repo o nel bundle.

## Range: Gen 1–4

- **ID nazional Pokédex:** da 1 a 493 (Bulbasaur → Arceus).
- Per ora non sono inclusi Gen 5+; estensione futura possibile aggiungendo range allo script.

## Dove vivono gli sprite

| Elemento | Path | Uso |
|----------|------|-----|
| **Sprite PNG** | `static/pokedex-sprites/{id}.png` | Serviti da Vite/Tauri come asset statici; URL in app: `/pokedex-sprites/1.png` ecc. |

- La cartella `static/pokedex-sprites/` è inclusa nel build; l'app referenzia gli sprite solo con path relativi (es. `/pokedex-sprites/1.png`).
- Nessuna richiesta HTTP esterna per gli sprite in produzione.

## Script di download

- **Script:** `scripts/download-pokedex-sprites.mjs`
- **Esecuzione:** da root del progetto: `node scripts/download-pokedex-sprites.mjs` (oppure `npm run pokedex-sprites` se configurato).
- **Cosa fa:** scarica da PokeAPI/sprites (raw GitHub) i PNG per id 1–493 e li salva in `static/pokedex-sprites/{id}.png`. Crea la cartella se non esiste; può essere eseguito in CI o in locale prima del build.
- **Quando:** prima del primo build che include la Pokedex, o quando si vuole aggiornare gli asset. I file scaricati possono essere committati in repo per avere tutto offline senza passaggio di download in build.

## Riferimenti in app

- **URL sprite in frontend:** `/pokedex-sprites/${id}.png` (es. id 1 → `/pokedex-sprites/1.png`). Con `adapter-static` e Tauri, `static/` è servita dalla root.
- **Fallback:** se un file manca (id fuori range o non ancora scaricato), usare placeholder locale (es. icona generica o primo sprite) oppure nascondere l’immagine; **non** fare fallback su URL esterni.

## Aggiornamenti doc collegati

- [offline-data-strategy](./offline-data-strategy.md): risorse (sprite) = bundle o pre-scaricati, mai rete a runtime per core.
- [pokedex-personal](./pokedex-personal.md): sprite Pokedex come da questo doc.
- [knowledge-database](./knowledge-database.md): per Pokedex gen 1–4 gli sprite sono in `static/pokedex-sprites/` e inclusi nel bundle.

## Data decisione

2026-01-29
