public class Add(IAstNode lhs, IAstNode rhs) : IAstNode
{
    public void Analyze(Analyzer a)
    {
        lhs.Analyze(a);
        rhs.Analyze(a);
    }

    public object? Interpret(Interpreter inter)
    {
        return (lhs.Interpret(inter) ?? "nil", rhs.Interpret(inter) ?? "nil") switch
        {
            (double d1, double d2) => d1 + d2,
            (RuntimeList l1, RuntimeList l2) => new RuntimeList(l1.list.Concat(l2.list).ToList()),
            _ => lhs.ToString() + rhs.ToString()
        };
    }

    public override string ToString()
    {
        return $"({lhs} + {rhs})";
    }
}



