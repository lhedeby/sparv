
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
        list.Add(Documentation.Print().ToCompletionItem());
        list.Add(Documentation.Split().ToCompletionItem());
        list.Add(Documentation.ReadFile().ToCompletionItem());
        list.Add(Documentation.Len().ToCompletionItem());
        list.Add(Documentation.Parse().ToCompletionItem());
        list.Add(Documentation.Typeof().ToCompletionItem());
        list.Add(Documentation.ReadInput().ToCompletionItem());
        return list;
    }
}

