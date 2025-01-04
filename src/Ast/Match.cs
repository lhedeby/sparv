public record class Match(IAstNode expr, List<(IAstNode lhs, IAstNode rhs)> arms) : IAstNode
{
    public void Analyze(Analyzer a)
    {
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
