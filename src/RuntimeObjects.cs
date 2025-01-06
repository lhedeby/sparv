public record class RuntimeFunc(List<string> Parameters, List<IAstNode> Stmts, Dictionary<string, object?> Closure)
{
    public override string? ToString()
    {
        return "<function>";
    }
}

public record class RuntimeList(List<object?> List)
{
    public override string ToString()
    {
        return $"[{string.Join(", ", List)}]";
    }
}
public record class RuntimeObject(Dictionary<string, object?> Obj)
{
    public override string ToString()
    {
        return $$"""{ {{string.Join(", ", Obj)}} }""";
    }
}
