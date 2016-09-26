use std::collections::HashMap;

use error::Error;
use parser::node;
use number::Number;

#[derive(Clone, Debug, PartialEq)]
pub enum Json {
    Object(HashMap<String, Json>),
    Array(Vec<Json>),
    String(String),
    Number(Number),
    Boolean(bool),
    Null,
}

impl Json {
    pub fn parse(text: &str) -> Result<Json, Error>
    {
        let mut slice    = text.chars();
        let mut peekable = (&mut slice).peekable();

        node(&mut peekable)
    }

    pub fn to_string(&self) -> String
    {
        let mut string: String = String::new();

        match self {
            &Json::Null => {
                string.push_str("null");
            },
            &Json::Boolean(value) => {
                string.push_str(
                    if value { "true"  }
                    else     { "false" }
                );
            },
            _ => {},
        }

        string
    }

    pub fn as_null(&self) -> ()
    {
        if *self != Json::Null {
            panic!("Expecting Json::Null, got {:?}", self);
        }
    }

    pub fn as_bool(&self) -> bool
    {
        if let Json::Boolean(value) = *self {
            return value;

        } else {
            panic!("Expecting Json::Boolean, got {:?}", self);
        }
    }

    pub fn as_number(&self) -> Number
    {
        if let Json::Number(ref value) = *self {
            value.clone()

        } else {
            panic!("Expecting Json::Number, got {:?}", self);
        }
    }

    pub fn as_string_ptr(&self) -> &String
    {
        if let Json::String(ref value) = *self {
            return value;

        } else {
            panic!("Expecting Json::String, got {:?}", self);
        }
    }

    pub fn as_string(&self) -> String
    {
        self.as_string_ptr().clone()
    }

    pub fn as_array_ptr(&self) -> &Vec<Json>
    {
        if let Json::Array(ref value) = *self {
            return value;

        } else {
            panic!("Expecting Json::Boolean, got {:?}", self);
        }
    }

    pub fn as_array(&self) -> Vec<Json>
    {
        self.as_array_ptr().clone()
    }

    pub fn as_object_ptr(&self) -> &HashMap<String, Json>
    {
        if let Json::Object(ref value) = *self {
            return value;

        } else {
            panic!("Expecting Json::Boolean, got {:?}", self);
        }
    }

    pub fn as_object(&self) -> HashMap<String, Json>
    {
        self.as_object_ptr().clone()
    }
}
impl From<HashMap<String, Json>> for Json
{
    fn from(map: HashMap<String, Json>) -> Json
    {
        Json::Object(map)
    }
}

impl From<Vec<Json>> for Json
{
    fn from(vector: Vec<Json>) -> Json
    {
        Json::Array(vector)
    }
}

impl From<String> for Json
{
    fn from(string: String) -> Json
    {
        Json::String(string)
    }
}

impl From<u64> for Json
{
    fn from(number: u64) -> Json
    {
        Json::Number(Number::Unsigned(number))
    }
}

impl From<i64> for Json
{
    fn from(number: i64) -> Json
    {
        Json::Number(Number::Integer(number))
    }
}

impl From<f64> for Json
{
    fn from(number: f64) -> Json
    {
        Json::Number(Number::Float(number))
    }
}

impl From<bool> for Json
{
    fn from(value: bool) -> Json
    {
        Json::Boolean(value)
    }
}

impl From<()> for Json
{
    fn from(_: ()) -> Json
    {
        Json::Null
    }
}

