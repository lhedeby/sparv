public class SparvException : Exception
{
    public int Line { get; set; }
    public int Start { get; set; }
    public int End { get; set; }

    public SparvException(string message, int line, int start, int end) : base(message)
    {
        Line = line;
        Start = start;
        End = end;
    }
    public SparvException(string message, Token token) : base(message)
    {
        Line = token.Line;
        Start = token.Start;
        End = token.End;
    }

    public override string ToString()
    {
        return $"{Message}";
    }

    public void PrintError(string source)
    {
        var lines = source.Split('\n');
        var maxLen = Line.ToString().Length + 2;
        Console.WriteLine($"{(Line - 1).ToString().PadRight(maxLen)}| ...");
        Console.WriteLine($"{"".PadLeft(maxLen)}|");
        Console.WriteLine($"{Line.ToString().PadRight(maxLen)}| {lines[Line]}");
        Console.WriteLine($"{"".PadRight(maxLen)}|{"".PadLeft(Start + 1)}{"".PadRight(End - Start, '^')}");
        Console.WriteLine($"{(Line + 1).ToString().PadRight(maxLen)}| ...");
        Console.WriteLine($">>> {Message} at {Line}:{Start}");
    }
}
