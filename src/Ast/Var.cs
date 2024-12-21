public class Var(string identifier, IAstNode expr) : IAstNode
{
    public AnalyzerKind Analyze(Analyzer a)
    {
        a.AddVar(identifier, expr.Analyze(a));
        return AnalyzerKind.Nil;
    }

    public object? Interpret(Interpreter inter)
    {
        inter.AddVar(identifier, expr.Interpret(inter));
        return null;
    }

    public override string ToString()
    {
        return $"(var({identifier}, {expr}))";
    }
}



