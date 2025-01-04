public class Number(Token token) : IAstNode
{
    private double _value = double.Parse(token.Value);

    public void Analyze(Analyzer a) {}

    public object? Interpret(Interpreter inter)
    {
        return _value;
    }

    public override string? ToString()
    {
        return $"(number({_value}))";
    }
}



