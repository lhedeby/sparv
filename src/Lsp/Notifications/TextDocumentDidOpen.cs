public record class DidOpenTextDocumentParams(TextDocumentItem TextDocument) : IClientRequest
{
    public object? Handle(State state, StreamWriter writer)
    {
        lock (state)
        {
            state.Documents.Add(TextDocument.Uri, TextDocument);
            state.HasChanged = true;
        }
        return null;
    }
}


