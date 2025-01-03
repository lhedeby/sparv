
public record class HoverParams(TextDocumentIdentifier TextDocument, Position Position) : IClientRequest
{
    public object? Handle(State state, StreamWriter writer)
    {
        foreach (var token in state.Tokens)
        {
            if (Position.Line == token.Line && Position.Character >= token.Start && Position.Character < token.End)
            {
                var content = token.Value switch
                {
                    // Native functions
                    "print" => Documentation.Print().ToCompletionItem().Documentation,
                    "len" => Documentation.Len().ToCompletionItem().Documentation,
                    "typeof" => Documentation.Len().ToCompletionItem().Documentation,
                    "read_file" => Documentation.Len().ToCompletionItem().Documentation,
                    "split" => Documentation.Len().ToCompletionItem().Documentation,
                    "parse" => Documentation.Len().ToCompletionItem().Documentation,
                    "read_input" => Documentation.Len().ToCompletionItem().Documentation,
                    "abs" => Documentation.Len().ToCompletionItem().Documentation,
                    _ => TokenHover(token)
                };
                if (content is not null)
                {
                    return new Hover(
                        content,
                        new(new(token.Line, token.Start), new(token.Line, token.End)));
                }
            }
        }
        return null;
    }

    private MarkupContent? TokenHover(Token token)
    {
        var text = token.Kind switch
        {
            TokenKind.Identifier => $"Variable: '{token.Value}'",
            _ => null
        };
        if (text is not null)
            return new MarkupContent("plaintext", text);
        return null;
    }
}
