public class Nil : IAstNode
{
    public void Analyze(Analyzer a)
    {
    }

    public object? Interpret(Interpreter inter)
    {
        return null;
    }

    public override string ToString()
    {
        return $"(nil)";
    }
}



