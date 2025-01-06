public class RangeList(IAstNode lhs, IAstNode rhs, Token token) : IAstNode
{
    public void Analyze(Analyzer a)
    {
        lhs.Analyze(a);
        rhs.Analyze(a);
    }

    public object? Interpret(Interpreter inter)
    {

        if (lhs.Interpret(inter) is not double start)
            throw new SparvException("Start of range must be a number", token);
        if (rhs.Interpret(inter) is not double end)
            throw new SparvException("End of range must be a number", token);


        // TODO: Just do something better
        List<object?> list = Enumerable.Range((int)start, (int)(end-start)).Select(x => (object?)(double)x).ToList();
        return new RuntimeList(list);
    }

    public override string? ToString()
    {
        return $"(RangeList ({lhs}, {rhs}))";
    }
}



