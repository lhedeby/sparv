public record class DidChangeTextDocumentParams(VersionedTextDocumentIdentifier TextDocument, List<TextDocumentContentChangeEvent> ContentChanges) : IClientRequest
{
    public object? Handle(State state, StreamWriter writer)
    {
        lock (state)
        {
            var doc = state.Documents[TextDocument.Uri];
            doc.Text = ContentChanges.First().Text;
            doc.Version = TextDocument.Version;
            state.HasChanged = true;
        }
        return null;
    }
}
