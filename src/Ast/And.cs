public class And(IAstNode lhs, IAstNode rhs) : IAstNode
{
    public AnalyzerKind Analyze(Analyzer a) => AnalyzerKind.Bool;

    public object? Interpret(Interpreter inter)
    {
        if (lhs.Interpret(inter) is not bool lVal)
            throw new Exception("TODO: lhs not bool");
        if (!lVal) return false;

        if (rhs.Interpret(inter) is not bool rVal)
            throw new Exception("TODO: rhs not bool");
        return rVal;
    }

    public override string ToString()
    {
        return $"({lhs} and {rhs})";
    }
}
