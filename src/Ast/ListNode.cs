public class ListNode(List<IAstNode> list) : IAstNode
{
    public void Analyze(Analyzer a) {}

    public object? Interpret(Interpreter inter)
    {
        return new RuntimeList(list.Select(item => item.Interpret(inter)).ToList());
    }
}
