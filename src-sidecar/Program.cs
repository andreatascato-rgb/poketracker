using System.Reflection;
using System.Text;
using System.Text.Json;
using System.Linq;
using PKHeX.Core;

Console.OutputEncoding = Encoding.UTF8;

// Normalizza argomenti: Tauri su Windows può passare (exePath, command, path) quindi args[0]
// può essere il path dell'eseguibile. Prova prima (args[0], args[1]), poi (args[1], args[2]).
static bool IsValidCommand(string? c) =>
    c == "detect" || c == "pokedex" || c == "pokedex_auto" || c == "probe" || c == "trainer";

string command;
string? path;
if (args.Length >= 1 && IsValidCommand(args[0].Trim().ToLowerInvariant()))
{
    command = args[0].Trim().ToLowerInvariant();
    path = args.Length > 1 ? args[1] : null;
}
else if (args.Length >= 2 && IsValidCommand(args[1].Trim().ToLowerInvariant()))
{
    command = args[1].Trim().ToLowerInvariant();
    path = args.Length > 2 ? args[2] : null;
}
else
{
    WriteError("Usage: <command> [path]. Commands: detect <path>, pokedex <path>, pokedex_auto <path>, trainer <path>, probe.");
    Environment.Exit(1);
    return;
}

if (command == "probe")
{
    PkHexPokedexProbe.Run();
    Environment.Exit(0);
    return;
}

if (command != "detect" && command != "pokedex" && command != "pokedex_auto" && command != "trainer")
{
    WriteError("Unknown command. Use: detect <path>, pokedex <path>, pokedex_auto <path>, or trainer <path>");
    Environment.Exit(1);
    return;
}

if (string.IsNullOrEmpty(path) || !File.Exists(path))
{
    WriteError("File not found: " + (path ?? ""));
    Environment.Exit(1);
    return;
}

try
{
    byte[] data = File.ReadAllBytes(path);
    int fileLength = data.Length;
    const int Gen4ExpectedSize = 524288; // 512KB

    // Trim a 512KB se più grande (Gen4 e .dsv DeSmuME hanno footer che rompe il riconoscimento).
    bool isFooterPresent = data.Length > Gen4ExpectedSize;
    if (isFooterPresent)
    {
        byte[] trimmedData = new byte[Gen4ExpectedSize];
        Array.Copy(data, 0, trimmedData, 0, Gen4ExpectedSize);
        data = trimmedData;
        Console.Error.WriteLine($"[parser] load: trimmato a {data.Length} (0x{data.Length:X}) byte");
    }

    SaveFile? sav = SaveUtil.GetVariantSAV(data);
    if (sav == null)
    {
        try
        {
            sav = new SAV4HGSS(data);
            Console.Error.WriteLine("[parser] load: GetVariantSAV null, caricato forzato come SAV4HGSS");
        }
        catch (Exception ex)
        {
            WriteError("Unrecognized save format: " + ex.Message);
            Environment.Exit(1);
            return;
        }
    }

    LogSaveGameAndVersion(sav);
    LogDebugSaveInfo(fileLength, isFooterPresent, sav, data, entriesCount: null);

    if (command == "detect")
    {
        string gameName = SaveDetectHelper.GetGameNameItalian(sav);
        int? langIdRaw = SaveDetectHelper.GetLanguageId(sav);
        string languageLabel = SaveDetectHelper.GetLanguageLabel(sav, path);

        var result = new
        {
            game = gameName,
            version = languageLabel,
            generation = SaveDetectHelper.GetGeneration(sav),
            languageIdRaw = langIdRaw
        };

        string json = JsonSerializer.Serialize(result);
        Console.WriteLine(json);
        Environment.Exit(0);
        return;
    }

    if (command == "trainer")
    {
        var trainerData = TrainerHelper.ExtractTrainerData(sav);
        Console.WriteLine(JsonSerializer.Serialize(trainerData));
        Environment.Exit(0);
        return;
    }

    if (command == "pokedex")
    {
        var entries = PokedexHelper.GetAllSpeciesStatus(sav);
        var result = new { entries };
        string json = JsonSerializer.Serialize(result);
        Console.WriteLine(json);
        Environment.Exit(0);
        return;
    }

    if (command == "pokedex_auto")
    {
        List<PokedexHelper.PokedexEntry> entries;
        string typeName = sav.GetType().Name;
        string languageLabel = SaveDetectHelper.GetLanguageLabel(sav, path);
        bool isFrLg = typeName == "SAV3FRLG" || typeName == "SAV3FR" || typeName == "SAV3LG";
        bool isEmerald = typeName == "SAV3E" || typeName == "SAV3Emerald" || typeName.IndexOf("Emerald", StringComparison.OrdinalIgnoreCase) >= 0;
        bool isSAV4 = typeName.StartsWith("SAV4", StringComparison.OrdinalIgnoreCase) || sav.Generation == 4;
        bool isHgSs = typeName.IndexOf("HGSS", StringComparison.OrdinalIgnoreCase) >= 0;
        string parserName = isFrLg ? "FRLG" : isEmerald ? "Emerald" : isSAV4 ? "SAV4" : "nessuno";
        Console.Error.WriteLine($"[parser] pokedex_auto: type={typeName} lang={languageLabel} → {parserName}");

        // Gen4 (HGSS, D/P/Pt): un solo percorso, reflection su sav.Pokedex.
        if (isSAV4)
        {
            try
            {
                entries = PokedexHelper.GetAllSpeciesStatusSAV4Pokedex(sav);
                Console.Error.WriteLine($"[parser] pokedex_auto SAV4.Pokedex: {entries.Count} entries");
                if (entries.Count == 0)
                {
                    try
                    {
                        sav = new SAV4HGSS(data);
                        entries = PokedexHelper.GetAllSpeciesStatusSAV4Pokedex(sav);
                        Console.Error.WriteLine($"[parser] pokedex_auto: Pokedex vuoto → forzato SAV4HGSS, entries = {entries.Count}");
                    }
                    catch (Exception ex2)
                    {
                        Console.Error.WriteLine($"[parser] pokedex_auto SAV4HGSS fallback: {ex2.Message}");
                    }
                }
                LogDebugSaveInfo(fileLength, isFooterPresent, sav, data, entries.Count);
            }
            catch (Exception ex)
            {
                Console.Error.WriteLine($"[parser] pokedex_auto SAV4.Pokedex failed: {ex.Message}");
                entries = new List<PokedexHelper.PokedexEntry>();
                LogDebugSaveInfo(fileLength, isFooterPresent, sav, data, 0);
            }
        }
        else if (isFrLg)
        {
            switch (languageLabel)
            {
                case "ITA":
                    entries = FrLgPokedexParser.Parse(data, path ?? "", "ITA");
                    break;
                case "USA":
                    entries = FrLgPokedexParser.Parse(data, path ?? "", "USA");
                    break;
                case "JPN":
                    entries = FrLgPokedexParser.Parse(data, path ?? "", "JPN");
                    break;
                case "FRA":
                    entries = FrLgPokedexParser.Parse(data, path ?? "", "FRA");
                    break;
                case "GER":
                    entries = FrLgPokedexParser.Parse(data, path ?? "", "GER");
                    break;
                case "SPA":
                    entries = FrLgPokedexParser.Parse(data, path ?? "", "SPA");
                    break;
                default:
                    entries = FrLgPokedexParser.Parse(data, path ?? "", languageLabel.Length > 0 ? languageLabel : "—");
                    break;
            }
        }
        else if (isEmerald)
        {
            switch (languageLabel)
            {
                case "ITA":
                    entries = EmeraldPokedexParser.Parse(data, path ?? "", "ITA");
                    break;
                case "USA":
                    entries = EmeraldPokedexParser.Parse(data, path ?? "", "USA");
                    break;
                case "JPN":
                    entries = EmeraldPokedexParser.Parse(data, path ?? "", "JPN");
                    break;
                case "FRA":
                    entries = EmeraldPokedexParser.Parse(data, path ?? "", "FRA");
                    break;
                case "GER":
                    entries = EmeraldPokedexParser.Parse(data, path ?? "", "GER");
                    break;
                case "SPA":
                    entries = EmeraldPokedexParser.Parse(data, path ?? "", "SPA");
                    break;
                default:
                    entries = EmeraldPokedexParser.Parse(data, path ?? "", languageLabel.Length > 0 ? languageLabel : "—");
                    break;
            }
        }
        else
        {
            entries = new List<PokedexHelper.PokedexEntry>();
            Console.Error.WriteLine($"[parser] pokedex_auto: nessun parser per type={typeName}, 0 entries");
        }
        Console.Error.WriteLine($"[parser] pokedex_auto: {entries.Count} entries");
        var result = new { entries };
        string json = JsonSerializer.Serialize(result);
        Console.WriteLine(json);
        Environment.Exit(0);
        return;
    }
}
catch (Exception ex)
{
    WriteError(ex.Message);
    Environment.Exit(1);
}

