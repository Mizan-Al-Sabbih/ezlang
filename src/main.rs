pub mod compiler;
pub mod diagonostic;
pub mod lexer;
pub mod utility;

fn main() {
    let src = "id";
    let comp = compiler::Compiler::new(src);
    println!("{:?}", comp.lex_tokens());
}
