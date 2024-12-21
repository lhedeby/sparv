using System.Text.Json;

public static class Json
{
    private static readonly JsonSerializerOptions _options = new JsonSerializerOptions
    {
        PropertyNamingPolicy = JsonNamingPolicy.CamelCase
    };

    public static JsonSerializerOptions Options { get => _options; }

    public static string Serialize<T>(T obj)
    {
        return JsonSerializer.Serialize(obj, _options);
    }
    public static T? Deserialize<T>(string json)
    {
        return JsonSerializer.Deserialize<T>(json, _options);
    }
    public static T? Deserialize<T>(char[] json)
    {
        return JsonSerializer.Deserialize<T>(json, _options);
    }
}
