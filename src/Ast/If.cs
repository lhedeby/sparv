public class If(IAstNode expr, List<IAstNode> ifStmts, List<IAstNode> elseStmts) : IAstNode
{
    public AnalyzerKind Analyze(Analyzer a) => AnalyzerKind.Nil;

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



