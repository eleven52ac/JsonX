//! Recursive-descent parser that consumes `Tokenizer` output into a `Value`
//! tree. `format`, `minify` and `statistics` all operate on this tree today;
//! the large-file path is expected to bypass it in favor of streaming
//! directly off tokens (see `indexer`), since materializing a full tree for
//! a multi-hundred-MB document is exactly what the product spec forbids.

use crate::tokenizer::{Token, TokenKind, Tokenizer};

#[derive(Debug, Clone)]
pub enum Value {
    Null,
    Bool(bool),
    Number(f64),
    String(String),
    Array(Vec<Value>),
    Object(Vec<(String, Value)>),
}

#[derive(Debug)]
pub struct ParseError {
    pub message: String,
    pub offset: usize,
}

pub struct Parser<'a> {
    tokenizer: Tokenizer<'a>,
    lookahead: Option<Token>,
}

impl<'a> Parser<'a> {
    pub fn new(input: &'a str) -> Self {
        Parser { tokenizer: Tokenizer::new(input), lookahead: None }
    }

    fn advance(&mut self) -> Result<Option<Token>, ParseError> {
        if let Some(tok) = self.lookahead.take() {
            return Ok(Some(tok));
        }
        self.tokenizer.next_token().map_err(|e| ParseError { message: e.message, offset: e.offset })
    }

    fn peek(&mut self) -> Result<Option<&Token>, ParseError> {
        if self.lookahead.is_none() {
            self.lookahead =
                self.tokenizer.next_token().map_err(|e| ParseError { message: e.message, offset: e.offset })?;
        }
        Ok(self.lookahead.as_ref())
    }

    pub fn parse(&mut self) -> Result<Value, ParseError> {
        let value = self.parse_value()?;
        match self.advance()? {
            None => Ok(value),
            Some(tok) => Err(ParseError { message: "unexpected trailing content".into(), offset: tok.offset }),
        }
    }

    fn parse_value(&mut self) -> Result<Value, ParseError> {
        let tok = self
            .advance()?
            .ok_or_else(|| ParseError { message: "unexpected end of input".into(), offset: usize::MAX })?;
        match tok.kind {
            TokenKind::ObjectStart => self.parse_object(),
            TokenKind::ArrayStart => self.parse_array(),
            TokenKind::String(s) => Ok(Value::String(s)),
            TokenKind::Number(n) => Ok(Value::Number(n)),
            TokenKind::True => Ok(Value::Bool(true)),
            TokenKind::False => Ok(Value::Bool(false)),
            TokenKind::Null => Ok(Value::Null),
            _ => Err(ParseError { message: "unexpected token".into(), offset: tok.offset }),
        }
    }

    fn parse_object(&mut self) -> Result<Value, ParseError> {
        let mut entries = Vec::new();
        if let Some(tok) = self.peek()? {
            if tok.kind == TokenKind::ObjectEnd {
                self.advance()?;
                return Ok(Value::Object(entries));
            }
        }
        loop {
            let key_tok = self
                .advance()?
                .ok_or_else(|| ParseError { message: "unexpected end of input in object".into(), offset: usize::MAX })?;
            let key = match key_tok.kind {
                TokenKind::String(s) => s,
                _ => return Err(ParseError { message: "expected string key".into(), offset: key_tok.offset }),
            };
            let colon = self
                .advance()?
                .ok_or_else(|| ParseError { message: "expected ':'".into(), offset: usize::MAX })?;
            if colon.kind != TokenKind::Colon {
                return Err(ParseError { message: "expected ':'".into(), offset: colon.offset });
            }
            let value = self.parse_value()?;
            entries.push((key, value));

            let sep = self
                .advance()?
                .ok_or_else(|| ParseError { message: "expected ',' or '}'".into(), offset: usize::MAX })?;
            match sep.kind {
                TokenKind::Comma => continue,
                TokenKind::ObjectEnd => break,
                _ => return Err(ParseError { message: "expected ',' or '}'".into(), offset: sep.offset }),
            }
        }
        Ok(Value::Object(entries))
    }

    fn parse_array(&mut self) -> Result<Value, ParseError> {
        let mut items = Vec::new();
        if let Some(tok) = self.peek()? {
            if tok.kind == TokenKind::ArrayEnd {
                self.advance()?;
                return Ok(Value::Array(items));
            }
        }
        loop {
            let value = self.parse_value()?;
            items.push(value);
            let sep = self
                .advance()?
                .ok_or_else(|| ParseError { message: "expected ',' or ']'".into(), offset: usize::MAX })?;
            match sep.kind {
                TokenKind::Comma => continue,
                TokenKind::ArrayEnd => break,
                _ => return Err(ParseError { message: "expected ',' or ']'".into(), offset: sep.offset }),
            }
        }
        Ok(Value::Array(items))
    }
}
