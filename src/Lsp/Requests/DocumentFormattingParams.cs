public record class DocumentFormattingParams(TextDocumentIdentifier TextDocument, FormattingOptions options) : IClientRequest
{
    public object? Handle(State state, StreamWriter writer)
    {
        var doc = state.Documents[TextDocument.Uri];
        if (doc is null)
            return null;

        return Formatter.TextEdits(doc.Text, writer);
    }



}
