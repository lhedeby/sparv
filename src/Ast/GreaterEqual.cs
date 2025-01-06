public class GreaterEqual(IAstNode lhs, IAstNode rhs, Token token) : IAstNode
{
    public void Analyze(Analyzer a)
    {
        lhs.Analyze(a);
        rhs.Analyze(a);
    }

    public object? Interpret(Interpreter inter)
    {
        if (lhs.Interpret(inter) is not double d1) throw new SparvException("Left hand side of this is not a number", token);
        if (rhs.Interpret(inter) is not double d2) throw new SparvException("Right hand side of this is not a number", token);
        return d1 >= d2;
    }

    public override string ToString()
    {
        return $"({lhs} >= {rhs})";
    }
}



