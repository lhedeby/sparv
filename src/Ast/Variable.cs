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
        return a.Vars.FirstOrDefault(x => x.Item1 == _name).Item2;
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



