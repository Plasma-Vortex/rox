#[derive(Debug)]
pub enum TokenType {
    /*
    LEFT_PAREN, RIGHT_PAREN, LEFT_BRACE, RIGHT_BRACE,
    COMMA, DOT, MINUS, PLUS, SEMICOLON, SLASH, STAR,

    // One or two character tokens.
    BANG, BANG_EQUAL,
    EQUAL, EQUAL_EQUAL,
    GREATER, GREATER_EQUAL,
    LESS, LESS_EQUAL,

    // Literals.
    IDENTIFIER, STRING, NUMBER,

    // Keywords.
    AND, CLASS, ELSE, FALSE, FUN, FOR, IF, NIL, OR,
    PRINT, RETURN, SUPER, THIS, TRUE, VAR, WHILE,

    EOF
    */
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

pub struct Scanner {
    source: String,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: i32,
}

impl Scanner {
    pub fn new(source: &str) -> Scanner {
        Scanner {
            source: source.to_owned(),
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
        }
    }

    pub fn scan_tokens(&mut self) -> &Vec<Token> {
        while self.current < self.source.len() {
            self.start = self.current;
            self.scan_token();
        }
        &self.tokens
    }

    fn scan_token(&mut self) {
        // scan token from start to current and append it to self.tokens
    }
}
