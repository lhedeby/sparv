public class Index(IAstNode list, IAstNode index, Token token) : IAstNode
{
    public object? Interpret(Interpreter inter)
    {
        var i = index.Interpret(inter) switch
        {
            int n => n,
            double n => (int)n,
            _ => throw new SparvException("List access must be a number", token)
        };
        if (i < 0)
            throw new SparvException("Index cant be negative", token);

        var r = list.Interpret(inter);

        if (r is RuntimeList rl)
        {
            if (i >= rl.List.Count)
                throw new SparvException("Index cant be higher than the length of the list", token);
            return rl.List[i];
        }

        if (r is string s)
        {
            if (i >= s.Length)
                throw new SparvException("Index cant be higher than the length of the string", token);
            return s[i].ToString();
        }
        throw new SparvException("Not a list or string", token);
    }

    public IAstNode List { get => list; }
    public IAstNode Indexer { get => index; }

    public override string? ToString()
    {
        return $"(Index ({list}, {index}))";
    }

    public void Analyze(Analyzer a)
    {
        list.Analyze(a);
        index.Analyze(a);
    }
}



