public class Formatter
{
    const int INDENT_SIZE = 4;

    private int _indent;
    private string _text;
    private string[] _lines;
    private int _line;
    private int _p;
    private bool _arrowIndent;
    private StreamWriter _writer;

    private Formatter(string text, StreamWriter writer)
    {
        _indent = 0;
        _line = -1;
        _text = text;
        _writer = writer;
        _p = 0;
        _lines = _text.Replace("\r", "").Split("\n");
        _arrowIndent = false;
    }

    public static List<TextEdit> TextEdits(string text, StreamWriter writer)
    {
        var formatter = new Formatter(text, writer);
        return formatter.Format();
    }

    private List<TextEdit> Format()
    {
        var textEdits = new List<TextEdit>();
        var lines = _text.Replace("\r", "").Split("\n");


        while (++_line < _lines.Length)
        {
            _p = 0;
            ConsumeWhiteSpace();
            if (AtEndOfLine())
            {
                // Remove whitespace if the line is empty
                if (_p > 0)
                    textEdits.Add(TextEdit(_line, 0, _p, ""));

                continue;
            }

            if (CurrentChar() == '}' || CurrentChar() == ']')
                DecreaseIndent();

            var indent = _arrowIndent ? _indent + INDENT_SIZE : _indent;
            if (indent != _p)
                textEdits.Add(TextEdit(_line, 0, _p, new string(' ', indent)));
            var firstChar = _p;

            while (!AtEndOfLine())
            {
                var textEdit = CurrentChar() switch
                {
                    '{' => OpenBrace(),
                    '}' => ClosingBrace(firstChar == _p),
                    '[' => OpenBracket(),
                    ']' => ClosingBracket(),
                    ';' => Semicolon(),
                    ' ' => WhiteSpace(),
                    ',' => Comma(),
                    '-' => Arrow(),
                    _ => Advance()
                };
                if (textEdit is not null)
                    textEdits.Add(textEdit);
            }

        }
        foreach (var te in textEdits)
            _writer.Log($"TextEdits {te}");

        return textEdits;
    }

    private TextEdit? Arrow()
    {
        _p++;
        if (!AtEndOfLine() && CurrentChar() == '>')
        {
            _arrowIndent = true;
            _p++;
            var start = _p;
            ConsumeWhiteSpace();
            var end = _p;
            if (start != end)
                return TextEdit(_line, start, end, NewLine());
        }
        return null;
    }

    private TextEdit? OpenBracket()
    {
        _p++;
        IncreaseIndent();
        return null;
    }

    private TextEdit? ClosingBracket()
    {
        _p++;
        DecreaseIndent();
        return null;
    }

    private TextEdit? OpenBrace()
    {
        IncreaseIndent();
        _p++;
        if (AtEndOfLine())
            return null;
        var start = _p;
        ConsumeWhiteSpace();
        return TextEdit(_line, start, _p, NewLine());
    }
    private TextEdit? ClosingBrace(bool isFirstChar)
    {
        _writer.Log($"closing brace isFirstchar {isFirstChar}");
        var start = _p;
        _p++;
        if (isFirstChar) return null;
        DecreaseIndent();
        if (AtEndOfLine()) return null;
        return TextEdit(_line, start, start, NewLine());
    }

    private TextEdit? Semicolon()
    {
        _p++;
        _arrowIndent = false;
        if (AtEndOfLine())
            return null;
        var start = _p;
        ConsumeWhiteSpace();
        if (start != _p)
            return TextEdit(_line, start, _p, AtEndOfLine() ? "" : NewLine());
        return null;
    }

    private TextEdit? WhiteSpace()
    {
        _p++;
        if (AtEndOfLine())
            return null;
        if (CurrentChar() != ' ')
            return null;
        var start = _p;
        ConsumeWhiteSpace();
        return TextEdit(_line, start, _p, "");
    }

    private TextEdit? Comma()
    {
        _p++;
        if (AtEndOfLine())
            return null;
        if (CurrentChar() != ' ')
            return TextEdit(_line, _p, _p, " ");
        return null;
    }


    private void ConsumeWhiteSpace()
    {
        while (_p < _lines[_line].Length && _lines[_line][_p] == ' ')
        {
            _p++;
        }
    }
    private void IncreaseIndent() => _indent += INDENT_SIZE;
    private void DecreaseIndent() => _indent = Math.Max(_indent - INDENT_SIZE, 0);
    private bool AtEndOfLine() => _p >= _lines[_line].Length;
    private char CurrentChar() => _lines[_line][_p];
    private string NewLine() => $"{Environment.NewLine}{new string(' ', _indent)}";

    private TextEdit? Advance()
    {
        _p++;
        return null;
    }

    private TextEdit TextEdit(int line, int start, int end, string text) => new(new(new(line, start), new(line, end)), text);
}
