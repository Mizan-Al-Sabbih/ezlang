use crate::utility::Span;
use colored::Colorize;

#[derive(Debug, PartialEq, Eq)]
pub enum DiagonosticTypes {
    Err,
    Warning,
}

#[derive(Debug, PartialEq, Eq)]
struct Diagonostic<'a> {
    ty: DiagonosticTypes,
    msg: &'a str,
    span: Span,
}

impl<'a> Diagonostic<'a> {
    fn new(ty: DiagonosticTypes, msg: &'a str, span: Span) -> Self {
        Self { ty, msg, span }
    }
}

#[derive(Debug)]
pub struct DiagonosticsBag<'a> {
    source: &'a str,
    bag: Vec<Diagonostic<'a>>,
}

impl<'a> DiagonosticsBag<'a> {
    pub fn new(source: &'a str) -> Self {
        Self {
            source,
            bag: vec![],
        }
    }

    pub fn report_error(&mut self, msg: &'a str, span: Span) {
        self.bag
            .push(Diagonostic::new(DiagonosticTypes::Err, msg, span))
    }

    pub fn report_warning(&mut self, msg: &'a str, span: Span) {
        self.bag
            .push(Diagonostic::new(DiagonosticTypes::Warning, msg, span))
    }

    pub fn show_diagonostic(&self) {
        for diag in &self.bag {
            if diag.ty == DiagonosticTypes::Err {
                eprintln!(
                    "{} {} [{}, {}]",
                    "Error:".bright_red(),
                    diag.msg,
                    diag.span.start,
                    diag.span.end
                );
            } else if diag.ty == DiagonosticTypes::Warning {
                eprintln!(
                    "{} {} [{}, {}]",
                    "Warning".bright_yellow(),
                    diag.msg,
                    diag.span.start,
                    diag.span.end
                );
            }
        }
    }
}
