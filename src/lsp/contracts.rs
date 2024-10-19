use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Serialize)]
pub struct Notification<T> {
    pub jsonrpc: String,
    pub method: String,
    pub params: Option<T>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PublishDiagnosticsParams {
    pub uri: String,
    pub diagnostics: Vec<Diagnostic>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Diagnostic {
    pub range: Range,
    pub severity: Option<DiagnosticSeverity>,
    pub code: Option<String>,
    pub source: Option<String>,
    pub message: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Range {
    pub start: Position,
    pub end: Position,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Position {
    pub line: usize,
    pub character: usize,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum DiagnosticSeverity {
    Error = 1,
    Warning = 2,
    Information = 3,
    Hint = 4,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DidOpenTextDocumentNotification {
    #[serde(rename = "textDocument")]
    pub text_document: TextDocumentItem,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SemanticTokenRequest {
    #[serde(rename = "textDocument")]
    pub text_document: TextDocumentIdentifier,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TextDocumentIdentifier {
    pub uri: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DidChangeTextDocumentNotification {
    #[serde(rename = "textDocument")]
    pub text_document: TextDocumentChangeItem,
    #[serde(rename = "contentChanges")]
    pub content_changes: Vec<TextDocumentContentChangeEvent>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TextDocumentContentChangeEvent {
    pub text: String,
}

#[derive(Serialize)]
pub struct Response<T> {
    pub id: Option<usize>,
    pub jsonrpc: String,
    pub result: T,
}

#[derive(Serialize)]
pub struct SemanticTokens {
    pub data: Vec<usize>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BaseMessage {
    pub jsonrpc: String,
    pub method: String,
    pub params: Option<Value>,
    pub id: Option<usize>,
}

#[derive(Serialize, Deserialize)]
pub struct InitializeRequest {
    #[serde(rename = "clientInfo")]
    client_info: Option<ClientInfo>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TextDocumentItem {
    pub uri: String,
    #[serde(rename = "languageId")]
    pub language_id: String,
    pub version: usize,
    pub text: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TextDocumentChangeItem {
    pub uri: String,
    pub version: usize,
}

#[derive(Serialize, Deserialize)]
pub struct ClientInfo {
    pub name: String,
    pub version: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct InitializeResult {
    pub capabilities: ServerCapabilities,
    #[serde(rename = "serverInfo")]
    pub server_info: Option<ServerInfo>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ServerCapabilities {
    #[serde(rename = "hoverProvider")]
    pub hover_provider: bool,
    #[serde(rename = "definitionProvider")]
    pub definition_provider: bool,
    #[serde(rename = "codeActionProvider")]
    pub code_action_provider: bool,
    #[serde(rename = "textDocumentSync")]
    pub text_document_sync: usize,
    #[serde(rename = "textDocumentSync.save")]
    pub text_document_sync_save: bool,
    #[serde(rename = "semanticTokensProvider")]
    pub semantic_tokens_provider: SemanticTokensOptions,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SemanticTokensOptions {
    pub legend: SemanticTokensLegend,
    pub full: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SemanticTokensLegend {
    #[serde(rename = "tokenTypes")]
    pub token_types: Vec<String>,
    #[serde(rename = "tokenModifiers")]
    pub token_modifiers: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ServerInfo {
    pub name: String,
    pub version: Option<String>,
}
