use std::collections::HashMap;

extern crate simple_json;
use simple_json::{ Json, Number };

#[test]
fn test_object_whitespaces()
{
    let text   = "{\n    \"size\": { \"x\": 800, \"y\": 600 }\n}";
    let result = Json::parse(text).unwrap();

    let mut size = HashMap::new();
    size.insert(String::from("x"), Json::Number(Number::Unsigned(800)));
    size.insert(String::from("y"), Json::Number(Number::Unsigned(600)));

    let mut correct = HashMap::new();
    correct.insert(String::from("size"), Json::Object(size));
    
    assert_eq!(result, Json::Object(correct));
}


