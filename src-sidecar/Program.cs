using System.Reflection;
using System.Text;
using System.Text.Json;
using PKHeX.Core;

Console.OutputEncoding = Encoding.UTF8;

if (args.Length < 2)
{
    WriteError("Usage: detect <path>");
    Environment.Exit(1);
    return;
}

string command = args[0];
string path = args[1];

if (command != "detect")
{
    WriteError("Unknown command. Use: detect <path>");
    Environment.Exit(1);
    return;
}

if (!File.Exists(path))
{
    WriteError("File not found: " + path);
    Environment.Exit(1);
    return;
}

try
{
    byte[] data = await File.ReadAllBytesAsync(path);
    SaveFile? sav = SaveUtil.GetVariantSAV(data);

    if (sav == null)
    {
        WriteError("Unrecognized save format");
        Environment.Exit(1);
        return;
    }

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
}
catch (Exception ex)
{
    WriteError(ex.Message);
    Environment.Exit(1);
}

static void WriteError(string message)
{
    var err = JsonSerializer.Serialize(new { error = message });
    Console.Error.WriteLine(err);
    Console.WriteLine(err);
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
