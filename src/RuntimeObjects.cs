public record class RuntimeFunc(List<string> parameters, List<IAstNode> stmts, Dictionary<string, object?> scope)
{
    public override string? ToString()
    {
        return "<function>";
    }
}

public record class RuntimeList(List<object?> list)
{
    public override string ToString()
    {
        return $"[{string.Join(", ", list)}]";
    }
}
public record class RuntimeObject(Dictionary<string, object?> obj)
{
    public override string ToString()
    {
        return $$"""{ {{string.Join(", ", obj)}} }""";
    }
}
