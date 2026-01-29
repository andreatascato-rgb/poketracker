# Pokedex: parsing e dove trovarlo in PKHeX

## Obiettivo

Chiarire che il parsing del Pokedex **viene eseguito** quando il watcher è attivo, e dove trovare i dati Pokedex nell’interfaccia PKHeX.

## Il parsing viene eseguito

Quando attivi il watcher su un salvataggio (o quando un file .sav osservato cambia), la catena è:

1. **Frontend** → `sync_pokedex_from_watched_savs_now()` (o alla prima attivazione `sync_sav_now(path)`).
2. **Backend Rust** → `extract_pokedex_from_save(app, path)` per ogni path watchato.
3. **Sidecar** → avvio del parser con argomenti `pokedex <path>`.
4. **Program.cs** → carica il save con PKHeX, chiama `PokedexHelper.GetAllSpeciesStatus(sav)`.
5. **PokedexHelper** → per specie 1..max chiama `sav.GetCaught(dexIndex)` e `sav.GetSeen(dexIndex)` (dexIndex = sid - 1 se 0-based, altrimenti sid), propaga "caught" alle pre-evoluzioni (Gen1 catene), costruisce `{ entries: [{ species_id, status }] }`.
6. **Rust** → riceve il JSON, fa merge (caught > seen > unseen) e scrive in `pokedex_state`.

Quindi il parsing del Pokedex **viene eseguito** a ogni sync (attivazione watcher o `sav-file-changed`).

## Dove trovare il Pokedex in PKHeX (GUI)

In PKHeX **non** c’è una voce di menu principale “Pokedex”. I dati Pokedex si trovano così:

- Apri il save in PKHeX.
- Vai al tab **SAV** (salvataggio).
- Nella sezione relativa al salvataggio cerca l’opzione **Pokedex** (o **Zukan**).
- Clicca **Modify…** per aprire l’editor Pokedex (seen/caught per specie).

Quindi: **SAV → Pokedex → Modify…**.

## Come verificare che il parsing sia eseguito

- **Log sidecar (stderr):** quando il parser viene invocato con `pokedex <path>`, scrive su stderr una riga tipo  
  `[parser] Pokedex extract: 493 species, seen=X, caught=Y`.  
  Se avvii l’app da terminale (es. `npm run tauri dev`) puoi vedere questa riga a ogni sync.
- **Probe PKHeX:** dalla cartella del sidecar esegui  
  `dotnet run -- probe`  
  per elencare interfacce/proprietà/metodi su `SaveFile` che contengono Zukan/Pokedex/Seen/Caught. Serve a verificare che l’API usata (`GetSeen`/`GetCaught`) esista nella versione di PKHeX.Core in uso.

## Fonte dati: PKHeX.Core

Il parsing usa **PKHeX.Core** (SaveFile, GetSeen, GetCaught). Non c’è un parsing “nostro” dei byte del save per il Pokedex: ci appoggiamo all’API di PKHeX. Un parsing alternativo da zero richiederebbe la documentazione dei layout Pokedex per ogni generazione e sarebbe molto più complesso.

### Indice dex 0-based vs 1-based

Alcuni save/generazioni usano un indice dex **0-based** (0 = prima specie = Bulbasaur). In `Program.cs`, `PokedexHelper.UseDexIndexZeroBased = true` fa sì che venga passato `(speciesId - 1)` a GetSeen/GetCaught, così l’output `species_id = 1` corrisponde a Bulbasaur. Se i dati risultano sbagliati (es. specie spostate di uno), provare a impostare `UseDexIndexZeroBased = false`.

### Pre-evoluzioni

Se nel save è segnata solo l’ultima evoluzione come “catturata”, il sidecar e il backend Rust propagano “caught” alle pre-evoluzioni (es. Venusaur caught → Ivysaur e Bulbasaur segnati caught). Le catene Gen1 sono definite in `PokedexHelper.EvolvesFrom` (allineate a Rust `EVOLVES_FROM`).

## Se “visti” non compaiono

- In **Gen1/2** molti giochi tracciano solo “catturati”; GetSeen può essere sempre false o uguale a GetCaught.
- Da **Gen3** in poi seen e caught sono di solito separati; se in PKHeX (SAV → Pokedex) vedi “visti” ma nell’app no, segnala il titolo e la versione del save per controllare eventuali bug nel flusso o nell’API.
