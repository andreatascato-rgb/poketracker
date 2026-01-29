# Analisi: dati estraibili dal salvataggio con PKHeX

## Obiettivo

Elencare **tutti i dati** che possiamo estrarre da un file `.sav` usando la libreria **PKHeX.Core** (già in uso nel sidecar). Serve per capire cosa implementare nel parser e dove usare i dati (pagine, DB, UI).

**Stato attuale:** il sidecar espone solo il comando `detect` (gioco, versione, generazione, lingua). Tutto il resto va aggiunto con nuovi comandi/contratti.

---

## 1. Dati allenatore (Trainer)

Accessibili da `SaveFile` (e da blocchi tipo `TrainerData` / `State` a seconda della generazione).

| Dato | Proprietà/API PKHeX | Note |
|------|---------------------|------|
| **Nome allenatore (OT)** | `sav.OT` | Nome del giocatore. |
| **Trainer ID (TID)** | `sav.TID`, `sav.TrainerTID` | ID pubblico (numero a 5 cifre in molti giochi). |
| **Secret ID (SID)** | `sav.SID`, `sav.TrainerSID` | ID nascosto; con TID definisce "shiny lock" e proprietà. |
| **Denaro (Money)** | `sav.Money`, `sav.Currency` | Soldi in tasca; nome campo varia per generazione (es. Gen9). |
| **Badge / medaglie** | Proprietà/block per numero badge | Es. `sav.M badges`, blocchi `State`; dipende da Gen. |
| **Ore di gioco** | `sav.PlayedHours`, `sav.PlayedMinutes` | Tempo totale di gioco. |
| **Lingua di gioco** | `sav.Language`, `LanguageID` | Già usato indirettamente in detect (LanguageLabels). |
| **Sesso allenatore** | `sav.Gender` | Dove supportato (Gen2+). |
| **ID avventura / Save ID** | Vari (es. `sav.SaveCount`, ID univoco) | Dove presente nel formato. |
| **Firma / titolo** | Blocchi specifici (es. Gen8/9) | Firma profilo, titoli sbloccati. |
| **Stile / aspetto** | Blocchi "Fashion" o simili | Gen6+ (abbigliamento, capelli, ecc.). |

**Dove usarli:** profilo allenatore, dashboard, statistiche giocatore, header "nome + soldi + tempo".

### Dashboard

Pagina principale del profilo: riepilogo rapido e accesso alle altre sezioni.

| Dato allenatore | Uso |
|------------------|-----|
| Nome (OT) | Titolo / intestazione profilo. |
| Ore di gioco | Card o indicatore "tempo giocato". |
| Badge (numero) | Progresso campagna, badge visivo. |
| Gioco / versione (da detect) | Contesto "salvataggio da X". |
| Denaro (opzionale) | Card "soldi in tasca" se si espone. |

### Statistiche

Pagina dedicata a numeri e progressi dell'allenatore.

| Dato allenatore | Uso |
|------------------|-----|
| Nome (OT) | Intestazione pagina. |
| Ore di gioco | Statistica tempo totale. |
| Badge (numero) | Statistica progresso campagna. |
| Lingua / versione (da detect) | Contesto. |
| (Futuro: record battute/catture se estratti) | Altre statistiche. |

### Pokedex

Pokedex personale: stato visto/catturato per specie (dati da sezione Pokedex); l'allenatore fornisce contesto.

| Dato allenatore | Uso |
|------------------|-----|
| Nome (OT) | Intestazione "Pokedex di [nome]". |
| Gioco / versione (da detect) | Range specie e Dex regionale da mostrare. |
| Lingua | Localizzazione nomi specie (se da DB/wiki). |

### Salvataggi

Lista/griglia dei salvataggi associati al profilo; dati da file/cartella, non solo da allenatore.

| Dato allenatore | Uso |
|------------------|-----|
| Nome (OT) | Per ogni card salvataggio: "Allenatore: [nome]" (se estratto dal .sav). |
| Gioco / versione (da detect) | Per ogni card: etichetta gioco, icona. |
| Ore / badge (opzionale) | Dettaglio card salvataggio (ultimo giocato, progresso). |

---

### Mi serve / Non mi serve (Allenatore)

| Mi serve | Perché |
|----------|--------|
| Nome allenatore (OT) | Identificare il salvataggio, profilo, header. |
| Ore di gioco (PlayedHours / PlayedMinutes) | Statistiche, dashboard, "quanto hai giocato". |
| Badge / medaglie (numero) | Progresso campagna, badge UI, statistiche. |
| Lingua di gioco | Già in detect; utile per localizzazione dati. |
| Generazione / versione gioco | Già in detect; contesto per tutte le altre pagine. |

