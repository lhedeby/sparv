public class ListNode(List<IAstNode> list) : IAstNode
{
    public AnalyzerKind Analyze(Analyzer a) => AnalyzerKind.List;

    public object? Interpret(Interpreter inter)
    {
        return new RuntimeList(list.Select(item => item.Interpret(inter)).ToList());
    }
}
