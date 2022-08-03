use crate::scanner::{Token, TokenType};

enum Expression {
    Literal(Literal),
    Unary { op: TokenType, e: Box::<Expression> },
    Binary { e1: Box::<Expression>, op: TokenType, e2: Box::<Expression> },
}

enum Literal {
    Num(f64),
    Str(String),
    Bool(bool),
    Nil,
}

enum UnaryOp {
    Minus,
    Not,
}

enum BinaryOp {
    EqualEqual,
    NotEqual,
    Less,
    LessEqual,
    Greater,
    GreaterEqual,
    Plus,
    Minus,
    Mult,
    Divide,
}

pub struct Parser {
    tokens: Vec<Token>,
    cur_idx: usize,
}

impl Parser {
    fn new(tokens: Vec<Token>) -> Self {
        Parser {
            tokens,
            cur_idx: 0,
        }
    }

    fn parse(&mut self) -> Expression {
        self.expression()
    }

    fn current(&self) -> Option<&Token> {
        self.tokens.get(self.cur_idx)
    }

    fn advance(&mut self) {
        self.cur_idx += 1;
    }

    fn advance_if_eq(&mut self, tokens: &Vec<TokenType>) -> Option<TokenType> {
        if let Some(cur) = self.current() {
            for token in tokens {
                if cur.kind == *token {
                    self.advance();
                    return Some(*token);
                }
            }
        }
        None
    }

    fn expression(&mut self) -> Expression {
        let options = vec![TokenType::NotEqual, TokenType::EqualEqual];

        let mut expr = self.comparison();
        while let Some(op) = self.advance_if_eq(&options) {
            let right = self.comparison();
            expr = Expression::Binary{
                e1: Box::new(expr),
                op,
                e2: Box::new(right),
            };
        }
        expr
    }

    fn comparison(&mut self) -> Expression {
        let options = vec![
            TokenType::Greater,
            TokenType::GreaterEqual,
            TokenType::Less,
            TokenType::LessEqual,
        ];

        let mut expr = self.term();
        while let Some(op) = self.advance_if_eq(&options) {
            let right = self.term();
            expr = Expression::Binary{
                e1: Box::new(expr),
                op,
                e2: Box::new(right),
            };
        }
        expr
    }

    fn term(&mut self) -> Expression {
        let options = vec![TokenType::Plus, TokenType::Minus];

        let mut expr = self.factor();
        while let Some(op) = self.advance_if_eq(&options) {
            let right = self.factor();
            expr = Expression::Binary{
                e1: Box::new(expr),
                op,
                e2: Box::new(right),
            };
        }
        expr
    }

    fn factor(&mut self) -> Expression {
        let options = vec![TokenType::Times, TokenType::Divide];

        let mut expr = self.unary();
        while let Some(op) = self.advance_if_eq(&options) {
            let right = self.unary();
            expr = Expression::Binary{
                e1: Box::new(expr),
                op,
                e2: Box::new(right),
            };
        }
        expr
    }

    fn unary(&mut self) -> Expression {
        let options = vec![TokenType::Minus, TokenType::Not];

        if let Some(op) = self.advance_if_eq(&options) {
            let right = self.unary();
            Expression::Unary{
                op,
                e: Box::new(right),
            }
        } else {
            self.primary()
        }
    }

    fn primary(&mut self) -> Expression {
        let expr = match self.current() {
            Some(cur) => match cur.kind {
                TokenType::NumLiteral => {
                    let num = cur.lexeme.parse().expect("Failed to parse number");
                    Expression::Literal(Literal::Num(num))
                },
                TokenType::StrLiteral => {
                    let s = cur.lexeme.clone();
                    Expression::Literal(Literal::Str(s))
                },
                TokenType::True => Expression::Literal(Literal::Bool(true)),
                TokenType::False => Expression::Literal(Literal::Bool(false)),
                TokenType::Nil => Expression::Literal(Literal::Nil),
                TokenType::LeftParen => {
                    self.advance(); // left paren
                    let inner = self.expression();
                    if self.advance_if_eq(&vec![TokenType::RightParen]) == None {
                        panic!("Error: no matching right parenthesis");
                    }
                    return inner;
                },
                _ => panic!("Expected literal, found wrong TokenType")
            }
            None => panic!("Expected literal, found EOF")
        };
        self.advance();
        expr
    }
}
