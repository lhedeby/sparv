
public class Return(IAstNode expr) : IAstNode
{
    public AnalyzerKind Analyze(Analyzer a)
    {
        throw new NotImplementedException();
    }

    public object? Interpret(Interpreter inter)
    {
        var value = expr.Interpret(inter);
        inter.SetReturnValue(value);
        return value;
    }
}