/// Scrive l'errore solo su stderr. Stdout resta riservato al JSON di successo così Rust non riceve righe spurie.
static void WriteError(string message)
{
    var err = JsonSerializer.Serialize(new { error = message });
    Console.Error.WriteLine(err);
}

static void LogSaveGameAndVersion(SaveFile sav)
{
    try
    {
        var t = sav.GetType();
        string? game = null, version = null;
        foreach (var p in t.GetProperties(BindingFlags.Public | BindingFlags.Instance))
        {
            if (string.Equals(p.Name, "Game", StringComparison.OrdinalIgnoreCase) && p.PropertyType == typeof(GameVersion))
                game = p.GetValue(sav)?.ToString();
            if (string.Equals(p.Name, "Version", StringComparison.OrdinalIgnoreCase))
                version = p.GetValue(sav)?.ToString();
        }
        if (game == null)
        {
            var gv = t.GetProperty("GameVersion", BindingFlags.Public | BindingFlags.Instance);
            if (gv != null) game = gv.GetValue(sav)?.ToString();
        }
        Console.Error.WriteLine($"[parser] load: sav.Game = {game ?? "—"}, sav.Version = {version ?? "—"}");
    }
    catch (Exception ex)
    {
        Console.Error.WriteLine($"[parser] load: LogSaveGameAndVersion: {ex.Message}");
    }
}

/// <summary>Debug log: FileLength, IsFooterPresent, SaveCount, CheckSumStatus.</summary>
static void LogDebugSaveInfo(int fileLength, bool isFooterPresent, SaveFile sav, byte[] data, int? entriesCount)
{
    try
    {
        Console.Error.WriteLine($"[parser] debug: FileLength = {fileLength}");
        Console.Error.WriteLine($"[parser] debug: IsFooterPresent = {isFooterPresent}");

        object? footer = null;
        for (var t = sav.GetType(); t != null; t = t.BaseType)
        {
            var p = t.GetProperty("Footer", BindingFlags.Public | BindingFlags.Instance);
            if (p != null) { try { footer = p.GetValue(sav); break; } catch { } }
        }
        if (footer != null)
        {
            var fc = footer.GetType().GetProperty("SaveCount", BindingFlags.Public | BindingFlags.Instance);
            if (fc != null) Console.Error.WriteLine($"[parser] debug: SaveCount = {fc.GetValue(footer) ?? "—"}");
        }
        else
            Console.Error.WriteLine("[parser] debug: SaveCount = (Footer non trovato)");

        object? checksumValid = null;
        for (var t = sav.GetType(); t != null; t = t.BaseType)
        {
            var p = t.GetProperty("ChecksumValid", BindingFlags.Public | BindingFlags.Instance);
            if (p != null) { try { checksumValid = p.GetValue(sav); break; } catch { } }
        }
        Console.Error.WriteLine($"[parser] debug: CheckSumStatus = {(checksumValid != null ? checksumValid.ToString() : "(proprietà non trovata)")}");
    }
    catch (Exception ex)
    {
        Console.Error.WriteLine($"[parser] debug: LogDebugSaveInfo: {ex.Message}");
    }
}

