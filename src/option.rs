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
    pub fn object(&self) -> &Self {
        match *self {
            JsonOption::Object(_) => self,
            _ => panic!("Expecting Object got {}", self),
        }
    }

    pub fn array(&self) -> &Self {
        match *self {
            JsonOption::Array(_) => self,
            _ => panic!("Expecting Array got {}", self),
        }
    }

    pub fn string(&self) -> &Self {
        match *self {
            JsonOption::String(_) => self,
            _ => panic!("Expecting String got {}", self),
        }
    }

    pub fn integer(&self) -> &Self {
        match *self {
            JsonOption::Integer(_) => self,
            _ => panic!("Expecting Integer got {}", self),
        }
    }

    pub fn number(&self) -> &Self {
        match *self {
            JsonOption::Number(_) => self,
            _ => panic!("Expecting Number got {}", self),
        }
    }

    pub fn none(&self) -> &Self {
        match *self {
            JsonOption::None => self,
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

