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

impl JsonOption {
    pub fn object(&self) -> &HashMap<String, JsonOption> {
        match *self {
            JsonOption::Object(ref object) => object,
            _ => panic!("Expecting Object got {}", self),
        }
    }

    pub fn array(&self) -> &Vec<JsonOption> {
        match *self {
            JsonOption::Array(ref array) => array,
            _ => panic!("Expecting Array got {}", self),
        }
    }

    pub fn string(&self) -> &String {
        match *self {
            JsonOption::String(ref string) => string,
            _ => panic!("Expecting String got {}", self),
        }
    }

    pub fn integer(&self) -> &i32 {
        match *self {
            JsonOption::Integer(ref integer) => integer,
            _ => panic!("Expecting Integer got {}", self),
        }
    }

    pub fn number(&self) -> &f32 {
        match *self {
            JsonOption::Number(ref number) => number,
            _ => panic!("Expecting Number got {}", self),
        }
    }

    pub fn none(&self) {
        match *self {
            _ => panic!("Expecting None got {}", self),
        }
    }
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

