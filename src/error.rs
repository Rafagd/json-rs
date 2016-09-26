#[derive(Debug, PartialEq)]
pub enum Error {
    UnexpectedEof,
    InvalidCharacter(String),
}

