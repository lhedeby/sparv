public class Or(IAstNode lhs, IAstNode rhs, Token token) : IAstNode
{
    public void Analyze(Analyzer a)
    {
        lhs.Analyze(a);
        rhs.Analyze(a);
    }

    public object? Interpret(Interpreter inter)
    {
        if (lhs.Interpret(inter) is not bool l)
            throw new SparvException("Left hand side of expression is not bool", token);
        if (l) return true;

        if (rhs.Interpret(inter) is not bool r)
            throw new SparvException("Right hand side of expression is not bool", token);
        return r;
    }

    public override string ToString()
    {
        return $"({lhs} or {rhs})";
    }
}



