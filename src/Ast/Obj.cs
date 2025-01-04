public class Obj(Dictionary<string, IAstNode> dict) : IAstNode
{
    public void Analyze(Analyzer a)
    {
    }

    public object? Interpret(Interpreter inter)
    {
        var rtd = new Dictionary<string, object?>();
        foreach (var (key, value) in dict)
        {
            rtd.Add(key, value.Interpret(inter));
        }
        return new RuntimeObject(rtd);
    }
}
