public record class CompletionParams(
    CompletionContext Context,
    TextDocumentIdentifier TextDocumentIdentifier,
    Position Position
) : IClientRequest
{
    public object? Handle(State state, StreamWriter writer)
    {
        writer.Log($"COMPLETION PARAMS: {Context}");
        var list = new List<CompletionItem>();
        foreach (var suggestion in state.Suggestions)
        {
            list.Add(
                new CompletionItem()
                {
                    Label = suggestion,
                    Kind = CompletionItemKind.Variable,
                    LabelDetails = new(suggestion, null),
                    Detail = $"Variable: '{suggestion}'",
                    Documentation = null,
                    InsertText = suggestion,
                    InsertTextFormat = InsertTextFormat.PlainText
                }

            );
        }
        foreach (var (key, value) in state.Functions)
        {
            list.Add(
                new CompletionItem()
                {
                    Label = key,
                    Kind = CompletionItemKind.Function,
                    LabelDetails = new($"{key}({string.Join(", ", value)})", null),
                    Detail = $"fun {key}({string.Join(", ", value)})",
                    Documentation = null,
                    InsertText = $"{key}(",
                    InsertTextFormat = InsertTextFormat.PlainText
                }

            );
        }

        list.AddRange(Documentation.CompletionItems);
        list.AddRange(Keywords.CompletionItems);
        return list;
    }
}

public static class Keywords
{
    public static List<CompletionItem> CompletionItems => _completionItems;
    private static readonly List<CompletionItem> _completionItems =
        new[] { "var", "fun", "nil", "true", "and", "else", "if", "or", "return", "while", "false", "for", "loop", "in", "match", "import" }
        .Select(kw => new CompletionItem() { Label = kw, Kind = CompletionItemKind.Keyword, InsertText = kw })
        .ToList();
}

