mod errors;
mod lexer;
mod parser;

use errors::AppError;
use lexer::Lexer;
use std::io::{self, ErrorKind, Read};

fn main() -> Result<(), AppError> {
    let mut buf = Vec::new();
    let mut descriptor = io::stdin().lock();
    // descriptor
    //     .read_to_end(&mut buf)
    //     .or_else(|err| match err.kind() {
    //         ErrorKind::Interrupted => Ok(0),
    //         error => return Err(error),
    //     })?;

    descriptor.read_to_end(&mut buf)?;

    let s = buf.into_iter().map(char::from).collect::<String>();

    let lexems = Lexer(s.chars()).get_lexems()?;
    println!("{lexems:?}");

    Ok(())
}
