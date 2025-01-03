
public class Fun(List<string> parameters, List<IAstNode> stmts) : IAstNode
{
    public AnalyzerKind Analyze(Analyzer a)
    {
        a.BeginScope();
        foreach (var p in parameters)
            a.AddVar(p, AnalyzerKind.Nil);
        foreach (var stmt in stmts)
            stmt.Analyze(a);
        a.EndScope();
        return AnalyzerKind.Function;
    }

    public object? Interpret(Interpreter inter)
    {
        return new RuntimeFunc(parameters, stmts, inter.GetScope());
    }
}



