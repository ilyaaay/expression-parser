#[derive(Debug)]
pub enum ParserErrors {
    Unexpected { position: usize, character: char },
}
