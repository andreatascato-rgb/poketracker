// Temporary: probe PKHeX SaveFile for Zukan/Pokedex API. Run with: dotnet run -- probe
using System.Reflection;
using PKHeX.Core;

public static class PkHexPokedexProbe
{
    public static void Run()
    {
        var t = typeof(SaveFile);
        Console.WriteLine("=== SaveFile type: " + t.FullName);
        foreach (var iface in t.GetInterfaces())
        {
            if (iface.Name.IndexOf("Zukan", StringComparison.OrdinalIgnoreCase) >= 0 ||
                iface.Name.IndexOf("Pokedex", StringComparison.OrdinalIgnoreCase) >= 0)
                Console.WriteLine("  Interface: " + iface.Name);
        }
        foreach (var p in t.GetProperties(BindingFlags.Public | BindingFlags.Instance))
        {
            if (p.Name.IndexOf("Zukan", StringComparison.OrdinalIgnoreCase) >= 0 ||
                p.Name.IndexOf("Pokedex", StringComparison.OrdinalIgnoreCase) >= 0 ||
                p.Name.IndexOf("Dex", StringComparison.OrdinalIgnoreCase) >= 0)
                Console.WriteLine("  Property: " + p.Name + " -> " + p.PropertyType.Name);
        }
        foreach (var m in t.GetMethods(BindingFlags.Public | BindingFlags.Instance))
        {
            if (m.Name.IndexOf("Seen", StringComparison.OrdinalIgnoreCase) >= 0 ||
                m.Name.IndexOf("Caught", StringComparison.OrdinalIgnoreCase) >= 0 ||
                m.Name.IndexOf("GetDex", StringComparison.OrdinalIgnoreCase) >= 0)
                Console.WriteLine("  Method: " + m.Name + "(" + string.Join(", ", m.GetParameters().Select(x => x.ParameterType.Name + " " + x.Name)) + ")");
        }
    }
}
