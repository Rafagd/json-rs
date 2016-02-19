use std::iter::Peekable;
use std::str::Chars;

use option::JsonOption;

pub trait JsonParser {
    fn parse_slice(slice: &mut Peekable<&mut Chars>) -> JsonOption;
}
