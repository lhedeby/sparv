using System.Text.Json;

enum Lifecyle
{
    Running,
    ShuttingDown,
    Off
}

public static class ExtensionsMethods
{
    public static void Log(this StreamWriter writer, string? msg)
    {
        writer.WriteLine($"{DateTime.Now.ToString()}: {msg}");
        writer.Flush();
    }
}

public class LspServer
{
    const string CONTENT_LENGTH = "Content-Length: ";

    private StreamWriter _logger;
    private State _state;
    private Lifecyle _lifecycle;
    private CancellationTokenSource _cts;

    private void Log(string? msg)
    {
        _logger.WriteLine($"{DateTime.Now.ToString()}: {msg}");
        _logger.Flush();
    }
    public LspServer()
    {
        _logger = new StreamWriter("./log");
        _state = new();
        _lifecycle = Lifecyle.Running;
        _cts = new();
        Log("server created");
        var test = DateTime.Now;
    }

    public async Task Start()
    {
        Log("Lsp server started");
        var stdin = Console.OpenStandardInput();
        var stdout = Console.OpenStandardOutput();

        using var reader = new StreamReader(stdin);
        using var writer = new StreamWriter(stdout);

        await Task.WhenAll(Listen(reader, writer));
        Log("Exited gracefully.");
    }

    private async void Diagnostics(StreamWriter writer)
    {
        if (_lifecycle == Lifecyle.Running)
        {
            try
            {
                _cts.Cancel();
                _cts = new CancellationTokenSource();
                await Task.Delay(500, _cts.Token);
                if (_cts.IsCancellationRequested)
                    return;
                _state.HasChanged = false;
                var pdp = new List<PublishDiagnosticsParams>();
                lock (_state)
                {
                    Log($"there is {_state.Documents.Count} documents");
                    foreach (var (uri, doc) in _state.Documents)
                    {
                        var parser = new Parser(doc.Text);
                        parser.Parse();
                        _state.Tokens = parser.Tokens;
                        _state.Suggestions = parser.Analyzer!.Vars;
                        _state.Functions = parser.Analyzer.Functions;
                        Log($"Document {uri} has {parser.Errors.Count} errors");
                        var diagnostics = parser.Errors.Select(e =>
                            new Diagnostic(
                                new Range(new(e.Line, e.Start), new(e.Line, e.End)),
                                DiagnosticSeverity.Error,
                                "sparv-lsp",
                                $"{e}")
                        ).ToArray();
                        pdp.Add(new PublishDiagnosticsParams(uri, null, diagnostics));

                    }
                }

                foreach (var d in pdp)
                {
                    await Send(writer, "textDocument/publishDiagnostics", d);
                }
            }
            catch (TaskCanceledException) {}
            catch (Exception e)
            {
                Log($"Diagnostics exception: {e.Message}");
                Log($"{e.StackTrace}");
            }

        }
    }

    private async Task Listen(StreamReader reader, StreamWriter writer)
    {
        while (_lifecycle != Lifecyle.Off)
        {
            var line = await reader.ReadLineAsync();
            if (!line!.StartsWith(CONTENT_LENGTH))
                Log($"Unknown header: {line}");
            var length = int.Parse(line[CONTENT_LENGTH.Length..]);
            await reader.ReadLineAsync();
            var buf = new char[length];
            await reader.ReadBlockAsync(buf, 0, length);
            Log($"msg: {new string(buf)}");
            try
            {
                var msg = Json.Deserialize<RequestMessage>(buf)!;
                Log($"method: {msg.Method}");
                Log($"body: {msg.Body}");

                var response = msg.Method switch
                {
                    "initialize" => Handle<InitializeParams>(msg.Body),
                    "textDocument/didOpen" => Handle<DidOpenTextDocumentParams>(msg.Body),
                    "textDocument/didChange" => Handle<DidChangeTextDocumentParams>(msg.Body),
                    "textDocument/formatting" => Handle<DocumentFormattingParams>(msg.Body),
                    "textDocument/semanticTokens/full" => Handle<SemanticTokensParams>(msg.Body),
                    "textDocument/hover" => Handle<HoverParams>(msg.Body),
                    "textDocument/completion" => Handle<CompletionParams>(msg.Body),
                    _ => null
                };
                if (msg.Method == "shutdown")
                    await Send<object?>(writer, msg.Id, null);

                if (msg.Method == "exit")
                    _lifecycle = Lifecyle.Off;

                if (response is not null)
                {
                    await Send(writer, msg.Id, response);
                }

            }
            catch (Exception e)
            {

                Log($"ERROR: Could not decode message: {e.Message}");
                Log($"{e.StackTrace}");
            }
            if (_state.HasChanged)
            {
                Diagnostics(writer);
                _state.HasChanged = false;
            }
        }
        Log("main thread died");
    }

    private object? Handle<T>(JsonElement request) where T : IClientRequest
    {
        T? body = request.Deserialize<T>(Json.Options);
        if (body is null)
        {
            Log("TODO: Nice error message");
            return null;
        }
        return body.Handle(_state, _logger);
    }

    private async Task Send<T>(StreamWriter writer, int id, T result)
    {
        var responseMessage = new ResponseMessage<T>("2.0", id, result);
        var text = Json.Serialize(responseMessage);
        Log($"Response {text}");

        var encodedMessage = $"{CONTENT_LENGTH}{text.Length}\r\n\r\n{text}";
        await writer.WriteAsync(encodedMessage);
        await writer.FlushAsync();
    }

    private async Task Send<T>(StreamWriter writer, string method, T result)
    {
        var responseMessage = new NotificationMessage<T>("2.0", method, result);
        var text = Json.Serialize(responseMessage);
        Log($"Notification {text}");

        var encodedMessage = $"{CONTENT_LENGTH}{text.Length}\r\n\r\n{text}";
        await writer.WriteAsync(encodedMessage);
        await writer.FlushAsync();
    }
}
