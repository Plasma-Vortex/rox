use crate::scanner::{Token, TokenType};

#[derive(PartialEq, Debug)]
enum Expression {
    Literal(Literal),
    Unary {
        op: TokenType,
        e: Box<Expression>,
    },
    Binary {
        e1: Box<Expression>,
        op: TokenType,
        e2: Box<Expression>,
    },
}

#[derive(PartialEq, Debug)]
enum Literal {
    Num(f64),
    Str(String),
    Bool(bool),
    Nil,
}

pub struct Parser {
    tokens: Vec<Token>,
    cur_idx: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser { tokens, cur_idx: 0 }
    }

    pub fn parse(&mut self) -> Expression {
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
            expr = Expression::Binary {
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
            expr = Expression::Binary {
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
            expr = Expression::Binary {
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
            expr = Expression::Binary {
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
            Expression::Unary {
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
                }
                TokenType::StrLiteral => {
                    let s = cur.lexeme.clone();
                    Expression::Literal(Literal::Str(s))
                }
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
                }
                _ => panic!("Expected literal, found wrong TokenType"),
            },
            None => panic!("Expected literal, found EOF"),
        };
        self.advance();
        expr
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let tokens = vec![
            Token {
                kind: TokenType::NumLiteral,
                lexeme: "6".to_owned(),
                line: 1,
            },
            Token {
                kind: TokenType::Divide,
                lexeme: "/".to_owned(),
                line: 1,
            },
            Token {
                kind: TokenType::NumLiteral,
                lexeme: "3".to_owned(),
                line: 1,
            },
        ];
        let mut parser = Parser::new(tokens);
        let expr = parser.parse();
        assert_eq!(
            expr,
            Expression::Binary {
                e1: Box::new(Expression::Literal(Literal::Num(6f64))),
                op: TokenType::Divide,
                e2: Box::new(Expression::Literal(Literal::Num(3f64))),
            }
        );
    }
}
