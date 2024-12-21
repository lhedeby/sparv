public class State()
{
    public Dictionary<string, TextDocumentItem> Documents { get; private set; } = new();
    public List<(string, AnalyzerKind)> Suggestions { get; set; } = new();
    public bool HasChanged { get; set; } = true;
}
