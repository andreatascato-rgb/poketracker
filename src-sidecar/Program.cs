using System.Reflection;
using System.Text;
using System.Text.Json;
using System.Linq;
using PKHeX.Core;

Console.OutputEncoding = Encoding.UTF8;

if (args.Length < 1)
{
    WriteError("Usage: <command> [path]. Commands: detect <path>, pokedex <path>, pokedex_auto <path>, probe.");
    Environment.Exit(1);
    return;
}

string command = args[0].Trim().ToLowerInvariant();
string? path = args.Length > 1 ? args[1] : null;

if (command == "probe")
{
    PkHexPokedexProbe.Run();
    Environment.Exit(0);
    return;
}

if (command != "detect" && command != "pokedex" && command != "pokedex_auto")
{
    WriteError("Unknown command. Use: detect <path>, pokedex <path>, or pokedex_auto <path>");
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
    byte[] data = await File.ReadAllBytesAsync(path);
    SaveFile? sav = SaveUtil.GetVariantSAV(data);

    if (sav == null)
    {
        WriteError("Unrecognized save format");
        Environment.Exit(1);
        return;
    }

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

    if (command == "pokedex")
    {
        var entries = PokedexHelper.GetAllSpeciesStatus(sav);
        int seen = 0, caught = 0;
        foreach (var e in entries)
        {
            if (e.Status == "seen") seen++;
            else if (e.Status == "caught") caught++;
        }
        Console.Error.WriteLine($"[parser] Pokedex extract: {entries.Count} species, seen={seen}, caught={caught}");
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
        if (typeName == "SAV3FRLG" || typeName == "SAV3FR" || typeName == "SAV3LG")
        {
            Console.Error.WriteLine($"[parser] pokedex_auto: rilevato {typeName}, uso parser FRLG (Rosso Fuoco/Verde Foglia).");
            entries = FrLgPokedexParser.Parse(data, path ?? "");
        }
        else
        {
            Console.Error.WriteLine($"[parser] pokedex_auto: save tipo {typeName}, nessun parser dedicato. Restituisco vuoto.");
            entries = new List<PokedexHelper.PokedexEntry>();
        }
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

/// <summary>Parser Pokedex per Rosso Fuoco / Verde Foglia (FRLG). Legge i byte raw del .sav secondo Bulbapedia:
/// Section 0: owned (caught) a 0x28 (49 byte), seen a 0x5C (49 byte). Blocchi A/B, sezioni ruotate, save index.</summary>
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

    public static List<PokedexHelper.PokedexEntry> Parse(byte[] data, string pathForLog)
    {
        Console.Error.WriteLine($"[parser] FRLG: file size={data.Length} bytes, path={pathForLog}");
        if (data.Length < BlockBStart + BlockSize)
        {
            Console.Error.WriteLine($"[parser] FRLG: file troppo piccolo per due blocchi, uso solo blocco A.");
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
        string blockUsed;
        if (secA < 0 && secB < 0)
        {
            Console.Error.WriteLine("[parser] FRLG: Section 0 non trovata in nessun blocco. Restituisco vuoto.");
            return new List<PokedexHelper.PokedexEntry>();
        }
        if (secB < 0 || (secA >= 0 && idxA >= idxB))
        {
            section0Start = secA;
            blockUsed = "A";
            Console.Error.WriteLine($"[parser] FRLG: blocco usato={blockUsed}, save_index={idxA}, section0_offset=0x{secA:X}");
        }
        else
        {
            section0Start = secB;
            blockUsed = "B";
            Console.Error.WriteLine($"[parser] FRLG: blocco usato={blockUsed}, save_index={idxB}, section0_offset=0x{secB:X}");
        }

        byte[] owned = new byte[PokedexBytes];
        byte[] seen = new byte[PokedexBytes];
        for (int i = 0; i < PokedexBytes; i++)
        {
            owned[i] = data[section0Start + OwnedOffset + i];
            seen[i] = data[section0Start + SeenOffset + i];
        }
        string ownedHex = string.Join(" ", owned.Take(16).Select(b => b.ToString("X2")));
        string seenHex = string.Join(" ", seen.Take(16).Select(b => b.ToString("X2")));
        Console.Error.WriteLine($"[parser] FRLG: blocco {blockUsed} — owned (primi 16 byte) = {ownedHex}");
        Console.Error.WriteLine($"[parser] FRLG: blocco {blockUsed} — seen  (primi 16 byte) = {seenHex}");

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
        int seenCount = list.Count(e => e.Status == "seen");
        int caughtCount = list.Count(e => e.Status == "caught");
        Console.Error.WriteLine($"[parser] FRLG: estratte {list.Count} specie, seen={seenCount}, caught={caughtCount}");
        var sample = list.Where(e => e.Status != "unseen").Take(10).Select(e => $"#{e.SpeciesId}={e.Status}");
        Console.Error.WriteLine($"[parser] FRLG: campione (primi 10 non-unseen) = {string.Join(", ", sample)}");
        return list;
    }
}

/// <summary>Estrae stato Pokedex (unseen/seen/caught) dal save usando SOLO l'API tipata PKHeX:
/// SaveFile.GetSeen e SaveFile.GetCaught. Alcuni giochi/save usano indice dex 0-based (0=Bulbasaur):
/// se true, passiamo (speciesId - 1) all'API così species_id 1 in output = Bulbasaur. Vedi issue pre-evoluzioni.</summary>
file static class PokedexHelper
{
    /// <summary>Se true, GetSeen/GetCaught vengono chiamati con (sid - 1) così National Dex #1 = indice 0 (Bulbasaur).</summary>
    private const bool UseDexIndexZeroBased = true;

    /// <summary>Coppie (evoluzione, base) Gen1: se l'evoluzione è caught, la base va almeno caught. Allineato a Rust EVOLVES_FROM.</summary>
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

    /// <summary>Restituisce tutte le specie valide per la generazione del save con status (unseen/seen/caught).
    /// Usa GetSeen/GetCaught; opzione 0-based per dex; propaga caught alle pre-evoluzioni.</summary>
    public static List<PokedexEntry> GetAllSpeciesStatus(SaveFile sav)
    {
        int gen = sav.Generation;
        int maxSpeciesId = GetMaxSpeciesIdForGeneration(gen);
        var statusBySpecies = new Dictionary<int, string>(maxSpeciesId);

        for (int sid = 1; sid <= maxSpeciesId; sid++)
        {
            ushort dexIndex = UseDexIndexZeroBased ? (ushort)(sid - 1) : (ushort)sid;
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
        int seenCount = list.Count(e => e.Status == "seen");
        int caughtCount = list.Count(e => e.Status == "caught");
        Console.Error.WriteLine($"[parser] Pokedex: Gen{gen}, max={maxSpeciesId}, entries={list.Count}, seen={seenCount}, caught={caughtCount}, dexIndex0Based={UseDexIndexZeroBased}");
        return list;
    }

    public record PokedexEntry(
        [property: System.Text.Json.Serialization.JsonPropertyName("species_id")] int SpeciesId,
        [property: System.Text.Json.Serialization.JsonPropertyName("status")] string Status
    );
}
