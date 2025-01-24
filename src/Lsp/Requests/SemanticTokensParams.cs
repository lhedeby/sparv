public record class SemanticTokensParams(TextDocumentIdentifier TextDocument) : IClientRequest
{
    public object? Handle(State state, StreamWriter writer)
    {
        var doc = state.Documents[TextDocument.Uri];
        var scanner = new Scanner(doc.Text);

        try
        {
            var tokens = scanner.Tokens().Where(token => token.Kind != TokenKind.EndOfFile).ToList();

            var res = new int[tokens.Count * 5];
            var previousLine = 0;
            var previousStart = 0;
            var i = 0;
            var j = 1;

            foreach (var token in tokens)
            {
                var type = TokenType(token.Kind, j < tokens.Count ? tokens[j].Kind : null);
                res[i++] = token.Line - previousLine;
                res[i++] = token.Start - (token.Line == previousLine ? previousStart : 0);
                res[i++] = token.End - token.Start;
                res[i++] = type;
                res[i++] = 0;

                previousLine = token.Line;
                previousStart = token.Start;
                j++;
            }

            return new SemanticTokens(res);
        }
        catch (SparvException) { }
        catch (Exception) { }
        return null;
    }



    private int TokenType(TokenKind kind, TokenKind? next) => kind switch
    {
        TokenKind.Identifier => next == TokenKind.LeftParen ? 2 : 1,
        TokenKind.Fun => 7,
        TokenKind.Comment => 3,
        TokenKind.String => 4,
        TokenKind.Number => 5,
        TokenKind.PlusEqual
        | TokenKind.Semicolon
        | TokenKind.Comma
        | TokenKind.Colon
        | TokenKind.Plus
        | TokenKind.Minus
        | TokenKind.Or
        | TokenKind.And
        | TokenKind.Dot
        | TokenKind.Star
        | TokenKind.Slash
        | TokenKind.Arrow
        | TokenKind.Less
        | TokenKind.LessEqual
        | TokenKind.Greater
        | TokenKind.GreaterEqual
        | TokenKind.EqualEqual => 6,
        _ => 7,
    };
}
