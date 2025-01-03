
public class Modulo : IAstNode
{
    IAstNode _lhs;
    IAstNode _rhs;
    public Modulo(IAstNode lhs, IAstNode rhs)
    {
        _lhs = lhs;
        _rhs = rhs;
    }

    public AnalyzerKind Analyze(Analyzer a)
    {
        return AnalyzerKind.Number;
    }

    public object? Interpret(Interpreter inter)
    {
        var lhs = _lhs.Interpret(inter);
        var rhs = _rhs.Interpret(inter);
        if (lhs is not double) throw new Exception("TODO: lhs not number");
        if (rhs is not double) throw new Exception("TODO: rhs not number");
        return (double)lhs % (double)rhs;
    }

    public override string ToString()
    {
        return $"({_lhs} % {_rhs})";
    }
}



