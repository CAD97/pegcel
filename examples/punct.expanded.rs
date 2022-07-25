mod syntax {
    pub mod kw {
        #[allow(non_camel_case_types)]
        pub struct yeet {
            pub span: ::syn::__private::Span,
        }
        #[doc(hidden)]
        #[allow(dead_code, non_snake_case)]
        pub fn yeet<__S: ::syn::__private::IntoSpans<[::syn::__private::Span; 1]>>(
            span: __S,
        ) -> yeet {
            yeet {
                span: ::syn::__private::IntoSpans::into_spans(span)[0],
            }
        }
        impl ::syn::__private::Default for yeet {
            fn default() -> Self {
                yeet {
                    span: ::syn::__private::Span::call_site(),
                }
            }
        }
        impl ::syn::token::CustomToken for yeet {
            fn peek(cursor: ::syn::buffer::Cursor) -> ::syn::__private::bool {
                if let ::syn::__private::Some((ident, _rest)) = cursor.ident() {
                    ident == "yeet"
                } else {
                    false
                }
            }
            fn display() -> &'static ::syn::__private::str {
                "`yeet`"
            }
        }
        impl ::syn::parse::Parse for yeet {
            fn parse(input: ::syn::parse::ParseStream) -> ::syn::parse::Result<yeet> {
                input
                    .step(|cursor| {
                        if let ::syn::__private::Some((ident, rest)) = cursor.ident() {
                            if ident == "yeet" {
                                return ::syn::__private::Ok((
                                    yeet { span: ident.span() },
                                    rest,
                                ));
                            }
                        }
                        ::syn::__private::Err(cursor.error("expected `yeet`"))
                    })
            }
        }
        impl ::syn::__private::ToTokens for yeet {
            fn to_tokens(&self, tokens: &mut ::syn::__private::TokenStream2) {
                let ident = ::syn::Ident::new("yeet", self.span);
                ::syn::__private::TokenStreamExt::append(tokens, ident);
            }
        }
        impl ::syn::__private::Copy for yeet {}
        #[allow(clippy::expl_impl_clone_on_copy)]
        impl ::syn::__private::Clone for yeet {
            fn clone(&self) -> Self {
                *self
            }
        }
        impl ::syn::__private::Debug for yeet {
            fn fmt(
                &self,
                f: &mut ::syn::__private::Formatter,
            ) -> ::syn::__private::fmt::Result {
                ::syn::__private::Formatter::write_str(f, "Keyword [yeet]")
            }
        }
        impl ::syn::__private::Eq for yeet {}
        impl ::syn::__private::PartialEq for yeet {
            fn eq(&self, _other: &Self) -> ::syn::__private::bool {
                true
            }
        }
        impl ::syn::__private::Hash for yeet {
            fn hash<__H: ::syn::__private::Hasher>(&self, _state: &mut __H) {}
        }
    }
    pub mod punct {
        pub struct Elvis {
            pub spans: [::syn::__private::Span; 0 + 1 + 1],
        }
        #[doc(hidden)]
        #[allow(dead_code, non_snake_case)]
        pub fn Elvis<
            __S: ::syn::__private::IntoSpans<[::syn::__private::Span; 0 + 1 + 1]>,
        >(spans: __S) -> Elvis {
            let _validate_len = 0 + 1 + 1;
            Elvis {
                spans: ::syn::__private::IntoSpans::into_spans(spans),
            }
        }
        impl ::syn::__private::Default for Elvis {
            fn default() -> Self {
                Elvis(::syn::__private::Span::call_site())
            }
        }
        impl ::syn::token::CustomToken for Elvis {
            fn peek(cursor: ::syn::buffer::Cursor) -> bool {
                ::syn::token::parsing::peek_punct(cursor, "?:")
            }
            fn display() -> &'static ::syn::__private::str {
                "`?:`"
            }
        }
        impl ::syn::parse::Parse for Elvis {
            fn parse(input: ::syn::parse::ParseStream) -> ::syn::parse::Result<Elvis> {
                let spans: [::syn::__private::Span; 0 + 1 + 1] = ::syn::token::parsing::punct(
                    input,
                    "?:",
                )?;
                Ok(Elvis(spans))
            }
        }
        impl ::syn::__private::ToTokens for Elvis {
            fn to_tokens(&self, tokens: &mut ::syn::__private::TokenStream2) {
                ::syn::token::printing::punct("?:", &self.spans, tokens)
            }
        }
        impl ::syn::__private::Copy for Elvis {}
        #[allow(clippy::expl_impl_clone_on_copy)]
        impl ::syn::__private::Clone for Elvis {
            fn clone(&self) -> Self {
                *self
            }
        }
        impl ::syn::__private::Debug for Elvis {
            fn fmt(
                &self,
                f: &mut ::syn::__private::Formatter,
            ) -> ::syn::__private::fmt::Result {
                ::syn::__private::Formatter::write_str(f, "Elvis")
            }
        }
        impl ::syn::__private::Eq for Elvis {}
        impl ::syn::__private::PartialEq for Elvis {
            fn eq(&self, _other: &Self) -> ::syn::__private::bool {
                true
            }
        }
        impl ::syn::__private::Hash for Elvis {
            fn hash<__H: ::syn::__private::Hasher>(&self, _state: &mut __H) {}
        }
    }
    pub use __pegcel_generated_token_macro__Token as Token;
}
fn main() {
    let _ = <crate::syntax::kw::yeet>::default();
    let _ = <crate::syntax::punct::Elvis>::default();
    let _ = <::syn::token::Do>::default();
    let _ = <::syn::token::Colon2>::default();
}
