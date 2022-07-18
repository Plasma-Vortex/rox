use crate::scanner::{Token, TokenType};

enum Expression {
    Literal(Literal),
    Unary { op: UnaryOp, e: Box::<Expression> },
    Binary { e1: Box::<Expression>, op: BinaryOp, e2: Box::<Expression> },
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

    fn parse(&self) -> Expression {
        self.expression()
    }

    // 1 + 2 == 3 == 7 - 4
    // ((1 + 2) == 3) == 7 - 4
    //

    fn current(&self) -> TokenType {
        self.tokens[self.cur_idx].kind
    }

    fn check_if_eq(&self, tokens: &Vec<TokenType>) -> Option<TokenType> {
        let cur = self.current();
        for token in tokens {
            if cur == *token {
                return Some(cur);
            }
        }
        return None;
    }

    fn expression(&self) -> Expression {
        let mut expr = self.comparison();
        let options = vec![TokenType::BangEqual, TokenType::EqualEqual];

        while let Some(t) = self.check_if_eq(&options) {
            let right = self.comparison();
            expr = Expression::Binary{
                e1: Box::new(expr),
                op: BinaryOp::EqualEqual,
                e2: Box::new(right),
            };
        }
        return expr;
    }

    fn comparison(&self) -> Expression {
        Expression::Literal(Literal::Nil)
    }
}
