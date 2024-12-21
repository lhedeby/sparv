
public class RangeList(IAstNode lhs, IAstNode rhs) : IAstNode
{
    public AnalyzerKind Analyze(Analyzer a)
    {
        throw new NotImplementedException();
    }

    public object? Interpret(Interpreter inter)
    {

        if (lhs.Interpret(inter) is not double start)
            throw new Exception("TODO: Start of range must be a number");
        if (rhs.Interpret(inter) is not double end)
            throw new Exception("TODO: Start of range must be a number");


        // TODO: Just do something better
        List<object?> list = Enumerable.Range((int)start, (int)(end-start)).Select(x => (object?)(double)x).ToList();
        return new RuntimeList(list);
    }

    public override string? ToString()
    {
        return $"(RangeList ({lhs}, {rhs}))";
    }
}



