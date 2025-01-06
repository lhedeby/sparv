public class And(IAstNode lhs, IAstNode rhs, Token token) : IAstNode
{
    public void Analyze(Analyzer a) {}

    public object? Interpret(Interpreter inter)
    {
        if (lhs.Interpret(inter) is not bool l)
            throw new SparvException("Left hand side is not a bool", token);
        if (!l) return false;

        if (rhs.Interpret(inter) is not bool r)
            throw new SparvException("Right hand side is not a bool", token);
        return r;
    }

    public override string ToString()
    {
        return $"({lhs} and {rhs})";
    }
}
