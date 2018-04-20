#![feature(proc_macro)]
#![feature(nll)]
#![feature(rustc_private)]
#![crate_type = "proc-macro"]

//! Macro to extract literals and Rust tokens from a JSON schema
//!
//! If you have a file `schema.json`:
//!
//! ```json
//! {
//!     "person": {
//!         "name": {
//!             "type": "String",
//!             "value": "Zazu"
//!         },
//!         "age": {
//!             "type": "Option<u32>",
//!             "value": 42
//!         }
//!     }
//! }
//! ```
//!
//! You can use a JSON pointer to specify the field for extracting a token or literal from JSON at compile time:
//!
//! ```rust,ignore
//! #![feature(proc_macro)]
//! extern crate static_json_pointer;
//! use static_json_pointer::json_token;
//!
//! // let name = String::from("Zazu");
//! let name = json_token!("schema.json", "/person/name/type")::from(json_literal!("schema.json", "/person/name/value"));
//!
//! // let age = Option<u32>::from(42);
//! let age = json_token!("schema.json", "/person/age/type")::from(json_literal!("schema.json", "/person/age/value"));
//!
//! assert_eq!(name, "Zazu".to_string());
//! assert_eq!(age, Some(42));
//! ```
//!
//! The deserialized JSON is cached during build to prevent redundant reading and parsing during build.

extern crate proc_macro;
extern crate proc_macro2;
extern crate serde_json;
#[macro_use] extern crate lazy_static;

use serde_json::Value;
use proc_macro2::TokenTree;
use std::collections::HashMap;
use std::sync::Mutex;

lazy_static! {
    static ref JSON_CACHE: Mutex<HashMap<String, Value>> = {
        Mutex::new(HashMap::new())
    };
}


fn extract_string_lit(text: &str) -> &str {
    let start = text.find("\"").expect("How does a literal not have a \"?");
    let end = text.rfind("\"").expect("How does a literal not have a \"?");
    unsafe { text.get_unchecked(start+1..(end)) }
}

fn extract_json(input: proc_macro::TokenStream) -> Value {
    let input: proc_macro2::TokenStream = input.into();

    let mut trees = input.into_iter();

    let token1 = trees.next().expect("json_token! expected 2 arguments");
    let _token2 = trees.next().expect("json_token! expected 2 arguments");
    let token3 = trees.next().expect("json_token! expected 2 arguments");

    // TODO: panic if too many tokens

    let lit1 = match token1 {
       TokenTree::Literal(lit) => lit.to_string(),
       _ => panic!("macro only accepts a string literal as first argument"),
    };


    // TODO: verify token2 is a comma

    let lit2 = match token3 {
       TokenTree::Literal(lit) => lit.to_string(),
       _ => panic!("macro only accepts a string literal as second argument"),
    };

    let file_path = extract_string_lit(&lit1);
    let pointer = extract_string_lit(&lit2);


    let mut cache = JSON_CACHE.lock().unwrap();
    let json_val = match cache.get(file_path) {
        Some(val) => val,
        None => {
            let json = ::std::fs::read(extract_string_lit(&lit1)).expect("failed to read JSON file specified by json_token! macro");
            let val: Value = serde_json::from_slice(&json).expect("json_token! file is not valid JSON");
            let _ = cache.insert(file_path.to_owned(), val);
            cache.get(file_path).unwrap()
        }
    };
    json_val.pointer(&pointer).expect("json_token! macro did not find a value for this JSON pointer").clone()
}

/// Inline interpolation macro
#[proc_macro]
pub fn json_token(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let json_token = extract_json(input);
    let json_token_str = json_token.as_str().expect("json_token! macro expected to find a string at this JSON pointer");

    let output: proc_macro2::TokenStream = json_token_str.parse().expect("JSON schema type isn't a valid Rust token");
    output.into()
}


#[proc_macro]
pub fn json_literal(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let json_token = extract_json(input);
    let json_token_str = serde_json::to_string(&json_token).unwrap();

    let output: proc_macro2::TokenStream = json_token_str.parse().expect("JSON schema type isn't a valid Rust token");
    output.into()
}

