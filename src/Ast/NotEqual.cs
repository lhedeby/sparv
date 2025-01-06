public class NotEqual(IAstNode lhs, IAstNode rhs) : IAstNode
{
    public void Analyze(Analyzer a)
    {
        lhs.Analyze(a);
        rhs.Analyze(a);
    }

    public object? Interpret(Interpreter inter)
    {
        return !EqualityComparer<object>.Default.Equals(lhs.Interpret(inter), rhs.Interpret(inter));
    }

    public override string ToString()
    {
        return $"({lhs} != {rhs})";
    }
}



