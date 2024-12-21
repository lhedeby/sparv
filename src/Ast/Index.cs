public class Index(IAstNode list, IAstNode index) : IAstNode
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
            _ => throw new Exception("TODO: Index Err")
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
        throw new NotImplementedException();
    }
}



