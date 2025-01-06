public class Reassign(IAstNode lhs, IAstNode rhs, Token token) : IAstNode
{
    public void Analyze(Analyzer a)
    {
        if (lhs is Variable s)
        {
            if (!a.VarExists(s.Name))
                a.AddError(new SparvException($"No variable named '{s.Name}' exists in this scope", s.Token.Line, s.Token.Start, s.Token.End));
        }
        lhs.Analyze(a);
        rhs.Analyze(a);
    }

    public object? Interpret(Interpreter inter)
    {
        if (lhs is Variable)
        {
            var name = ((Variable)lhs).Name;
            inter.Reassign(name, rhs.Interpret(inter));
            return null;
        }
        if (lhs is Index index)
        {
            if (index.List.Interpret(inter) is not RuntimeList l)
                throw new SparvException($"Trying to index something that is not a list", token);

            var indexer = index.Indexer.Interpret(inter) switch
            {
                int i => i,
                double d => (int)d,
                _ => throw new SparvException("Index must be a number", token)
            };

            var res = rhs.Interpret(inter);
            l.list[indexer] = res;
            return null;
        }
        throw new SparvException("TODO: Cant reassign", token);
    }
}



