use std::collections::HashMap;
use std::fmt::{ Display, Error, Formatter };

#[derive(Debug, PartialEq)]
pub enum JsonOption {
    Object(HashMap<String, JsonOption>),
    Array(Vec<JsonOption>),
    String(String),
    Integer(i32),
    Number(f32),
    None,
}

impl Display for JsonOption {
    fn fmt(&self, formatter: &mut Formatter) -> Result<(), Error> {
        match *self {
            JsonOption::Object(ref object) => {
                write!(formatter, "{:?}", object)
            },
            JsonOption::Array(ref vector) => {
                write!(formatter, "{:?}", vector)
            },
            JsonOption::String(ref string) => {
                write!(formatter, "String({})", string)
            },
            JsonOption::Integer(integer) => {
                write!(formatter, "Integer({})", integer)
            },
            JsonOption::Number(number) => {
                write!(formatter, "Number({})", number)
            },
            JsonOption::None => {
                write!(formatter, "None")
            },
        }
    }
}

