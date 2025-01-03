public class Analyzer
{
    private List<Dictionary<string, AnalyzerKind>> _variables;
    private List<SparvException> _errors;
    private IAstNode _root;
    public Analyzer(IAstNode root)
    {
        _root = root;
        _variables = new();
        _variables.Add(new());
        _errors = new();
    }

    public void Run()
    {
        _root.Analyze(this);
    }

    public List<SparvException> Errors { get => _errors; }
    
    public bool HasErrors { get => _errors.Count > 0; }
    public void AddError(SparvException e) => _errors.Add(e);
    public void AddVar(string s, AnalyzerKind kind) => _variables.Last().Add(s, kind);

    // TODO: How to handle scopes?
    public void BeginScope() => _variables.Add(new());
    public void EndScope() => _variables.RemoveAt(_variables.Count - 1);

    public bool VarExists(string key) => Get(key) is not null;
    public List<(string, AnalyzerKind)> Vars =>
        _variables
            .SelectMany(dict => 
                (dict.Select(kvp => (kvp.Key, kvp.Value))))
            .ToList();

    private AnalyzerKind? Get(string key)
    {
        for (int i = _variables.Count - 1; i >= 0; i--)
        {
            if (_variables[i].ContainsKey(key))
                return _variables[i][key];
        }
        return null;
    }
}

public enum AnalyzerKind
{
    Number,
    String,
    Bool,
    List,
    Object,
    Nil,
    Function
}
