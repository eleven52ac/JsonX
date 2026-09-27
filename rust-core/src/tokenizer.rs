//! Byte-oriented JSON tokenizer. Walks the input once, producing tokens with
//! their byte offsets so downstream stages (parser, indexer) never need to
//! rescan text that has already been classified. This is the seam where a
//! future chunk-fed streaming tokenizer replaces the current single-slice
//! implementation without changing the token shape consumers rely on.

#[derive(Debug, Clone, PartialEq)]
pub enum TokenKind {
    ObjectStart,
    ObjectEnd,
    ArrayStart,
    ArrayEnd,
    Colon,
    Comma,
    String(String),
    Number(f64),
    True,
    False,
    Null,
}

#[derive(Debug, Clone)]
pub struct Token {
    pub kind: TokenKind,
    pub offset: usize,
}

#[derive(Debug)]
pub struct TokenizeError {
    pub message: String,
    pub offset: usize,
}

pub struct Tokenizer<'a> {
    input: &'a [u8],
    pos: usize,
}

impl<'a> Tokenizer<'a> {
    pub fn new(input: &'a str) -> Self {
        Tokenizer { input: input.as_bytes(), pos: 0 }
    }

    fn skip_whitespace(&mut self) {
        while let Some(&b) = self.input.get(self.pos) {
            if b == b' ' || b == b'\t' || b == b'\n' || b == b'\r' {
                self.pos += 1;
            } else {
                break;
            }
        }
    }

    fn error(&self, message: &str) -> TokenizeError {
        TokenizeError { message: message.to_string(), offset: self.pos }
    }

    pub fn next_token(&mut self) -> Result<Option<Token>, TokenizeError> {
        self.skip_whitespace();
        let start = self.pos;
        let b = match self.input.get(self.pos) {
            Some(&b) => b,
            None => return Ok(None),
        };

        let kind = match b {
            b'{' => {
                self.pos += 1;
                TokenKind::ObjectStart
            }
            b'}' => {
                self.pos += 1;
                TokenKind::ObjectEnd
            }
            b'[' => {
                self.pos += 1;
                TokenKind::ArrayStart
            }
            b']' => {
                self.pos += 1;
                TokenKind::ArrayEnd
            }
            b':' => {
                self.pos += 1;
                TokenKind::Colon
            }
            b',' => {
                self.pos += 1;
                TokenKind::Comma
            }
            b'"' => self.read_string()?,
            b't' => self.read_literal("true", TokenKind::True)?,
            b'f' => self.read_literal("false", TokenKind::False)?,
            b'n' => self.read_literal("null", TokenKind::Null)?,
            b'-' | b'0'..=b'9' => self.read_number()?,
            other => return Err(self.error(&format!("unexpected character '{}'", other as char))),
        };

        Ok(Some(Token { kind, offset: start }))
    }

    fn read_literal(&mut self, literal: &str, kind: TokenKind) -> Result<TokenKind, TokenizeError> {
        let bytes = literal.as_bytes();
        if self.input[self.pos..].starts_with(bytes) {
            self.pos += bytes.len();
            Ok(kind)
        } else {
            Err(self.error(&format!("expected '{}'", literal)))
        }
    }

    fn read_string(&mut self) -> Result<TokenKind, TokenizeError> {
        self.pos += 1; // opening quote
        let mut out = String::new();
        loop {
            let b = *self.input.get(self.pos).ok_or_else(|| self.error("unterminated string"))?;
            match b {
                b'"' => {
                    self.pos += 1;
                    break;
                }
                b'\\' => {
                    self.pos += 1;
                    let esc = *self.input.get(self.pos).ok_or_else(|| self.error("unterminated escape"))?;
                    match esc {
                        b'"' => out.push('"'),
                        b'\\' => out.push('\\'),
                        b'/' => out.push('/'),
                        b'b' => out.push('\u{8}'),
                        b'f' => out.push('\u{c}'),
                        b'n' => out.push('\n'),
                        b'r' => out.push('\r'),
                        b't' => out.push('\t'),
                        b'u' => {
                            let hex = self
                                .input
                                .get(self.pos + 1..self.pos + 5)
                                .ok_or_else(|| self.error("invalid unicode escape"))?;
                            let hex_str = std::str::from_utf8(hex).map_err(|_| self.error("invalid unicode escape"))?;
                            let code = u32::from_str_radix(hex_str, 16).map_err(|_| self.error("invalid unicode escape"))?;
                            out.push(char::from_u32(code).unwrap_or('\u{FFFD}'));
                            self.pos += 4;
                        }
                        _ => return Err(self.error("invalid escape sequence")),
                    }
                    self.pos += 1;
                }
                _ => {
                    let ch_len = utf8_len(b);
                    let end = self.pos + ch_len;
                    let slice = self.input.get(self.pos..end).ok_or_else(|| self.error("invalid utf8"))?;
                    let s = std::str::from_utf8(slice).map_err(|_| self.error("invalid utf8"))?;
                    out.push_str(s);
                    self.pos = end;
                }
            }
        }
        Ok(TokenKind::String(out))
    }

    fn read_number(&mut self) -> Result<TokenKind, TokenizeError> {
        let start = self.pos;
        if self.input.get(self.pos) == Some(&b'-') {
            self.pos += 1;
        }
        while matches!(self.input.get(self.pos), Some(b'0'..=b'9')) {
            self.pos += 1;
        }
        if self.input.get(self.pos) == Some(&b'.') {
            self.pos += 1;
            while matches!(self.input.get(self.pos), Some(b'0'..=b'9')) {
                self.pos += 1;
            }
        }
        if matches!(self.input.get(self.pos), Some(b'e') | Some(b'E')) {
            self.pos += 1;
            if matches!(self.input.get(self.pos), Some(b'+') | Some(b'-')) {
                self.pos += 1;
            }
            while matches!(self.input.get(self.pos), Some(b'0'..=b'9')) {
                self.pos += 1;
            }
        }
        let text = std::str::from_utf8(&self.input[start..self.pos]).map_err(|_| self.error("invalid number"))?;
        text.parse::<f64>().map(TokenKind::Number).map_err(|_| self.error("invalid number"))
    }
}

fn utf8_len(first_byte: u8) -> usize {
    if first_byte & 0x80 == 0 {
        1
    } else if first_byte & 0xE0 == 0xC0 {
        2
    } else if first_byte & 0xF0 == 0xE0 {
        3
    } else if first_byte & 0xF8 == 0xF0 {
        4
    } else {
        1
    }
}
