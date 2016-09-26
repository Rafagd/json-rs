use std::collections::HashMap;
use std::iter::Peekable;
use std::str::Chars;

use error::Error;
use json::Json;
use parser::{ node, string };

pub fn object(slice: &mut Peekable<&mut Chars>) -> Result<Json, Error>
{
    let mut object = HashMap::new();
    let mut index  = String::new();

    #[derive(Debug, PartialEq)]
    enum Stages {
        Start,
        Index,
        Colon,
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
                '{' => { stage = Stages::Index; slice.next(); },

                // Waiting for quotation mark.
                _ => {
                    return Err(Error::InvalidCharacter(current.to_string()));
                },
            },
            Stages::Index => match current {
                '}' => { stage = Stages::End; },
                _   => {
                    stage = Stages::Colon;
                    index = match string(slice) {
                        Ok(Json::String(index)) => index,
                        Err(e) => { return Err(e); },
                        _      => { return Err(Error::InvalidCharacter(current.to_string())); }
                    };
                },
            },
            Stages::Colon => match current {
                ':' => { stage = Stages::Value; slice.next(); },

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

                object.insert(index.clone(), node);
            },
            Stages::Comma => match current {
                ',' => { stage = Stages::Index; slice.next(); },
                '}' => { stage = Stages::End; },

                // Waiting for valid escape code.
                _ => {
                    return Err(Error::InvalidCharacter(current.to_string()));
                },
            },
            Stages::End => match current {
                '}' => { slice.next(); break 'tokenizer; },
                // Waiting for valid escape code.
                _ => {
                    return Err(Error::InvalidCharacter(current.to_string()));
                },
            },
        }
    }

    Ok(Json::Object(object))
}

