extern crate core;

pub mod json_option;

pub use json_option::JsonOption;

use std::collections::HashMap;
use std::iter::Peekable;
use std::str::Chars;

use core::fmt::{ Display, Formatter, Error };

pub struct Json;

impl Json {
    pub fn parse(text: &str) -> JsonOption {
        let mut char_slice = text.chars();
        Json::parse_slice(&mut (&mut char_slice).peekable())
    }

    pub fn parse_slice(slice: &mut Peekable<&mut Chars>) -> JsonOption {
        let mut content = JsonOption::None;

        'tokenizer: loop {
            let current = match slice.peek() {
                Some(chr) => *chr,
                _         => { break 'tokenizer; }
            };

            match current {
                ' ' | '\r' | '\n' | '\t' => {
                    slice.next();
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
                _ => { break 'tokenizer; }
            }
        }

        content
    }

    fn parse_number(slice: &mut Peekable<&mut Chars>) -> JsonOption {
        let mut token    = String::from("");
        let mut is_float = false;
        
        'tokenizer: loop {
            let chr = match slice.peek() {
                Some(chr) => *chr,
                None      => break 'tokenizer,
            };

            match chr {
                '-' | '0'...'9' => {
                    token.push(chr);
                    slice.next();
                },
                '.' => {
                    token.push(chr);
                    is_float = true;
                    slice.next();
                },
                _ => { break 'tokenizer; }
            }
        }

        if is_float {
            JsonOption::Number(token.parse::<f32>().unwrap())
        } else {
            JsonOption::Integer(token.parse::<i32>().unwrap())
        }
    }

    fn parse_string(slice: &mut Peekable<&mut Chars>) -> JsonOption {
        let mut token      = String::from("");
        let mut proper_end = false;
        let mut is_escaped = false;
        
        // Consume starting "
        slice.next();

        'tokenizer: for chr in slice {
            match chr {
                '\\' => {
                    if is_escaped {
                        token.push(chr);
                        is_escaped = false;

                    } else {
                        is_escaped = true;
                    }
                },
                '"' => {
                    if is_escaped {
                        token.push(chr);
                        is_escaped = false;

                    } else {
                        proper_end = true;
                        break 'tokenizer;
                    }
                },
                _ => { token.push(chr); }
            }
        }

        if !proper_end {
            panic!("String wasn't properly terminated.");
        }

        JsonOption::String(token)
    }

    fn parse_array(slice: &mut Peekable<&mut Chars>) -> JsonOption {
        let mut content: Vec<JsonOption> = vec![];
        let mut proper_sep = true;
        let mut proper_end = false;
        
        // Consume starting [
        slice.next();

        'tokenizer: loop {
            loop {
                let chr = match slice.peek() {
                    Some(chr) => *chr,
                    None      => { println!("Broke"); break 'tokenizer },
                };

                match chr {
                    ' ' | '\n' | '\r' | '\t' => { slice.next(); },
                    ',' => {
                        if proper_sep {
                            panic!("Array has no elements between separator.");
                        }

                        proper_sep = true;
                        slice.next();
                    },
                    ']' => {
                        proper_sep = true;
                        proper_end = true;
                        slice.next();
                        break 'tokenizer;
                    },
                    _ => {
                        if !proper_sep {
                            break 'tokenizer;
                        }

                        content.push(Json::parse_slice(slice));
                        proper_sep = false;
                    }
                }
            }
        }

        if !proper_sep {
            panic!("Array has elements without separator.");
        }

        if !proper_end {
            panic!("Array wasn't properly terminated.");
        }

        JsonOption::Array(content)
    }

    fn parse_object(slice: &mut Peekable<&mut Chars>) -> JsonOption {
        let mut content    = HashMap::new();
        let mut proper_kv  = false;
        let mut proper_sep = true;
        let mut proper_end = false;
        
        let mut value;
        // Consume starting {
        slice.next();

        'tokenizer: loop {
            let mut key   = String::from("");

            'sepcheck: loop {
                let chr = match slice.peek() {
                    Some(chr) => *chr,
                    None      => { println!("Broke"); break 'tokenizer },
                };

                match chr {
                    ' ' | '\n' | '\r' | '\t' => { slice.next(); },
                    ',' => {
                        if proper_sep {
                            panic!("Object has no elements between separator.");
                        }

                        proper_sep = true;
                        slice.next();
                    },
                    ':' => {
                        if proper_kv {
                            panic!("Object has two or more key-value separators together.");
                        }

                        proper_kv = true;
                        slice.next();
                    },
                    '}' => {
                        proper_sep = true;
                        proper_end = true;
                        slice.next();
                        break 'tokenizer;
                    },
                    _ => {
                        if !proper_sep {
                            break 'tokenizer;
                        }

                        if !proper_kv && (key != "") {
                            panic!("Object has no separator between key and value.");
                        }

                        if !proper_kv {
                            key = match Json::parse_slice(slice) {
                                JsonOption::String(string) => string,
                                _ => panic!("Object has invalid element as key."),
                            };

                        } else {
                            value      = Json::parse_slice(slice);
                            proper_kv  = false;
                            proper_sep = false;
                            break 'sepcheck;
                        }
                    }
                }
            }

            content.insert(key, value);
        }

        if !proper_sep {
            panic!("Object has elements without separator.");
        }

        if !proper_end {
            panic!("Object wasn't properly terminated.");
        }

        println!("Token: {:?}", content);

        JsonOption::Object(content)
    }
}

impl Display for Json {
    fn fmt(&self, formatter: &mut Formatter) -> Result<(), Error> {
        write!(formatter, "{}", self)
    }
}

