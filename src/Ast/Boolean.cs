public class True : IAstNode
{
    public void Analyze(Analyzer a) {}
    public object? Interpret(Interpreter inter) => true;
    public override string ToString() => $"(bool(true))";
}

public class False : IAstNode
{
    public void Analyze(Analyzer a) {}
    public object? Interpret(Interpreter inter) => false;
    public override string ToString() => $"(bool(false))";
}
