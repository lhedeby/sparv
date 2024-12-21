public record class ServerCapabilities(
    CompletionOptions CompletionProvider,
    bool DocumentFormattingProvider,
    // bool HoverProvider,
    // bool DefinitionProvider,
    // bool CodeActionProvider,
    int TextDocumentSync,
    SemanticTokensOptions SemanticTokensProvider
// [property: JsonPropertyName("textDocumentSync.save")] bool TextDocumentSyncSave,
);
