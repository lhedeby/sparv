public class Less : IAstNode
{
    IAstNode _lhs;
    IAstNode _rhs;
    public Less(IAstNode lhs, IAstNode rhs)
    {
        _lhs = lhs;
        _rhs = rhs;
    }

    public void Analyze(Analyzer a) {}

    public object? Interpret(Interpreter inter)
    {
        var lhs = _lhs.Interpret(inter);
        var rhs = _rhs.Interpret(inter);
        if (lhs is not double) throw new Exception("TODO: lhs not number");
        if (rhs is not double) throw new Exception("TODO: rhs not number");
        return (double)lhs < (double)rhs;
    }

    public override string ToString()
    {
        return $"({_lhs} < {_rhs})";
    }
}



