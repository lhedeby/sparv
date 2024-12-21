public record class Set(IAstNode lhs, string identifier, IAstNode rhs) : IAstNode
{
    public AnalyzerKind Analyze(Analyzer a)
    {
        throw new NotImplementedException();
    }

    public object? Interpret(Interpreter inter)
    {
        if (lhs.Interpret(inter) is not RuntimeObject o)
            throw new Exception("TODO: not an object");
        o.obj[identifier] = rhs.Interpret(inter);
        return null;
    }
}
