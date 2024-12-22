public class Formatter
{
    const int INDENT_SIZE = 4;

    private int _indent;
    private string _text;
    private string[] _lines;
    private int _line;
    private int _p;
    private bool _arrowIndent;
    private List<(int line, int p)> _pipes;
    private bool _isMatch;
    private List<TextEdit> _textEdits;
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
        _isMatch = false;
        _pipes = new();
        _textEdits = new();
    }

    public void AddTextEdit(int line, int start, int end, string text)
    {
        _textEdits.Add(TextEdit(line, start, end, text));
    }

    public static List<TextEdit> TextEdits(string text, StreamWriter writer)
    {
        var formatter = new Formatter(text, writer);
        return formatter.Format();
    }

    private List<TextEdit> Format()
    {
        // var textEdits = new List<TextEdit>();
        var lines = _text.Replace("\r", "").Split("\n");


        while (++_line < _lines.Length)
        {
            _p = 0;
            ConsumeWhiteSpace();
            if (AtEndOfLine())
            {
                // Remove whitespace if the line is empty
                if (_p > 0)
                    AddTextEdit(_line, 0, _p, "");
                // textEdits.Add(TextEdit(_line, 0, _p, ""));

                continue;
            }

            if (CurrentChar() == '}' || CurrentChar() == ']')
                DecreaseIndent();

            var indent = _arrowIndent ? _indent + INDENT_SIZE : _indent;
            if (indent != _p)
                AddTextEdit(_line, 0, _p, new string(' ', indent));
            // textEdits.Add(TextEdit(_line, 0, _p, new string(' ', indent)));
            var firstChar = _p;

            while (!AtEndOfLine())
            {
                _writer.Log($"p:{_p}");
                switch (CurrentChar())
                {
                    case '|': Pipe(); break;
                    case '{': OpenBrace(); break;
                    case '}': ClosingBrace(firstChar == _p); break;
                    case '[': OpenBracket(); break;
                    case ']': ClosingBracket(); break;
                    case ';': Semicolon(); break;
                    case ' ': WhiteSpace(); break;
                    case ',': Comma(); break;
                    case '-': Arrow(); break;
                    default: Advance(); break;
                };
                // if (textEdit is not null)
                //     textEdits.Add(textEdit);
            }

        }
        // foreach (var te in textEdits)
        //     _writer.Log($"TextEdits {te}");

        // return textEdits;
        return _textEdits;
    }


    private void Arrow()
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
                AddTextEdit(_line, start, end, NewLine());
        }
    }

    private void OpenBracket()
    {
        _p++;
        IncreaseIndent();
    }

    private void ClosingBracket()
    {
        _p++;
        DecreaseIndent();
    }

    private void ResolvePipes()
    {
        _isMatch = false;
        var pMax = 0;
        foreach (var pipe in _pipes)
        {
            pMax = Math.Max(pipe.p, pMax);
        }
        foreach (var pipe in _pipes)
        {
            if (pipe.p != pMax)
            {
                AddTextEdit(pipe.line, pipe.p, pipe.p, new string(' ', pMax-pipe.p));
                _writer.Log("adding a thing");
            }
        }
    }

    private void Pipe()
    {
        _isMatch = true;
        _pipes.Add((_line, _p));
        _p++;
    }

    private void OpenBrace()
    {
        IncreaseIndent();
        _p++;
        if (AtEndOfLine())
            return;
        var start = _p;
        ConsumeWhiteSpace();
        AddTextEdit(_line, start, _p, NewLine());
    }
    private void ClosingBrace(bool isFirstChar)
    {

        ResolvePipes();
        var start = _p;
        _p++;
        if (isFirstChar) return;
        DecreaseIndent();
        if (AtEndOfLine()) return;
        AddTextEdit(_line, start, start, NewLine());
    }

    private void Semicolon()
    {
        _p++;
        _arrowIndent = false;
        if (AtEndOfLine())
            return;
        var start = _p;
        ConsumeWhiteSpace();
        if (start != _p)
            AddTextEdit(_line, start, _p, AtEndOfLine() ? "" : NewLine());
    }

    private void WhiteSpace()
    {
        _p++;
        if (AtEndOfLine())
            return;
        if (CurrentChar() != ' ')
            return;
        var start = _p;
        ConsumeWhiteSpace();
        if (CurrentChar() == '|')
            return;
        AddTextEdit(_line, start, _p, "");
    }

    private void Comma()
    {
        _p++;
        if (AtEndOfLine())
            return;
        if (CurrentChar() != ' ')
            AddTextEdit(_line, _p, _p, " ");
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

    private void Advance()
    {
        _p++;
    }

    private TextEdit TextEdit(int line, int start, int end, string text) => new(new(new(line, start), new(line, end)), text);
}
