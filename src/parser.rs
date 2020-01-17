use std::cell::{Cell, RefCell};

use super::lexer::{Lexer, Token, Operator};

struct Expression {
    operation: Token,
    destination: Token,
    source: Option<Token>,
}

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
    fn new(code: &'a String) -> Parser<'a> {
        Parser {
            tokens: Vec::new(),
            lexer: RefCell::new(Lexer::new(code.chars())),
            position: Cell::new(0),
        }
    }

    fn parse(&self) -> Production {
        while let Some(token) = self._next_token() {
            let prod = match token {
                Token::Ident(_, _, _) => Production::Label {
                    token,
                    position: self.position.get(),
                },
                Token::Op(operator, _, _) => self.handle_expression(token, operator),
                token => panic!("Received incorret token found: {:?}", token),
            };
        }
        Production::EOF
    }

    fn handle_expression(&self, token: Token, operator: Operator) -> Production {
        if let Some(token) = self._next_token() {

        }

        Production::EOF
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
mod tests {}
