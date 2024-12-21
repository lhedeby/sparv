using System.Text.Json;
using System.Text.Json.Serialization;

public record class RequestMessage(
        string Jsonrpc,
        int Id,
        string Method,
        [property: JsonPropertyName("params")] JsonElement Body);
