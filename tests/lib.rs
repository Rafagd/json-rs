use std::collections::HashMap;

extern crate simple_json;
use simple_json::{ Json, JsonNumber };

#[test]
fn valid_null()
{
	let json = Json::parse("null");
	assert_eq!(json, Ok(Json::Null));
}

#[test]
fn valid_bool()
{
	let json = Json::parse("false");
	assert_eq!(json, Ok(Json::Boolean(false)));
	
	let json = Json::parse("true");
	assert_eq!(json, Ok(Json::Boolean(true)));
}

#[test]
fn valid_integer_number()
{
	let json = Json::parse("0");
	assert_eq!(json, Ok(Json::Number(JsonNumber::Integer(0))));

	let json = Json::parse("-0");
	assert_eq!(json, Ok(Json::Number(JsonNumber::Integer(0))));

	let json = Json::parse("1");
	assert_eq!(json, Ok(Json::Number(JsonNumber::Integer(1))));

	let json = Json::parse("-1");
	assert_eq!(json, Ok(Json::Number(JsonNumber::Integer(-1))));

	let json = Json::parse("20");
	assert_eq!(json, Ok(Json::Number(JsonNumber::Integer(20))));

	let json = Json::parse("-20");
	assert_eq!(json, Ok(Json::Number(JsonNumber::Integer(-20))));

	let json = Json::parse("21");
	assert_eq!(json, Ok(Json::Number(JsonNumber::Integer(21))));

	let json = Json::parse("-21");
	assert_eq!(json, Ok(Json::Number(JsonNumber::Integer(-21))));
}

#[test]
fn valid_float_number()
{
	let json = Json::parse("0.0");
	assert_eq!(json, Ok(Json::Number(JsonNumber::Float(0.0))));

	let json = Json::parse("-0.0");
	assert_eq!(json, Ok(Json::Number(JsonNumber::Float(0.0))));

	let json = Json::parse("1.1");
	assert_eq!(json, Ok(Json::Number(JsonNumber::Float(1.1))));

	let json = Json::parse("-1.1");
	assert_eq!(json, Ok(Json::Number(JsonNumber::Float(-1.1))));

	let json = Json::parse("20.01");
	assert_eq!(json, Ok(Json::Number(JsonNumber::Float(20.01))));

	let json = Json::parse("-20.01");
	assert_eq!(json, Ok(Json::Number(JsonNumber::Float(-20.01))));

	let json = Json::parse("21.12");
	assert_eq!(json, Ok(Json::Number(JsonNumber::Float(21.12))));

	let json = Json::parse("-21.12");
	assert_eq!(json, Ok(Json::Number(JsonNumber::Float(-21.12))));
}

#[test]
fn valid_exponent_number()
{
	let json = Json::parse("0e0");
	assert_eq!(json, Ok(Json::Number(JsonNumber::Float(0.))));

	let json = Json::parse("-0e0");
	assert_eq!(json, Ok(Json::Number(JsonNumber::Float(0.))));

	let json = Json::parse("1e1");
	assert_eq!(json, Ok(Json::Number(JsonNumber::Float(1e1))));

	let json = Json::parse("-1e1");
	assert_eq!(json, Ok(Json::Number(JsonNumber::Float(-1e1))));

	let json = Json::parse("20.01e10");
	assert_eq!(json, Ok(Json::Number(JsonNumber::Float(20.01e10))));

	let json = Json::parse("-20.01e10");
	assert_eq!(json, Ok(Json::Number(JsonNumber::Float(-20.01e10))));

	let json = Json::parse("21.12e2");
	assert_eq!(json, Ok(Json::Number(JsonNumber::Float(21.12e2))));

	let json = Json::parse("-21.12e2");
	assert_eq!(json, Ok(Json::Number(JsonNumber::Float(-21.12e2))));
}

#[test]
fn valid_string()
{
	let json = Json::parse("\"\"");
	assert_eq!(json, Ok(Json::String(String::new())));

	let json = Json::parse("\"This is a normal ASCII string.\"");
	assert_eq!(json, Ok(Json::String(String::from("This is a normal ASCII string."))));

	let json = Json::parse("\"I also use unicode: ٩(-̮̮̃-̃)۶ ٩(●̮̮̃•̃)۶ ٩(͡๏̯͡๏)۶ ٩(-̮̮̃•̃).\"");
	assert_eq!(json, Ok(Json::String(String::from("I also use unicode: ٩(-̮̮̃-̃)۶ ٩(●̮̮̃•̃)۶ ٩(͡๏̯͡๏)۶ ٩(-̮̮̃•̃)."))));

	let json = Json::parse("\"I can escape some things, like \\\"\\\\\\/\\b\\f\\n\\r\\t!\"");
	assert_eq!(json, Ok(Json::String(String::from("I can escape some things, like \"\\/\u{0008}\u{000C}\n\r\t!"))));

	let json = Json::parse("\"I can even escape unicode: \\u3042.\"");
	assert_eq!(json, Ok(Json::String(String::from("I can even escape unicode: あ."))));
}

#[test]
fn valid_array()
{
	let json = Json::parse("[]");
	assert_eq!(json, Ok(Json::Array(vec![])));

	let json = Json::parse("[1,2.0,\"String\",[],{}]");
	assert_eq!(json, Ok(Json::Array(vec![
        Json::Number(JsonNumber::Integer(1)),
        Json::Number(JsonNumber::Float(2.)),
        Json::String(String::from("String")),
        Json::Array(vec![]),
        Json::Object(HashMap::new()),
    ])));
}

#[test]
fn valid_object()
{
	let json = Json::parse("{}");
	assert_eq!(json, Ok(Json::Object(HashMap::new())));

    let mut map = HashMap::new();
    map.insert(String::from("i"), Json::Number(JsonNumber::Integer(1)));
    map.insert(String::from("f"), Json::Number(JsonNumber::Float(2.)));
    map.insert(String::from("s"), Json::String(String::from("String")));
    map.insert(String::from("a"), Json::Array(vec![]));
    map.insert(String::from("o"), Json::Object(HashMap::new()));

    let json = Json::parse("{\"i\":1,\"f\":2.0,\"s\":\"String\",\"a\":[],\"o\":{}}");
	assert_eq!(json, Ok(Json::Object(map)));
}
