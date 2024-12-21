
public class Subtract : IAstNode
{
    IAstNode _lhs;
    IAstNode _rhs;
    public Subtract(IAstNode lhs, IAstNode rhs)
    {
        _lhs = lhs;
        _rhs = rhs;
    }

    public AnalyzerKind Analyze(Analyzer a)
    {
        throw new NotImplementedException();
    }

    public object? Interpret(Interpreter inter)
    {
        var lhs = _lhs.Interpret(inter);
        var rhs = _rhs.Interpret(inter);
        if (lhs is not double && lhs is not int) throw new Exception("TODO: lhs not number");
        if (rhs is not double && rhs is not int) throw new Exception("TODO: rhs not number");
        return (double)lhs - (double)rhs;
    }

    public override string ToString()
    {
        return $"({_lhs} - {_rhs})";
    }
}