/// <summary>Helper per detect: mappe GameVersion/tipo → italiano, LanguageID → ITA/USA, ecc.</summary>
file static class SaveDetectHelper
{
    public static string GetGameNameItalian(SaveFile sav)
    {
        string versionKey = sav.Version.ToString();
        string typeName = sav.GetType().Name;

        if (GameNamesIt.TryGetValue(versionKey, out string? name))
            return name;
        if (GameNamesByType.TryGetValue(typeName, out name))
            return name;
        return versionKey;
    }

    /// <summary>Restituisce etichetta lingua/regione (ITA, USA, …). "Versione" in UI = lingua/regione. Se dal save non si ottiene nulla di utile (null o mappa a "—"), prova a inferire dal path.</summary>
    public static string GetLanguageLabel(SaveFile sav, string? path = null)
    {
        int? langId = GetLanguageId(sav);
        string? fromMap = null;
        if (langId is int id && LanguageLabels.TryGetValue(id, out string? label))
            fromMap = label;
        // Quando dal save non abbiamo una lingua utile (null, o 0/6 che mappano a "—"), usa il path
        bool nienteUtileDalSave = fromMap == null || fromMap == "—";
        if (nienteUtileDalSave && !string.IsNullOrEmpty(path))
        {
            string fromPath = InferLanguageFromPath(path);
            if (fromPath.Length > 0)
                return fromPath;
        }
        if (fromMap != null && fromMap != "—")
            return fromMap;
        if (langId.HasValue)
            return $"L{langId}";
        return "—";
    }

    /// <summary>Per Gen1-3 PKHeX spesso non espone la lingua dal save; si può dedurre dal nome/path file. Cerca ITA, USA, JPN, ecc. nel path (case insensitive).</summary>
    public static string InferLanguageFromPath(string path)
    {
        if (string.IsNullOrEmpty(path)) return "";
        var p = path.AsSpan();
        if (Contains(p, "ITA") || Contains(p, "ITALIAN") || Contains(p, "ITALY")) return "ITA";
        if (Contains(p, "USA") || Contains(p, "ENG") || Contains(p, "ENGLISH")) return "USA";
        if (Contains(p, "JPN") || Contains(p, "JAPAN")) return "JPN";
        if (Contains(p, "FRA") || Contains(p, "FRENCH")) return "FRA";
        if (Contains(p, "GER") || Contains(p, "GERMAN")) return "GER";
        if (Contains(p, "SPA") || Contains(p, "SPANISH")) return "SPA";
        if (Contains(p, "KOR") || Contains(p, "KOREAN")) return "KOR";
        if (Contains(p, "CHT")) return "CHT";
        if (Contains(p, "CHS")) return "CHS";
        return "";
    }

    private static bool Contains(ReadOnlySpan<char> haystack, string needle)
    {
        return haystack.IndexOf(needle.AsSpan(), StringComparison.OrdinalIgnoreCase) >= 0;
    }

    /// <summary>Restituisce il numero di generazione (1–9). Prova proprietà Generation, altrimenti ricava dal nome tipo (SAV9→9, SAV8→8, …).</summary>
    public static int GetGeneration(SaveFile sav)
    {
        var t = sav.GetType();
        var prop = t.GetProperty("Generation", BindingFlags.Public | BindingFlags.Instance);
        if (prop != null)
        {
            try
            {
                if (prop.GetValue(sav) is int g && g >= 1 && g <= 9)
                    return g;
            }
            catch { /* ignore */ }
        }
        for (var b = t.BaseType; b != null; b = b.BaseType)
        {
            prop = b.GetProperty("Generation", BindingFlags.Public | BindingFlags.Instance);
            if (prop != null)
            {
                try
                {
                    if (prop.GetValue(sav) is int g && g >= 1 && g <= 9)
                        return g;
                }
                catch { /* ignore */ }
                break;
            }
        }
        // Fallback: dal nome tipo (SAV9SV → 9, SAV8BS → 8, SAV1Game → 1)
        string name = t.Name;
        if (name.Length >= 4 && name.StartsWith("SAV", StringComparison.OrdinalIgnoreCase) && char.IsDigit(name[3]))
            return name[3] - '0';
        return 0;
    }

    /// <summary>Legge l'ID lingua dal save. Prova proprietà dirette, poi blocchi e sottoblocchi (Blocks, Data, State, ecc.). Restituisce null se non trovato (es. Gen1-3 dove PKHeX inferisce dal nome file).</summary>
    public static int? GetLanguageId(SaveFile sav)
    {
        var t = sav.GetType();
        // 1) Proprietà dirette su SaveFile/tipo derivato (anche qualsiasi prop con "Language"/"Lang" nel nome)
        int? v = GetLangFromType(t, sav);
        if (v.HasValue) return v.Value;
        foreach (var prop in t.GetProperties(BindingFlags.Public | BindingFlags.Instance))
        {
            if ((prop.Name.IndexOf("Language", StringComparison.OrdinalIgnoreCase) >= 0 ||
                 prop.Name.IndexOf("Lang", StringComparison.OrdinalIgnoreCase) >= 0) &&
                (prop.PropertyType == typeof(int) || prop.PropertyType == typeof(byte)))
            {
                try
                {
                    var val = prop.GetValue(sav);
                    if (val is int i) return i;
                    if (val is byte b) return b;
                }
                catch { /* ignore */ }
            }
        }
        // 2) Salire nella gerarchia
        for (var b = t.BaseType; b != null; b = b.BaseType)
        {
            v = GetLangFromType(b, sav);
            if (v.HasValue) return v.Value;
        }
        // 3) Cercare in blocchi e sottoblocchi (Blocks, Data, SaveBlock, State, Blk, Block, …)
        string[] blockNames = { "Blocks", "Data", "SaveBlock", "Blk", "Block", "State", "SaveData", "TrainerData" };
        foreach (var p in t.GetProperties(BindingFlags.Public | BindingFlags.Instance))
        {
            bool isBlock = false;
            foreach (var bn in blockNames)
            {
                if (string.Equals(bn, p.Name, StringComparison.OrdinalIgnoreCase)) { isBlock = true; break; }
            }
            if (!isBlock) continue;
            try
            {
                var block = p.GetValue(sav);
                if (block == null) continue;
                v = GetLangFromObject(block, depth: 2);
                if (v.HasValue) return v.Value;
            }
            catch { /* ignore */ }
        }
        // 4) Scansione completa: tutte le proprietà del save che restituiscono un oggetto (es. Gen3 Trainer/Options)
        foreach (var p in t.GetProperties(BindingFlags.Public | BindingFlags.Instance))
        {
            if (p.PropertyType.IsPrimitive || p.PropertyType == typeof(string) || p.PropertyType.IsEnum) continue;
            if (p.PropertyType.IsValueType && !p.PropertyType.IsPrimitive) continue; // evita struct grossi
            try
            {
                var val = p.GetValue(sav);
                if (val == null) continue;
                v = GetLangFromObject(val, depth: 2);
                if (v.HasValue) return v.Value;
            }
            catch { /* ignore */ }
        }
        return null;
    }

    private static int? GetLangFromObject(object obj, int depth)
    {
        if (obj == null || depth <= 0) return null;
        var type = obj.GetType();
        int? v = GetLangFromType(type, obj);
        if (v.HasValue) return v.Value;
        // Se è una raccolta (es. dictionary di blocchi), scorrere i valori
        if (type.IsGenericType)
        {
            var gen = type.GetGenericTypeDefinition();
            if (gen == typeof(System.Collections.Generic.Dictionary<,>))
            {
                try
                {
                    var valuesProp = type.GetProperty("Values", BindingFlags.Public | BindingFlags.Instance);
                    var values = valuesProp?.GetValue(obj) as System.Collections.IEnumerable;
                    if (values != null)
                    {
                        foreach (var item in values)
                        {
                            if (item == null) continue;
                            v = GetLangFromObject(item, depth - 1);
                            if (v.HasValue) return v.Value;
                        }
                    }
                }
                catch { /* ignore */ }
            }
        }
        foreach (var prop in type.GetProperties(BindingFlags.Public | BindingFlags.Instance))
        {
            if (prop.PropertyType.IsPrimitive || prop.PropertyType == typeof(string)) continue;
            try
            {
                var child = prop.GetValue(obj);
                if (child == null) continue;
                v = GetLangFromObject(child, depth - 1);
                if (v.HasValue) return v.Value;
            }
            catch { /* ignore */ }
        }
        return null;
    }

    private static int? GetLangFromType(Type type, object obj)
    {
        var prop = type.GetProperty("Language", BindingFlags.Public | BindingFlags.Instance)
            ?? type.GetProperty("LanguageID", BindingFlags.Public | BindingFlags.Instance)
            ?? type.GetProperty("GameLanguage", BindingFlags.Public | BindingFlags.Instance);
        if (prop == null) return null;
        try
        {
            var val = prop.GetValue(obj);
            if (val is int i) return i;
            if (val is byte b) return b;
            if (val is Enum e) return Convert.ToInt32(e);
        }
        catch { /* ignore */ }
        return null;
    }

    /// <summary>GameVersion.ToString() → nome italiano. Gen1–Gen4 completi, poi Gen5+.</summary>
    private static readonly Dictionary<string, string> GameNamesIt = new(StringComparer.OrdinalIgnoreCase)
    {
        // Gen1
        ["RD"] = "Rosso",
        ["GN"] = "Verde",
        ["BU"] = "Blu",
        ["YW"] = "Giallo",
        ["RB"] = "Rosso/Blu",
        ["GY"] = "Giallo",
        // Gen2
        ["GD"] = "Oro",
        ["SI"] = "Argento",
        ["C"] = "Cristallo",
        ["GS"] = "Oro/Argento",
        // Gen3
        ["R"] = "Rubino",
        ["S"] = "Zaffiro",
        ["E"] = "Smeraldo",
        ["RS"] = "Rubino/Zaffiro",
        ["FR"] = "Rosso Fuoco",
        ["LG"] = "Verde Foglia",
        ["FRLG"] = "Rosso Fuoco/Verde Foglia",
        // Gen4
        ["D"] = "Diamante",
        ["P"] = "Perla",
        ["Pt"] = "Platino",
        ["DP"] = "Diamante/Perla",
        ["DPPt"] = "Diamante/Perla/Platino",
        ["HG"] = "Oro Heart",
        ["SS"] = "Argento Soul",
        ["HGSS"] = "Oro Heart/Argento Soul",
        // Gen5
        ["B"] = "Nero",
        ["W"] = "Bianco",
        ["B2"] = "Nero 2",
        ["W2"] = "Bianco 2",
        ["BW"] = "Nero/Bianco",
        ["B2W2"] = "Nero 2/Bianco 2",
        // Gen6
        ["X"] = "Pokémon X",
        ["Y"] = "Pokémon Y",
        ["OR"] = "Rubino Omega",
        ["AS"] = "Zaffiro Alpha",
        ["XY"] = "Pokémon X/Y",
        ["ORAS"] = "Rubino Omega/Zaffiro Alpha",
        // Gen7
        ["SN"] = "Sole",
        ["MN"] = "Luna",
        ["SM"] = "Sole/Luna",
        ["US"] = "Ultra Sole",
        ["UM"] = "Ultra Luna",
        ["USUM"] = "Ultra Sole/Ultra Luna",
        // Gen8
        ["SW"] = "Spada",
        ["SH"] = "Scudo",
        ["SWSH"] = "Spada/Scudo",
        ["BD"] = "Diamante Lucente",
        ["SP"] = "Perla Splendente",
        ["BDSP"] = "Diamante Lucente/Perla Splendente",
        ["GP"] = "Let's Go Pikachu",
        ["GE"] = "Let's Go Eevee",
        ["LGPE"] = "Let's Go Pikachu/Eevee",
        ["PLA"] = "Leggende Pokémon: Arceus",
        // Gen9
        ["SV"] = "Scarlatto/Violetto",
    };

    /// <summary>Nome tipo save (GetType().Name) → nome italiano. Copre Gen1–Gen4 e oltre.</summary>
    private static readonly Dictionary<string, string> GameNamesByType = new(StringComparer.OrdinalIgnoreCase)
    {
        // Gen9
        ["SAV9SV"] = "Scarlatto/Violetto",
        // Gen8
        ["SAV8BS"] = "Diamante Lucente/Perla Splendente",
        ["SAV8SWSH"] = "Spada/Scudo",
        ["SAV8LA"] = "Leggende Pokémon: Arceus",
        ["SAV7b"] = "Let's Go Pikachu/Eevee",
        // Gen7
        ["SAV7SM"] = "Sole/Luna",
        ["SAV7USUM"] = "Ultra Sole/Ultra Luna",
        // Gen6
        ["SAV6AO"] = "Rubino Omega/Zaffiro Alpha",
        ["SAV6XY"] = "Pokémon X/Y",
        // Gen5
        ["SAV5BW"] = "Nero/Bianco",
        ["SAV5B2W2"] = "Nero 2/Bianco 2",
        // Gen4
        ["SAV4"] = "Diamante/Perla/Platino",
        ["SAV4DP"] = "Diamante/Perla",
        ["SAV4Pt"] = "Platino",
        ["SAV4HGSS"] = "Oro Heart/Argento Soul",
        // Gen3
        ["SAV3"] = "Rubino/Zaffiro/Smeraldo",
        ["SAV3RS"] = "Rubino/Zaffiro",
        ["SAV3E"] = "Smeraldo",
        ["SAV3FRLG"] = "Rosso Fuoco/Verde Foglia",
        // Gen2
        ["SAV2"] = "Oro/Argento/Cristallo",
        ["SAV2Game"] = "Oro/Argento/Cristallo",
        ["SAV2GCS"] = "Oro/Argento/Cristallo",
        // Gen1
        ["SAV1"] = "Rosso/Blu/Giallo",
        ["SAV1Game"] = "Rosso/Blu/Giallo",
        ["SAV1GB"] = "Rosso/Blu/Giallo",
    };

    /// <summary>LanguageID → etichetta. Schema nei save Pokémon dalla Gen3 in poi: 1=JPN, 2=ENG, 3=FRA, 4=ITA, 5=GER, 6=salto, 7=SPA (0x01–0x07, 0x06 non usato).</summary>
    private static readonly Dictionary<int, string> LanguageLabels = new()
    {
        [0] = "—",
        [1] = "JPN",
        [2] = "USA",
        [3] = "FRA",
        [4] = "ITA",
        [5] = "GER",
        [6] = "—",
        [7] = "SPA",
        [8] = "KOR",
        [9] = "CHS",
        [10] = "CHT",
    };
}

