#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum OpType {
    Plus,
    Minus,
    Multiplication,
    Division,
    Modulas,
    Equal,
    EqualEqual,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum KeywordType {
    Var,
    Const,
    If,
    Int,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum TokenTypes {
    // Comment { is_line_comment: bool },
    Op(OpType),
    Comma,
    Colon,
    Semicolon,
    Integer,
    Float,
    Keyword(KeywordType),
    SingleQuote,
    DoubleQuote,
    Identifier,
    Unknown,
    Eof,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Span {
    pub start: usize,
    pub end: usize,
    pub len: usize,
}

impl Span {
    pub fn new(start: usize, end: usize) -> Self {
        Self {
            start,
            end,
            len: (end - start) + 1,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Token {
    ty: TokenTypes,
    span: Span,
}

impl Token {
    pub fn new(ty: TokenTypes, span: Span) -> Self {
        Self { ty, span }
    }
}
