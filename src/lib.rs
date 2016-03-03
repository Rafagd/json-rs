pub mod option;
    mod parser;
    mod node;
    mod number;
    mod string;
    mod array;
    mod object;

pub use option::JsonOption;
    use parser::JsonParser;
    use node::NodeParser;

pub struct Json;

impl Json {
    pub fn parse(text: &str) -> JsonOption {
        let mut slice = text.chars();
        NodeParser::parse_slice(&mut (&mut slice).peekable())
    }
}


