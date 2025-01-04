public class State()
{
    public Dictionary<string, TextDocumentItem> Documents { get; private set; } = new();
    public List<string> Suggestions { get; set; } = new();
    public Dictionary<string, List<string>> Functions { get; set; } = new();
    public List<Token> Tokens { get; set; } = new();
    public bool HasChanged { get; set; } = true;
}
