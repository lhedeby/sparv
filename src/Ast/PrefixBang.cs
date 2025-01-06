public class PrefixBang(IAstNode node, Token token) : IAstNode
{
    public void Analyze(Analyzer a)
    {
    }

    public object? Interpret(Interpreter inter)
    {
        if (node.Interpret(inter) is not bool b)
            throw new SparvException("'!' expression must be followed by a boolean", token);
        return !b;
    }
}



