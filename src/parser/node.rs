use std::iter::Peekable;
use std::str::Chars;

use error::Error;
use json::Json;
use parser::*;

pub fn node(slice: &mut Peekable<&mut Chars>) -> Result<Json, Error>
{
    let mut content = Err(Error::UnexpectedEof);

    'tokenizer: loop {
        let current = match slice.peek() {
            Some(chr) => *chr,
            None      => { break 'tokenizer },
        };

        match current {
            ' ' | '\r' | '\n' | '\t' => {
                slice.next();
            },
            'n' => {
                content = null(slice);
            },
            'f' | 't' => {
                content = boolean(slice);
            },
            '0'...'9' | '-' => {
                content = number(slice);
            },
            '"' => {
                content = string(slice);
            },
            '[' => {
                content = array(slice);
            },
            '{' => {
                content = object(slice);
            },

            _ => { break 'tokenizer }
        }
    }

    content
}

