public class Analyzer
{
    private Dictionary<string, List<string>> _functions;
    private List<List<string>> _variables;
    private List<SparvException> _errors;
    private IAstNode _root;
    public Analyzer(IAstNode root)
    {
        _root = root;
        _variables = new();
        _variables.Add(new());
        _errors = new();
        _functions = new();
    }

    public void Run()
    {
        _root.Analyze(this);
    }

    // TODO
    public List<SparvException> Errors { get => _errors; }
    public Dictionary<string, List<string>> Functions => _functions;

    public bool HasErrors { get => _errors.Count > 0; }
    public void AddError(SparvException e) => _errors.Add(e);
    public void AddVar(string identifier) 
    {
        _variables.Last().Add(identifier);
    }
    public bool VarExistsInCurrentScope(string identifier) => _variables.Last().Contains(identifier);

    // TODO: How to handle scopes?
    public void BeginScope() => _variables.Add(new());
    public void EndScope() => _variables.RemoveAt(_variables.Count - 1);

    public bool VarExists(string key) => Get(key);
    public List<string> Vars =>
        _variables
            .SelectMany(l => l.Select(x => x))
            .ToList();

    private bool Get(string s)
    {
        for (int i = _variables.Count - 1; i >= 0; i--)
        {
            if (_variables[i].Contains(s))
                return true;
        }
        return false;
    }
}
