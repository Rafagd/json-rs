use std::iter::Peekable;
use std::str::Chars;

use option::JsonOption;
use parser::JsonParser;

pub struct NumberParser;

impl JsonParser for NumberParser {
    fn parse_slice(slice: &mut Peekable<&mut Chars>) -> JsonOption {
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
}
