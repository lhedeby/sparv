
public record class HoverParams(TextDocumentIdentifier TextDocument, Position Position) : IClientRequest
{
    public object? Handle(State state, StreamWriter writer)
    {
        throw new NotImplementedException();
    }
}
