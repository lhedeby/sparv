public record class Match(IAstNode expr, List<(IAstNode lhs, IAstNode rhs)> arms) : IAstNode
{
    public AnalyzerKind Analyze(Analyzer a)
    {
        return AnalyzerKind.Nil;
    }

    public object? Interpret(Interpreter inter)
    {
        var e = expr.Interpret(inter);
        foreach (var arm in arms)
        {
            if (EqualityComparer<object>.Default.Equals(e, arm.lhs.Interpret(inter)))
                return arm.rhs.Interpret(inter);
        }
        return null;
    }
}