/// <summary>Parser Pokedex per Rosso Fuoco / Verde Foglia (FRLG), differenziati per lingua (ITA, USA, JPN, FRA, GER, SPA).
/// Stessa struttura save per tutte le lingue; il parametro language serve per log e per estensioni future.</summary>
file static class FrLgPokedexParser
{
    private const int BlockSize = 0xE000;       // 57344
    private const int BlockAStart = 0;
    private const int BlockBStart = 0xE000;
    private const int SectionSize = 4096;
    private const int SectionIdOffset = 0xFF4;  // 2 bytes LE
    private const int SaveIndexOffset = 0xFFC;  // 4 bytes LE
    private const int OwnedOffset = 0x28;
    private const int SeenOffset = 0x5C;
    private const int PokedexBytes = 49;        // 49 * 8 = 392 bit, specie 0..391
    private const int MaxSpeciesGen3 = 386;

    private static readonly (int Evolved, int Base)[] EvolvesFrom = new[]
    {
        (2, 1), (3, 2), (5, 4), (6, 5), (8, 7), (9, 8), (11, 10), (12, 11), (14, 13), (15, 14),
        (17, 16), (18, 17), (20, 19), (21, 20), (22, 21), (24, 23), (25, 24), (26, 25), (28, 27), (29, 28),
        (30, 29), (31, 30), (33, 32), (34, 33), (36, 35), (38, 37), (40, 39), (42, 41), (44, 43), (45, 44),
        (47, 46), (49, 48), (51, 50), (53, 52), (55, 54), (57, 56), (59, 58), (61, 60), (62, 61), (64, 63),
        (65, 64), (67, 66), (68, 67), (70, 69), (71, 70), (73, 72), (75, 74), (76, 75), (78, 77), (80, 79),
        (82, 81), (85, 84), (87, 86), (89, 88), (91, 90), (94, 93), (97, 96), (99, 98), (101, 100),
        (103, 102), (105, 104), (107, 106), (110, 109), (112, 111), (115, 114), (117, 116), (119, 118),
        (121, 120), (124, 123), (126, 125), (128, 127), (131, 130), (134, 133), (136, 135), (139, 138),
        (141, 140), (143, 142), (146, 145), (149, 148),
    };

    /// <summary>Parse con lingua esplicita (ITA, USA, JPN, FRA, GER, SPA, o "—" per generico). Stessa logica byte per tutte le lingue.</summary>
    public static List<PokedexHelper.PokedexEntry> Parse(byte[] data, string pathForLog, string language = "—")
    {
        if (data.Length < BlockBStart + BlockSize)
        {
            // File troppo piccolo per due blocchi, uso solo blocco A
        }

        (int section0Index, uint saveIndex) FindSection0(int blockStart)
        {
            for (int i = 0; i < 14; i++)
            {
                int sectionStart = blockStart + i * SectionSize;
                if (sectionStart + SectionSize > data.Length) return (-1, 0);
                int id = data[sectionStart + SectionIdOffset] | (data[sectionStart + SectionIdOffset + 1] << 8);
                if (id == 0)
                {
                    uint saveIdx = (uint)(data[sectionStart + SaveIndexOffset] | (data[sectionStart + SaveIndexOffset + 1] << 8) |
                        (data[sectionStart + SaveIndexOffset + 2] << 16) | (data[sectionStart + SaveIndexOffset + 3] << 24));
                    return (sectionStart, saveIdx);
                }
            }
            return (-1, 0);
        }

        var (secA, idxA) = FindSection0(BlockAStart);
        var (secB, idxB) = FindSection0(BlockBStart);
        int section0Start;
        if (secA < 0 && secB < 0)
        {
            return new List<PokedexHelper.PokedexEntry>();
        }
        if (secB < 0 || (secA >= 0 && idxA >= idxB))
        {
            section0Start = secA;
        }
        else
        {
            section0Start = secB;
        }

        byte[] owned = new byte[PokedexBytes];
        byte[] seen = new byte[PokedexBytes];
        for (int i = 0; i < PokedexBytes; i++)
        {
            owned[i] = data[section0Start + OwnedOffset + i];
            seen[i] = data[section0Start + SeenOffset + i];
        }
        static bool GetBit(byte[] buf, int bitIndex)
        {
            if (bitIndex < 0 || (bitIndex >> 3) >= buf.Length) return false;
            return (buf[bitIndex >> 3] & (1 << (bitIndex & 7))) != 0;
        }

        var statusBySpecies = new Dictionary<int, string>(MaxSpeciesGen3);
        for (int sid = 1; sid <= MaxSpeciesGen3; sid++)
        {
            int idx = sid - 1;
            bool caught = GetBit(owned, idx);
            bool s = GetBit(seen, idx);
            string status = caught ? "caught" : (s ? "seen" : "unseen");
            statusBySpecies[sid] = status;
        }

        foreach (var (evolved, base_) in EvolvesFrom)
        {
            if (evolved > MaxSpeciesGen3) continue;
            if (statusBySpecies[evolved] == "caught")
            {
                statusBySpecies[base_] = "caught";
            }
        }

        var list = statusBySpecies.OrderBy(kv => kv.Key).Select(kv => new PokedexHelper.PokedexEntry(kv.Key, kv.Value)).ToList();
        return list;
    }
}

