public interface IAstNode
{
    object? Interpret(Interpreter inter);
    AnalyzerKind Analyze(Analyzer a);
}

public static class Extensions
{
    public static string ListToString(this List<IAstNode> list)
    {
        return $"[{string.Join(',', list)}]";
    }

    public static void Run(this List<IAstNode> list, Interpreter inter)
    {
        foreach (var stmt in list)
        {
            if (inter.HasReturned)
                return;
            stmt.Interpret(inter);
        }
    }
}



