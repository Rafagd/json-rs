use std::iter::Peekable;
use std::str::Chars;

use error::Error;
use json::Json;

pub fn null(slice: &mut Peekable<&mut Chars>) -> Result<Json, Error>
{
    let s = "null";

    for c in s.chars() {
        let current = match slice.next() {
            Some(chr) => chr,
            None      => { return Err(Error::UnexpectedEof); }
        };

        if current != c {
            return Err(Error::InvalidCharacter(current.to_string()));
        }
    }

    Ok(Json::Null)
}

