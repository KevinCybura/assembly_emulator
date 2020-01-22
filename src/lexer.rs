use crate::lexer::Token::Op;
use std::str::{Chars, FromStr};

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Operator {
    ADD,
    SUB,
    MOV,
    EQ,
    NEQ,
    JMP,
}

impl FromStr for Operator {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "add" => Ok(Self::ADD),
            "sub" => Ok(Self::SUB),
            "mov" => Ok(Self::MOV),
            "EQ" => Ok(Self::EQ),
            "NEQ" => Ok(Self::NEQ),
            "JMP" => Ok(Self::JMP),
            err => Err(format!("Found unsupported operation: {:?}", err)),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Token {
    Op(Operator, u32, u32),
    Ident(String, u32, u32),
    REGISTER(String, u32, u32),
    IMMEDIATE(String, u32, u32),
    EOF,
}

pub struct Lexer<'a> {
    cur: Option<char>,
    file_pos: Chars<'a>,
    row: u32,
    col: u32,
}

impl<'a> Lexer<'a> {
    pub fn new(mut chars: Chars<'a>) -> Self {
        Lexer {
            cur: chars.next(),
            file_pos: chars,
            row: 0,
            col: 0,
        }
    }

    pub fn consume(&mut self) {
        self.col += 1;
        self.cur = self.file_pos.next();
    }

    pub fn parse_token(&mut self) -> Token {
        let mut token = String::new();
        while let Some(c) = self.cur {
            self.consume();
            if c.is_whitespace() {
                if c == '\n' {
                    self.row += 1;
                    self.col = 0;
                }
                continue;
            }

            token.push(c);
            if c.is_numeric() {
                return self.intermediate(token);
            }
            match c {
                '%' => return self.register(token),
                '.' => return self.ident(token),
                _ => return self.op(token),
            }
        }
        Token::EOF
    }

    fn intermediate(&mut self, mut token: String) -> Token {
        while let Some(c) = self.cur {
            self.consume();
            if c.is_whitespace() {
                break;
            }

            if !c.is_numeric() {
                panic!("Expected a number found {:?}", c);
            }

            token.push(c)
        }
        Token::IMMEDIATE(token, self.row, self.col)
    }
    fn register(&mut self, mut token: String) -> Token {
        let c = match self.cur {
            Some(c) => c,
            None => panic!("Expected register name"),
        };
        self.consume();

        if c.is_whitespace() {
            panic!("Expect register name found white space");
        }

        if c.is_numeric() || !c.is_alphabetic() {
            panic!(
                "Registers name have the following syntax %<char><integer> found: {:?}",
                c
            );
        }
        token.push(c);

        let c = match self.cur {
            Some(c) => c,
            None => panic!("Expected register name"),
        };
        self.consume();

        if c.is_whitespace() {
            panic!("Expect register name found white space");
        }

        if !c.is_numeric() {
            panic!("Registers name have the following syntax %<char><integer>");
        }

        token.push(c);

        Token::REGISTER(token, self.row, self.col)
    }
    fn ident(&mut self, mut token: String) -> Token {
        if let Some(c) = self.cur {
            if c.is_whitespace() {
                panic!("Labels must have a least one character after the .");
            }
            token.push(c);
        }
        self.consume();

        while let Some(c) = self.cur {
            self.consume();
            if c.is_whitespace() {
                break;
            }
            token.push(c);
        }
        Token::Ident(token, self.row, self.col)
    }
    fn op(&mut self, mut token: String) -> Token {
        while let Some(c) = self.cur {
            self.consume();
            if c.is_whitespace() {
                break;
            }
            token.push(c);
        }
        let operator = Operator::from_str(&token).unwrap();
        Token::Op(operator, self.row, self.col)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_consume() {
        let mut l = Lexer::new("A String".chars());
        assert!(l.cur.is_some());
        assert_eq!(l.cur, Some('A'));
        l.consume();
        assert_eq!(l.cur, Some(' '));
        l.consume();
        assert_eq!(l.cur, Some('S'));
        l.consume();
        assert_eq!(l.cur, Some('t'));
        l.consume();
        assert_eq!(l.cur, Some('r'));
        l.consume();
        assert_eq!(l.cur, Some('i'));
        l.consume();
        assert_eq!(l.cur, Some('n'));
        l.consume();
        assert_eq!(l.cur, Some('g'));
    }

    #[test]
    fn test_ident() {
        let mut l = Lexer::new(".label".chars());
        let token = l.parse_token();
        assert_eq!(token, Token::Ident(".label".to_owned(), 0, 6));
        let mut l = Lexer::new(" .label ".chars());
        let token = l.parse_token();
        assert_eq!(token, Token::Ident(".label".to_owned(), 0, 8));
        let mut l = Lexer::new(".l".chars());
        let token = l.parse_token();
        assert_eq!(token, Token::Ident(".l".to_owned(), 0, 2));
        let mut l = Lexer::new(".1".chars());
        let token = l.parse_token();
        assert_eq!(token, Token::Ident(".1".to_owned(), 0, 2));
        let mut l = Lexer::new(" .1 ".chars());
        let token = l.parse_token();
        assert_eq!(token, Token::Ident(".1".to_owned(), 0, 4));
    }

    #[test]
    fn test_register() {
        let mut l = Lexer::new("%r1".chars());
        let token = l.parse_token();
        assert_eq!(token, Token::REGISTER("%r1".to_owned(), 0, 3));
        let mut l = Lexer::new("r1".chars());
        let s = String::from("%");
        let token = l.register(s);
        assert_eq!(token, Token::REGISTER("%r1".to_owned(), 0, 2))
    }

    #[test]
    fn test_parse_token() {
        let mut l = Lexer::new(
            "\
            .main\n
            add %r1 %r2
            SUB %r1 1
        "
            .chars(),
        );
        let token = l.parse_token();
        assert_eq!(token, Token::Ident(".main".to_owned(), 0, 6));
        let token = l.parse_token();
        assert_eq!(token, Token::Op(Operator::ADD, 1, 16));
        let token = l.parse_token();
        assert_eq!(token, Token::REGISTER("%r1".to_owned(), 1, 19));
        let token = l.parse_token();
        assert_eq!(token, Token::REGISTER("%r2".to_owned(), 1, 23));
        let token = l.parse_token();
        assert_eq!(token, Token::Op(Operator::SUB, 2, 16));
        let token = l.parse_token();
        assert_eq!(token, Token::REGISTER("%r1".to_owned(), 2, 19));
        let token = l.parse_token();
        assert_eq!(token, Token::IMMEDIATE("1".to_owned(), 2, 22));
    }
}
