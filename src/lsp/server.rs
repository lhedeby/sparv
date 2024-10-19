use serde::de::DeserializeOwned;
use serde::Serialize;

use crate::log;
use crate::lsp::contracts::{BaseMessage, Diagnostic, DidChangeTextDocumentNotification, DidOpenTextDocumentNotification, InitializeRequest, InitializeResult, Notification, Position, PublishDiagnosticsParams, Range, Response, SemanticTokenRequest, SemanticTokens};
use crate::lsp::logger;
use crate::parser::{Declaration, Parser};
use crate::scanner::Scanner;
use crate::token::TokenKind;

use std::collections::HashMap;

use std::io::{BufRead, Read, Write};

use std::sync::{Arc, Mutex};
use std::thread;

use std::time::Duration;

use super::contracts::{SemanticTokensLegend, SemanticTokensOptions, ServerCapabilities, ServerInfo, TextDocumentItem};

const HEADER: &'static str = "Content-Length: ";
const HEADER_LEN: usize = HEADER.len();

pub fn start() {
    log!("Server started...");
    let state = Arc::new(Mutex::new(State::new()));
    let readonly_state = Arc::clone(&state);

    let tree = Arc::new(Mutex::new(Vec::<Declaration>::new()));

    let listening_thread = thread::spawn(move || {
        let stdin = std::io::stdin();
        let tree = Arc::clone(&tree);
        let mut content_len = 0;
        loop {
            for line in stdin.lock().lines() {
                let line = line.expect("Failed to read line");

                if line.is_empty() {
                    break; // End of headers (empty line after \r\n)
                }

                // Look for the Content-Length header
                if line.starts_with(HEADER) {
                    content_len = line[HEADER_LEN..].parse().expect("test");
                }
            }
            let mut body = vec![0; content_len];
            stdin
                .lock()
                .read_exact(&mut body)
                .expect("Failed to read content");

            // Process the body (assume it's a UTF-8 JSON string)
            let body_str = std::str::from_utf8(&body).expect("Invalid UTF-8 in body");
            log!("the body {}", body_str);
            match serde_json::from_slice::<BaseMessage>(&body) {
                Ok(v) => match v.method.as_str() {
                    "initialize" => {
                        log!("initialize");
                        let mut guard = state.lock().unwrap();
                        handle_request::<InitializeRequest, InitializeResult>(v, &mut guard)
                    }
                    "textDocument/didOpen" => {
                        log!("textDocument/didOpen");
                        let notification: DidOpenTextDocumentNotification =
                            serde_json::from_value(v.params.unwrap()).unwrap();

                        let mut guard = state.lock().unwrap();
                        guard.update(notification.text_document);
                    }
                    "textDocument/didChange" => {
                        log!("textDocument/didChange");
                        let notification: DidChangeTextDocumentNotification =
                            serde_json::from_value(v.params.unwrap()).unwrap();

                        let mut guard = state.lock().unwrap();
                        let doc = guard
                            .documents
                            .get_mut(&notification.text_document.uri)
                            .unwrap();
                        doc.text = notification
                            .content_changes
                            .first()
                            .expect("should not be empty")
                            .text
                            .clone();
                        doc.version = notification.text_document.version;
                    }
                    "textDocument/semanticTokens/full" => {
                        log!("textDocument/semanticTokens/full");
                        let request: SemanticTokenRequest =
                            serde_json::from_value(v.params.unwrap()).unwrap();

                        let guard = state.lock().unwrap();
                        let doc = guard.documents.get(&request.text_document.uri).unwrap();
                        let mut response_array = vec![];
                        let mut prev_line = 1;
                        let mut prev_start = 0;
                        match Scanner::get_tokens(&doc.text) {
                            Ok(tokens) => {
                                for token in &tokens {
                                    let test = match token.kind {
                                        TokenKind::Identifier(_) => 1,
                                        TokenKind::Fun => 2,
                                        TokenKind::Comment => 3,
                                        TokenKind::String(_) => 4,
                                        TokenKind::Number(_) => 5,
                                        TokenKind::PlusEqual
                                        | TokenKind::Semicolon
                                        | TokenKind::Comma
                                        | TokenKind::Colon
                                        | TokenKind::Plus
                                        | TokenKind::Minus
                                        | TokenKind::Or
                                        | TokenKind::And
                                        | TokenKind::Dot
                                        | TokenKind::Star
                                        | TokenKind::Slash
                                        | TokenKind::Arrow
                                        | TokenKind::Less
                                        | TokenKind::LessEqual
                                        | TokenKind::Greater
                                        | TokenKind::GreaterEqual
                                        | TokenKind::EqualEqual => 6,
                                        _ => 7,
                                    };
                                    response_array.push(token.line - prev_line);
                                    if token.line == prev_line {
                                        response_array.push(token.start - prev_start);
                                    } else {
                                        response_array.push(token.start);
                                    }
                                    prev_line = token.line;
                                    prev_start = token.start;

                                    response_array.push(token.column - token.start);
                                    response_array.push(test);
                                    response_array.push(0);
                                }
                                let response = Response::<SemanticTokens> {
                                    id: v.id,
                                    jsonrpc: "2.0".to_string(),
                                    result: SemanticTokens {
                                        data: response_array,
                                    },
                                };
                                send_message(&response);
                            }
                            Err(e) => log!("Error scanning tokens {}", e),
                        }
                    }
                    _ => log!("unknown message"),
                },
                Err(e) => log!("Serialization error: {}", e),
            }
            log!("State: {:?}", state);
        }
    });

    let diagnostic_thread = thread::spawn(move || loop {
        thread::sleep(Duration::from_secs(5));
        let guard = readonly_state.lock().unwrap();
        for (_, document) in &guard.documents {
            match Parser::parse(&document.text, &document.uri) {
                Ok(_tree) => {
                    // TODO: Generate warnings here
                    log!("parsing went ok, what a day!");

                    let params = PublishDiagnosticsParams {
                        diagnostics: vec![],
                        uri: document.uri.clone(),
                    };

                    let notification = Notification {
                        jsonrpc: "2.0".to_owned(),
                        method: "textDocument/publishDiagnostics".to_owned(),
                        params: Some(params),
                    };
                    send_message(&notification);
                }
                Err(e) => {
                    log!("LSPError - Parsing: {}, {}, {:?}", e, e.line, e.cols);
                    let params = PublishDiagnosticsParams {
                        diagnostics: vec![Diagnostic {
                            source: Some("sparv-lsp".to_owned()),
                            message: format!("{}", e),
                            code: None,
                            range: Range {
                                start: Position {
                                    line: e.line - 1,
                                    character: e.cols.unwrap().0,
                                },

                                end: Position {
                                    line: e.line - 1,
                                    character: e.cols.unwrap().1,
                                },
                            },
                            severity: None,
                        }],
                        uri: document.uri.clone(),
                    };

                    let notification = Notification {
                        jsonrpc: "2.0".to_owned(),
                        method: "textDocument/publishDiagnostics".to_owned(),
                        params: Some(params),
                    };
                    send_message(&notification);
                }
            }
        }
        log!("State from diagnostic thread {:?}", guard);
    });
    listening_thread.join().unwrap();
    diagnostic_thread.join().unwrap();
}

