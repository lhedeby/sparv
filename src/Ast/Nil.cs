
public class Nil : IAstNode
{
    public AnalyzerKind Analyze(Analyzer a)
    {
        throw new NotImplementedException();
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



