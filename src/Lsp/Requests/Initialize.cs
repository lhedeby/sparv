public record class InitializeParams(
    int? ProcessId,
    ClientInfo? ClientInfo,
    string? Locale,
    string? RootPath,
    ClientCapabilities Capabilities
// DocumentUri? RootUri,
// LSPAny? InitializationOptions,
// TraceValue? Trace,
// WorkspaceFolder[]? WorkspaceFolders
) : IClientRequest
{
    object? IClientRequest.Handle(State state, StreamWriter writer)
    {
        return new InitializeResult(
            new ServerCapabilities(
                new([">", "."], false),
                true,
                true,
                1,
                new SemanticTokensOptions(
                    new([
                        "parameter",
                        "variable",
                        "function",
                        "comment",
                        "string",
                        "number",
                        "operator",
                        "keyword",
                        "macro"
                    ], []),
                    true
                )
            ),
            new ServerInfo("sparv-lsp", "0.1.0")
        );
    }
}

public record class InitializeResult(
    ServerCapabilities Capabilities,
    ServerInfo? ServerInfo
);



