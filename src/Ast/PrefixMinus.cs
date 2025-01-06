public class PrefixMinus(IAstNode node, Token token) : IAstNode
{
    public void Analyze(Analyzer a)
    {
    }

    public object? Interpret(Interpreter inter)
    {
        if (node.Interpret(inter) is not double d)
            throw new SparvException("'-' must be followed by a number", token);
        return -d;
    }
}



