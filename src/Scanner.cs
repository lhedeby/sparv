class Scanner
{
    private int _start;
    private int _current;
    private int _line;
    private int _col;
    private int _startCol;

    private Dictionary<string, TokenKind> _twoCharTokens;

    private string _source;


    private char CurrentChar()
    {
        return _source[_current];
    }

    public string Source => _source;

    public Scanner(string source)
    {
        _start = _current = _line = _col = _startCol = 0;
        _source = source.Replace("\r", "");

        _twoCharTokens = new()
        {
            ["!="] = TokenKind.BangEqual,
            ["=="] = TokenKind.EqualEqual,
            [">="] = TokenKind.GreaterEqual,
            ["<="] = TokenKind.LessEqual,
            ["->"] = TokenKind.Arrow,
            ["+="] = TokenKind.PlusEqual,
            ["-="] = TokenKind.MinusEqual,
        };
    }

    /// <summary>
    /// Parses all the text in the Scanner to tokens.
    /// </summary>
    /// <returns>
    /// A IEnumerable of the tokens representing the file content. The last token is
    /// always an 'EndOfFile' token. 
    /// </returns>
    public IEnumerable<Token> Tokens()
    {
        while (!IsAtEnd())
            yield return NextToken();
    }

    private Token MakeToken(TokenKind kind) =>
        new Token(
            kind,
            _source[_start.._current],
            _line,
            _startCol,
            _startCol + (_current - _start)
        );

    private Token NextToken()
    {
        SkipWhitespace();
        _start = _current;
        _startCol = _col;
        if (IsAtEnd())
            return MakeToken(TokenKind.EndOfFile);
        var c = Advance();
        return c switch
        {
            >= 'a' and <= 'z' or >= 'A' and <= 'Z' => Identifier(),
            >= '0' and <= '9' => Number(),
            '(' => MakeToken(TokenKind.LeftParen),
            ')' => MakeToken(TokenKind.RightParen),
            '[' => MakeToken(TokenKind.LeftBracket),
            ']' => MakeToken(TokenKind.RightBracket),
            '{' => MakeToken(TokenKind.LeftBrace),
            '}' => MakeToken(TokenKind.RightBrace),
            ';' => MakeToken(TokenKind.Semicolon),
            ',' => MakeToken(TokenKind.Comma),
            '.' => MakeToken(TokenKind.Dot),
            '/' => MakeToken(Comment()),
            '*' => MakeToken(TokenKind.Star),
            ':' => MakeToken(TokenKind.Colon),
            '%' => MakeToken(TokenKind.Percent),
            '|' => MakeToken(TokenKind.Pipe),
            '-' => MakeToken(TwoCharTokens(c, TokenKind.Minus)),
            '+' => MakeToken(TwoCharTokens(c, TokenKind.Plus)),
            '!' => MakeToken(TwoCharTokens(c, TokenKind.Bang)),
            '=' => MakeToken(TwoCharTokens(c, TokenKind.Equal)),
            '<' => MakeToken(TwoCharTokens(c, TokenKind.Less)),
            '>' => MakeToken(TwoCharTokens(c, TokenKind.Greater)),
            '"' => String(),
            _ => throw new SparvException($"Unexpected char '{c}'", _line, _startCol, _current),
        };
    }

    private TokenKind Comment()
    {
        if (CurrentChar() != '/')
            return TokenKind.Slash;
        while (!IsAtEnd() && CurrentChar() != '\n') Advance();
        return TokenKind.Comment;
    }

    private TokenKind TwoCharTokens(char first, TokenKind single)
    {
        var second = CurrentChar();
        var s = $"{first}{second}";
        if (_twoCharTokens.ContainsKey(s))
        {
            Advance();
            return _twoCharTokens[s];
        }
        return single;
    }

    private TokenKind Keywords(string s)
    {
        return s switch
        {
            "var" => TokenKind.Var,
            "true" => TokenKind.True,
            "and" => TokenKind.And,
            "else" => TokenKind.Else,
            "if" => TokenKind.If,
            "nil" => TokenKind.Nil,
            "or" => TokenKind.Or,
            "return" => TokenKind.Return,
            "while" => TokenKind.While,
            "false" => TokenKind.False,
            "for" => TokenKind.For,
            "loop" => TokenKind.Loop,
            "fun" => TokenKind.Fun,
            "in" => TokenKind.In,
            "import" => TokenKind.Import,
            "match" => TokenKind.Match,
            _ => TokenKind.Identifier
        };
    }

    private Token Identifier()
    {
        while (!IsAtEnd())
        {
            var c = CurrentChar();
            if (!char.IsAsciiLetter(c) && !char.IsNumber(c) && c != '_')
            {
                break;
            }
            Advance();
        }
        var kind = Keywords(_source[_start.._current]);
        return MakeToken(kind);
    }
    private Token Number()
    {
        while (!IsAtEnd())
        {
            var c = CurrentChar();
            if (!char.IsNumber(c) && c != '_' && c != '.')
            {
                break;
            }
            Advance();
        }
        return MakeToken(TokenKind.Number);
    }
    private Token String()
    {
        while (!IsAtEnd() && Advance() != '"') ;
        if (IsAtEnd()) throw new SparvException("Unterminated string", _line, _startCol, _col);
        return MakeToken(TokenKind.String);
    }

    private bool IsAtEnd() => _current >= _source.Length;

    private void SkipWhitespace()
    {
        while (!IsAtEnd())
        {
            var c = CurrentChar();
            if (c == '\n')
            {
                _line += 1;
                _col = 0;
                _current += 1;
            }
            else if (char.IsWhiteSpace(c))
            {
                Advance();
            }
            else
            {
                break;
            }
        }
    }

    private char Advance()
    {
        var c = CurrentChar();
        _current += 1;
        _col += 1;
        return c;
    }

}

public record Token(TokenKind Kind, string Value, int Line, int Start, int End);

public enum TokenKind
{
    // Single-character tokens.
    /// <summary> TokenKind: ( </summary>
    LeftParen,
    /// <summary> TokenKind: ) </summary>
    RightParen,
    /// <summary> TokenKind: [ </summary>
    LeftBracket,
    /// <summary> TokenKind: ] </summary>
    RightBracket,
    /// <summary> TokenKind: { </summary>
    LeftBrace,
    /// <summary> TokenKind: } </summary>
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Slash,
    Star,
    Colon,
    Percent,
    Pipe,
    // One or two character tokens.
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,
    Arrow,
    PlusEqual,
    MinusEqual,
    // Literals.
    Identifier,
    String,
    Number,
    // Keywords.
    Var,
    And,
    Else,
    False,
    For,
    Loop,
    Fun,
    If,
    Nil,
    Or,
    Return,
    True,
    In,
    While,
    Import,
    Match,
    EndOfFile,
    // Comments
    Comment,
}
