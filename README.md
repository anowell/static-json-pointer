## json_schma_type

Experimental macro to extract a Rust type from a JSON schema

If you have a file `schema.json`:

```json
{
    "person": {
        "name": {
            "type": "&'static str"
        },
        "age": {
            "type": "Option<u32>"
        }
    }
}
```

You can use a JSON pointer to specify the field, Then you can you can extract the type at compile time with:

```rust
#![feature(proc_macro)]
extern crate json_schema_type;
use json_schema_type::json_type;

let name: json_type!("schema.json", "/person/name/type") = "Zazu";
let age: json_type!("schema.json", "/person/age/type") = Some(22);
```

## Why

I have a CLI tool that prompts for many config values, transforms them, verifies them, and writes them to a JSON file.
Prompting, transforming, and verificiation all vary a bit depending on the type. Those types happen to be defined in a schema file.

Perhaps I should have stuck to fallible runtime abstraction with lots of helpers to handle the runtime type checking of `Value` structs.
In some ways that would have been easier, in others more complicated.

Or perhaps I should have written it in Python. But what fun would that be?

## Caveats

Note: this will reparse the entire JSON for every invocation at compile time.

Any clever tricks to cache parsing would be wonderfully welcome!

That is all.