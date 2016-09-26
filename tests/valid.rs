use std::collections::HashMap;

extern crate simple_json;
use simple_json::{ Json, Number };

macro_rules! test {
    ($T: ty, $jsn:expr, $val:expr) => {{
        let json = Json::parse($jsn).unwrap();
        let val: $T = json.into();
        assert_eq!(val, $val);
    }};
}

#[test]
fn valid_null()
{
    test!((), "null", ());
}

#[test]
fn valid_bool()
{
    test!(bool, "false", false);
    test!(bool, "true",  true);
}

#[test]
fn valid_integer_number()
{
    test!(i64, "0",  0);
    test!(i64, "1",  1);
    test!(i64, "20", 20);
    test!(i64, "21", 21);

    test!(i64, "-0",   0);
    test!(i64, "-1",  -1);
    test!(i64, "-20", -20);
    test!(i64, "-21", -21);
}

#[test]
fn valid_float_number()
{
    test!(f64, "0.0",   0.);
    test!(f64, "1.1",   1.1);
    test!(f64, "20.01", 20.01);
    test!(f64, "21.12", 21.12);

    test!(f64, "-0.0",    0.);
    test!(f64, "-1.1",   -1.1);
    test!(f64, "-20.01", -20.01);
    test!(f64, "-21.12", -21.12);
}

#[test]
fn valid_exponent_number()
{
    test!(f64, "0e0",      0.0e0);
    test!(f64, "1e1",      1.0e1);
    test!(f64, "20.01e10", 20.01e10);
    test!(f64, "21.12e2",  21.12e2);

    test!(f64, "-0e0",       0.0e0);
    test!(f64, "-1e1",      -1.0e1);
    test!(f64, "-20.01e10", -20.01e10);
    test!(f64, "-21.12e2",  -21.12e2);
}

#[test]
fn valid_string()
{
    test!(String,
        "\"\"",
        String::from("")
    );

	test!(String,
        "\"This is a normal ASCII string.\"",
        String::from("This is a normal ASCII string.")
    );

    test!(String,
        "\"I can also use unicode: ٩(-̮̮̃-̃)۶ ٩(●̮̮̃•̃)۶ ٩(͡๏̯͡๏)۶ ٩(-̮̮̃•̃).\"",
        String::from("I can also use unicode: ٩(-̮̮̃-̃)۶ ٩(●̮̮̃•̃)۶ ٩(͡๏̯͡๏)۶ ٩(-̮̮̃•̃).")
    );

	test!(String,
        "\"I can escape some things, like \\\"\\\\\\/\\b\\f\\n\\r\\t!\"",
        String::from("I can escape some things, like \"\\/\u{0008}\u{000C}\n\r\t!")
    );

	test!(String,
        "\"I can even escape unicode: \\u3042.\"",
        String::from("I can even escape unicode: あ.")
    );
}

#[test]
fn valid_array()
{
	let json = Json::parse("[]");
	assert_eq!(json, Ok(Json::Array(vec![])));

	let json = Json::parse("[1,2.0,\"String\",[],{}]");
	assert_eq!(json, Ok(Json::Array(vec![
        Json::Number(Number::Unsigned(1)),
        Json::Number(Number::Float(2.)),
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
    map.insert(String::from("i"), Json::Number(Number::Unsigned(1)));
    map.insert(String::from("f"), Json::Number(Number::Float(2.)));
    map.insert(String::from("s"), Json::String(String::from("String")));
    map.insert(String::from("a"), Json::Array(vec![]));
    map.insert(String::from("o"), Json::Object(HashMap::new()));

    let json = Json::parse("{\"i\":1,\"f\":2.0,\"s\":\"String\",\"a\":[],\"o\":{}}");
	assert_eq!(json, Ok(Json::Object(map)));
}

