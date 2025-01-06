public class Divide(IAstNode lhs, IAstNode rhs, Token token) : IAstNode
{
    public void Analyze(Analyzer a) {}

    public object? Interpret(Interpreter inter)
    {
        if (lhs.Interpret(inter) is not double l)
            throw new SparvException("Left hand side is not a number", token);
        if (rhs.Interpret(inter) is not double r)
            throw new SparvException("Left hand side is not a number", token);
        return l / r;
    }

    public override string ToString()
    {
        return $"({lhs} / {rhs})";
    }
}