/// <summary>Parser Pokedex per Smeraldo (Emerald), differenziato per lingua (ITA, USA, JPN, FRA, GER, SPA).
/// Stessa struttura save Section 0 di FRLG: blocco A/B, owned 0x28, seen 0x5C, 49 byte, 386 specie Gen3. Fonte: Bulbapedia Save data structure (Generation III).</summary>
file static class EmeraldPokedexParser
{
    private const int BlockSize = 0xE000;
    private const int BlockAStart = 0;
    private const int BlockBStart = 0xE000;
    private const int SectionSize = 4096;
    private const int SectionIdOffset = 0xFF4;
    private const int SaveIndexOffset = 0xFFC;
    private const int OwnedOffset = 0x28;
    private const int SeenOffset = 0x5C;
    private const int PokedexBytes = 49;
    private const int MaxSpeciesGen3 = 386;

    private static readonly (int Evolved, int Base)[] EvolvesFrom = new[]
    {
        (2, 1), (3, 2), (5, 4), (6, 5), (8, 7), (9, 8), (11, 10), (12, 11), (14, 13), (15, 14),
        (17, 16), (18, 17), (20, 19), (21, 20), (22, 21), (24, 23), (25, 24), (26, 25), (28, 27), (29, 28),
        (30, 29), (31, 30), (33, 32), (34, 33), (36, 35), (38, 37), (40, 39), (42, 41), (44, 43), (45, 44),
        (47, 46), (49, 48), (51, 50), (53, 52), (55, 54), (57, 56), (59, 58), (61, 60), (62, 61), (64, 63),
        (65, 64), (67, 66), (68, 67), (70, 69), (71, 70), (73, 72), (75, 74), (76, 75), (78, 77), (80, 79),
        (82, 81), (85, 84), (87, 86), (89, 88), (91, 90), (94, 93), (97, 96), (99, 98), (101, 100),
        (103, 102), (105, 104), (107, 106), (110, 109), (112, 111), (115, 114), (117, 116), (119, 118),
        (121, 120), (124, 123), (126, 125), (128, 127), (131, 130), (134, 133), (136, 135), (139, 138),
        (141, 140), (143, 142), (146, 145), (149, 148),
    };

    /// <summary>Parse con lingua esplicita (ITA, USA, JPN, FRA, GER, SPA, o "—" per generico). Stessa logica byte per tutte le lingue.</summary>
    public static List<PokedexHelper.PokedexEntry> Parse(byte[] data, string pathForLog, string language = "—")
    {
        (int section0Index, uint saveIndex) FindSection0(int blockStart)
        {
            for (int i = 0; i < 14; i++)
            {
                int sectionStart = blockStart + i * SectionSize;
                if (sectionStart + SectionSize > data.Length) return (-1, 0);
                int id = data[sectionStart + SectionIdOffset] | (data[sectionStart + SectionIdOffset + 1] << 8);
                if (id == 0)
                {
                    uint saveIdx = (uint)(data[sectionStart + SaveIndexOffset] | (data[sectionStart + SaveIndexOffset + 1] << 8) |
                        (data[sectionStart + SaveIndexOffset + 2] << 16) | (data[sectionStart + SaveIndexOffset + 3] << 24));
                    return (sectionStart, saveIdx);
                }
            }
            return (-1, 0);
        }

        var (secA, idxA) = FindSection0(BlockAStart);
        var (secB, idxB) = FindSection0(BlockBStart);
        int section0Start;
        if (secA < 0 && secB < 0)
            return new List<PokedexHelper.PokedexEntry>();
        if (secB < 0 || (secA >= 0 && idxA >= idxB))
            section0Start = secA;
        else
            section0Start = secB;

        byte[] owned = new byte[PokedexBytes];
        byte[] seen = new byte[PokedexBytes];
        for (int i = 0; i < PokedexBytes; i++)
        {
            owned[i] = data[section0Start + OwnedOffset + i];
            seen[i] = data[section0Start + SeenOffset + i];
        }
        static bool GetBit(byte[] buf, int bitIndex)
        {
            if (bitIndex < 0 || (bitIndex >> 3) >= buf.Length) return false;
            return (buf[bitIndex >> 3] & (1 << (bitIndex & 7))) != 0;
        }

        var statusBySpecies = new Dictionary<int, string>(MaxSpeciesGen3);
        for (int sid = 1; sid <= MaxSpeciesGen3; sid++)
        {
            int idx = sid - 1;
            bool caught = GetBit(owned, idx);
            bool s = GetBit(seen, idx);
            string status = caught ? "caught" : (s ? "seen" : "unseen");
            statusBySpecies[sid] = status;
        }

        foreach (var (evolved, base_) in EvolvesFrom)
        {
            if (evolved > MaxSpeciesGen3) continue;
            if (statusBySpecies[evolved] == "caught")
                statusBySpecies[base_] = "caught";
        }

        return statusBySpecies.OrderBy(kv => kv.Key).Select(kv => new PokedexHelper.PokedexEntry(kv.Key, kv.Value)).ToList();
    }
}

