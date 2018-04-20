#![feature(proc_macro)]

#[cfg(test)]
extern crate static_json_pointer;
extern crate serde_json;

#[cfg(test)]
mod tests {
    use static_json_pointer::{json_token, json_literal};
    use std::any::TypeId;


    #[test]
    fn string() {
        assert_eq!(
            TypeId::of::<String>(),
            TypeId::of::<json_token!("schema.json", "/foo/type")>())
        ;
    }

    #[test]
    fn number() {
        assert_eq!(
            TypeId::of::<u32>(),
            TypeId::of::<json_token!("schema.json", "/bar/type")>())
        ;
    }

    #[test]
    fn opt_str() {
        assert_eq!(
            TypeId::of::<Option<String>>(),
            TypeId::of::<json_token!("schema.json", "/baz/type")>())
        ;
    }

    #[test]
    fn static_str() {
        assert_eq!(
            TypeId::of::<&'static str>(),
            TypeId::of::<json_token!("schema.json", "/qux/type")>())
        ;
    }


    #[test]
    fn literal_string() {
        assert_eq!(
            "FOO",
            json_literal!("schema.json", "/foo/value")
        );
    }

    #[test]
    fn literal_num() {
        assert_eq!(
            42,
            json_literal!("schema.json", "/bar/value")
        );
    }

    #[test]
    fn null_token_and_literal() {
        let null: json_token!("schema.json", "/baz/type") = None;
        assert_eq!(
            null,
            json_literal!("schema.json", "/baz/value")
        );
    }

}
