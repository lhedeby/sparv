public class True : IAstNode
{
    public AnalyzerKind Analyze(Analyzer a) => AnalyzerKind.Bool;

    public object? Interpret(Interpreter inter)
    {
        return true;
    }

    public override string ToString()
    {
        return $"(bool(true))";
    }
}

public class False : IAstNode
{
    public AnalyzerKind Analyze(Analyzer a) => AnalyzerKind.Bool;

    public object? Interpret(Interpreter inter)
    {
        return false;
    }

    public override string ToString()
    {
        return $"(bool(false))";
    }
}
