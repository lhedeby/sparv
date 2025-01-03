
public class While(IAstNode expr, List<IAstNode> stmts) : IAstNode
{
    public AnalyzerKind Analyze(Analyzer a)
    {
        // if (expr.Analyze(a) != AnalyzerKind.Bool)
        //     a.AddError(new SparvException("While expression must be bool", 0, 0, 0));
        foreach (var stmt in stmts)
            stmt.Analyze(a);
        return AnalyzerKind.Nil;
    }

    public object? Interpret(Interpreter inter)
    {

        while (expr.Interpret(inter) is bool b && b)
        // var e = expr.Interpret(inter);
        // while (e is bool && (bool)e)
        {
            inter.BeginScope();
            stmts.Run(inter);
            inter.EndScope();
            if (inter.HasReturned) return null;
            // e = expr.Interpret(inter);
        }
        return null;
    }
}



