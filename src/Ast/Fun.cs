
public class Fun(List<string> paramaters, List<IAstNode> stmts) : IAstNode
{
    public AnalyzerKind Analyze(Analyzer a)
    {
        return AnalyzerKind.Function;
    }

    public object? Interpret(Interpreter inter)
    {
        return new RuntimeFunc(paramaters, stmts, inter.GetScope());
    }
}



