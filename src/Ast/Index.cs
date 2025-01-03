public class Index(IAstNode list, IAstNode index, Token token) : IAstNode
{
    public object? Interpret(Interpreter inter)
    {
        var i = index.Interpret(inter) switch
        {
            int n => n,
            double n => (int)n,
            _ => throw new Exception($"TODO: Not a number: {index}")
        };

        return list.Interpret(inter) switch
        {
            RuntimeList list => list.list[(int)i],
            string s => s[(int)i].ToString(),
            _ => throw new SparvException("Trying to index something that should not be indexed", token)
        };
    }

    public IAstNode List { get => list; }
    public IAstNode Indexer { get => index; }

    public override string? ToString()
    {
        return $"(Index ({list}, {index}))";
    }

    public AnalyzerKind Analyze(Analyzer a)
    {
        var kind = list.Analyze(a);
        // TODO
        // if (kind is not AnalyzerKind.List or AnalyzerKind.String)
        //     a.AddError(new SparvException("Trying to index something that is not a list or string at", token));
        return kind;
    }
}



