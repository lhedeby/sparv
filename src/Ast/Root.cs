public class Root(List<IAstNode> nodes) : IAstNode
{
    public AnalyzerKind Analyze(Analyzer a)
    {
        foreach (var node in nodes)
        {
            node.Analyze(a);
        }
        return AnalyzerKind.Nil;
    }

    public object? Interpret(Interpreter inter)
    {
        foreach (var node in nodes)
        {
            node.Interpret(inter);
        }
        return null;
    }

    public override string ToString()
    {
        return string.Join('\n', nodes);
    }
}



