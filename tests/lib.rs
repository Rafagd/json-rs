use std::collections::HashMap;

extern crate simple_json;
use simple_json::{ Json, Number };

#[test]
fn valid_null()
{
	let json = Json::parse("null").unwrap();
	assert_eq!(json.as_null(), ());
}

#[test]
fn valid_bool()
{
	let json = Json::parse("false").unwrap();
	assert_eq!(json.as_bool(), false);
	
	let json = Json::parse("true").unwrap();
	assert_eq!(json.as_bool(), true);
}

#[test]
fn valid_integer_number()
{
    macro_rules! test {
        ($jsn:expr, $val:expr) => {{
            let json = Json::parse($jsn).unwrap();
            let val: i64 = json.as_number().into();
            assert_eq!(val, $val);
        }};
    }

    test!("0",  0);
    test!("1",  1);
    test!("20", 20);
    test!("21", 21);

    test!("-0",   0);
    test!("-1",  -1);
    test!("-20", -20);
    test!("-21", -21);
}

#[test]
fn valid_float_number()
{
    macro_rules! test {
        ($jsn:expr, $val:expr) => {{
            let json = Json::parse($jsn).unwrap();
            let val: f64 = json.as_number().into();
            assert_eq!(val, $val);
        }};
    }

    test!("0.0",   0.);
    test!("1.1",   1.1);
    test!("20.01", 20.01);
    test!("21.12", 21.12);

    test!("-0.0",    0.);
    test!("-1.1",   -1.1);
    test!("-20.01", -20.01);
    test!("-21.12", -21.12);

    // Unsigned/Integer into float
    test!("34",   34.);
    test!("-56", -56.);
}

#[test]
fn valid_exponent_number()
{
    macro_rules! test {
        ($jsn:expr, $val:expr) => {{
            let json = Json::parse($jsn).unwrap();
            let val: f64 = json.as_number().into();
            assert_eq!(val, $val);
        }};
    }

    test!("0e0",      0.0e0);
    test!("1e1",      1.0e1);
    test!("20.01e10", 20.01e10);
    test!("21.12e2",  21.12e2);

    test!("-0e0",       0.0e0);
    test!("-1e1",      -1.0e1);
    test!("-20.01e10", -20.01e10);
    test!("-21.12e2",  -21.12e2);

    // Unsigned/Integer into float
    test!("34",   3.4e1);
    test!("-56", -5.6e1);
}

#[test]
fn valid_string()
{
    macro_rules! test {
        ($jsn:expr, $val:expr) => {{
            let json = Json::parse($jsn).unwrap();
            assert_eq!(json.as_string(), $val);
        }};
    }

    test!(
        "\"\"",
        ""
    );

	test!(
        "\"This is a normal ASCII string.\"",
        "This is a normal ASCII string."
    );

    test!(
        "\"I can also use unicode: ٩(-̮̮̃-̃)۶ ٩(●̮̮̃•̃)۶ ٩(͡๏̯͡๏)۶ ٩(-̮̮̃•̃).\"",
        "I can also use unicode: ٩(-̮̮̃-̃)۶ ٩(●̮̮̃•̃)۶ ٩(͡๏̯͡๏)۶ ٩(-̮̮̃•̃)."
    );

	test!(
        "\"I can escape some things, like \\\"\\\\\\/\\b\\f\\n\\r\\t!\"",
        "I can escape some things, like \"\\/\u{0008}\u{000C}\n\r\t!"
    );

	test!(
        "\"I can even escape unicode: \\u3042.\"",
        "I can even escape unicode: あ."
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

#[test]
fn json_to_number()
{
    macro_rules! test {
        ($T: ident, $jsn:expr, $val:expr) => {{
            let json = Json::parse($jsn).unwrap();
            let val: $T = json.as_number().into();
            assert_eq!(val, $val);
        }};
    }

    test!(u64, "1",   1);
    test!(u64, "-2",  ((-2 as i64) as u64));
    test!(u64, "3.4", 3);

    test!(i64, "5",   5);
    test!(i64, "-6", -6);
    test!(i64, "7.8", 7);

    test!(f64, "9",   9.);
    test!(f64, "-1", -1.);
    test!(f64, "0.2", 0.2);
}
