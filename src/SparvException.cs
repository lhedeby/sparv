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

    public override string ToString()
    {
        return $"{Message}";
    }
}
