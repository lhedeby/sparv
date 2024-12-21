public record class ResponseMessage<T>(string Jsonrpc, int Id, T Result);