fn send_message<T: Serialize>(obj: &T) {
    let msg = serde_json::to_string(&obj).unwrap();
    log!("Sending to client: {}", msg);

    let encoded_message = format!("{}{}\r\n\r\n{}", HEADER, msg.len(), msg);
    let mut stdout = std::io::stdout().lock();
    stdout.write_all(encoded_message.as_bytes()).unwrap();
    stdout.flush().unwrap();
}


fn handle_request<R, T>(message: BaseMessage, state: &mut State)
where
    R: DeserializeOwned + Request<T>,
    T: Serialize,
{
    let req: R = serde_json::from_value(message.params.unwrap()).unwrap();
    let result = req.handle(state);

    let response = Response {
        id: message.id,
        jsonrpc: "2.0".to_owned(),
        result,
    };
    send_message(&response);
}


#[derive(Debug)]
pub struct State {
    pub documents: HashMap<String, TextDocumentItem>,
}

impl State {
    pub fn new() -> State {
        State {
            documents: HashMap::new(),
        }
    }

    pub fn update(&mut self, document: TextDocumentItem) {
        self.documents.insert(document.uri.to_string(), document);
    }

    pub fn get_doc(&self, s: &str) -> Option<&TextDocumentItem> {
        self.documents.get(s)
    }
}


pub trait Request<T: Serialize> {
    fn handle(&self, state: &mut State) -> T;
}
impl Request<InitializeResult> for InitializeRequest {
    fn handle(&self, _: &mut State) -> InitializeResult {
        InitializeResult {
            server_info: Some(ServerInfo {
                name: "sparv-lsp".to_owned(),
                version: Some("0.0.1".to_owned()),
            }),
            capabilities: ServerCapabilities {
                hover_provider: true,
                definition_provider: true,
                text_document_sync: 1,
                code_action_provider: true,
                text_document_sync_save: true,
                semantic_tokens_provider: SemanticTokensOptions {
                    full: Some(true),
                    legend: SemanticTokensLegend {
                        token_types: vec![
                            "parameter".to_owned(),
                            "variable".to_owned(),
                            "function".to_owned(),
                            "comment".to_owned(),
                            "string".to_owned(),
                            "number".to_owned(),
                            "operator".to_owned(),
                            "keyword".to_owned(),
                        ],
                        token_modifiers: vec![],
                    },
                },
            },
        }
    }
}

