# Contratto Parser `trainer`

## Obiettivo

Definisce il contratto JSON tra il sidecar C# (comando `trainer`) e il backend Rust per l'estrazione dei dati allenatore dal salvataggio.

---

## Comando sidecar

```bash
parser.exe trainer <path-to-sav>
```

**Input:**
- `<path-to-sav>`: percorso assoluto al file `.sav`

**Output:** JSON su stdout

---

## Contratto JSON

```json
{
  "money": 12500,
  "playedHours": 24,
  "playedMinutes": 35
}
```

### Campi

| Campo | Tipo | Nullable | Descrizione |
|-------|------|----------|-------------|
| `money` | `uint` | ✅ | Denaro in tasca. `null` se non disponibile o errore estrazione. Gen9 usa `Currency` invece di `Money`. |
| `playedHours` | `int` | ✅ | Ore di gioco totali. `null` se non disponibile. |
| `playedMinutes` | `int` | ✅ | Minuti di gioco (0-59). `null` se non disponibile. |

**Note:**
- Tutti i campi sono **nullable**: se PKHeX non espone la proprietà per quella generazione/tipo di save, il campo è `null`.
- `money`: Gen1-8 usano `SaveFile.Money` (uint); Gen9 usa `SaveFile.Currency` (uint). Il parser prova entrambi.
- `playedHours` / `playedMinutes`: proprietà `SaveFile.PlayedHours` e `SaveFile.PlayedMinutes`. Tipi possono variare (int, ushort, byte) — il parser converte a `int`.

---

## Esempi

### Salvataggio Gen 3 (Rosso Fuoco)

```json
{
  "money": 42350,
  "playedHours": 15,
  "playedMinutes": 42
}
```

### Salvataggio Gen 4 (Platino)

```json
{
  "money": 125000,
  "playedHours": 120,
  "playedMinutes": 18
}
```

### Salvataggio dove dati non disponibili

```json
{
  "money": null,
  "playedHours": null,
  "playedMinutes": null
}
```

---

## Gestione errori

**Errore caricamento save:**
- Exit code `1`, messaggio su stderr: `[parser] error: Unrecognized save format: ...`

**Errore estrazione campo specifico:**
- Campo impostato a `null`, log su stderr: `[parser] trainer: Money extraction failed: ...`
- Il JSON viene comunque restituito con gli altri campi (se disponibili)

---

## Integrazione Rust

### Tipo Rust (da definire)

```rust
#[derive(Debug, Deserialize)]
pub struct TrainerData {
    pub money: Option<u32>,
    #[serde(rename = "playedHours")]
    pub played_hours: Option<i32>,
    #[serde(rename = "playedMinutes")]
    pub played_minutes: Option<i32>,
}
```

### Invocazione

```rust
let output = Command::new(&parser_path)
    .args(&["trainer", sav_path])
    .output()
    .await?;

let trainer_data: TrainerData = serde_json::from_slice(&output.stdout)?;
```

### Dove usare

- **Dashboard allenatore:** mostrare denaro e tempo nella card Allenatore
- **Statistiche:** tempo totale di gioco
- **DB (opzionale):** salvare snapshot dati allenatore per profilo/save

---

## Riferimenti

- **Parser inventory:** `docs/project/parser-inventory.md`
- **Analisi dati estraibili:** `docs/temp/analisi-dati-estraibili-pkhex.md`
- **Sidecar:** `src-sidecar/Program.cs` (comando `trainer`, classe `TrainerHelper`)
