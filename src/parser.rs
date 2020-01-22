use std::cell::{Cell, RefCell};

use super::lexer::{Lexer, Operator, Token};

#[derive(Debug, PartialEq)]
struct Expression {
    operation: Token,
    right: Token,
    left: Option<Token>,
}

impl Expression {
    fn new() -> Expression {
        Expression {
            operation: Token::EOF,
            right: Token::EOF,
            left: None,
        }
    }
}

#[derive(Debug, PartialEq)]
enum Production {
    Label { token: Token, position: u64 },
    Expr(Expression),
    EOF,
}

struct Parser<'a> {
    tokens: Vec<Production>,
    lexer: RefCell<Lexer<'a>>,
    position: Cell<u64>,
}

impl<'a> Parser<'a> {
    fn new(code: &'a str) -> Parser<'a> {
        Parser {
            tokens: Vec::new(),
            lexer: RefCell::new(Lexer::new(code.chars())),
            position: Cell::new(0),
        }
    }

    fn parse(&self) -> Vec<Production> {
        let mut productions = Vec::new();
        while let Some(token) = self._next_token() {
            let prod = match token {
                Token::Ident(_, _, _) => Production::Label {
                    token,
                    position: self.position.get(),
                },
                Token::Op(operator, _, _) =>  self.handle_expression(token, operator),
                token => panic!("Received incorret token found: {:?}", token),
            };
            productions.push(prod);
        }
        productions
    }

    fn handle_expression(&self, token: Token, operator: Operator) -> Production {
        let mut expr = Expression::new();
        match token {
            Token::Op(..) => expr.operation = token,
            token => panic!("Unreachable: {:?}", token),
        }

        if let Some(token) = self._next_token() {
            match token {
                Token::REGISTER(..) => expr.right = token,
                other => panic!("Expected register found: {:?}", other),
            }
        }
        if let Some(token) = self._next_token() {
            match token {
                Token::REGISTER(..) => expr.left = Some(token),
                other => panic!("Expected register found: {:?}", other),
            }
        }
        Production::Expr(expr)
    }

    fn _next_token(&self) -> Option<Token> {
        self.position.set(self.position.get() + 1);
        match self.lexer.borrow_mut().parse_token() {
            Token::EOF => None,
            token => Some(token),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::Operator::*;

    #[test]
    fn test_handle_expression() {
        let p = Parser::new("add %r1 %r2");
        let production = p.parse();
        assert_eq!(
            production,
            vec![Production::Expr(Expression {
                operation: Token::Op(ADD, 0, 4),
                right: Token::REGISTER("%r1".to_owned(), 0, 7),
                left: Some(Token::REGISTER("%r2".to_owned(), 0, 11))
            })]
        );
    }

    #[test]
    fn test_next_token() {
        let p = Parser::new("add %r1 %r2");
        let token = p._next_token();
        assert!(token.is_some());
        assert_eq!(token, Some(Token::Op(ADD, 0, 4)));
        let token = p._next_token();
        assert!(token.is_some());
        assert_eq!(token, Some(Token::REGISTER("%r1".to_owned(), 0, 7)));
        let token = p._next_token();
        assert!(token.is_some());
        assert_eq!(token, Some(Token::REGISTER("%r2".to_owned(), 0, 11)));
        let token = p._next_token();
        assert!(token.is_none());
    }
}
