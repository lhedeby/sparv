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

        while (expr.Interpret(inter) switch
        {
            bool b => b,
            null => false,
            _ => true,
        })
        {
            inter.BeginScope();
            stmts.Run(inter);
            inter.EndScope();
            if (inter.HasReturned) return null;
        }
        return null;
    }
}