/// <summary>Estrae stato Pokedex (unseen/seen/caught) dal save usando SOLO l'API tipata PKHeX:
/// SaveFile.GetSeen e SaveFile.GetCaught. Alcuni giochi/save usano indice dex 0-based (0=Bulbasaur):
/// se true, passiamo (speciesId - 1) all'API così species_id 1 in output = Bulbasaur. Vedi issue pre-evoluzioni.</summary>
file static class PokedexHelper
{
    /// <summary>Se true, GetSeen/GetCaught vengono chiamati con (sid - 1) così National Dex #1 = indice 0 (Bulbasaur).</summary>
    private const bool UseDexIndexZeroBased = true;

    /// <summary>Coppie (evoluzione, base) Gen1–4: se l'evoluzione è caught, la base va almeno caught. Usato da HGSS e da pokedex generico.</summary>
    private static readonly (int Evolved, int Base)[] EvolvesFrom = new[]
    {
        // Gen1
        (2, 1), (3, 2), (5, 4), (6, 5), (8, 7), (9, 8), (11, 10), (12, 11), (14, 13), (15, 14),
        (17, 16), (18, 17), (20, 19), (21, 20), (22, 21), (24, 23), (25, 24), (26, 25), (28, 27), (29, 28),
        (30, 29), (31, 30), (33, 32), (34, 33), (36, 35), (38, 37), (40, 39), (42, 41), (44, 43), (45, 44),
        (47, 46), (49, 48), (51, 50), (53, 52), (55, 54), (57, 56), (59, 58), (61, 60), (62, 61), (64, 63),
        (65, 64), (67, 66), (68, 67), (70, 69), (71, 70), (73, 72), (75, 74), (76, 75), (78, 77), (80, 79),
        (82, 81), (85, 84), (87, 86), (89, 88), (91, 90), (94, 93), (97, 96), (99, 98), (101, 100),
        (103, 102), (105, 104), (107, 106), (110, 109), (112, 111), (115, 114), (117, 116), (119, 118),
        (121, 120), (124, 123), (126, 125), (128, 127), (131, 130), (134, 133), (136, 135), (139, 138),
        (141, 140), (143, 142), (146, 145), (149, 148),
        // Gen2
        (154, 153), (153, 152), (157, 156), (156, 155), (160, 159), (159, 158), (162, 161), (164, 163),
        (166, 165), (168, 167), (171, 170), (176, 175), (178, 177), (181, 180), (180, 179), (184, 183),
        (186, 185), (189, 188), (188, 187), (192, 191), (195, 194), (197, 196), (199, 198), (203, 202),
        (205, 204), (208, 207), (210, 209), (212, 211), (214, 213), (217, 216), (219, 218), (221, 220),
        (224, 223), (226, 225), (227, 225), (230, 229), (229, 228), (233, 232), (237, 236), (242, 241),
        (245, 244), (248, 247), (247, 246), (250, 249), (251, 250),
        // Gen3
        (254, 253), (253, 252), (257, 256), (256, 255), (260, 259), (259, 258), (262, 261), (264, 263),
        (267, 266), (266, 265), (269, 268), (272, 271), (271, 270), (277, 276), (279, 278), (282, 281),
        (281, 280), (286, 285), (289, 288), (292, 291), (295, 294), (297, 296), (303, 302), (306, 305),
        (305, 304), (308, 307), (310, 309), (312, 311), (315, 314), (319, 318), (321, 320), (323, 322),
        (326, 325), (328, 327), (330, 329), (329, 328), (332, 331), (334, 333), (336, 335), (338, 337),
        (340, 339), (342, 341), (344, 343), (346, 345), (348, 347), (350, 349), (352, 351), (354, 353),
        (356, 355), (358, 357), (362, 361), (361, 360), (365, 364), (364, 363), (368, 367), (373, 372),
        (372, 371), (376, 375), (375, 374), (378, 377), (379, 377), (381, 380), (380, 379), (384, 383),
        (386, 385),
        // Gen4 (evoluzioni dirette; cross-gen dove base è Gen1-3)
        (389, 388), (388, 387), (392, 391), (391, 390), (395, 394), (394, 393), (398, 397), (397, 396),
        (400, 399), (402, 401), (405, 404), (404, 403), (407, 406), (409, 408), (411, 410), (414, 413),
        (413, 412), (416, 415), (415, 414), (419, 418), (418, 417), (421, 420), (423, 422), (424, 422),
        (426, 425), (428, 427), (430, 429), (432, 431), (435, 434), (437, 436), (441, 440), (442, 441),
        (445, 444), (444, 443), (448, 447), (450, 449), (452, 451), (454, 453), (457, 456), (456, 455),
        (460, 459), (461, 215), (462, 82), (463, 108), (464, 112), (465, 114), (466, 125), (467, 126),
        (468, 176), (470, 469), (471, 469), (472, 470), (473, 361), (475, 281), (476, 299), (477, 356),
        (478, 361), (226, 458),
    };

    static int? PrevEvolution(int speciesId)
    {
        foreach (var (evolved, base_) in EvolvesFrom)
            if (evolved == speciesId) return base_;
        return null;
    }

    /// <summary>Propaga "caught" alle pre-evoluzioni: se una specie è catturata, le forme base vanno almeno "caught".</summary>
    private static void PropagateCaughtToPrevolutions(Dictionary<int, string> statusBySpecies)
    {
        var caughtIds = statusBySpecies.Where(kv => kv.Value == "caught").Select(kv => kv.Key).ToList();
        foreach (var sid in caughtIds)
        {
            var current = sid;
            int? prev;
            while ((prev = PrevEvolution(current)) != null)
            {
                statusBySpecies[prev.Value] = "caught";
                current = prev.Value;
            }
        }
    }

    /// <summary>Numero massimo di specie per generazione (National Dex fino a quella Gen). Fonte: PKHeX / dati ufficiali.</summary>
    private static int GetMaxSpeciesIdForGeneration(int generation)
    {
        return generation switch
        {
            1 => 151,
            2 => 251,
            3 => 386,
            4 => 493,
            5 => 649,
            6 => 721,
            7 => 809,
            8 => 905,
            9 => 1025,
            _ => 493,
        };
    }

    /// <summary>Gen4 (HGSS, D/P/Pt): usa SAV4.Pokedex.GetSeen(i) e GetCaught(i) con i da 1 a 493. Accesso via reflection.</summary>
    public static List<PokedexEntry> GetAllSpeciesStatusSAV4Pokedex(SaveFile sav)
    {
        object? pokedexObj = null;
        for (var t = sav.GetType(); t != null; t = t.BaseType)
        {
            var prop = t.GetProperty("Pokedex", BindingFlags.Public | BindingFlags.Instance);
            if (prop != null)
            {
                try { pokedexObj = prop.GetValue(sav); break; } catch { }
            }
        }
        if (pokedexObj == null)
            return new List<PokedexEntry>();

        var pokedexType = pokedexObj.GetType();
        var getSeen = pokedexType.GetMethods(BindingFlags.Public | BindingFlags.Instance)
            .FirstOrDefault(m => m.Name == "GetSeen" && m.GetParameters().Length == 1 && (m.GetParameters()[0].ParameterType == typeof(int) || m.GetParameters()[0].ParameterType == typeof(ushort)));
        var getCaught = pokedexType.GetMethods(BindingFlags.Public | BindingFlags.Instance)
            .FirstOrDefault(m => m.Name == "GetCaught" && m.GetParameters().Length == 1 && (m.GetParameters()[0].ParameterType == typeof(int) || m.GetParameters()[0].ParameterType == typeof(ushort)));
        if (getSeen == null || getCaught == null)
            return new List<PokedexEntry>();

        // HGSS: ciclo da 1 a 493
        const int HGSS_MaxSpecies = 493;
        var statusBySpecies = new Dictionary<int, string>(HGSS_MaxSpecies);
        var paramType = getSeen.GetParameters()[0].ParameterType;

        for (int i = 1; i <= HGSS_MaxSpecies; i++)
        {
            object indexArg = paramType == typeof(ushort) ? (ushort)i : i;
            bool seen = false, caught = false;
            try
            {
                caught = (bool)getCaught.Invoke(pokedexObj, new[] { indexArg })!;
                seen = (bool)getSeen.Invoke(pokedexObj, new[] { indexArg })!;
            }
            catch { }
            string status = caught ? "caught" : (seen ? "seen" : "unseen");
            statusBySpecies[i] = status;
        }

        PropagateCaughtToPrevolutions(statusBySpecies);
        return statusBySpecies.OrderBy(kv => kv.Key).Select(kv => new PokedexEntry(kv.Key, kv.Value)).ToList();
    }

    /// <summary>Restituisce tutte le specie valide per la generazione del save con status (unseen/seen/caught).
    /// Usa GetSeen/GetCaught; useZeroBased: true = indice 0 per Bulbasaur (Gen3), false = indice 1 per Bulbasaur (Gen4 HGSS).</summary>
    public static List<PokedexEntry> GetAllSpeciesStatus(SaveFile sav, bool? useZeroBasedOverride = null)
    {
        bool useZeroBased = useZeroBasedOverride ?? UseDexIndexZeroBased;
        int gen = sav.Generation;
        int maxSpeciesId = GetMaxSpeciesIdForGeneration(gen);
        var statusBySpecies = new Dictionary<int, string>(maxSpeciesId);

        for (int sid = 1; sid <= maxSpeciesId; sid++)
        {
            ushort dexIndex = useZeroBased ? (ushort)(sid - 1) : (ushort)sid;
            bool caught;
            bool seen;
            try
            {
                caught = sav.GetCaught(dexIndex);
                seen = sav.GetSeen(dexIndex);
            }
            catch
            {
                caught = false;
                seen = false;
            }
            string status = caught ? "caught" : (seen ? "seen" : "unseen");
            statusBySpecies[sid] = status;
        }

        PropagateCaughtToPrevolutions(statusBySpecies);

        var list = statusBySpecies.OrderBy(kv => kv.Key).Select(kv => new PokedexEntry(kv.Key, kv.Value)).ToList();
        return list;
    }

    public record PokedexEntry(
        [property: System.Text.Json.Serialization.JsonPropertyName("species_id")] int SpeciesId,
        [property: System.Text.Json.Serialization.JsonPropertyName("status")] string Status
    );
}

