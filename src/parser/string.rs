use std::iter::Peekable;
use std::str::Chars;
use ::std::char;

use error::Error;
use json::Json;

pub fn string(slice: &mut Peekable<&mut Chars>) -> Result<Json, Error>
{
    let mut token   = String::new();
    let mut unicode = String::new();

    #[derive(Debug, PartialEq)]
    enum Stages {
        Start,
        Unescaped,
        Escaped,
        EscapedUnicode,
        AfterUnicode,
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
                '"'       => { stage = Stages::Unescaped; slice.next(); },

                // Waiting for quotation mark.
                _ => {
                    return Err(Error::InvalidCharacter(current.to_string()));
                },
            },
            Stages::Unescaped => match current {
                '\\' => { stage = Stages::Escaped; slice.next(); },
                '"'  => { stage = Stages::End;     slice.next(); },
                _    => { token.push(current);     slice.next(); },
            },
            Stages::Escaped => match current {
                'u'  => { stage = Stages::EscapedUnicode; unicode = String::new(); slice.next(); },
                '"'  => { stage = Stages::Unescaped; token.push('\u{0022}'); slice.next(); },
                '\\' => { stage = Stages::Unescaped; token.push('\u{005C}'); slice.next(); },
                '/'  => { stage = Stages::Unescaped; token.push('\u{002F}'); slice.next(); },
                'b'  => { stage = Stages::Unescaped; token.push('\u{0008}'); slice.next(); },
                'f'  => { stage = Stages::Unescaped; token.push('\u{000C}'); slice.next(); },
                'n'  => { stage = Stages::Unescaped; token.push('\u{000A}'); slice.next(); },
                'r'  => { stage = Stages::Unescaped; token.push('\u{000D}'); slice.next(); },
                't'  => { stage = Stages::Unescaped; token.push('\u{0009}'); slice.next(); },

                // Waiting for valid escape code.
                _ => {
                    return Err(Error::InvalidCharacter(current.to_string()));
                },
            },
            Stages::EscapedUnicode => match current {
                '0'...'9' => { unicode.push(current); slice.next(); },
                'A'...'F' => { unicode.push(current); slice.next(); },
                'a'...'f' => { unicode.push(current); slice.next(); },
                _ => { stage = Stages::AfterUnicode; }
            },
            Stages::AfterUnicode => {
                if unicode.len() != 4 {
                    return Err(Error::InvalidCharacter(format!("U+{}", unicode)));
                }

                let code = u32::from_str_radix(unicode.as_str(), 16).unwrap();
                token.push(char::from_u32(code).unwrap());

                stage = Stages::Unescaped;
            },
            Stages::End => match current {
                _ => { break 'tokenizer; },
            },
        }
    }

    Ok(Json::String(token))
}

