
public class PrefixBang : IAstNode
{
    private IAstNode _node;
    public PrefixBang(IAstNode node)
    {
        _node = node;
    }

    public void Analyze(Analyzer a)
    {
    }

    public object? Interpret(Interpreter inter)
    {
        var node = _node.Interpret(inter);
        if (node is not bool) throw new Exception("TODO: Not a bool");
        return !((bool)node);
    }
}



