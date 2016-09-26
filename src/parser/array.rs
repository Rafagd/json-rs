use std::iter::Peekable;
use std::str::Chars;

use error::Error;
use json::Json;
use parser::node;

pub fn array(slice: &mut Peekable<&mut Chars>) -> Result<Json, Error>
{
    let mut array = vec![];

    #[derive(Debug, PartialEq)]
    enum Stages {
        Start,
        FirstValue,
        Value,
        Comma,
        End,
    }

    let mut stage = Stages::Start;

    'tokenizer: loop {
        let current = match slice.peek() {
            Some(chr) => *chr,
            None      => { break 'tokenizer },
        };

        match stage {
            Stages::Start => match current {
                '[' => { stage = Stages::FirstValue; slice.next(); },

                // Waiting for quotation mark.
                _ => {
                    return Err(Error::InvalidCharacter(current.to_string()));
                },
            },
            Stages::FirstValue => match current {
                ']' => { stage = Stages::End; },
                _   => {
                    stage = Stages::Comma;

                    let node = match node(slice) {
                        Ok(node) => node,
                        Err(e)   => { return Err(e) },
                    };

                    array.push(node);
                },
            },
            Stages::Comma => match current {
                ',' => { stage = Stages::Value; slice.next(); },
                ']' => { stage = Stages::End; },

                // Waiting for valid escape code.
                _ => {
                    return Err(Error::InvalidCharacter(current.to_string()));
                },
            },
            Stages::Value => {
                stage = Stages::Comma;

                let node = match node(slice) {
                    Ok(node) => node,
                    Err(e)   => { return Err(e) },
                };

                array.push(node);
            },
            Stages::End => match current {
                ']' => { slice.next(); break 'tokenizer; },
                // Waiting for valid escape code.
                _ => {
                    return Err(Error::InvalidCharacter(current.to_string()));
                },
            },
        }
    }

    Ok(Json::Array(array))
}

