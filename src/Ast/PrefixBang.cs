public class PrefixBang(IAstNode node) : IAstNode
{
    public void Analyze(Analyzer a)
    {
        node.Analyze(a);
    }

    public object? Interpret(Interpreter inter)
    {
        return !(node.Interpret(inter) switch
        {
            bool b => b,
            null => false,
            _ => true,
        });
    }
}



