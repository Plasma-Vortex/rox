use std::iter::Peekable;
use std::str::CharIndices;
use unicode_xid::UnicodeXID;

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
    iter: Peekable<CharIndices<'a>>,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: i32,
}

impl<'a> Scanner<'a> {
    pub fn new(source: &str) -> Scanner {
        let source = source.to_owned();
        //let iter: () = source.char_indices().peekable();
        Scanner {
            source,
            iter: source.char_indices().peekable(),
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
        }
    }

    pub fn scan_tokens(&mut self) -> Result<&Vec<Token>, &'static str> {
        loop {
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
        }
        Ok(&self.tokens)
    }

    // Returns None for whitespace (no token)
    fn scan_token(&mut self) -> Result<(usize, TokenType), &'static str> {
        if let Some(&(start, c)) = self.iter.next() {
            Ok((start, match c {
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
                    if self.iter.next_if_eq(&(start+1, '=')).is_some() {
                        TokenType::BangEqual
                    } else {
                        TokenType::Bang
                    }
                }
                '=' => {
                    if self.iter.next_if_eq(&(start+1, '=')).is_some() {
                        TokenType::EqualEqual
                    } else {
                        TokenType::Equal
                    }
                }
                '<' => {
                    if self.iter.next_if_eq(&(start+1, '=')).is_some() {
                        TokenType::LessEqual
                    } else {
                        TokenType::Less
                    }
                }
                '>' => {
                    if self.iter.next_if_eq(&(start+1, '=')).is_some() {
                        TokenType::GreaterEqual
                    } else {
                        TokenType::Greater
                    }
                }
                '/' => {
                    if self.iter.next_if_eq(&(start+1, '/')).is_some() {
                        while self.iter.next_if(|&(_, c)| c != '\n').is_some() {}
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
                    while self.iter.next_if(|c| c.is_ascii_digit()).is_some() {}
                    if self.decimal_point() {
                        self.iter.next();
                        while self.iter.next_if(|c| c.is_ascii_digit()).is_some() {}
                    }
                    TokenType::NumberLiteral
                }
                c if UnicodeXID::is_xid_start(c) => {
                    while self
                        .iter
                        .next_if(|c| UnicodeXID::is_xid_continue(c))
                        .is_some()
                    {}
                    TokenType::StringLiteral
                }
                c => Err(&format!(
                    "Found unexpected character {} while scanning line {}",
                    c, self.line
                )),
            }))
        } else {
            Ok((self.source.len(), TokenType::Eof))
        }
    }

    fn decimal_point(&self) -> bool {
        let mut iter = self.iter.clone();
        iter.next() == Some('.') && iter.next().is_ascii_digit()
    }

    fn next_while(&mut self, p: impl FnOnce(char) -> bool) {
        while self.iter.next_if(|&(_, c)| p(c)).is_some() {}
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
}