| Non mi serve (per ora) | Motivo |
|------------------------|--------|
| TID, SID | Servono per editing/legalità; per lettura e UI non li mostriamo. |
| Denaro (Money) | Opzionale: si può aggiungere in dashboard se vuoi "soldi in tasca". |
| Sesso allenatore | Solo se prevedi UI che lo mostra (avatar, testi). |
| ID avventura / Save ID | Interno; non serve in UI. |
| Firma, titoli, stile/aspetto | Complessi e Gen-specifici; rinviabili. |

---

## 2. Dati Pokémon (Party + Box)

`SaveFile` espone **party** e **box** tramite interfacce tipo `IBoxDetail` / array di slot.

| Dato | Come si ottiene | Note |
|------|-----------------|------|
| **Party (squadra attiva)** | `sav.PartyData`, `sav.GetPartySlot()`, `sav.Party` | Array/list di `PKM` (max 6). |
| **Box (deposito)** | `sav.BoxCount`, `sav.GetBoxName()`, slot per box/index | Ogni slot è un `PKM` o vuoto. |
| **Numero di Pokémon in party** | `sav.PartyCount`, `sav.PartyData.Count` | 0–6. |
| **Per ogni singolo PKM** | Oggetto `PKM` (PK1–PK9 a seconda Gen) | Vedi tabella sotto. |

### Per ogni singolo Pokémon (PKM)

| Dato | Proprietà tipica PKM | Note |
|------|----------------------|------|
| **Species (specie)** | `pkm.Species`, `pkm.EncounterSpecies` | ID specie (es. 25 = Pikachu). |
| **Form** | `pkm.Form` | Forma alternativa (Alola, Gigamax, ecc.). |
| **Nickname** | `pkm.Nickname` | Soprannome se impostato. |
| **Level** | `pkm.CurrentLevel`, `pkm.Stat_Level` | Livello attuale. |
| **Esperienza** | `pkm.EXP` | Punti esperienza. |
| **Natura** | `pkm.Nature` | Natura (Modesta, Audace, ecc.). |
| **Abilità** | `pkm.Ability`, `pkm.AbilityNumber` | Abilità attuale e indice. |
| **IV (Individual Values)** | `pkm.IV_HP`, `pkm.IV_ATK`, … oppure array `pkm.IVs` | 6 valori. |
| **EV (Effort Values)** | `pkm.EV_HP`, … oppure `pkm.EVs` | 6 valori. |
| **Mosse** | `pkm.Move1`–`Move4`, `pkm.Moves` | Mosse attuali; PP dove presente. |
| **OT (Original Trainer)** | `pkm.OT_Name`, `pkm.OT_Gender` | Nome e sesso del trainer originale. |
| **TID/SID OT** | `pkm.TID`, `pkm.SID` | Per "legalità" e shiny. |
| **Palla** | `pkm.Ball` | Tipo di Poké Ball. |
| **Sesso** | `pkm.Gender` | Maschio/Femmina/Genderless. |
| **Shiny** | `pkm.IsShiny` | Se è shiny. |
| **Pokerus** | `pkm.PokerusStatus` | Dove supportato. |
| **Ribbon / titoli** | `pkm.RibbonCount`, flag ribbon | Titoli e ribbon. |
| **Met data** | `pkm.Met_Level`, `pkm.Met_Location`, `pkm.MetDate` | Dove è stato incontrato/catturato. |
| **Marking / preferiti** | `pkm.Marking`, flag preferito | Stelle/marking e "preferito" in box. |

**Dove usarli:** pagina "Squadra", pagina "Box", Pokedex personale (specie possedute), statistiche (livello medio, specie, forme).

### Mi serve / Non mi serve (Pokémon)

| Mi serve | Perché |
|----------|--------|
| Party (squadra attiva): quali slot pieni | Pagina Squadra, numero Pokémon in squadra. |
| Per ogni PKM in party: Species, Form, Nickname, Level | Nome, icona, livello in lista squadra. |
| Per ogni PKM in party: Ball, Shiny, Gender | Dettaglio card (palla, shiny, sesso). |
| Box: numero box, nome box (opzionale) | Navigazione box, titoli. |
| Per ogni PKM in box: Species, Form, Nickname, Level, Ball, Shiny | Lista/griglia box; stessi dati della squadra. |
| Species + Form (per ogni PKM) | Pokedex personale "specie possedute", icone, filtri. |

| Non mi serve (per ora) | Motivo |
|------------------------|--------|
| IV, EV, Natura, Abilità, Mosse | Dettaglio "scheda tecnica" avanzata; rinviabile. |
| Esperienza (EXP) | Derivabile da livello; raramente mostrato in UI. |
| OT, TID/SID del PKM | Legalità/editing; non necessario per visualizzazione. |
| Pokerus, Ribbon, Met data (dove catturato) | Dettaglio avanzato; si può aggiungere dopo. |
| Marking / preferiti | Solo se fai "preferiti in box" in UI. |

