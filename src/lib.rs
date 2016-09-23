use std::collections::HashMap;
use std::iter::Peekable;
use std::str::Chars;

#[derive(Debug, PartialEq)]
pub enum Error {
    UnexpectedEof,
    InvalidCharacter(String),
}

#[derive(Debug, PartialEq)]
pub enum JsonNumber {
    Integer(i64),
    Float(f64),
}

#[derive(Debug, PartialEq)]
pub enum Json {
    Object(HashMap<String, Json>),
    Array(Vec<Json>),
    String(String),
    Number(JsonNumber),
    Boolean(bool),
    Null,
}

impl Json {
    pub fn parse(text: &str) -> Result<Json, Error>
    {
        let mut slice    = text.chars();
        let mut peekable = (&mut slice).peekable();

        Json::parse_node(&mut peekable)
    }

    fn parse_node(slice: &mut Peekable<&mut Chars>) -> Result<Json, Error>
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
                    content = Json::parse_null(slice);
                },
                'f' | 't' => {
                    content = Json::parse_boolean(slice);
                },
                '0'...'9' | '-' => {
                    content = Json::parse_number(slice);
                },
                '"' => {
                    content = Json::parse_string(slice);
                },
                '[' => {
                    content = Json::parse_array(slice);
                },
                '{' => {
                    content = Json::parse_object(slice);
                },

                _ => { break 'tokenizer }
            }
        }

        content
    }

    fn parse_null(slice: &mut Peekable<&mut Chars>) -> Result<Json, Error>
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

    fn parse_boolean(slice: &mut Peekable<&mut Chars>) -> Result<Json, Error> 
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

    fn parse_number(slice: &mut Peekable<&mut Chars>) -> Result<Json, Error>
    {
        let mut token = String::new();

        #[derive(Debug, PartialEq)]
        enum Stages {
            Sign,
            AfterSign,
            AfterZero,
            Integer,
            AfterDot,
            Fraction,
            AfterExp,
            AfterExpSign,
            Exponent,
            End,
        }

        let mut stage = Stages::Sign;

        'tokenizer: loop {
            let current = match slice.peek() {
                Some(chr) => *chr,
                None      => { break 'tokenizer },
            };

            match stage {
                // Waiting for sign or number.
                Stages::Sign => match current {
                    '-'       => { stage = Stages::AfterSign; token.push(current); slice.next(); },
                    '0'       => { stage = Stages::AfterZero; token.push(current); slice.next(); },
                    '1'...'9' => { stage = Stages::Integer;   token.push(current); slice.next(); },

                    // Waiting for a number.
                    _ => {
                        return Err(Error::InvalidCharacter(current.to_string()));
                    },
                },
                Stages::AfterSign => match current {
                    '0'       => { stage = Stages::AfterZero; token.push(current); slice.next(); },
                    '1'...'9' => { stage = Stages::Integer;   token.push(current); slice.next(); },
                    '.'       => { stage = Stages::AfterDot;  token.push(current); slice.next(); },
                    'e' | 'E' => { stage = Stages::AfterExp;  token.push(current); slice.next(); },

                    // Waiting for a number.
                    _ => {
                        return Err(Error::InvalidCharacter(current.to_string()));
                    },
                },
                Stages::AfterZero => match current {
                    '.'       => { stage = Stages::AfterDot; token.push(current); slice.next(); },
                    'e' | 'E' => { stage = Stages::AfterExp; token.push(current); slice.next(); },
                    _         => { break 'tokenizer; },
                },
                Stages::Integer => match current {
                    '0'...'9' => { stage = Stages::Integer;   token.push(current); slice.next(); },
                    '.'       => { stage = Stages::AfterDot;  token.push(current); slice.next(); },
                    'e' | 'E' => { stage = Stages::AfterExp;  token.push(current); slice.next(); },
                    _         => { break 'tokenizer; },
                },
                Stages::AfterDot => match current {
                    '0'...'9' => { stage = Stages::Fraction; token.push(current); slice.next(); },
                    'e' | 'E' => { stage = Stages::AfterExp; token.push(current); slice.next(); },

                    // Waiting for a number.
                    _ => {
                        return Err(Error::InvalidCharacter(current.to_string()));
                    },
                },
                Stages::Fraction => match current {
                    '0'...'9' => { stage = Stages::Fraction; token.push(current); slice.next(); },
                    'e' | 'E' => { stage = Stages::AfterExp; token.push(current); slice.next(); },
                    _         => { break 'tokenizer; },
                },
                Stages::AfterExp => match current {
                    '+' | '-' => { stage = Stages::AfterExpSign; token.push(current); slice.next(); },
                    '0'       => { stage = Stages::End;          token.push(current); slice.next(); },
                    '1'...'9' => { stage = Stages::Exponent;     token.push(current); slice.next(); },

                    // Waiting for a number.
                    _ => {
                        return Err(Error::InvalidCharacter(current.to_string()));
                    },
                },
                Stages::AfterExpSign => match current {
                    '1'...'9' => { stage = Stages::Exponent; token.push(current); slice.next(); },

                    // Waiting for a number.
                    _ => {
                        return Err(Error::InvalidCharacter(current.to_string()));
                    },
                },
                Stages::Exponent => match current {
                    '0'...'9' => { token.push(current); slice.next(); },
                    _         => { break 'tokenizer; },
                },
                Stages::End => match current {
                    _         => { break 'tokenizer; },
                },
            }
        }

        Ok(match stage {
            Stages::Integer | Stages::AfterZero => {
                Json::Number(
                    JsonNumber::Integer(token.parse::<i64>().unwrap())
                )
            },
            _ => {
                Json::Number(
                    JsonNumber::Float(token.parse::<f64>().unwrap())
                )
            },
        })
    }

    fn parse_string(slice: &mut Peekable<&mut Chars>) -> Result<Json, Error>
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
                    token.push(std::char::from_u32(code).unwrap());

                    stage = Stages::Unescaped;
                },
                Stages::End => match current {
                    _ => { break 'tokenizer; },
                },
            }
        }

        Ok(Json::String(token))
    }

    fn parse_array(slice: &mut Peekable<&mut Chars>) -> Result<Json, Error>
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

                        let node = match Json::parse_node(slice) {
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

                    let node = match Json::parse_node(slice) {
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

    fn parse_object(slice: &mut Peekable<&mut Chars>) -> Result<Json, Error>
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
                        index = match Json::parse_string(slice) {
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

                    let node = match Json::parse_node(slice) {
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
}

