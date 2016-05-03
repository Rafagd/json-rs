Runtime JSON parser for Rust
============================
[![master status](https://travis-ci.org/Rafagd/json-rs.svg?branch=master)](https://travis-ci.org/Rafagd/json-rs) 
[![Coverage Status](https://coveralls.io/repos/github/Rafagd/json-rs/badge.svg?branch=master)](https://coveralls.io/github/Rafagd/json-rs?branch=master)

A simple json implementation abusing the Option feature a lot. I have searched the internet for a simple and easy to use JSON library for rust, but I only found two projects (which are way better and probably more efficient than this one).

Problem is that one of them is still a work in progres by the rust dev team, and the other one is simply broken right now. As I don't really need speed that much on my current project (I just need to read some config files at startup), I've decided that I could try and make a dumb and naive implementation, which you can see here.

It runs in runtime instead of compile time and it relies heavily on recursion. It also doesn't support streams, so maybe it would be wise to avoid using it on projects that require deeply nested trees, huge file sizes or anything more complex than a bunch of configuration files, really.

```rust
extern crate simple_json;

use simple_json::{ Json, JsonOption };

fn main() {
    let mut text = "{ \"integer\": 12, \"float\": 80.5, \"string\": \"A JSON sample\", \"array\": [ 1, 2, 3 ], \"object\": { \"a\": \"b\" } }";
    let result = Json::parse(text);
    
    println!("Result: {}", result);
}
```

Should result in something like this:

```rust
Object({ //HashMap<String, JsonOption>
  "integer": Integer(12),
  "float":   Number(80.5),
  "string":  String("A JSON sample"),
  "array": [ // Vec<JsonOption>
    Integer(1), Integer(2), Integer(3)
  ],
  "object": { // HashMap<String, JsonOption>
    "a": String("b")
  }
});
```