---

## 3. Pokedex (Zukan / Dex)

Stato "visto" e "catturato" per ogni specie. In PKHeX spesso esposto come **Zukan** o proprietà tipo `Pokedex`.

| Dato | Come si ottiene | Note |
|------|-----------------|------|
| **Specie viste** | Flag "seen" per specie (es. `GetSeen(species)`, `Pokedex.GetSeen`) | Varia per Gen (bitfield, array, ecc.). |
| **Specie catturate** | Flag "caught" / "owned" (es. `GetCaught(species)`, "Owned" in UI) | Per completamento Dex. |
| **Forme viste/catturate** | Dove supportato (Gen6+) | Forme alternative nella Dex. |
| **Conteggi** | Numero totale visti, catturati | Utili per percentuale completamento. |
| **Dex regionale** | Quale Dex è attivo (Kanto, Galar, Paldea, ecc.) | Gen dipendente. |

**Dove usarli:** pagina **Pokedex personale** (stato visto/catturato per ogni specie), badge "Dex completato", statistiche.

### Mi serve / Non mi serve (Pokedex)

| Mi serve | Perché |
|----------|--------|
| Per ogni specie: visto (sì/no) | Stato "non visto" vs "visto" in Pokedex personale. |
| Per ogni specie: catturato (sì/no) | Stato "catturato", icona Pokéball, completamento. |
| Conteggi: totale visti, totale catturati | Percentuale completamento, badge "Dex X%". |
| Range specie valide per il gioco (Gen/Dex regionale) | Sapere quali specie mostrare nella lista (es. solo Paldea). |

| Non mi serve (per ora) | Motivo |
|------------------------|--------|
| Forme viste/catturate per specie | Complica molto; si può fare "specie = vista/catturata" senza forme. |
| Dettaglio "quale Dex è attivo" (nome) | Basta sapere il range specie dal gioco/versione. |

---

## 4. Storia / eventi / progresso gioco

Gestiti da **event flag** e talvolta **event constant**. Struttura dipende dalla generazione.

| Dato | Come si ottiene | Note |
|------|-----------------|------|
| **Event flag** | Blocchi "EventFlag", "EventWork", "Story" | Bit/flag che indicano "quest completata", "NPC parlato", ecc. |
| **Event constant** | Valori numerici (es. step story) | Es. "gym badge count", "main story chapter". |
| **Ricerca flag tra due save** | Strumento "Event Flags Research" in PKHeX | Confronto save prima/dopo per vedere quali flag sono cambiati. |
| **Story completion** | Set di flag specifici per titolo | Non un singolo "story_done"; spesso si copia la regione flag da un save completato. |

**Limitazione:** non esiste un'API unica "dammi la storia completata". Va mappato titolo per titolo (es. "Gym 1 sconfitto", "Lega battuta") sui relativi flag.

**Dove usarli:** pagina "Storia / Progresso", badge "Storia completata", checklist obiettivi (opzionale, complesso).

### Mi serve / Non mi serve (Storia / eventi)

| Mi serve | Perché |
|----------|--------|
| Numero badge (se non già in dati allenatore) | Progresso campagna; spesso già coperto da "Badge" in Allenatore. |
| Un indicatore "storia principale completata" (se esiste per il titolo) | Badge "Storia finita", solo dove PKHeX o il formato lo espongono in modo chiaro. |

| Non mi serve (per ora) | Motivo |
|------------------------|--------|
| Singoli event flag (quest, NPC, step) | Troppo Gen-specifico e complesso; mappatura titolo per titolo. |
| Event constant grezzi | Stesso motivo; servirebbero tabelle di significato per ogni gioco. |
| Ricerca flag tra due save | Feature da editor, non da tracker/visualizzazione. |

**Nota:** per una pagina "Progresso" semplice basta **badge + ore** (già in Allenatore). Una "Storia completata" richiede mappatura flag per titolo; si può rinviare.

---

## 5. Inventario e oggetti

| Dato | Come si ottiene | Note |
|------|-----------------|------|
| **Oggetti in borsa** | Blocco "Items", "Inventory", `sav.Inventory` (dove presente) | Lista (item id + quantità). |
| **PC item / deposito** | Dove supportato (es. Gen8/9) | Oggetti in deposito. |
| **Strumenti chiave** | Flag o slot speciali (Bike, Rod, ecc.) | Dipende da Gen. |
| **Poké Ball disponibili** | Sottoinsieme inventario | Per "quali palle ha l'allenatore". |
| **TM/TR posseduti** | Bitfield o lista (Gen dipendente) | Mosse insegnabili. |

