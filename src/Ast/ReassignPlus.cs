
public class ReassignPlus(IAstNode lhs, IAstNode rhs) : IAstNode
{
    public AnalyzerKind Analyze(Analyzer a)
    {
        return AnalyzerKind.Nil;
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
                (string l, string r) => inter.Reassign(name, l + r),
                (double l, double r) => inter.Reassign(name, l + r),
                (RuntimeList l, RuntimeList r) => inter.Reassign(name, new RuntimeList(l.list.Concat(r.list).ToList())),
                _ => throw new Exception("TODO: Invalid types"),
            };
        }
        throw new Exception("TODO: Cant set lhs");
    }
}



