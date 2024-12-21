using System.Text.Json.Serialization;

public record class NotificationMessage<T>(
    string Jsonrpc,
    string Method,
    [property: JsonPropertyName("params")] T Body
);
