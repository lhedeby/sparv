public class While(IAstNode expr, List<IAstNode> stmts) : IAstNode
{
    public void Analyze(Analyzer a)
    {
        expr.Analyze(a);
        foreach (var stmt in stmts)
            stmt.Analyze(a);
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



