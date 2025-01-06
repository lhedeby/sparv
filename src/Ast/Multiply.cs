public class Multiply(IAstNode lhs, IAstNode rhs, Token token) : IAstNode
{
    public void Analyze(Analyzer a)
    {
        lhs.Analyze(a);
        rhs.Analyze(a);
    }

    public object? Interpret(Interpreter inter)
    {
        if (lhs.Interpret(inter) is not double l)
            throw new SparvException("Left hand side of this multiply expression is not a number", token);
        if (rhs.Interpret(inter) is not double r)
            throw new SparvException("Right hand side of this multiply expression is not a number", token);
        return l * r;
    }

    public override string ToString()
    {
        return $"({lhs} * {rhs})";
    }
}



