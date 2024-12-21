
public class ReassignMinus(IAstNode lhs, IAstNode rhs) : IAstNode
{
    public AnalyzerKind Analyze(Analyzer a)
    {
        throw new NotImplementedException();
    }

    public object? Interpret(Interpreter inter)
    {
        if (lhs is Variable)
        {
            var name = ((Variable)lhs).Name;
            var lhsValue = lhs.Interpret(inter);
            var rhsValue = rhs.Interpret(inter);
            return (lhsValue, rhsValue) switch
            {
                (double l, double r) => inter.Reassign(name, l - r),
                _ => throw new Exception("TODO: must be numbers"),
            };
        }
        throw new Exception("TODO: lhs not a variable");
    }
}



