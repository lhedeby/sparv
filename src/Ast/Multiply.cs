public class Multiply(IAstNode lhs, IAstNode rhs, Token token) : IAstNode
{
    public void Analyze(Analyzer a)
    {
        lhs.Analyze(a);
        rhs.Analyze(a);
    }

    public object? Interpret(Interpreter inter)
    {
        return (lhs.Interpret(inter), rhs.Interpret(inter)) switch
        {
            (double l, double r) => l * r,
            (string l, double r) => string.Concat(Enumerable.Repeat(l, (int)r)),
            (double l, string r) => string.Concat(Enumerable.Repeat(r, (int)l)),
            (_, _) => throw new SparvException("Unexpected type in multiply expression", token)
        };
        // if (lhs.Interpret(inter) is not double l)
        //     throw new SparvException("Left hand side of this multiply expression is not a number", token);
        // if (rhs.Interpret(inter) is not double r)
        //     throw new SparvException("Right hand side of this multiply expression is not a number", token);
        // return l * r;
    }

    public override string ToString()
    {
        return $"({lhs} * {rhs})";
    }
}



