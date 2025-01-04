public class True : IAstNode
{
    public void Analyze(Analyzer a) {}

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
    public void Analyze(Analyzer a) {}

    public object? Interpret(Interpreter inter)
    {
        return false;
    }

    public override string ToString()
    {
        return $"(bool(false))";
    }
}
