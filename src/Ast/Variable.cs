public class Variable : IAstNode
{
    private Token _token;
    private string _name;
    public Variable(Token token)
    {
        _token = token;
        _name = token.Value;
    }
    public string Name { get => _name; }
    public Token Token { get => _token; }

    public AnalyzerKind Analyze(Analyzer a)
    {
        if (a.Vars.Any(x => x.Item1 == _name)) {
            return a.Vars.First(x => x.Item1 == _name).Item2;
        }
        a.AddError(new SparvException("Variable is not delcared", _token.Line, _token.Start, _token.End));
        return AnalyzerKind.Nil;
    }

    public object? Interpret(Interpreter inter)
    {
        var res = inter.GetVar(_name);
        return res;
    }

    public override string ToString()
    {
        return $"(Variable({_name}))";
    }

}



