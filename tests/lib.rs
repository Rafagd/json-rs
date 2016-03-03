extern crate json;

use json::{ Json, JsonOption };
use std::collections::HashMap;

#[test]
fn parse_integer() {
    let mut json = Json::parse("1337");
    assert_eq!(json, JsonOption::Integer(1337));

    json = Json::parse("-1337");
    assert_eq!(json, JsonOption::Integer(-1337));

    json = Json::parse("0");
    assert_eq!(json, JsonOption::Integer(0));
}

#[test]
fn parse_number() {
    let mut json = Json::parse("1.337");
    assert_eq!(json, JsonOption::Number(1.337));

    json = Json::parse("-1.337");
    assert_eq!(json, JsonOption::Number(-1.337));

    json = Json::parse("0.0");
    assert_eq!(json, JsonOption::Number(0.0));
}

#[test]
fn parse_string() {
    let mut json = Json::parse("\"Hello\"");
    assert_eq!(json, JsonOption::String("Hello".to_string()));

    json = Json::parse("\"He\\\"llo\"");
    assert_eq!(json, JsonOption::String("He\"llo".to_string()));

    json = Json::parse("\"He\\\\llo\"");
    assert_eq!(json, JsonOption::String("He\\llo".to_string()));
}

#[test]
fn parse_array() {
    let mut json = Json::parse("[]");
    assert_eq!(json, JsonOption::Array(vec![]));
    
    json = Json::parse("[1]");
    assert_eq!(json, JsonOption::Array(vec![
        JsonOption::Integer(1)
    ]));
    
    json = Json::parse("[1,\"string\",]");
    assert_eq!(json, JsonOption::Array(vec![
        JsonOption::Integer(1),
        JsonOption::String("string".to_string()),
    ]));
    
    json = Json::parse("[1,\"string\",[]]");
    assert_eq!(json, JsonOption::Array(vec![
        JsonOption::Integer(1),
        JsonOption::String("string".to_string()),
        JsonOption::Array(vec![]),
    ]));
}

#[test]
fn parse_object() {
    let mut json = Json::parse("{}");
    let mut map  = HashMap::new();
    assert_eq!(json, JsonOption::Object(map));
    
    json = Json::parse("{\"x\":0}");
    map  = HashMap::new();
    map.insert("x".to_string(), JsonOption::Integer(0));
    assert_eq!(json, JsonOption::Object(map));
    
    json = Json::parse("{\"x\":0, \"y\":[1]}");
    map  = HashMap::new();
    map.insert("x".to_string(), JsonOption::Integer(0));
    map.insert("y".to_string(), JsonOption::Array(vec![JsonOption::Integer(1)]));
    assert_eq!(json, JsonOption::Object(map));
}
