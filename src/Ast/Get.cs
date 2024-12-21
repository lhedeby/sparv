public record class Get(IAstNode lhs, string identifier) : IAstNode
{
    public AnalyzerKind Analyze(Analyzer a)
    {
        throw new NotImplementedException();
    }

    public object? Interpret(Interpreter inter)
    {
        if (lhs.Interpret(inter) is not RuntimeObject o)
            throw new Exception("TODO: not an object");
        return o.obj[identifier];
    }
}
