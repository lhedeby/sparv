public class ReassignPlus(IAstNode lhs, IAstNode rhs, Token token) : IAstNode
{
    public void Analyze(Analyzer a)
    {
        lhs.Analyze(a);
        rhs.Analyze(a);
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
                (string l, string r) => inter.Reassign(name, l + r),
                (double l, double r) => inter.Reassign(name, l + r),
                // (RuntimeList l, RuntimeList r) => inter.Reassign(name, new RuntimeList(l.list.Concat(r.list).ToList())),
                (RuntimeList l, RuntimeList r) => ConcatList(l, r),
                _ => throw new SparvException($"lhs: '{lhsValue}', rhs: '{rhsValue}'", token),
            };
        }
        if (lhs is Get g)
        {
            if (g.lhs.Interpret(inter) is not RuntimeObject o)
                throw new SparvException("Left side of this must be an object", token);
            if (g.identifier.Interpret(inter) is not string s)
                throw new SparvException("Field does not exist on the object", token);

            var lhsValue = lhs.Interpret(inter);
            var rhsValue = rhs.Interpret(inter);
            return (lhsValue, rhsValue) switch
            {
                (string l, string r) => o.obj[s] = l + r,
                (double l, double r) => o.obj[s] = l + r,
                (RuntimeList l, RuntimeList r) => ConcatList(l, r),
                _ => throw new SparvException($"lhs: {lhsValue ?? "nil"}, rhs: {rhsValue}", token),
            };
        }
        throw new SparvException("TODO: Cant reassign", token);
    }

    private object? ConcatList(RuntimeList l1, RuntimeList l2)
    {
        l1.list.AddRange(l2.list);
        return null;
    }
}



