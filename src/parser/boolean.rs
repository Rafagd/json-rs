use std::iter::Peekable;
use std::str::Chars;

use error::Error;
use json::Json;

pub fn boolean(slice: &mut Peekable<&mut Chars>) -> Result<Json, Error> 
{
    let value;

    let s = {
        let current = match slice.peek() {
            Some(chr) => *chr,
            None      => { return Err(Error::UnexpectedEof); }
        };

        match current {
            'f' => {
                value = false;
                "false"
            },
            't' => {
                value = true;
                "true"
            },
            _   => { return Err(Error::InvalidCharacter(current.to_string())); }
        }
    };

    for c in s.chars() {
        let current = match slice.next() {
            Some(chr) => chr,
            None      => { return Err(Error::UnexpectedEof); }
        };

        if current != c {
            return Err(Error::InvalidCharacter(current.to_string()));
        }
    }

    Ok(Json::Boolean(value))
}

