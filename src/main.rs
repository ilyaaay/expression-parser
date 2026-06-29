mod errors;
mod lexer;

use errors::AppError;
use lexer::Lexer;
use std::io;

fn main() -> Result<(), AppError> {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf)?;

    Lexer(&buf).get_lexems()?;

    Ok(())
}
