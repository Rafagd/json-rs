Runtime JSON parser for Rust
============================
[![Current version](https://crates.io/crates/simple_json)](https://img.shields.io/crates/v/simple_json.svg)
[![Master status](https://travis-ci.org/Rafagd/json-rs.svg?branch=master)](https://travis-ci.org/Rafagd/json-rs) 
[![Coverage status](https://coveralls.io/repos/github/Rafagd/json-rs/badge.svg?branch=master)](https://coveralls.io/github/Rafagd/json-rs?branch=master)

`simple_json` is a JSON run-time recursive parser created for simplyfing both reading and writing JSON-encoded data. It's also supposed to be compatible with the current stable release so if you detect any problems, please report them.

Although I believe this package runs fast enough for most cases, it wasn't made with performance or big data sets in mind. In those cases you may be better using [Serde](https://github.com/serde-rs/serde) instead.

This package abuses Rust's `Option` feature and creates a tree, where all nodes fit into one of 6 types:

- `Json::Object(HashMap<String, Json>)`
- `Json::Array(Vec<Json>)`
- `Json::String(String)`
- `Json::Number(simple_json::Number)`
- `Json::Boolean(bool)`
- `Json::Null`

All of them implement the `From` trait in both ways, so converting between them and their native counterparts should be as easy as calling `into()`.

`Number` is another enum created to represent the multiple different types a JSON Number can hold. The parser will try to choose the most sensible type for each situation, so this should be transparent to user.

A `Number` can be one of 3 types:

- `Number::Float(f64)`, if its a floating number (ie. `3.2`) or if its written in scientific notation (ie. `3e2`);
- `Number::Integer(i64)`, if the parser receives a signed number (ie. `-1`);
- `Number::Unsigned(u64)`, for everything else (ie. `42`).

Here's a simple example:

```rust
extern crate simple_json;

use simple_json::Json;

fn main() {
    let mut text = "{ \"integer\": 12, \"float\": 80.5, \"string\": \"A JSON sample\", \"array\": [ 1, 2, 3 ], \"object\": { \"a\": \"b\" } }";
    let result = Json::parse(text).unwrap();
    
    println!("Result: {}", result.to_string());
}
```

Output:

```rust
Result: {
  "integer": 12,
  "float":   80.5,
  "string":  "A JSON sample",
  "array": [ 1, 2, 3 ],
  "object": {
    "a": "b"
  }
}
```
