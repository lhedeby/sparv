public class Subtract(IAstNode lhs, IAstNode rhs, Token token) : IAstNode
{
    public void Analyze(Analyzer a)
    {
        lhs.Analyze(a);
        rhs.Analyze(a);
    }

    public object? Interpret(Interpreter inter)
    {
        var e1 = lhs.Interpret(inter);
        var e2 = rhs.Interpret(inter);
        if (e1 is not double and not int)
            throw new SparvException("Left hand side of subtract expression is not a number", token);
        if (e2 is not double and not int)
            throw new SparvException("Right hand side of subtract expression is not a number", token);
        return (double)e1 - (double)e2;
    }

    public override string ToString()
    {
        return $"({lhs} - {rhs})";
    }
}



