public class Call(List<IAstNode> paramaters, IAstNode expr, Token token) : IAstNode
{
    public AnalyzerKind Analyze(Analyzer a)
    {

        Console.WriteLine($"analyze call: {expr.Analyze(a)}");
        Console.WriteLine($"analyze call: {expr}");
        if (expr.Analyze(a) is not AnalyzerKind.Function)
            a.AddError(new SparvException("trying to call something that is not a function", token.Line, token.Start, token.End));
        
        return AnalyzerKind.Nil;
    }

    public object? Interpret(Interpreter inter)
    {
        var f = expr.Interpret(inter) as RuntimeFunc;
        // TODO: Is this correct?
        if (f is null)
            throw new Exception("TODO: trying to call something that is not a function.");
        // TODO: Check this in the analysis
        if (f.parameters.Count != paramaters.Count)
            throw new Exception("Wrong amount of parameters");

        inter.BeginScope();
        foreach (var (key, value) in f.scope)
        {
            inter.AddVar(key, value);
        }
        inter.BeginScope();
        foreach (var (key, value) in f.parameters.Zip(paramaters))
        {
            inter.AddVar(key, value.Interpret(inter));
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
        return $"(call([{string.Join(',', paramaters)}], {expr}))";
    }

}



