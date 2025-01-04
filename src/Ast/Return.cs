
public class Return(IAstNode expr) : IAstNode
{
    public void Analyze(Analyzer a)
    {
        expr.Analyze(a);
    }

    public object? Interpret(Interpreter inter)
    {
        var value = expr.Interpret(inter);
        inter.SetReturnValue(value);
        return value;
    }
}



