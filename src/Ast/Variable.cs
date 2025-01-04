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

    public void Analyze(Analyzer a)
    {
        if (!a.Vars.Contains(_name))
            a.AddError(new SparvException("Variable is not delcared", _token.Line, _token.Start, _token.End));
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



