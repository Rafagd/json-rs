use std::iter::Peekable;
use std::str::Chars;

use option::JsonOption;
use parser::JsonParser;

pub struct StringParser;

impl JsonParser for StringParser {
    fn parse_slice(slice: &mut Peekable<&mut Chars>) -> JsonOption {
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
}
