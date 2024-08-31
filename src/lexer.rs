use crate::compiler::Compiler;
use crate::utility::{KeywordType, Span, Token, TokenTypes};

#[derive(Debug)]
pub struct Lexer<'a> {
    compiler: &'a Compiler<'a>,
    source: &'a str,
    index: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(compiler: &'a Compiler<'a>) -> Self {
        Self {
            compiler,
            source: compiler.source,
            index: 0,
        }
    }

    fn peek(&self) -> char {
        self.source.chars().nth(self.index).unwrap()
    }

    fn peek_ahead(&self, offset: usize) -> char {
        self.source.chars().nth(offset + self.index).unwrap()
    }

    fn consume(&mut self) {
        self.index += 1
    }

    fn is_whitespace(&self) -> bool {
        matches!(self.peek(), ' ' | '\t' | '\r')
    }

    fn lex_number(&mut self) -> Token {
        let mut val = self.peek().to_string();
        let start = self.index;
        self.consume();
        while self.peek_ahead(1).is_ascii_digit() || self.peek_ahead(1) == '.' {
            val.push(self.peek());
            self.consume();
        }
        val.push(self.peek());
        let end = self.index;
        self.consume();

        use TokenTypes as T;
        if val.contains('.') {
            Token::new(T::Float, Span::new(start, end))
        } else {
            Token::new(T::Integer, Span::new(start, end))
        }
    }

    fn lex_num_or_keyword_or_id(&mut self) -> Token {
        if self.peek().is_ascii_digit() {
            return self.lex_number();
        }
        let mut val = self.peek().to_string();
        let start = self.index;
        self.consume();
        while self.peek_ahead(1).is_ascii_alphanumeric() || self.peek_ahead(1) == '_' {
            val.push(self.peek());
            self.consume();
        }
        val.push(self.peek());
        let end = self.index;
        self.consume();

        use KeywordType as KT;
        use TokenTypes as T;
        use TokenTypes::Keyword as K;
        return Token::new(
            match val.as_str() {
                "if" => K(KT::If),
                "var" => K(KT::Var),
                "const" => K(KT::Const),
                _ => T::Identifier,
            },
            Span::new(start, end),
        );
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Token;
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            use std::cmp::Ordering;
            match self.source.len().cmp(&self.index) {
                Ordering::Equal => {
                    self.consume();
                    return Some(Token::new(
                        TokenTypes::Eof,
                        Span::new(self.index - 1, self.index - 1),
                    ));
                }
                Ordering::Greater => {
                    if self.is_whitespace() {
                        self.consume();
                    } else {
                        break;
                    }
                }
                Ordering::Less => return None,
            }
        }

        let start = self.index;

        use TokenTypes as T;
        Some(match self.peek() {
            ',' => Token::new(T::Comma, Span::new(start, self.index)),
            ':' => {
                self.consume();
                Token::new(T::Colon, Span::new(start, self.index - 1))
            }
            _ => self.lex_num_or_keyword_or_id(),
        })
    }
}
