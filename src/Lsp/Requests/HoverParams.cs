public record class HoverParams(TextDocumentIdentifier TextDocument, Position Position) : IClientRequest
{
    public object? Handle(State state, StreamWriter writer)
    {
        foreach (var token in state.Tokens)
        {
            if (Position.Line == token.Line && Position.Character >= token.Start && Position.Character < token.End)
            {
                    return new Hover(
                        Text(state, token.Value),
                        new(new(token.Line, token.Start), new(token.Line, token.End)));
            }
        }
        return null;
    }


    private MarkupContent Text(State state, string identifier)
    {
        if (Documentation.IsNative(identifier))
            return Documentation.CompletionItem(identifier).Documentation!;
        if (state.Functions.ContainsKey(identifier))
            return new MarkupContent("plaintext", $"{identifier}({string.Join(", ",state.Functions[identifier])})");

        return new MarkupContent("plaintext", $"Variable: '{identifier}'");
    }
}
