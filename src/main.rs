mod errors;
mod lexer;
mod parser;

use errors::AppError;
use lexer::Lexer;
use parser::Parser;
use std::io;

fn main() -> Result<(), AppError> {
    loop {
        let mut buf = String::new();
        let discriptor = io::stdin();
        discriptor.read_line(&mut buf)?;

        let x = Lexer(&buf.trim_end()).get_lexems()?;
        println!("{x:?}");
    }

    Ok(())
}
