public class ListNode(List<IAstNode> list) : IAstNode
{
    public void Analyze(Analyzer a)
    {
        foreach (var node in list)
        {
            node.Analyze(a);
        }
    }

    public object? Interpret(Interpreter inter)
    {
        return new RuntimeList(list.Select(item => item.Interpret(inter)).ToList());
    }
}
