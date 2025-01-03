
public class Or(IAstNode lhs, IAstNode rhs) : IAstNode
{
    public AnalyzerKind Analyze(Analyzer a)
    {
        return AnalyzerKind.Bool;
    }

    public object? Interpret(Interpreter inter)
    {
        if (lhs.Interpret(inter) is not bool lVal)
            throw new Exception("TODO: lhs not bool");
        if (lVal) return true;

        if (rhs.Interpret(inter) is not bool rVal)
            throw new Exception("TODO: lhs not bool");
        return rVal;
    }

    public override string ToString()
    {
        return $"({lhs} or {rhs})";
    }
}



