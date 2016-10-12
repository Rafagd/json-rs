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
            &Json::Number(ref value) => {
                string.push_str(value.to_string().as_str());
            },
            &Json::String(ref value) => {
                string.push('"');
                for chr in value.chars() {
                    if chr == '"' {
                        string.push('\\');
                    }
                    string.push(chr);
                }
                string.push('"');
            },
            &Json::Array(ref value) => {
                let mut first = true;

                string.push('[');
                for elem in value {
                    if !first {
                        string.push(',');
                    }
                    string.push_str(elem.to_string().as_str());
                    first = false;
                }
                string.push(']');
            },
            &Json::Object(ref value) => {
                let mut first = true;

                string.push('{');
                for (k, v) in value {
                    if !first {
                        string.push(',');
                    }
                    string.push('"');
                    for chr in k.chars() {
                        if chr == '"' {
                            string.push('\\');
                        }
                        string.push(chr);
                    }
                    string.push('"');
                    string.push(':');
                    string.push_str(v.to_string().as_str());
                    first = false;
                }
                string.push('}');
            },
        }

        string
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

impl<'a> From<&'a str> for Json
{
    fn from(string: &'a str) -> Json
    {
        Json::String(String::from(string))
    }
}

impl From<u64> for Json
{
    fn from(number: u64) -> Json
    {
        Json::Number(Number::Unsigned(number))
    }
}

impl From<i32> for Json
{
    fn from(number: i32) -> Json
    {
        Json::Number(Number::Integer(number as i64))
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

impl From<Json> for HashMap<String, Json>
{
    fn from(json: Json) -> HashMap<String, Json>
    {
        if let Json::Object(ref value) = json {
            return value.clone();

        } else {
            panic!("Expecting Json::Boolean, got {:?}", json);
        }
    }
}

impl From<Json> for Vec<Json>
{
    fn from(json: Json) -> Vec<Json>
    {
        if let Json::Array(ref value) = json {
            return value.clone();

        } else {
            panic!("Expecting Json::Boolean, got {:?}", json);
        }
    }
}

impl From<Json> for String
{
    fn from(json: Json) -> String
    {
        if let Json::String(ref value) = json {
            return value.clone();

        } else {
            panic!("Expecting Json::String, got {:?}", json);
        }
    }
}

impl From<Json> for u64
{
    fn from(json: Json) -> u64
    { 
        match json {
            Json::Number(value) => value.into(),
            _ => {
                panic!("Expecting Json::Number, got {:?}", json);
            }
        }
    }
}

impl From<Json> for i64
{
    fn from(json: Json) -> i64
    {
        if let Json::Number(ref value) = json {
            value.clone().into()

        } else {
            panic!("Expecting Json::Number, got {:?}", json);
        }
    }
}

impl From<Json> for f64
{
    fn from(json: Json) -> f64
    {
        if let Json::Number(ref value) = json {
            value.clone().into()

        } else {
            panic!("Expecting Json::Number, got {:?}", json);
        }
    }
}

impl From<Json> for bool
{
    fn from(json: Json) -> bool
    {
        if let Json::Boolean(value) = json {
            return value;

        } else {
            panic!("Expecting Json::Boolean, got {:?}", json);
        }
    }
}

impl From<Json> for ()
{
    fn from(json: Json) -> ()
    {
        if json != Json::Null {
            panic!("Expecting Json::Null, got {:?}", json);
        }
    }
}

