public record class Get(IAstNode lhs, IAstNode identifier, Token token) : IAstNode
{
    public void Analyze(Analyzer a)
    {
        lhs.Analyze(a);
    }

    public object? Interpret(Interpreter inter)
    {
        if (lhs.Interpret(inter) is not RuntimeObject o)
            throw new SparvException("Trying to access field on something that is not and object", token);
        if (identifier.Interpret(inter) is not string s)
            throw new SparvException("Expected string as identifier", token);
        object? res = null;
        o.obj.TryGetValue(s, out res);
        return res;
    }
}
