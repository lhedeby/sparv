
public class PrefixBang : IAstNode
{
    private IAstNode _node;
    public PrefixBang(IAstNode node)
    {
        _node = node;
    }

    public AnalyzerKind Analyze(Analyzer a)
    {
        return AnalyzerKind.Bool;
    }

    public object? Interpret(Interpreter inter)
    {
        var node = _node.Interpret(inter);
        if (node is not bool) throw new Exception("TODO: Not a bool");
        return !((bool)node);
    }
}



