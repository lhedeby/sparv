
public class NotEqual : IAstNode
{
    IAstNode _lhs;
    IAstNode _rhs;
    public NotEqual(IAstNode lhs, IAstNode rhs)
    {
        _lhs = lhs;
        _rhs = rhs;
    }

    public AnalyzerKind Analyze(Analyzer a)
    {
        return AnalyzerKind.Bool;
    }

    public object? Interpret(Interpreter inter)
    {
        var lhs = _lhs.Interpret(inter);
        var rhs = _rhs.Interpret(inter);
        return !EqualityComparer<object>.Default.Equals(lhs, rhs);
    }

    public override string ToString()
    {
        return $"({_lhs} != {_rhs})";
    }
}



