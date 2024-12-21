public class Parser
{
    private int _p;
    private List<Token> _tokens;
    private string _source;
    public string Source => _source;
    public List<SparvException> Errors { get; set; }
    public bool HasErrors { get => Errors.Count > 0; }
    public Analyzer? Analyzer { get; set; }
    public Parser(string source)
    {
        var scanner = new Scanner(source);
        _source = source;
        _tokens = scanner.Tokens().ToList();
        _p = 0;
        Errors = new();
    }


    public IAstNode Parse()
    {
        _tokens = _tokens.Where(x => x.Kind != TokenKind.Comment).ToList();
        var decls = new List<IAstNode>();
        try
        {
            while (CurrentTokenKind() != TokenKind.EndOfFile)
            {
                var decl = ParseDecl();
                decls.Add(decl);
            }
        }
        catch (SparvException se)
        {
            Errors.Add(se);
        }
        catch (Exception e)
        {
            Errors.Add(new SparvException($"Non sparv exception: {e.Message}", 0, 0, 0));
        }
        IAstNode root = new Root(decls);

        Analyzer = new Analyzer(root);
        Analyzer.Run();
        Errors.AddRange(Analyzer.Errors);
        return root;
    }

    private IAstNode ParseDecl() => CurrentTokenKind() switch
    {
        TokenKind.Fun => FunctionDecl(),
        TokenKind.Import => Import(),
        _ => ParseStmt()
    };

    private IAstNode ParseStmt() => CurrentTokenKind() switch
    {
        TokenKind.While => While(),
        TokenKind.If => If(),
        TokenKind.Return => Return(),
        TokenKind.Var => Var(),
        // TODO: Should this be here?
        // TokenKind.Identifier => Identifier(),
        TokenKind.For => For(),
        _ => Expr()
    };

    private IAstNode FunctionDecl()
    {
        Consume(TokenKind.Fun);
        var name = Advance().Value;
        var func = Function();
        return new Var(name, func);
    }

    private IAstNode Function()
    {
        Consume(TokenKind.LeftParen);

        var parameters = new List<string>();
        if (CurrentTokenKind() != TokenKind.RightParen)
        {
            parameters.Add(CurrentToken().Value);
            Consume(TokenKind.Identifier);
            while (Advance().Kind == TokenKind.Comma)
            {
                parameters.Add(CurrentToken().Value);
                Consume(TokenKind.Identifier);
            }
        }
        else
        {
            Consume(TokenKind.RightParen);
        }

        var stmts = new List<IAstNode>();

        if (CurrentTokenKind() == TokenKind.LeftBrace)
        {
            Consume(TokenKind.LeftBrace);
            while (CurrentTokenKind() != TokenKind.RightBrace)
                stmts.Add(ParseStmt());
            Consume(TokenKind.RightBrace);
        }
        else
        {
            stmts.Add(new Return(ParseExpr(0)));
        }

        return new Fun(parameters, stmts);

    }
    private IAstNode Import()
    {
        return default;
    }

    /*
     * Statements
    */

    private IAstNode While()
    {
        Consume(TokenKind.While);
        var expr = ParseExpr(0);
        Consume(TokenKind.LeftBrace);
        var stmts = new List<IAstNode>();
        while (CurrentTokenKind() != TokenKind.RightBrace)
            stmts.Add(ParseStmt());
        Consume(TokenKind.RightBrace);
        return new While(expr, stmts);
    }
    private IAstNode If()
    {
        Consume(TokenKind.If);
        var expr = ParseExpr(0);
        Consume(TokenKind.LeftBrace);

        var ifStmts = new List<IAstNode>();
        while (CurrentTokenKind() != TokenKind.RightBrace)
            ifStmts.Add(ParseStmt());
        Consume(TokenKind.RightBrace);

        var elseStmts = new List<IAstNode>();
        if (CurrentTokenKind() == TokenKind.Else)
        {
            Consume(TokenKind.Else);
            Consume(TokenKind.LeftBrace);
            while (CurrentTokenKind() != TokenKind.RightBrace)
                elseStmts.Add(ParseStmt());
            Consume(TokenKind.RightBrace);
        }
        return new If(expr, ifStmts, elseStmts);
    }
    private IAstNode Return()
    {
        Consume(TokenKind.Return);
        return new Return(Expr());
    }
    private IAstNode Var()
    {
        Consume(TokenKind.Var);
        var identifier = Advance();
        Consume(TokenKind.Equal);
        var expr = Expr();
        return new Var(identifier.Value, expr);

    }
    private IAstNode Identifier(Token token)
    {
        IAstNode expr = new Variable(token);

        // if (lhs is Variable)
        // {
        //     var nativeFunction = NativeFunctions.Get(((Variable)lhs).Name, parameters);
        //     if (nativeFunction != null) return nativeFunction!;
        // }

        // for (; ; )
        // {
        //     var newExpr = CurrentTokenKind() switch
        //     {
        //         TokenKind.LeftParen => Call(token, expr),
        //         _ => null
        //     };
        //     if (newExpr is null) break;
        //     expr = newExpr;
        // }
        return expr;
    }

    private IAstNode Grouping()
    {
        var expr = ParseExpr(0);
        Consume(TokenKind.RightParen);
        return expr;
    }


    private IAstNode For()
    {
        Consume(TokenKind.For);
        var identifier = CurrentToken();
        Consume(TokenKind.Identifier);
        Consume(TokenKind.In);
        var expr = ParseExpr(0);

        Consume(TokenKind.LeftBrace);
        var stmts = new List<IAstNode>();
        while (CurrentTokenKind() != TokenKind.RightBrace)
            stmts.Add(ParseStmt());
        Consume(TokenKind.RightBrace);

        return new For(identifier.Value, expr, stmts);
    }

    /*
     * Expressions
    */

    private IAstNode Expr()
    {
        var node = ParseExpr(0);
        Consume(TokenKind.Semicolon);
        return node;
    }

    private IAstNode ParseExpr(int precedence)
    {
        var token = Advance();
        var lhs = ParsePrefix(token);

        while (precedence < InfixPrecedence(CurrentTokenKind()))
        {
            var innerToken = Advance();
            lhs = ParseInfix(lhs, innerToken);
        }
        return lhs;
    }

    private IAstNode ParseInfix(IAstNode lhs, Token token)
    {
        if (token.Kind == TokenKind.LeftParen) return Call(lhs, token);
        if (token.Kind == TokenKind.LeftBracket) return Index(lhs);
        if (token.Kind == TokenKind.Dot) return GetOrSet(lhs);
        var rhs = ParseExpr(InfixPrecedence(token.Kind));
        return token.Kind switch
        {
            TokenKind.Or => new Or(lhs, rhs),
            TokenKind.And => new And(lhs, rhs),
            TokenKind.BangEqual => new NotEqual(lhs, rhs),
            TokenKind.EqualEqual => new Equal(lhs, rhs),
            TokenKind.Greater => new Greater(lhs, rhs),
            TokenKind.GreaterEqual => new GreaterEqual(lhs, rhs),
            TokenKind.Less => new Less(lhs, rhs),
            TokenKind.LessEqual => new LessEqual(lhs, rhs),
            TokenKind.Star => new Multiply(lhs, rhs),
            TokenKind.Slash => new Divide(lhs, rhs),
            TokenKind.Percent => new Modulo(lhs, rhs),
            TokenKind.Plus => new Add(lhs, rhs),
            TokenKind.Minus => new Subtract(lhs, rhs),
            TokenKind.Equal => new Reassign(lhs, rhs),
            TokenKind.PlusEqual => new ReassignPlus(lhs, rhs),
            TokenKind.MinusEqual => new ReassignMinus(lhs, rhs),
            TokenKind.Colon => new RangeList(lhs, rhs),
            // TokenKind.Dot => GetOrSet(lhs, rhs),
            // TODO: Should we just parse arrows as double call?
            TokenKind.Arrow => ArrowCall(lhs, rhs, token),
            _ => throw new Exception("todo")
        };
    }

    private IAstNode GetOrSet(IAstNode lhs)
    {
        var identifier = CurrentToken().Value;
        Consume(TokenKind.Identifier);
        if (CurrentTokenKind() == TokenKind.Equal)
        {
            Consume(TokenKind.Equal);
            return new Set(lhs, identifier, ParseExpr(0));
        }
        else
            return new Get(lhs, identifier);
    }

    private IAstNode Index(IAstNode lhs)
    {
        var rhs = ParseExpr(0);
        Consume(TokenKind.RightBracket);
        return new Index(lhs, rhs);
    }

    private IAstNode ArrowCall(IAstNode lhs, IAstNode rhs, Token token)
    {
        var parameters = new List<IAstNode> { lhs };


        if (NativeFunctions.Get(rhs, parameters) is IAstNode native)
            return native;
        return new Call(parameters, rhs, token);
    }

    private IAstNode Call(IAstNode lhs, Token token)
    {
        var parameters = new List<IAstNode>();
        while (CurrentTokenKind() != TokenKind.RightParen)
        {
            parameters.Add(ParseExpr(0));
            if (CurrentTokenKind() != TokenKind.RightParen)
                Consume(TokenKind.Comma);
        }
        Consume(TokenKind.RightParen);

        if (NativeFunctions.Get(lhs, parameters) is IAstNode native)
            return native;

        return new Call(parameters, lhs, token);
    }

    private IAstNode ParsePrefix(Token token)
    {
        return token.Kind switch
        {
            TokenKind.Minus => new PrefixMinus(ParseExpr(PrefixPrecedence())),
            TokenKind.Bang => new PrefixBang(ParseExpr(PrefixPrecedence())),
            TokenKind.LeftBracket => List(),
            TokenKind.Identifier => Identifier(token),
            TokenKind.Number => Number(token),
            TokenKind.String => String(token),
            TokenKind.True => new True(),
            TokenKind.False => new False(),
            TokenKind.Nil => new Nil(),
            TokenKind.Fun => Function(),
            TokenKind.LeftParen => Grouping(),
            TokenKind.LeftBrace => Obj(),
            _ => throw new SparvException($"Unexcepted token '{token.Kind}'", token.Line, token.Start, token.End)
        };
    }

    private int InfixPrecedence(TokenKind kind) => kind switch
    {
        TokenKind.Equal or TokenKind.PlusEqual or TokenKind.MinusEqual => 1,
        TokenKind.Arrow => 2,
        TokenKind.Or => 3,
        TokenKind.And => 4,
        TokenKind.BangEqual or TokenKind.EqualEqual => 5,
        TokenKind.Greater or TokenKind.GreaterEqual or TokenKind.Less or TokenKind.LessEqual => 6,
        TokenKind.Plus or TokenKind.Minus => 7,
        TokenKind.Star or TokenKind.Slash or TokenKind.Percent => 8,
        TokenKind.Colon => 8, // was 10
        TokenKind.LeftParen or TokenKind.LeftBracket => 10,
        TokenKind.Dot => 10,
        _ => 0,
    };

    private int PrefixPrecedence() => 9;

    private IAstNode Obj()
    {
        var dict = new Dictionary<string, IAstNode>();
        while (CurrentTokenKind() != TokenKind.RightBrace)
        {
            var identifier = CurrentToken();
            Consume(TokenKind.Identifier);
            Consume(TokenKind.Colon);
            var expr = ParseExpr(0);
            dict.Add(identifier.Value, expr);
            if (CurrentTokenKind() != TokenKind.RightBrace)
                Consume(TokenKind.Comma);
        }
        Consume(TokenKind.RightBrace);
        return new Obj(dict);

    }

    private IAstNode List()
    {
        var list = new List<IAstNode>();
        if (CurrentTokenKind() != TokenKind.RightBracket)
        {
            list.Add(ParseExpr(0));
            while (Advance().Kind == TokenKind.Comma)
            {
                list.Add(ParseExpr(0));
            }
        }
        else
        {
            Consume(TokenKind.RightBracket);
        }
        return new ListNode(list);
    }

    private IAstNode Number(Token token)
    {
        return new Number(token);
    }
    private IAstNode String(Token token)
    {
        return new StringNode(token);
    }


    /*
     * Helpers
    */
    private TokenKind CurrentTokenKind() => _tokens[_p].Kind;
    private Token CurrentToken() => _tokens[_p];
    private Token Advance() => _tokens[_p++];
    private TokenKind Consume(TokenKind kindToConsume)
    {
        var token = Advance();
        if (token.Kind != kindToConsume)
            throw new SparvException(
                $"Expected '{kindToConsume}' after this",
                _tokens[_p - 2].Line,
                _tokens[_p - 2].Start,
                _tokens[_p - 2].End
            );
        return token.Kind;
    }
}



