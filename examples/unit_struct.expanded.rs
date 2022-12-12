//! A unit struct with attributes.
//!
//! ```no_compile
//! #[path = "s.tmpl"]
//! struct S;
//! ```
use syn::{Attribute, Ident, Token};
struct UnitStruct {
    attrs: Vec<Attribute>,
    struct_token: (),
    name: Ident,
    semi_token: (),
}
#[automatically_derived]
impl ::pegcel::std::clone::Clone for UnitStruct {
    fn clone(&self) -> Self {
        Self {
            attrs: self.attrs.clone(),
            struct_token: self.struct_token.clone(),
            name: self.name.clone(),
            semi_token: self.semi_token.clone(),
        }
    }
}
#[automatically_derived]
impl ::pegcel::std::fmt::Debug for UnitStruct {
    fn fmt(
        &self,
        f: &mut ::pegcel::std::fmt::Formatter<'_>,
    ) -> ::pegcel::std::fmt::Result {
        f.debug_struct("UnitStruct")
            .field("attrs", &self.attrs)
            .field("struct_token", &self.struct_token)
            .field("name", &self.name)
            .field("semi_token", &self.semi_token)
            .finish()
    }
}
#[automatically_derived]
impl ::pegcel::std::hash::Hash for UnitStruct {
    fn hash<H: ::pegcel::std::hash::Hasher>(&self, state: &mut H) {
        self.attrs.hash(state);
        self.struct_token.hash(state);
        self.name.hash(state);
        self.semi_token.hash(state);
    }
}
#[automatically_derived]
impl ::pegcel::std::cmp::PartialEq for UnitStruct {
    fn eq(&self, other: &Self) -> bool {
        self.attrs == other.attrs && self.struct_token == other.struct_token
            && self.name == other.name && self.semi_token == other.semi_token
    }
}
#[automatically_derived]
impl ::pegcel::std::cmp::Eq for UnitStruct {}
#[automatically_derived]
impl ::pegcel::syn::parse::Parse for UnitStruct {
    fn parse(
        input: ::pegcel::syn::parse::ParseStream,
    ) -> ::pegcel::syn::parse::Result<Self> {
        ::pegcel::std::result::Result::Ok(Self {
            attrs: input.call(Attribute::parse_outer)?,
            struct_token: input.parse()?,
            name: input.parse()?,
            semi_token: input.parse()?,
        })
    }
}
impl ::pegcel::quote::ToTokens for UnitStruct {
    fn to_tokens(&self, tokens: &mut ::pegcel::proc_macro2::TokenStream) {
        use ::pegcel::__IterableToTokens;
        self.attrs.to_tokens(tokens);
        self.struct_token.to_tokens(tokens);
        self.name.to_tokens(tokens);
        self.semi_token.to_tokens(tokens);
    }
}
fn main() {}
