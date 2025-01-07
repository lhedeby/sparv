public class Call(List<IAstNode> parameters, IAstNode expr, Token token) : IAstNode
{
    public void Analyze(Analyzer a)
    {
        foreach (var p in parameters)
        {
            p.Analyze(a);
        }
        expr.Analyze(a);
    }

    public object? Interpret(Interpreter inter)
    {
        if (expr.Interpret(inter) is not RuntimeFunc f)
            throw new SparvException("trying to call something that is not a function", token.Line, token.Start, token.End);
        // TODO: Check this in the analysis
        if (f.Parameters.Count != parameters.Count)
            throw new SparvException("Wrong amount of parameters in function call", token);
        var resolved_params = parameters.Select(p => p.Interpret(inter)).ToList();

        inter.BeginScope();
        foreach (var (key, value) in f.Closure)
        {
            inter.AddVar(key, value);
        }
        foreach (var (key, value) in f.Parameters.Zip(resolved_params))
        {
            inter.AddVar(key, value);
        }
        f.Stmts.Run(inter);
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
