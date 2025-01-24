public class Or(IAstNode lhs, IAstNode rhs) : IAstNode
{
    public void Analyze(Analyzer a)
    {
        lhs.Analyze(a);
        rhs.Analyze(a);
    }

    public object? Interpret(Interpreter inter)
    {
        var b1 = lhs.Interpret(inter) switch
        {
            bool b => b,
            null => false,
            _ => true,
        };
        if (b1) return true;

        return rhs.Interpret(inter) switch
        {
            bool b => b,
            null => false,
            _ => true,
        };
    }

    public override string ToString()
    {
        return $"({lhs} or {rhs})";
    }
}



