public class If(IAstNode expr, List<IAstNode> ifStmts, List<IAstNode> elseStmts) : IAstNode
{
    public AnalyzerKind Analyze(Analyzer a)
    {
        a.BeginScope();
        foreach (var stmt in ifStmts)
            stmt.Analyze(a);
        a.EndScope();
        a.BeginScope();
        foreach (var stmt in elseStmts)
            stmt.Analyze(a);
        a.EndScope();
        return AnalyzerKind.Nil;
    }

    public object? Interpret(Interpreter inter)
    {
        var t = expr.Interpret(inter);
        if (t is bool && (bool)t)
        {
            ifStmts.Run(inter);
        }
        else
        {
            elseStmts.Run(inter);
        }
        return null;
    }
}



