
public class PrefixMinus : IAstNode
{
    private IAstNode _node;
    public PrefixMinus(IAstNode node)
    {
        _node = node;
    }

    public AnalyzerKind Analyze(Analyzer a)
    {
        throw new NotImplementedException();
    }

    public object? Interpret(Interpreter inter)
    {
        var node = _node.Interpret(inter);
        if (node is not double) throw new Exception("TODO: Not a number");
        return -((double)node);
    }
}



