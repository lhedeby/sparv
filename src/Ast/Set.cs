public record class Set(IAstNode lhs, IAstNode identifier, IAstNode rhs, Token token) : IAstNode
{
    public void Analyze(Analyzer a)
    {
        lhs.Analyze(a);
        identifier.Analyze(a);
        rhs.Analyze(a);
    }

    public object? Interpret(Interpreter inter)
    {
        if (lhs.Interpret(inter) is not RuntimeObject o)
            throw new SparvException("Trying to dot something that is not an object", token);

        if (identifier.Interpret(inter) is not string s)
            throw new SparvException("Field does not exist on the object", token);
        o.Obj[s] = rhs.Interpret(inter);
        return null;
    }
}
