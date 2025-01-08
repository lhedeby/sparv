public class If(IAstNode expr, List<IAstNode> ifStmts, List<IAstNode> elseStmts) : IAstNode
{
    public void Analyze(Analyzer a)
    {
        a.BeginScope();
        foreach (var stmt in ifStmts)
            stmt.Analyze(a);
        a.EndScope();
        a.BeginScope();
        foreach (var stmt in elseStmts)
            stmt.Analyze(a);
        a.EndScope();
    }

    public object? Interpret(Interpreter inter)
    {
        var boolean = expr.Interpret(inter) switch
        {
            bool b => b,
            null => false,
            _ => true,
        };
        if (boolean)
        {
            ifStmts.Run(inter);
            if (inter.HasReturned) return null;
        }
        else
        {
            elseStmts.Run(inter);
            if (inter.HasReturned) return null;
        }
        return null;
    }
}



