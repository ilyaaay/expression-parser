mod tokenizer;

use std::io;
use tokenizer::Error;

fn main() -> Result<(), Error> {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf)?;

    let x = tokenizer::Tokenizer(&buf).parse()?;

    println!("tokens: {:?}", x);

    Ok(())
}
