public class NativeFunctions
{
    public static IAstNode? Get(IAstNode node, List<IAstNode> parameters)
    {
        if (node is not Variable v)
            return null;
        return v.Name switch
        {
            "print" => new Print(parameters, v.Token),
            "len" => new Len(parameters, v.Token),
            "typeof" => new Typeof(parameters, v.Token),
            "read_file" => new ReadFile(parameters, v.Token),
            "split" => new Split(parameters, v.Token),
            "parse" => new Parse(parameters, v.Token),
            "read_input" => new ReadInput(parameters, v.Token),
            "abs" => new Abs(parameters, v.Token),
            _ => null

        };
    }
}

public class Abs : IAstNode
{

    IAstNode _parameter;
    Token _token;
    public Abs(List<IAstNode> parameters, Token token)
    {
        if (parameters.Count != 1)
            throw new SparvException("abs() takes 1 argument", token);

        _parameter = parameters.First();
        _token = token;
    }

    public void Analyze(Analyzer a)
    {
        _parameter.Analyze(a);
    }

    public object? Interpret(Interpreter inter)
    {
        return _parameter.Interpret(inter) switch
        {
            double s => double.Abs(s),
            int s => int.Abs(s),
            _ => throw new SparvException("Argument must be a number", _token)
        };
    }
}

public class ReadInput : IAstNode
{
    public ReadInput(List<IAstNode> parameters, Token token)
    {

        if (parameters.Count > 0)
            throw new SparvException("read_input() takes no arguments", token);
    }

    public void Analyze(Analyzer a)
    {
    }

    public object? Interpret(Interpreter inter)
    {
        return Console.ReadLine();
    }
}

public class Parse : IAstNode
{
    IAstNode _parameter;
    Token _token;
    public Parse(List<IAstNode> parameters, Token token)
    {
        if (parameters.Count != 1)
            throw new SparvException("parse() only take 1 parameter", token.Line, token.Start, token.End);
        _parameter = parameters.First();
        _token = token;
    }

    public void Analyze(Analyzer a)
    {
        _parameter.Analyze(a);
    }

    public object? Interpret(Interpreter inter)
    {
        return _parameter.Interpret(inter) switch
        {
            string s => double.Parse(s),
            _ => throw new SparvException("Argument must be a number", _token)
        };
    }

}

public class Split : IAstNode
{
    List<IAstNode> _parameters;
    public Split(List<IAstNode> parameters, Token token)
    {
        if (parameters.Count != 2)
            throw new SparvException("split() takes 2 parameters", token.Line, token.Start, token.End);
        _parameters = parameters;
    }

    public void Analyze(Analyzer a)
    {
        foreach (var p in _parameters)
            p.Analyze(a);
    }

    public object? Interpret(Interpreter inter)
    {
        var s = _parameters[0].Interpret(inter).ToString();
        var de = (_parameters[1].Interpret(inter) as string)
            .Replace("\\n", "\n")
            .Replace("\\r", "\r");

        return new RuntimeList(
                    s.Split(de, StringSplitOptions.RemoveEmptyEntries)
                    .Select(x => (object?)x)
                    .ToList());
    }
}


public class ReadFile : IAstNode
{
    IAstNode _parameter;
    Token _token;

    public ReadFile(List<IAstNode> parameters, Token token)
    {
        if (parameters.Count != 1)
            throw new SparvException("read_file() only take 1 parameter", token.Line, token.Start, token.End);
        _parameter = parameters.First();
        _token = token;
    }

    public void Analyze(Analyzer a)
    {
        _parameter.Analyze(a);
    }

    public object? Interpret(Interpreter inter)
    {
        if (_parameter.Interpret(inter) is not string path)
            throw new SparvException("Argument must be a string", _token);
        using var sr = new StreamReader(path);
        return sr.ReadToEnd();
    }
}

public class Print : IAstNode
{
    IAstNode _parameter;

    public Print(List<IAstNode> parameters, Token token)
    {
        if (parameters.Count != 1)
            throw new SparvException("print() only take 1 parameter", token.Line, token.Start, token.End);
        _parameter = parameters.First();
    }

    public void Analyze(Analyzer a)
    {
        _parameter.Analyze(a);
    }

    public object? Interpret(Interpreter inter)
    {
        var text = _parameter.Interpret(inter);
        Console.WriteLine($"{text ?? "nil"}");
        return text;
    }
}

public class Len : IAstNode
{
    IAstNode _parameter;
    Token _token;

    public Len(List<IAstNode> parameters, Token token)
    {
        if (parameters.Count != 1)
            throw new SparvException("len() only take 1 parameter", token.Line, token.Start, token.End);
        _parameter = parameters.First();
        _token = token;
    }

    public void Analyze(Analyzer a)
    {
        _parameter.Analyze(a);
    }

    public object? Interpret(Interpreter inter) => _parameter.Interpret(inter) switch
    {
        RuntimeList l => (double)l.List.Count,
        string s => (double)s.Length,
        RuntimeObject o => (double)o.Obj.Count,
        _ => throw new SparvException("Cant calculate length of argument", _token)
    };

    public override string? ToString()
    {
        return $"len({_parameter})";
    }
}

public class Typeof : IAstNode
{

    IAstNode _parameter;
    public Typeof(List<IAstNode> parameters, Token token)
    {
        if (parameters.Count != 1)
            throw new SparvException("typeof() only take 1 parameter", token.Line, token.Start, token.End);
        _parameter = parameters.First();
    }

    public void Analyze(Analyzer a)
    {
        _parameter.Analyze(a);
    }

    public object? Interpret(Interpreter inter) => _parameter.Interpret(inter) switch
    {
        double => "<number>",
        string => "<string>",
        RuntimeList l => "<list>",
        RuntimeFunc => "<function>",
        bool => "<bool>",
        _ => "<nil>"
    };

    public override string? ToString()
    {
        return $"len({_parameter})";
    }
}

