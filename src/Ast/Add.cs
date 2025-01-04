public class Add : IAstNode
{
    IAstNode _lhs;
    IAstNode _rhs;
    public Add(IAstNode lhs, IAstNode rhs)
    {
        _lhs = lhs;
        _rhs = rhs;
    }

    public void Analyze(Analyzer a)
    {
    }

    public object? Interpret(Interpreter inter)
    {
        var lhs = _lhs.Interpret(inter) ?? "nil";
        var rhs = _rhs.Interpret(inter) ?? "nil";

        object? res = (lhs, rhs) switch
        {
            (double d1, double d2) => d1 + d2,
            (RuntimeList l1, RuntimeList l2) => new RuntimeList(l1.list.Concat(l2.list).ToList()),
            _ => lhs.ToString() + rhs.ToString()
        };

        return res;
    }

    public override string ToString()
    {
        return $"({_lhs} + {_rhs})";
    }
}



