public class Call(List<IAstNode> parameters, IAstNode expr, Token token) : IAstNode
{
    public void Analyze(Analyzer a)
    {
    }

    public object? Interpret(Interpreter inter)
    {
        if (expr.Interpret(inter) is not RuntimeFunc f)
            throw new SparvException("trying to call something that is not a function", token.Line, token.Start, token.End);
        // TODO: Check this in the analysis
        if (f.parameters.Count != parameters.Count)
            throw new SparvException("Wrong amount of parameters in function call", token);
        var resolved_params = parameters.Select(p => p.Interpret(inter)).ToList();

        inter.BeginScope();
        foreach (var (key, value) in f.scope)
        {
            inter.AddVar(key, value);
        }
        inter.BeginScope();
        foreach (var (key, value) in f.parameters.Zip(resolved_params))
        {
            inter.AddVar(key, value);
        }
        f.stmts.Run(inter);
        // TODO: Probably remove this double scope
        inter.EndScope();
        inter.EndScope();
        if (inter.HasReturned)
        {
            var returnValue = inter.ReturnValue;
            inter.ResetReturn();
            return returnValue;
        }
        return null;
    }

    public override string ToString()
    {
        return $"(call([{string.Join(',', parameters)}], {expr}))";
    }

}
