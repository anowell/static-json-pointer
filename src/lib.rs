#![feature(proc_macro)]
#![feature(rustc_private)]
#![crate_type = "proc-macro"]

//! Experimental macro to extract a Rust type from a JSON schema
//!
//! If you have a file `schema.json`:
//!
//! {
//!     "person": {
//!         "name": {
//!             "type": "&'static str"
//!         },
//!         "age": {
//!             "type": "Option<u32>"
//!         }
//!     }
//! }
//! ```
//!
//! You can use a JSON pointer to specify the field, Then you can you can extract the type at compile time with:
//!
//! ```rust
//! #![feature(proc_macro)]
//! extern crate json_schema_type;
//! use json_schema_type::json_type;
//!
//! let name: json_type!("schema.json", "/person/name/type") = "Zazu";
//! let age: json_type!("schema.json", "/person/age/type") = Some(22);
//! ```
//!
//! Note: this will reparse the entire JSON for every invocation at compile time.
//!
//! Any clever tricks to cache parsing would be wonderfully welcome!
//!
//! That is all.

extern crate proc_macro;
extern crate proc_macro2;
extern crate serde_json;
use serde_json::Value;

use proc_macro2::TokenTree;

fn extract_string_lit(text: &str) -> &str {
    let start = text.find("\"").expect("How does a literal not have a \"?");
    let end = text.rfind("\"").expect("How does a literal not have a \"?");
    unsafe { text.get_unchecked(start+1..(end)) }
}

/// Inline interpolation macro
#[proc_macro]
pub fn json_type(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input: proc_macro2::TokenStream = input.into();

    let mut trees = input.into_iter();

    let token1 = trees.next().expect("macro expected a string literal");
    let _token2 = trees.next().expect("macro expected 2 arguments");
    let token3 = trees.next().expect("macro expected 2 arguments");

    // TODO: panic if too many tokens

    let lit1 = match token1 {
       TokenTree::Literal(lit) => lit.to_string(),
       _ => panic!("macro only accepts a string literal as first argument"),
    };
    let json = ::std::fs::read_to_string(extract_string_lit(&lit1)).expect("JSON file note found");

    // TODO: verify token2 is a comma

    let lit2 = match token3 {
       TokenTree::Literal(lit) => lit.to_string(),
       _ => panic!("macro only accepts a string literal as second argument"),
    };
    let pointer = extract_string_lit(&lit2);

    let val: Value = serde_json::from_str(&json).expect("first argument was not valid JSON");
    let json_type = val.pointer(&pointer).expect("no value found at JSON pointer");
    let json_type_str = json_type.as_str().expect("expected value at JSON pointer to be a string");

    let output: proc_macro2::TokenStream = json_type_str.parse().expect("JSON schema type isn't a valid Rust token");
    output.into()
}

