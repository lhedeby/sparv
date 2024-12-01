use std::{iter::Peekable, str::CharIndices};

use crate::lsp::contracts::{DocumentFormattingParams, TextEdit};

use super::contracts::{Position, Range};

pub struct Formatter {}

fn consume_whitespace(chars: &mut Peekable<CharIndices<'_>>) -> usize {
    let mut whitespace = 0;
    while let Some((_, ' ')) = chars.peek() {
        chars.next();
        whitespace += 1;
    }
    whitespace
}

fn text_edit(line: usize, start_char: usize, end_char: usize, new_text: String) -> TextEdit {
    TextEdit {
        range: Range {
            start: Position {
                line,
                character: start_char,
            },
            end: Position {
                line,
                character: end_char,
            },
        },
        new_text,
    }
}

impl Formatter {
    pub fn format_document(text: &str, _params: &DocumentFormattingParams) -> Vec<TextEdit> {

        const INDENT_SIZE: usize = 4;

        let mut text_edits: Vec<TextEdit> = vec![];
        let mut line_no = 0;
        let mut indent: usize = 0;
        for line in text.lines() {
            let mut chars = line.char_indices().peekable();

            let whitespace = consume_whitespace(&mut chars);
            if let Some((_, '}')) = chars.peek() {
                indent = indent.saturating_sub(INDENT_SIZE);
            }
            if indent != whitespace {
                text_edits.push(text_edit(line_no, 0, whitespace, " ".repeat(indent)));
            }

            while let Some((i, c)) = chars.next() {
                match c {
                    ';' => {
                        let whitespace = consume_whitespace(&mut chars);
                        if let Some((_, _)) = chars.peek() {
                            text_edits.push(text_edit(
                                line_no,
                                i + 1,
                                i + 1 + whitespace,
                                format!("\n{}", " ".repeat(indent)),
                            ));
                        } else if whitespace > 0 {
                            text_edits.push(text_edit(
                                line_no,
                                i + 1,
                                i + 1 + whitespace,
                                format!(""),
                            ));
                        }
                    }
                    '{' => {
                        indent += INDENT_SIZE;
                        if let Some(_) = chars.peek() {
                            let whitespace = consume_whitespace(&mut chars);
                            text_edits.push(text_edit(
                                line_no,
                                i + 1,
                                i + 1 + whitespace,
                                format!("\n{}", " ".repeat(indent)),
                            ));
                        }
                    }
                    ' ' => {
                        while let Some((n_i, n_c)) = chars.peek() {
                            if *n_c != ' ' {
                                text_edits.push(text_edit(line_no, i, *n_i, " ".to_string()));
                                break;
                            }
                            chars.next();
                        }
                    }
                    ',' => {
                        match chars.peek() {
                            Some((_, ' ')) => {}
                            _ => text_edits.push(text_edit(line_no, i + 1, i + 1, " ".to_string())),
                        }
                        if let Some((_, ' ')) = chars.peek() {}
                    }
                    _ => {}
                }
            }
            line_no += 1;
        }
        text_edits
    }
}
