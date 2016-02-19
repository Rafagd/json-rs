use std::collections::HashMap;
use std::iter::Peekable;
use std::str::Chars;

use option::JsonOption;
use parser::JsonParser;
use node::NodeParser;

pub struct ObjectParser;

impl JsonParser for ObjectParser {
    fn parse_slice(slice: &mut Peekable<&mut Chars>) -> JsonOption {
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
                    None      => { break 'tokenizer },
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
                            key = match NodeParser::parse_slice(slice) {
                                JsonOption::String(string) => string,
                                _ => panic!("Object has invalid element as key."),
                            };

                        } else {
                            value      = NodeParser::parse_slice(slice);
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

        JsonOption::Object(content)
    }
}
