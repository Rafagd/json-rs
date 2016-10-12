extern crate simple_json;

use simple_json::Json;

#[test]
fn test_example()
{
    let text   = "{ \"integer\": 12, \"float\": 80.5, \"string\": \"A JSON sample\", \"array\": [ 1, 2, 3 ], \"object\": { \"a\": \"b\" } }";
    let result = Json::parse(text).unwrap();
    
    println!("Result: {:?}", result.to_string());
}

