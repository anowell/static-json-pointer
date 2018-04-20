## static-json-pointer

Macro to extract Rust tokens and literals from a JSON schema

If you have a file `schema.json`:

```json
{
    "person": {
        "name": {
            "type": "String",
            "value": "Zazu"
        },
        "age": {
            "type": "Option<u32>",
            "value": 42
        }
    }
}
```

You can use a JSON pointer to specify the field for extracting a token or literal from JSON at compile time:

```rust
#![feature(proc_macro)]
extern crate static_json_pointer;
use static_json_pointer::json_token;

// let name = String::from("Zazu");
let name = json_token!("schema.json", "/person/name/type")::from(json_literal!("schema.json", "/person/name/value"));

// let age = Option<u32>::from(42);
let age = json_token!("schema.json", "/person/age/type")::from(json_literal!("schema.json", "/person/age/value"));

assert_eq!(name, "Zazu".to_string());
assert_eq!(age, Some(42));
```

The deserialized JSON is cached during build to prevent redundant reading and parsing during build.
