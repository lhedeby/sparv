
public record class CompletionParams(CompletionContext Context) : IClientRequest
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
                    Label = suggestion.Item1,
                    Kind = CompletionItemKind.Variable,
                    LabelDetails = new(suggestion.Item2.ToString(), null),
                    Detail = $"Variable of type {suggestion.Item2}",
                    Documentation = null,
                    InsertText = suggestion.Item1,
                    InsertTextFormat = InsertTextFormat.PlainText
                }

            );
        }
        list.Add(
            new CompletionItem()
            {
                Label = "print",
                Kind = CompletionItemKind.Function,
                LabelDetails = new("Built-in function", null),
                Detail = "'input' print(string | object)",
                Documentation = new(
                    "markdown",
                    "# `print`\n\nPrints a message to the console.\n\n# **Signature**\n```plaintext\nvoid print_(string message)\n```\n\n#### **Parameters**\n- `message` *(string)*: The text to print to the console.\n\n#### **Example**\n```lox\nprint_(\"Hello, world!\");\n```\n"),
                InsertText = "print($1)",
                InsertTextFormat = InsertTextFormat.Snippet
            });
        return list;
    }
}
