public class ReassignMinus(IAstNode lhs, IAstNode rhs, Token token) : IAstNode
{
    public AnalyzerKind Analyze(Analyzer a)
    {
        return AnalyzerKind.Nil;
    }

    public object? Interpret(Interpreter inter)
    {
        if (lhs is Variable v)
        {
            var name = v.Name;
            var lhsValue = lhs.Interpret(inter);
            var rhsValue = rhs.Interpret(inter);
            return (lhsValue, rhsValue) switch
            {
                (double l, double r) => inter.Reassign(name, l - r),
                _ => throw new SparvException($"lhs: '{lhsValue}', rhs: '{rhsValue}'", token),
            };
        }

        if (lhs is Get g)
        {
            if (g.lhs.Interpret(inter) is not RuntimeObject o)
                throw new SparvException("Left side of this must be an object", token);
            if (g.identifier.Interpret(inter) is not string s)
                throw new Exception("TODO: expected string as identifier");

            var lhsValue = lhs.Interpret(inter);
            var rhsValue = rhs.Interpret(inter);

            return (lhsValue, rhsValue) switch
            {
                (double l, double r) => o.obj[s] = l - r,
                _ => throw new SparvException($"lhs: '{lhsValue}', rhs: '{rhsValue}'", token),
            };
        }
        throw new Exception("TODO: lhs not a variable");
    }
}



