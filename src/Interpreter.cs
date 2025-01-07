public class Interpreter
{
    private List<Dictionary<string, object?>> _variables;
    private bool _hasReturned;
    private object? _returnValue;

    public Interpreter()
    {
        _variables = new();
        _variables.Add(new());
        _hasReturned = false;
        _returnValue = null;
    }

    public void SetReturnValue(object? v)
    {
        _returnValue = v;
        _hasReturned = true;
    }

    public void ResetReturn()
    {
        _hasReturned = false;
        _returnValue = null;
    }
    public bool HasReturned { get => _hasReturned; }
    public object? ReturnValue { get => _returnValue; }


    public void AddVar(string key, object? value)
    {
        _variables.Last().Add(key, value);
    }
    public object? Reassign(string key, object? value)
    {
        for (int i = _variables.Count - 1; i >= 0; i--)
        {
            if (_variables[i].ContainsKey(key))
            {
                _variables[i][key] = value;
                return value;
            }
        }
        // Analyzer should handle this and we should never reach this point
        throw new Exception($"TODO: Variable not found {key}");
    }
    public object? GetVar(string key)
    {
        for (int i = _variables.Count - 1; i >= 0; i--)
        {
            if (_variables[i].ContainsKey(key))
                return _variables[i][key];
        }
        throw new Exception($"Could not find variable '{key}'");
    }

    public Dictionary<string, object?> GetScope()
    {
        if (_variables.Count > 1)
            return _variables.Last();
        return new();
    }

    public void PrintVars()
    {
        var i = 1;
        foreach (var env in _variables)
        {
            Console.WriteLine($"==== {i++} ====");
            foreach (var variable in env)
            {
                Console.WriteLine($"key: {variable.Key}, value: {variable.Value}");
            }
        }
    }

    public void BeginScope()
    {
        _variables.Add(new());
    }
    public void EndScope()
    {
        _variables.RemoveAt(_variables.Count - 1);
    }
}