static class TrainerHelper
{
    public class TrainerData
    {
        public uint? money { get; set; }
        public int? playedHours { get; set; }
        public int? playedMinutes { get; set; }
    }

    public static TrainerData ExtractTrainerData(SaveFile sav)
    {
        var data = new TrainerData();
        var savType = sav.GetType();
        Console.Error.WriteLine($"[parser] trainer: SaveFile type = {savType.FullName}");

        // Money / Currency — cerca in gerarchia classi (proprietà può essere in SaveFile base)
        try
        {
            PropertyInfo? moneyProp = null;
            // Cerca "Money" risalendo la gerarchia
            for (var t = savType; t != null && moneyProp == null; t = t.BaseType)
            {
                moneyProp = t.GetProperty("Money", BindingFlags.Public | BindingFlags.Instance | BindingFlags.DeclaredOnly);
            }
            // Fallback: cerca senza DeclaredOnly (include ereditati)
            moneyProp ??= savType.GetProperty("Money", BindingFlags.Public | BindingFlags.Instance);

            if (moneyProp != null)
            {
                Console.Error.WriteLine($"[parser] trainer: Found Money prop, type = {moneyProp.PropertyType.Name}");
                var moneyVal = moneyProp.GetValue(sav);
                if (moneyVal != null)
                {
                    data.money = Convert.ToUInt32(moneyVal);
                }
            }
            else
            {
                // Gen9 usa "Currency" invece di "Money"
                var currencyProp = savType.GetProperty("Currency", BindingFlags.Public | BindingFlags.Instance);
                if (currencyProp != null)
                {
                    Console.Error.WriteLine($"[parser] trainer: Found Currency prop, type = {currencyProp.PropertyType.Name}");
                    var currencyVal = currencyProp.GetValue(sav);
                    if (currencyVal != null)
                    {
                        data.money = Convert.ToUInt32(currencyVal);
                    }
                }
                else
                {
                    Console.Error.WriteLine("[parser] trainer: No Money/Currency property found");
                }
            }
        }
        catch (Exception ex)
        {
            Console.Error.WriteLine($"[parser] trainer: Money extraction failed: {ex.Message}");
        }

        // PlayedHours / PlayedMinutes — cerca in gerarchia
        try
        {
            PropertyInfo? hoursProp = savType.GetProperty("PlayedHours", BindingFlags.Public | BindingFlags.Instance);
            PropertyInfo? minutesProp = savType.GetProperty("PlayedMinutes", BindingFlags.Public | BindingFlags.Instance);

            if (hoursProp != null)
            {
                Console.Error.WriteLine($"[parser] trainer: Found PlayedHours prop, type = {hoursProp.PropertyType.Name}");
                var hoursVal = hoursProp.GetValue(sav);
                if (hoursVal != null)
                {
                    data.playedHours = Convert.ToInt32(hoursVal);
                }
            }
            else
            {
                Console.Error.WriteLine("[parser] trainer: No PlayedHours property found");
            }

            if (minutesProp != null)
            {
                Console.Error.WriteLine($"[parser] trainer: Found PlayedMinutes prop, type = {minutesProp.PropertyType.Name}");
                var minutesVal = minutesProp.GetValue(sav);
                if (minutesVal != null)
                {
                    data.playedMinutes = Convert.ToInt32(minutesVal);
                }
            }
            else
            {
                Console.Error.WriteLine("[parser] trainer: No PlayedMinutes property found");
            }
        }
        catch (Exception ex)
        {
            Console.Error.WriteLine($"[parser] trainer: PlayedHours/Minutes extraction failed: {ex.Message}");
        }

        return data;
    }
}
