public class Add(IAstNode lhs, IAstNode rhs) : IAstNode
{
    public void Analyze(Analyzer a)
    {
        lhs.Analyze(a);
        rhs.Analyze(a);
    }

    public object? Interpret(Interpreter inter)
    {
        var l = lhs.Interpret(inter) ?? "nil";
        var r = rhs.Interpret(inter) ?? "nil";

        return (l, r) switch
        {
            (double d1, double d2) => d1 + d2,
            (RuntimeList l1, RuntimeList l2) => new RuntimeList(l1.list.Concat(l2.list).ToList()),
            _ => l.ToString() + r.ToString()
        };
    }

    public override string ToString()
    {
        return $"({lhs} + {rhs})";
    }
}



