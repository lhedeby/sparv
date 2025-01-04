public class Var(Token token, IAstNode expr) : IAstNode
{
    public void Analyze(Analyzer a)
    {
        if (expr is Fun fun)
            a.Functions.Add(token.Value, fun.Parameters);
        if (a.VarExistsInCurrentScope(token.Value))
            a.AddError(new SparvException($"Variable with name '{token.Value}' already exists in the current scope", token));
        else
            a.AddVar(token.Value);
        expr.Analyze(a);
    }

    public object? Interpret(Interpreter inter)
    {
        inter.AddVar(token.Value, expr.Interpret(inter));
        return null;
    }

    public override string ToString()
    {
        return $"(var({token.Value}, {expr}))";
    }
}



