use std::iter::Peekable;
use std::str::Chars;

use option::JsonOption;
use parser::JsonParser;
use number::NumberParser;
use string::StringParser;
use array::ArrayParser;
use object::ObjectParser;

pub struct NodeParser;

impl JsonParser for NodeParser {
    fn parse_slice(slice: &mut Peekable<&mut Chars>) -> JsonOption {
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
                    content = NumberParser::parse_slice(slice);
                },
                '"' => {
                    content = StringParser::parse_slice(slice);
                },
                '[' => {
                    content = ArrayParser::parse_slice(slice);
                },
                '{' => {
                    content = ObjectParser::parse_slice(slice);
                },
                _ => { break 'tokenizer; }
            }
        }

        content
    }
}
