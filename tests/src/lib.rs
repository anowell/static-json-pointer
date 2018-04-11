#![feature(proc_macro)]

#[cfg(test)]
extern crate json_schema_type;

#[cfg(test)]
mod tests {
    use json_schema_type::json_type;
    use std::any::TypeId;

    #[test]
    fn string() {
        assert_eq!(
            TypeId::of::<String>(),
            TypeId::of::<json_type!("schema.json", "/foo/type")>())
        ;
    }

    #[test]
    fn number() {
        assert_eq!(
            TypeId::of::<u32>(),
            TypeId::of::<json_type!("schema.json", "/bar/type")>())
        ;
    }

    #[test]
    fn opt_str() {
        assert_eq!(
            TypeId::of::<Option<String>>(),
            TypeId::of::<json_type!("schema.json", "/baz/type")>())
        ;
    }

    #[test]
    fn static_str() {
        assert_eq!(
            TypeId::of::<&'static str>(),
            TypeId::of::<json_type!("schema.json", "/qux/type")>())
        ;
    }
}
