use std::iter::Peekable;
use std::str::Chars;

#[derive(Debug)]
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
    source: String,
    iter: Peekable<CharsIndices<'a>>,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: i32,
}

impl<'a> Scanner<'a> {
    pub fn new(source: &str) -> Scanner {
        let source = source.to_owned();
        Scanner {
            source,
            iter: source.char_indices().peekable(),
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
        }
    }

    pub fn scan_tokens(&mut self) -> &Vec<Token> {
        while let Some(kind) = self.scan_token() {
            let end = self.iter.offset();
            self.tokens.push(Token {
                kind,
                lexeme: self.source[self.start..end].to_owned(),
                line: self.line,
            });
            self.start = end;
        }
        &self.tokens
    }

    // Returns None for whitespace (no token)
    fn scan_token(&mut self) -> Result<TokenType, &'static str> {
        if let Some(_, c) = self.iter.next() {
            match c {
                '(' => Ok(TokenType::LeftParen),
                ')' => Ok(TokenType::RightParen),
                '{' => Ok(TokenType::LeftBrace),
                '}' => Ok(TokenType::RightBrace),
                ',' => Ok(TokenType::Comma),
                '.' => Ok(TokenType::Dot),
                '-' => Ok(TokenType::Minus),
                '+' => Ok(TokenType::Plus),
                ';' => Ok(TokenType::Semicolon),
                '*' => Ok(TokenType::Star),
                '!' => {
                    if self.iter.next_if_eq('=').is_some() {
                        Ok(TokenType::BangEqual)
                    } else {
                        Ok(TokenType::Bang)
                    }
                }
                '=' => {
                    if self.iter.next_if_eq('=').is_some() {
                        Ok(TokenType::EqualEqual)
                    } else {
                        Ok(TokenType::Equal)
                    }
                }
                '<' => {
                    if self.iter.next_if_eq('=').is_some() {
                        Ok(TokenType::LessEqual)
                    } else {
                        Ok(TokenType::Less)
                    }
                }
                '>' => {
                    if self.iter.next_if_eq('=').is_some() {
                        Ok(TokenType::GreaterEqual)
                    } else {
                        Ok(TokenType::Greater)
                    }
                }
                '/' => {
                    if self.iter.next_if_eq('/').is_some() {
                        while self.iter.next_if(|c| c != '\n').is_some() {}
                        Ok(TokenType::Whitespace)
                    } else {
                        Ok(TokenType::Slash)
                    }
                }
                '\n' => {
                    self.line += 1;
                    Ok(TokenType::Whitespace)
                }
                '"' => {
                    while let Some(c) = self.iter.next() {
                        if c == '"' {
                            return Ok(TokenType::StringLiteral);
                        } else if c == '\n' {
                            self.line += 1;
                        }
                    }
                    // EOF in string
                    Err("Unterminated string")
                }
                c if c.is_whitespace() => Ok(TokenType::Whitespace),
                c if is_digit(c) => {
                    while self.iter.next_if(|c| is_digit(c)).is_some() {}
                    if self.iter.peek() == Some('.') && is_digit(self.double_peek()) {
                        self.iter.next();
                        while self.iter.next_if(|c| is_digit(c)).is_some() {}
                    }
                    Ok(TokenType::NumberLiteral)
                }
                c if is_alpha(c) => {
                    while self.iter.next_if(|c| is_alphanumeric(c)).is_some() {}
                    Ok(TokenType::StringLiteral)
                }
                c => Err(format!(
                    "Found unexpected character {} while scanning line {}",
                    c, self.line
                )),
            }
        } else {
            Ok(TokenType::Eof)
        }
    }

    fn double_peek(&self) -> char {
        ' '
    }

    fn is_alpha(c: char) -> bool {}
}
