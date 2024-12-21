public class For(string i, IAstNode expr, List<IAstNode> stmts) : IAstNode
{
    public AnalyzerKind Analyze(Analyzer a)
    {
        return AnalyzerKind.Nil;
    }

    public object? Interpret(Interpreter inter)
    {
        if (expr.Interpret(inter) is not RuntimeList list)
            throw new Exception("list is null");

        foreach (var v in list.list)
        {
            inter.BeginScope();
            inter.AddVar(i, v);
            stmts.Run(inter);
            inter.EndScope();
            if (inter.HasReturned) return null;
        }

        return null;
    }

    public override string ToString()
    {
        return $"(for ({i}, {expr}, {stmts.ListToString()}))";
    }
}



