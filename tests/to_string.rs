use std::collections::HashMap;

extern crate simple_json;
use simple_json::{ Json, Number };

macro_rules! test {
    ($jsn:expr, $val:expr) => {{
        let json = Json::from($jsn);
        assert_eq!(json.to_string(), $val);
    }};
}

#[test]
fn null_to_string()
{
    test!((), "null");
}

#[test]
fn bool_to_string()
{
    test!(false, "false");
    test!(true,  "true");
}

#[test]
fn integer_to_string()
{
    test!(0,  "0");
    test!(1,  "1");
    test!(20, "20");
    test!(21, "21");

    test!(-0,  "0");
    test!(-1,  "-1");
    test!(-20, "-20");
    test!(-21, "-21");
}

#[test]
fn float_to_string()
{
    test!(0.,    "0");
    test!(1.1,   "1.1");
    test!(20.01, "20.01");
    test!(21.12, "21.12");

    test!(-0.,    "0");
    test!(-1.1,   "-1.1");
    test!(-20.01, "-20.01");
    test!(-21.12, "-21.12");
}

#[test]
fn string_to_string()
{
    test!("", "\"\"");

	test!(
        "This is a normal ASCII string.",
        "\"This is a normal ASCII string.\""
    );

    test!(
        "I can also use unicode: ٩(-̮̮̃-̃)۶ ٩(●̮̮̃•̃)۶ ٩(͡๏̯͡๏)۶ ٩(-̮̮̃•̃).",
        "\"I can also use unicode: ٩(-̮̮̃-̃)۶ ٩(●̮̮̃•̃)۶ ٩(͡๏̯͡๏)۶ ٩(-̮̮̃•̃).\""
    );

	test!(
        "I can escape some things, like \"\\/\u{0008}\u{000C}\n\r\t!",
        "\"I can escape some things, like \\\"\\/\u{0008}\u{000C}\n\r\t!\""
    );

	test!(
        "I can even escape unicode: あ.",
        "\"I can even escape unicode: \u{3042}.\""
    );
}

#[test]
fn array_to_string()
{
    test!(vec![], "[]");

    test!(
        vec![
            Json::from(1),
            Json::from(2.3),
            Json::from("String"), 
            Json::from(vec![]),
            Json::from(HashMap::new()),
        ],
        "[1,2.3,\"String\",[],{}]"
    );
}

#[test]
fn map_to_object()
{
    test!(HashMap::new(), "{}");

    let mut map = HashMap::new();
    map.insert(String::from("i"), Json::from(-1));
    map.insert(String::from("f"), Json::from(2.3));
    map.insert(String::from("s"), Json::from("String"));
    map.insert(String::from("a"), Json::from(vec![]));
    map.insert(String::from("o"), Json::from(HashMap::new()));

    assert_eq!(
        Json::from(map),
        Json::parse("{\"i\":-1,\"f\":2.3,\"s\":\"String\",\"a\":[],\"o\":{}}").unwrap()
    );
}
