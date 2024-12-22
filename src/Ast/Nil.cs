public class Nil : IAstNode
{
    public AnalyzerKind Analyze(Analyzer a)
    {
        return AnalyzerKind.Nil;
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



