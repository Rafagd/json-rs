use std::collections::HashMap;

extern crate simple_json;
use simple_json::{ Json, Number };

macro_rules! test_json_to {
    ($T: ty, $json:expr, $value:expr) => {{
        let json = Json::parse($json).unwrap();
        let val: $T = json.into();
        assert_eq!(val, $value);
    }};
}

macro_rules! test_json_from {
    ($value:expr, $expect:expr) => {{
        let val: Json = $value.into();
        assert_eq!(val, $expect);
    }};
}

#[test]
fn json_to_null()
{
    test_json_to!((), "null", ());
}

#[test]
fn json_to_bool()
{
    test_json_to!(bool, "false", false);
    test_json_to!(bool, "true",  true);
}

#[test]
fn json_to_number()
{
    test_json_to!(u64, "1",   1);
    test_json_to!(u64, "-2",  ((-2 as i64) as u64));
    test_json_to!(u64, "3.4", 3);

    test_json_to!(i64, "5",   5);
    test_json_to!(i64, "-6", -6);
    test_json_to!(i64, "7.8", 7);

    test_json_to!(f64, "9",   9.);
    test_json_to!(f64, "-1", -1.);
    test_json_to!(f64, "0.2", 0.2);
}

#[test]
fn json_to_string()
{
    test_json_to!(String, "\"This is a nice string\"", "This is a nice string");
}

#[test]
fn json_to_array()
{
    test_json_to!(Vec<Json>, "[ 1, 2.0, \"String\", [], {} ]", vec![
        Json::from(1 as u64),
        Json::from(2.),
        Json::from("String"),
        Json::from(vec![]),
        Json::from(HashMap::new()),
    ])
}

#[test]
fn json_to_object()
{
    let mut map = HashMap::new();

    map.insert(String::from("u"), Json::from(1 as u64));
    map.insert(String::from("i"), Json::from(-3));
    map.insert(String::from("f"), Json::from(2.));
    map.insert(String::from("s"), Json::from("String"));
    map.insert(String::from("a"), Json::from(vec![]));
    map.insert(String::from("o"), Json::from(HashMap::new()));

    test_json_to!(HashMap<String, Json>, "{\"u\":1,\"i\":-3,\"f\":2.0,\"s\":\"String\",\"a\":[],\"o\":{}}", map);
}

#[test]
fn null_to_json()
{
    test_json_from!((), Json::Null);
}

#[test]
fn bool_to_json()
{
    test_json_from!(false, Json::Boolean(false));
    test_json_from!(true,  Json::Boolean(true));
}

#[test]
fn number_to_json()
{
    test_json_from!(1 as u64, Json::Number(Number::Unsigned(1)));
    test_json_from!(2,        Json::Number(Number::Integer(2)));
    test_json_from!(3.,       Json::Number(Number::Float(3.)));
}

#[test]
fn string_to_json()
{
    test_json_from!("This is a nice string", Json::String(String::from("This is a nice string")));
    test_json_from!(String::from("Strings"), Json::String(String::from("Strings")));
}

#[test]
fn array_to_json()
{
    let vec: Vec<Json> = From::from(vec![
        Json::from(1),
        Json::from(2.),
        Json::from("String"),
        Json::from(vec![]),
        Json::from(HashMap::new()),
    ]);

    test_json_from!(vec, Json::Array(vec![
        Json::from(1),
        Json::from(2.),
        Json::from("String"),
        Json::from(vec![]),
        Json::from(HashMap::new()),
    ]));
}

#[test]
fn object_to_json()
{
    let mut map = HashMap::new();

    map.insert(String::from("u"), Json::from(1 as u64));
    map.insert(String::from("i"), Json::from(-3));
    map.insert(String::from("f"), Json::from(2.));
    map.insert(String::from("s"), Json::from("String"));
    map.insert(String::from("a"), Json::from(vec![]));
    map.insert(String::from("o"), Json::from(HashMap::new()));

    let mut exp = HashMap::new();

    exp.insert(String::from("u"), Json::from(1 as u64));
    exp.insert(String::from("i"), Json::from(-3));
    exp.insert(String::from("f"), Json::from(2.));
    exp.insert(String::from("s"), Json::from("String"));
    exp.insert(String::from("a"), Json::from(vec![]));
    exp.insert(String::from("o"), Json::from(HashMap::new()));

    test_json_from!(map, Json::Object(exp));
}
