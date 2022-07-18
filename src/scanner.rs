use std::iter::Peekable;
use std::str::Chars;
use unicode_xid::UnicodeXID;

#[derive(Debug, PartialEq, Copy, Clone)]
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

#[derive(Debug, PartialEq)]
pub struct Token {
    pub kind: TokenType,
    lexeme: String,
    line: i32,
}

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
            if token.kind != TokenType::Whitespace {
                self.tokens.push(token);
                if eof {
                    break;
                }
            }
        }
        Ok(&self.tokens)
    }

    // Returns None for whitespace (no token)
    fn scan_token(&mut self) -> Result<Token, &'static str> {
        self.start = self.current;
        let kind = if let Some(c) = self.next() {
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
                    if self.next_if_eq('=') {
                        TokenType::BangEqual
                    } else {
                        TokenType::Bang
                    }
                }
                '=' => {
                    if self.next_if_eq('=') {
                        TokenType::EqualEqual
                    } else {
                        TokenType::Equal
                    }
                }
                '<' => {
                    if self.next_if_eq('=') {
                        TokenType::LessEqual
                    } else {
                        TokenType::Less
                    }
                }
                '>' => {
                    if self.next_if_eq('=') {
                        TokenType::GreaterEqual
                    } else {
                        TokenType::Greater
                    }
                }
                '/' => {
                    if self.next_if_eq('/') {
                        self.next_while(|&c| c != '\n');
                        TokenType::Whitespace
                    } else {
                        TokenType::Slash
                    }
                }
                '\n' => {
                    self.line += 1;
                    TokenType::Whitespace
                }
                '"' => self.string()?,
                c if c.is_whitespace() => TokenType::Whitespace,
                c if c.is_ascii_digit() => {
                    self.next_while(|&c| c.is_ascii_digit());
                    if self.decimal_point() {
                        self.next(); // read the decimal point
                        self.next_while(|&c| c.is_ascii_digit());
                    }
                    TokenType::NumberLiteral
                }
                c if UnicodeXID::is_xid_start(c) => {
                    self.next_while(|&c| UnicodeXID::is_xid_continue(c));
                    // TODO: test indexing
                    let ident = &self.source[self.start..self.current];
                    match ident {
                        "and"    => TokenType::And,
                        "class"  => TokenType::Class,
                        "else"   => TokenType::Else,
                        "false"  => TokenType::False,
                        "for"    => TokenType::For,
                        "fun"    => TokenType::Fun,
                        "if"     => TokenType::If,
                        "nil"    => TokenType::Nil,
                        "or"     => TokenType::Or,
                        "print"  => TokenType::Print,
                        "return" => TokenType::Return,
                        "super"  => TokenType::Super,
                        "this"   => TokenType::This,
                        "true"   => TokenType::True,
                        "var"    => TokenType::Var,
                        "while"  => TokenType::While,
                        _        => TokenType::Identifier,
                    }
                }
                _ => {
                    // TODO: more details of c and line
                    return Err("Found unexpected character");
                }
            }
        } else {
            TokenType::Eof
        };
        let token = Token {
            kind,
            lexeme: self.source[self.start..self.current].to_owned(),
            line: self.line,
        };
        self.start = self.current;
        Ok(token)
    }

    fn decimal_point(&self) -> bool {
        let mut iter = self.iter.clone();
        iter.next() == Some('.') && iter.next().filter(|&c| c.is_ascii_digit()).is_some()
    }

    fn next(&mut self) -> Option<char> {
        let ret = self.iter.next();
        if let Some(c) = ret {
            self.current += c.len_utf8();
        }
        ret
    }

    fn next_if_eq(&mut self, expected: char) -> bool {
        if let Some(c) = self.iter.next_if_eq(&expected) {
            self.current += c.len_utf8();
            return true;
        }
        false
    }

    fn next_while(&mut self, p: impl FnOnce(&char) -> bool + Copy) {
        while let Some(c) = self.iter.next_if(p) {
            self.current += c.len_utf8();
        }
    }

    fn string(&mut self) -> Result<TokenType, &'static str> {
        while let Some(c) = self.iter.next() {
            self.current += c.len_utf8();
            if c == '"' {
                return Ok(TokenType::StringLiteral)
            } else if c == '\n' {
                self.line += 1;
            }
        }
        // EOF in string
        Err("Unterminated string")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test1() {
        let source = fs::read_to_string("test1.lox").expect("Failed to read file");
        let mut s = Scanner::new(&source);
        assert_eq!(s.scan_tokens(), Ok(&vec![
            Token { kind: TokenType::Var, lexeme: "var".to_string(), line: 1 },
            Token { kind: TokenType::Identifier, lexeme: "i".to_string(), line: 1 },
            Token { kind: TokenType::Equal, lexeme: "=".to_string(), line: 1 },
            Token { kind: TokenType::NumberLiteral, lexeme: "1".to_string(), line: 1 },
            Token { kind: TokenType::Semicolon, lexeme: ";".to_string(), line: 1 },
            Token { kind: TokenType::Eof, lexeme: "".to_string(), line: 2 },
        ]));
    }

    #[test]
    fn test2() {
        let source = fs::read_to_string("test2.lox").expect("Failed to read file");
        let mut s = Scanner::new(&source);
        assert_eq!(s.scan_tokens(), Ok(&vec![
            Token { kind: TokenType::Var, lexeme: "var".to_string(), line: 1 },
            Token { kind: TokenType::Identifier, lexeme: "s".to_string(), line: 1 },
            Token { kind: TokenType::Equal, lexeme: "=".to_string(), line: 1 },
            Token { kind: TokenType::StringLiteral, lexeme: "\"Hello, World!\"".to_string(), line: 1 },
            Token { kind: TokenType::Semicolon, lexeme: ";".to_string(), line: 1 },
            Token { kind: TokenType::Eof, lexeme: "".to_string(), line: 2 },
        ]));
    }

    #[test]
    fn test3() {
        let source = fs::read_to_string("test3.lox").expect("Failed to read file");
        let mut s = Scanner::new(&source);
        assert_eq!(s.scan_tokens(), Ok(&vec![
            Token { kind: TokenType::Var, lexeme: "var".to_string(), line: 1 },
            Token { kind: TokenType::Identifier, lexeme: "a".to_string(), line: 1 },
            Token { kind: TokenType::Equal, lexeme: "=".to_string(), line: 1 },
            Token { kind: TokenType::NumberLiteral, lexeme: "1".to_string(), line: 1 },
            Token { kind: TokenType::Semicolon, lexeme: ";".to_string(), line: 1 },
            Token { kind: TokenType::Var, lexeme: "var".to_string(), line: 2 },
            Token { kind: TokenType::Identifier, lexeme: "b".to_string(), line: 2 },
            Token { kind: TokenType::Equal, lexeme: "=".to_string(), line: 2 },
            Token { kind: TokenType::NumberLiteral, lexeme: "2".to_string(), line: 2 },
            Token { kind: TokenType::Semicolon, lexeme: ";".to_string(), line: 2 },
            Token { kind: TokenType::Var, lexeme: "var".to_string(), line: 3 },
            Token { kind: TokenType::Identifier, lexeme: "c".to_string(), line: 3 },
            Token { kind: TokenType::Equal, lexeme: "=".to_string(), line: 3 },
            Token { kind: TokenType::Identifier, lexeme: "a".to_string(), line: 3 },
            Token { kind: TokenType::Plus, lexeme: "+".to_string(), line: 3 },
            Token { kind: TokenType::Identifier, lexeme: "b".to_string(), line: 3 },
            Token { kind: TokenType::Star, lexeme: "*".to_string(), line: 3 },
            Token { kind: TokenType::Identifier, lexeme: "a".to_string(), line: 3 },
            Token { kind: TokenType::Minus, lexeme: "-".to_string(), line: 3 },
            Token { kind: TokenType::Identifier, lexeme: "b".to_string(), line: 3 },
            Token { kind: TokenType::Slash, lexeme: "/".to_string(), line: 3 },
            Token { kind: TokenType::Identifier, lexeme: "a".to_string(), line: 3 },
            Token { kind: TokenType::Semicolon, lexeme: ";".to_string(), line: 3 },
            Token { kind: TokenType::Print, lexeme: "print".to_string(), line: 4 },
            Token { kind: TokenType::Identifier, lexeme: "c".to_string(), line: 4 },
            Token { kind: TokenType::Semicolon, lexeme: ";".to_string(), line: 4 },
            Token { kind: TokenType::Eof, lexeme: "".to_string(), line: 5 },
        ]));
    }

}
