public class For(string i, IAstNode expr, List<IAstNode> stmts, Token token) : IAstNode
{
    public void Analyze(Analyzer a)
    {
        expr.Analyze(a);
        a.BeginScope();
        a.AddVar(i);
        foreach (var stmt in stmts)
            stmt.Analyze(a);
        a.EndScope();
    }

    public object? Interpret(Interpreter inter)
    {
        var eval = expr.Interpret(inter);
        if (eval is RuntimeList list)
        {
            foreach (var v in list.List)
            {
                inter.BeginScope();
                inter.AddVar(i, v);
                stmts.Run(inter);
                inter.EndScope();
                if (inter.HasReturned) return null;
            }
            return null;
        }
        else if (eval is string s)
        {
            foreach (var v in s)
            {
                inter.BeginScope();
                inter.AddVar(i, v.ToString());
                stmts.Run(inter);
                inter.EndScope();
                if (inter.HasReturned) return null;
            }
            return null;
        }

        throw new SparvException($"Cant iterate over {eval ?? "nil"}", token);
    }

    public override string ToString()
    {
        return $"(for ({i}, {expr}, {stmts.ListToString()}))";
    }
}



