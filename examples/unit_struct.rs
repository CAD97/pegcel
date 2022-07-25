//! A unit struct with attributes.
//!
//! ```no_compile
//! #[path = "s.tmpl"]
//! struct S;
//! ```

use syn::{Attribute, Ident, Token};

pegcel::define_syntax! {
    struct UnitStruct {
        attrs: Vec<Attribute> as Attribute::parse_outer,
        struct_token: Token![struct],
        name: Ident,
        semi_token: Token![;],
    }
}

fn main() {}
