public class Equal(IAstNode lhs, IAstNode rhs) : IAstNode
{
    public void Analyze(Analyzer a)
    {
    }

    public object? Interpret(Interpreter inter)
    {
        var v = (lhs.Interpret(inter), rhs.Interpret(inter)) switch
        {
            (string l, string r) => l == r,
            (double l, double r) => l == r,
            (bool l, bool r) => l == r,
            (null, null) => true,
            _ => false
        };
        return v;
    }

    public override string ToString()
    {
        return $"({lhs} == {rhs})";
    }
}



