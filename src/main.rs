mod errors;
mod lexer;
mod parser;

use errors::AppError;
use lexer::Lexer;
use std::io;

fn main() -> Result<(), AppError> {
    loop {
        let mut buf = String::new();
        let descriptor = io::stdin();
        descriptor.read_line(&mut buf)?;

        let lexems = Lexer(buf.chars()).get_lexems()?;
        println!("{lexems:?}");
    }
}
