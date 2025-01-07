public class Loop(IAstNode expr, List<IAstNode> stmts, Token token) : IAstNode
{
    public void Analyze(Analyzer a)
    {
        expr.Analyze(a);
        foreach (var stmt in stmts)
            stmt.Analyze(a);
    }

    public object? Interpret(Interpreter inter)
    {
        if (expr.Interpret(inter) is not double d)
            throw new SparvException("Loop must be followed by a number", token);
        for (int i = 0; i < d; i++)
        {
            foreach (var stmt in stmts)
            {
                stmt.Interpret(inter);
            }
        }
        return null;
    }
}
