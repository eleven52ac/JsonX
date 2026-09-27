//! Validates JSON text and translates a byte offset parse error into a
//! human-facing line/column, without needing a full `Value` tree by the
//! caller (though today it gets one for free by parsing).

use crate::parser::Parser;

pub struct ValidationError {
    pub message: String,
    pub line: usize,
    pub column: usize,
    pub offset: usize,
}

pub fn validate(input: &str) -> Result<(), ValidationError> {
    let mut parser = Parser::new(input);
    parser.parse().map(|_| ()).map_err(|e| {
        let (line, column) = line_column_at(input, e.offset);
        ValidationError { message: e.message, line, column, offset: e.offset }
    })
}

fn line_column_at(input: &str, offset: usize) -> (usize, usize) {
    let mut line = 1;
    let mut column = 1;
    for (i, ch) in input.char_indices() {
        if i >= offset {
            break;
        }
        if ch == '\n' {
            line += 1;
            column = 1;
        } else {
            column += 1;
        }
    }
    (line, column)
}