**Dove usarli:** pagina "Inventario", "Oggetti", filtri (es. "ha Master Ball?").

### Mi serve / Non mi serve (Inventario)

| Mi serve | Perché |
|----------|--------|
| Lista oggetti in borsa: id oggetto + quantità | Pagina Inventario (nome + icona + quantità). |
| Strumenti chiave posseduti (Bike, Rod, ecc.) | Progresso gioco, "cosa hai sbloccato"; opzionale. |

| Non mi serve (per ora) | Motivo |
|------------------------|--------|
| PC item / deposito | Seconda priorità; si può aggiungere dopo la borsa. |
| TM/TR posseduti (bitfield) | Dettaglio avanzato; utile solo per vista "mosse insegnabili". |
| Poké Ball come sottoinsieme | Derivabile dalla lista borsa filtrando per id palla. |

---

## 6. Altri dati utili

| Dato | Come si ottiene | Note |
|------|-----------------|------|
| **Versione gioco** | `sav.Version` | Già usato in detect (GameNamesIt). |
| **Generazione** | `sav.Generation` o da tipo save (SAV9, SAV8…) | Già esposto in detect. |
| **Nome box** | `sav.GetBoxName(boxIndex)` | Nomi personalizzati dei box. |
| **Wallpaper / tema box** | Dove supportato | Aspetto box. |
| **Mystery Gift / regali** | Blocchi "Mystery Gift", "Card" | Eventi scaricati, regali non ancora ritirati. |
| **Record (battute, catture, ecc.)** | Statistiche allenatore (es. "total_battles", "capture_wild") | Dove presenti nel save; in Gen9 in blocchi specifici. |
| **League Card (Gen8)** | Dati carta allenatore | Avatar, titoli, statistiche pubbliche. |
| **Co-op / Raid (Gen8/9)** | Flag o dati raid | Dove disponibili. |

**Dove usarli:** statistiche giocatore, "Record", pagina Raid/Co-op (se si implementa).

### Mi serve / Non mi serve (Altri)

| Mi serve | Perché |
|----------|--------|
| Nome box (GetBoxName) | Titoli box in pagina Box; migliora UX. |
| Versione / generazione | Già in detect; non duplicare, ma usare come contesto. |

| Non mi serve (per ora) | Motivo |
|------------------------|--------|
| Wallpaper / tema box | Solo estetico in-game; non necessario in app. |
| Mystery Gift / regali | Feature dedicata; rinviabile. |
| Record (battute, catture) | Statistiche "nice to have"; bassa priorità. |
| League Card, Co-op/Raid | Gen-specifico; si può aggiungere in seguito. |

---

## Riepilogo per categoria

| Categoria | Dati principali | Complessità estrazione | Priorità per pagine |
|-----------|-----------------|------------------------|----------------------|
| **Allenatore** | Nome, TID, SID, soldi, badge, ore, sesso, lingua | Bassa (proprietà dirette) | Alta: profilo, dashboard |
| **Pokémon** | Party + Box, per PKM: specie, livello, mosse, IV/EV, natura, OT, palla, shiny, met | Media (iterare slot, serializzare PKM) | Alta: squadra, box, Dex personale |
| **Pokedex** | Visto/catturato per specie, conteggi, forme | Media (API Zukan/Dex per Gen) | Alta: Pokedex personale |
| **Storia/Eventi** | Event flag, constant | Alta (mappatura per titolo) | Media: pagina "Progresso" |
| **Inventario** | Oggetti borsa, quantità, TM, strumenti | Media (blocchi Items) | Media: pagina Inventario |
| **Altri** | Box name, record, Mystery Gift, League Card | Variabile | Bassa / futura |

---

## Prossimi passi suggeriti

1. **Estendere il sidecar** con nuovi comandi (es. `trainer`, `party`, `boxes`, `pokedex`, `inventory`) che leggono `SaveFile` e restituiscono JSON.
2. **Definire contratti JSON** per ogni comando (campi, tipi, valori null dove opzionale).
3. **Rust:** invocare il sidecar con il path del `.sav` e il comando; deserializzare JSON e salvare in DB o restituire al frontend.
4. **DB:** decidere quali dati persistire (es. snapshot allenatore + party + pokedex per save/profilo) e quali solo in memoria.
5. **Pagine:** usare i dati per Squadra, Box, Pokedex personale, Inventario, Progresso (in quest'ordine di priorità se vuoi).

Se vuoi, il passo successivo può essere: (A) definire il contratto JSON per "trainer" + "party" e l'implementazione lato sidecar, oppure (B) scegliere una pagina (es. Pokedex personale) e collegare i dati estratti end-to-end.
