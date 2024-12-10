use std::fmt::{write, Display, Formatter};

use crate::ast::text::TextSpan;


#[derive(Debug, Clone, Eq,PartialEq,Copy)]
pub enum TokenKind {
    // Literals
    Number(i64),
    // Operators
    Plus,
    Minus,
    Asterisk,
    Slash,
    Equals,

   //Keyword
    Let,

    // Other
    OpenParen,
    CloseParen,
    Bad,
    Whitespace,
    Identifier,
    Function,
    Eof,
    Comma,
    OpenBrace,
    CloseBrace
}

impl Display for TokenKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            TokenKind::Number(_) => write!(f, "Number"),
            TokenKind::Plus => write!(f, "+"),
            TokenKind::Minus => write!(f, "-"),
            TokenKind::Asterisk => write!(f, "*"),
            TokenKind::Slash => write!(f, "/"),
            TokenKind::Bad => write!(f, "Bad"),
            TokenKind::Whitespace => write!(f, "Whitespace"),
            TokenKind::Eof => write!(f, "Eof"),
            TokenKind::Let => write!(f, "Let"),
            TokenKind::Identifier => write!(f, "Identifier"),
            TokenKind::Equals => write!(f, "="),
            TokenKind::OpenParen => write!(f,"Open parenthesis"),
            TokenKind::CloseParen => write!(f,"Closing parenthesis"),
            TokenKind::Function=>write!(f, "Function"),
            TokenKind::Comma=>write!(f, "Comma"),
            TokenKind::OpenBrace => write!(f,"Open Brace"),
            TokenKind::CloseBrace => write!(f,"Close Brace"),
          
        }
    }
}

#[derive(Debug, Clone, PartialEq,Eq)]
pub struct Token {
    pub kind: TokenKind,
    pub span: TextSpan,
}

impl Token {
    pub fn new(kind: TokenKind, span: TextSpan) -> Self {
        Self { kind, span }
    }
}

pub struct Lexer<'a> {
    input: &'a str,
    current_pos: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            input,
            current_pos: 0,
        }
    }

    pub fn next_token(&mut self) -> Option<Token> {
        if self.current_pos == self.input.len() {
            let eof_char: char = '\0';
            self.current_pos += 1;
            return Some(Token::new(
                TokenKind::Eof,
                TextSpan::new(0, 0, eof_char.to_string()),
            ));
        }
        let c = self.current_char();
        return c.map(|c| {
            let start = self.current_pos;
            let mut kind = TokenKind::Bad;
            if Self::is_number_start(&c) {
                let number: i64 = self.consume_number();
                kind = TokenKind::Number(number);
            } else if Self::is_whitespace(&c) {
                self.consume();
                kind = TokenKind::Whitespace;
            } else if Self::is_identifier_start(&c) {
                let identifier = self.consume_identifier();
                kind = match identifier.as_str() {
                    "let" => TokenKind::Let,
                    "function" => TokenKind::Function,
                    _ => TokenKind::Identifier,
                }
            } else {
                kind = self.consume_punctuation();
            }

            let end = self.current_pos;
            let literal = self.input[start..end].to_string();
            let span = TextSpan::new(start, end, literal);
            Token::new(kind, span)
        });
    }

    fn consume_punctuation(&mut self) -> TokenKind {
        let c = self.consume().unwrap();
        match c {
            '+' => TokenKind::Plus,
            '-' => TokenKind::Minus,
            '*'=> TokenKind::Asterisk,     
            '/' => TokenKind::Slash,
            '=' => TokenKind::Equals,
            '(' => TokenKind::OpenParen,
            ')'=>TokenKind::CloseParen,
            ','=>TokenKind::Comma,
            '{'=>TokenKind::OpenBrace,
            '}' => TokenKind::CloseBrace,
            _ => TokenKind::Bad,
        }
    }

    fn is_number_start(c: &char) -> bool {
        c.is_digit(10)
    }

    fn is_identifier_start(c: &char) -> bool {
        c.is_alphabetic() || c == &'_'
    }

    fn is_whitespace(c: &char) -> bool {
        c.is_whitespace()
    }

    fn current_char(&self) -> Option<char> {
        self.input.chars().nth(self.current_pos)
    }

    fn consume(&mut self) -> Option<char> {
        if self.current_pos >= self.input.len() {
            return None;
        }
        let c = self.current_char();
        self.current_pos += 1;

        c
    }

    fn consume_identifier(&mut self) -> String {
        let mut identifier = String::new();
        while let Some(c) = self.current_char() {
            if Self::is_identifier_start(&c) {
                self.consume().unwrap();
                identifier.push(c);
            } else {
                break;
            }
        }
        identifier
    }

    fn consume_number(&mut self) -> i64 {
        let mut number: i64 = 0;
        while let Some(c) = self.current_char() {
            if c.is_digit(10) {
                self.consume().unwrap();
                number = number * 10 + c.to_digit(10).unwrap() as i64;
            } else {
                break;
            }
        }
        number
    }
}
