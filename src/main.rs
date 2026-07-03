mod errors;
mod lexer;
mod parser;

use errors::AppError;
use lexer::Lexer;
use std::io::{self, BufRead};

fn main() -> Result<(), AppError> {
    let mut descriptor = io::stdin().lock();

    loop {
        let mut buf = String::new();
        let read_bytes = descriptor.read_line(&mut buf)?;

        if read_bytes == 0 {
            break;
        }

        let lexems = Lexer(buf.chars()).get_lexems()?;
        println!("{lexems:?}");
    }

    Ok(())
}
