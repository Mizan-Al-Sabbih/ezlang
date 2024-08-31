use crate::diagonostic::DiagonosticsBag;
use crate::lexer::Lexer;
use crate::utility::Token;

#[derive(Debug)]
pub struct Compiler<'a> {
    pub source: &'a str,
    diagonostics: DiagonosticsBag<'a>,
}

impl<'a> Compiler<'a> {
    pub fn new(source: &'a str) -> Self {
        Self {
            source,
            diagonostics: DiagonosticsBag::new(source),
        }
    }

    pub fn lex_tokens(&self) -> Vec<Token> {
        let lexer = Lexer::new(self);
        let mut tokens = Vec::new();

        for token in lexer {
            tokens.push(token);
        }

        tokens
    }
}
