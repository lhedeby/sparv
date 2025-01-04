public class Fun(List<string> parameters, List<IAstNode> stmts) : IAstNode
{
    public List<string> Parameters => parameters;

    public void Analyze(Analyzer a)
    {
        a.BeginScope();
        foreach (var p in parameters)
            a.AddVar(p);
        foreach (var stmt in stmts)
            stmt.Analyze(a);
        a.EndScope();
    }

    public object? Interpret(Interpreter inter)
    {
        return new RuntimeFunc(parameters, stmts, inter.GetScope());
    }
}



