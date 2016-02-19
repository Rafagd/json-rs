use std::iter::Peekable;
use std::str::Chars;

use option::JsonOption;
use parser::JsonParser;
use node::NodeParser;

pub struct ArrayParser;

impl JsonParser for ArrayParser {
    fn parse_slice(slice: &mut Peekable<&mut Chars>) -> JsonOption {
        let mut content: Vec<JsonOption> = vec![];
        let mut proper_sep = true;
        let mut proper_end = false;
        
        // Consume starting [
        slice.next();

        'tokenizer: loop {
            loop {
                let chr = match slice.peek() {
                    Some(chr) => *chr,
                    None      => { break 'tokenizer },
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

                        content.push(NodeParser::parse_slice(slice));
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
}
