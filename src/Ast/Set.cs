public record class Set(IAstNode lhs, IAstNode identifier, IAstNode rhs) : IAstNode
{
    public void Analyze(Analyzer a)
    {
    }

    public object? Interpret(Interpreter inter)
    {
        if (lhs.Interpret(inter) is not RuntimeObject o)
            throw new Exception("TODO: not an object");

        if (identifier.Interpret(inter) is not string s)
            throw new Exception("TODO: expected string as identifier");
        o.obj[s] = rhs.Interpret(inter);
        return null;
    }
}
