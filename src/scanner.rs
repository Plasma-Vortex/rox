use std::iter::Peekable;
use std::str::Chars;
use unicode_xid::UnicodeXID;

#[derive(Debug, PartialEq)]
pub enum TokenType {
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Slash,
    Star,

    // One or two character tokens.
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    // Literals.
    Identifier,
    StringLiteral,
    NumberLiteral,

    // Keywords.
    And,
    Class,
    Else,
    False,
    Fun,
    For,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,

    Whitespace,
    Eof,
}

#[derive(Debug)]
pub struct Token {
    kind: TokenType,
    lexeme: String,
    line: i32,
}

impl Token {
    pub fn new(kind: TokenType, lexeme: &str, line: i32) -> Token {
        Token {
            kind,
            lexeme: lexeme.to_owned(),
            line,
        }
    }
}

/*
 * source:
 * let x = 5;
 *
 * let x = "5";
 */

pub struct Scanner<'a> {
    source: &'a str,
    iter: Peekable<Chars<'a>>,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: i32,
}

impl<'a> Scanner<'a> {
    pub fn new(source: &str) -> Scanner {
        // let source = source.to_owned();
        Scanner {
            source,
            iter: source.chars().peekable(),
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
        }
    }

    pub fn scan_tokens(&mut self) -> Result<&Vec<Token>, &'static str> {
        loop {
            let token = self.scan_token()?;
            let eof = token.kind == TokenType::Eof;
            self.tokens.push(token);
            if eof {
                break;
            }
            /*
            let (start, kind) = self.scan_token()?;
            if let Some(&(end, _)) = self.iter.peek() {
                self.tokens.push(Token {
                    kind,
                    lexeme: self.source[start..end].to_owned(),
                    line: self.line,
                });
            } else {
                // TODO: EOF Token
                break;
            }
            */
        }
        Ok(&self.tokens)
    }

    // Returns None for whitespace (no token)
    fn scan_token(&mut self) -> Result<Token, &'static str> {
        let mut token_len = 0;
        let kind = if let Some(c) = self.iter.next() {
            token_len += c.len_utf8();
            match c {
                '(' => TokenType::LeftParen,
                ')' => TokenType::RightParen,
                '{' => TokenType::LeftBrace,
                '}' => TokenType::RightBrace,
                ',' => TokenType::Comma,
                '.' => TokenType::Dot,
                '-' => TokenType::Minus,
                '+' => TokenType::Plus,
                ';' => TokenType::Semicolon,
                '*' => TokenType::Star,
                '!' => {
                    if self.iter.next_if_eq(&'=').is_some() {
                        token_len += 1;
                        TokenType::BangEqual
                    } else {
                        TokenType::Bang
                    }
                }
                '=' => {
                    if self.iter.next_if_eq(&'=').is_some() {
                        token_len += 1;
                        TokenType::EqualEqual
                    } else {
                        TokenType::Equal
                    }
                }
                '<' => {
                    if self.iter.next_if_eq(&'=').is_some() {
                        token_len += 1;
                        TokenType::LessEqual
                    } else {
                        TokenType::Less
                    }
                }
                '>' => {
                    if self.iter.next_if_eq(&'=').is_some() {
                        token_len += 1;
                        TokenType::GreaterEqual
                    } else {
                        TokenType::Greater
                    }
                }
                '/' => {
                    if self.iter.next_if_eq(&'/').is_some() {
                        token_len += 1;
                        while let Some(c) = self.iter.next_if(|&c| c != '\n') {
                            token_len += c.len_utf8();
                        }
                        TokenType::Whitespace
                    } else {
                        TokenType::Slash
                    }
                }
                '\n' => {
                    self.line += 1;
                    TokenType::Whitespace
                }
                //'"' => self.string()?
                c if c.is_whitespace() => TokenType::Whitespace,
                c if c.is_ascii_digit() => {
                    while let Some(c) = self.iter.next_if(|&c| c.is_ascii_digit()) {
                        token_len += c.len_utf8();
                    }
                    if self.decimal_point() {
                        self.iter.next();
                        token_len += 1; // for the decimal point
                        while let Some(c) = self.iter.next_if(|&c| c.is_ascii_digit()) {
                            token_len += c.len_utf8();
                        }
                    }
                    TokenType::NumberLiteral
                }
                c if UnicodeXID::is_xid_start(c) => {
                    while let Some(c) = self.iter.next_if(|&c| UnicodeXID::is_xid_continue(c)) {
                        token_len += c.len_utf8();
                    }
                    TokenType::StringLiteral
                }
                _ => {
                    return Err("Found unexpected character"); /*&format!(
                        "Found unexpected character {} while scanning line {}",
                        c, self.line
                    )); */
                }
            }
        } else {
            TokenType::Eof
        };
        let end = self.start + token_len;
        let token = Token {
            kind,
            lexeme: self.source[self.start..end].to_owned(),
            line: self.line,
        };
        self.start = end;
        Ok(token)
    }

    fn decimal_point(&self) -> bool {
        let mut iter = self.iter.clone();
        iter.next() == Some('.') && iter.next().filter(|&c| c.is_ascii_digit()).is_some()
    }

    /*
    fn next_while(&mut self, p: impl FnOnce(char) -> bool) {
        while self.iter.next_if(|c| p(c)).is_some() {}
    }

    fn string(&mut self) -> Result<TokenType, &'static str> {
        while let Some((_, c)) = self.iter.next() {
            if c == '"' {
                return Ok(TokenType::StringLiteral);
            } else if c == '\n' {
                self.line += 1;
            }
        }
        // EOF in string
        Err("Unterminated string")
    }
    */
}
