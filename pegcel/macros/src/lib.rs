extern crate proc_macro;

#[allow(warnings)]
mod grammar;
mod manual_grammar;

use {
    crate::{
        grammar::{Item, *},
        manual_grammar::*,
    },
    indexmap::{IndexMap, IndexSet},
    itertools::{izip, zip, Itertools},
    matches::{assert_matches, matches},
    proc_macro2::{Span, TokenStream},
    quote::{quote, TokenStreamExt},
    syn::{parse_quote, spanned::Spanned, Ident, Result},
    uuid::Uuid,
};

// FIXME: We should have deterministic output
fn new_uuidv4_ident() -> Ident {
    Ident::new(
        &format!("_{}", Uuid::new_v4().to_simple()),
        Span::call_site(),
    )
}

#[proc_macro]
// procedural macros cannot expand to macro definitions (see issue #54727) so shim through a derive
// https://www.reddit.com/r/rust/comments/be9oqk/_/el4i6r3/
pub fn pegcel_syn(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ident = new_uuidv4_ident();
    let input: proc_macro2::TokenStream = input.into();
    (quote! {
        #[derive(::pegcel_macros::__pegcel_syn_dummy_derive)]
        #[real_input(#input)]
        struct #ident;
    })
    .into()
}

#[doc(hidden)]
#[proc_macro_derive(__pegcel_syn_dummy_derive, attributes(real_input))]
pub fn pegcel_syn_real(tokens: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input: Parenthesized<proc_macro2::TokenStream> = syn::parse2(
        syn::parse_macro_input!(tokens as syn::DeriveInput)
            .attrs
            .into_iter()
            .find(|attr| attr.path.is_ident("real_input"))
            .unwrap()
            .tts,
    )
    .unwrap();
    let input: proc_macro::TokenStream = input.0.into();

    // syn::Error::new(Span::call_site(),
    make_syn(syn::parse_macro_input!(input))
        .unwrap_or_else(|err| err.to_compile_error())
        // ).to_compile_error()
        .into()
}

struct Parenthesized<T>(T);
impl<T: syn::parse::Parse> syn::parse::Parse for Parenthesized<T> {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let content;
        syn::parenthesized!(content in input);
        Ok(Parenthesized(content.parse()?))
    }
}

impl Item {
    fn unnamed(&self) -> &UnnamedItem {
        match self {
            Item::Unnamed(item) => item,
            Item::Named(named) => &named.item,
        }
    }
}

impl Predicate {
    fn positive(&self) -> bool {
        match self {
            Predicate::Positive(_) => true,
            Predicate::Negative(_) => false,
        }
    }
}

fn make_syn(input: Grammar) -> Result<proc_macro2::TokenStream> {
    // this method only handles targeting syn
    assert_matches!(input.kind, GrammarUse::Syn(..));

    let mut output = TokenStream::new();

    // make sure the required crate dependencies are present
    output.append_all(quote! {
        // FIXME: these should probably be `pegcel::runtime::external::*` instead
        extern crate proc_macro2;
        extern crate quote;
        extern crate syn;
    });

    // include manual `use`s
    for Use { anchor, tree, .. } in &input.uses {
        output.append_all(quote! {
            use #anchor #tree ;
        })
    }

    // `mod kw`, `mod punct`, `macro Token`
    output.append_all(custom_tokens(&input));

    for item in input.items {
        output.append_all(make_production_def(item)?);
    }

    Ok(output)
}

fn custom_tokens(input: &Grammar) -> TokenStream {
    #[derive(Debug, Default)]
    struct Visitor {
        custom_keywords: IndexSet<Ident>,
        custom_punctuation: IndexMap<String, Ident>,
    };

    impl Visitor {
        fn to_defs(&self) -> TokenStream {
            let mut tokens = TokenStream::new();

            let keywords = &self.custom_keywords;
            let kws = &self.custom_keywords;
            let punct_name: Vec<_> = self.custom_punctuation.values().collect();
            let punct_name = &*punct_name;
            let punct_val: Vec<TokenStream> = self
                .custom_punctuation
                .keys()
                .map(|it| syn::parse_str(it).unwrap())
                .collect();
            let punct_val = &*punct_val;

            let token_macro = new_uuidv4_ident();

            tokens.append_all(quote! {
                pub mod kw {
                    #(::syn::custom_keyword!(#keywords);)*
                }
                pub mod punct {
                    #(::syn::custom_punctuation!(#punct_name, #punct_val);)*
                }
                #[doc(hidden)]
                macro_rules! #token_macro {
                    #((#keywords) => {self::kw::#kws};)*
                    #((#punct_val) => {self::punct::#punct_name};)*
                    ($($tt:tt)*) => {::syn::Token![$($tt)*]};
                }
                // FIXME: Provide some way to export this conditionally
                // error: cannot export macro_rules! macros from a `proc-macro` crate type currently
                // Also, we probably want to offer fallback to non-syn Token! to allow stacking
                pub(crate) use #token_macro as Token;
            });

            tokens
        }

        fn visit_grammar(&mut self, grammar: &Grammar) -> &mut Self {
            for named in &grammar.items {
                self.visit_unnamed_item(&named.item);
            }
            self
        }

        fn visit_unnamed_item(&mut self, item: &UnnamedItem) -> &mut Self {
            match item {
                UnnamedItem::Predicated(_, item) => self.visit_unnamed_item(item),
                UnnamedItem::Repeated(item, rep) => {
                    self.visit_unnamed_item(item);
                    match rep {
                        Repetition::OnePlus(_, Some(inter))
                        | Repetition::ZeroPlus(_, Some(inter)) => match inter {
                            Interspersion::Terminated(_, sym)
                            | Interspersion::Separated(_, sym) => self.visit_symbol(sym),
                        },
                        _ => self,
                    }
                }
                UnnamedItem::Block(items) => {
                    match &items.value {
                        Items::Sequence(items) => {
                            for item in items {
                                match &item {
                                    Item::Named(named) => self.visit_unnamed_item(&named.item),
                                    Item::Unnamed(item) => self.visit_unnamed_item(item),
                                };
                            }
                        }
                        Items::OrderedChoice(_, items) => {
                            for named in items {
                                self.visit_unnamed_item(&named.item);
                            }
                        }
                    }
                    self
                }
                UnnamedItem::Symbol(symb) => self.visit_symbol(symb),
            }
        }

        fn visit_symbol(&mut self, symbol: &Symbol) -> &mut Self {
            match symbol {
                Symbol::Path(_) => (),
                Symbol::Literal(lit) => match &*lit.value() {
                    "abstract" | "as" | "async" | "auto" | "become" | "box" | "break" | "const"
                    | "continue" | "crate" | "default" | "do" | "dyn" | "else" | "enum"
                    | "existential" | "extern" | "final" | "fn" | "for" | "if" | "impl" | "in"
                    | "let" | "loop" | "macro" | "match" | "mod" | "move" | "mut" | "override"
                    | "priv" | "pub" | "ref" | "return" | "Self" | "self" | "static" | "struct"
                    | "super" | "trait" | "try" | "type" | "typeof" | "union" | "unsafe"
                    | "unsized" | "use" | "virtual" | "where" | "while" | "yield" | "+" | "+="
                    | "&" | "&&" | "&=" | "@" | "!" | "^" | "^=" | ":" | "::" | "," | "/"
                    | "/=" | "." | ".." | "..." | "..=" | "=" | "==" | ">=" | ">" | "<=" | "<"
                    | "*=" | "!=" | "|" | "|=" | "||" | "#" | "?" | "->" | "<-" | "%" | "%="
                    | "=>" | ";" | "<<" | "<<=" | ">>" | ">>=" | "*" | "-" | "-=" | "~" | "_" => (),
                    s => {
                        if s.chars()
                            .all(|c| 'a' <= c && c <= 'z' || 'A' <= c && c <= 'Z')
                        {
                            self.custom_keywords
                                .insert(Ident::new(s, Span::call_site()));
                        } else {
                            self.custom_punctuation
                                .insert(lit.value(), new_uuidv4_ident());
                        }
                    }
                },
            }
            self
        }
    }

    Visitor::default().visit_grammar(input).to_defs()
}

fn make_production_def(input: NamedItem) -> Result<TokenStream> {
    let items = if let UnnamedItem::Block(Braced { value, .. }) = input.item {
        value
    } else {
        Err(syn::Error::new(
            input.item.span(),
            "Root production must be braced",
        ))?
    };

    match items {
        Items::OrderedChoice(_, items) => {
            make_enum_production_def(input.name, items.into_iter().collect())
        }
        Items::Sequence(items) => make_struct_production_def(input.name, items),
    }
}

fn make_enum_production_def(name: Ident, variants: Vec<NamedItem>) -> Result<TokenStream> {
    let variant_names = variants.iter().map(|item| item.name.clone()).collect_vec();
    let variant_names = &*variant_names;
    let name_ = variant_names.iter().map(|_| &name).collect_vec();
    let variant_fields: Vec<syn::Fields> = variants
        .iter()
        .map(|item| match &item.item {
            UnnamedItem::Block(Braced { value: block, .. }) => {
                let items = match block {
                    // FIXME: This shouldn't panic, rather Err with a useful span
                    Items::OrderedChoice(..) => Err(syn::Error::new(
                        item.item.span(),
                        "ordered choice not allowed here",
                    ))?,
                    Items::Sequence(items) => items,
                };
                Ok(make_fields(items, syn::Visibility::Inherited))
            }
            UnnamedItem::Symbol(_) | UnnamedItem::Repeated(..) => {
                // FIXME: This shouldn't panic, rather Err with a useful span
                let ty =
                    make_type(&item.item).unwrap_or_else(|| panic!("Non-advancing repetition"));;
                Ok(syn::Fields::Unnamed(syn::FieldsUnnamed {
                    paren_token: Default::default(),
                    unnamed: {
                        let mut punctuated = syn::punctuated::Punctuated::new();
                        punctuated.push(syn::Field {
                            attrs: vec![],
                            vis: syn::Visibility::Inherited,
                            ident: None,
                            colon_token: None,
                            ty,
                        });
                        punctuated
                    },
                }))
            }
            UnnamedItem::Predicated(..) => Err(syn::Error::new(
                item.item.span(),
                "predicate not allowed here",
            )),
        })
        .collect::<Result<_>>()?;

    let param = new_uuidv4_ident();

    // impl ToTokens

    let field_names: Vec<Vec<syn::Ident>> = variant_fields
        .iter()
        .map(|fields| match fields {
            syn::Fields::Named(_) => fields
                .iter()
                .map(|field| field.ident.as_ref().unwrap().clone())
                .collect_vec(),
            syn::Fields::Unnamed(_) => fields.iter().map(|_| new_uuidv4_ident()).collect_vec(),
            syn::Fields::Unit => unreachable!("Unexpected Unit type {:?}", name),
        })
        .collect_vec();
    let field_pats: Vec<syn::Pat> = izip!(&field_names, &variant_fields)
        .map(|(names, fields)| match fields {
            syn::Fields::Named(_) => parse_quote!({ #(#names),* }),
            syn::Fields::Unnamed(_) => parse_quote!(( #(#names),* )),
            syn::Fields::Unit => unreachable!("Unexpected Unit type {:?}", name),
        })
        .collect_vec();

    let mut to_tokens = TokenStream::new();

    for (fields, field_pat, variant_name, variant) in
        izip!(&field_names, &field_pats, variant_names, &variants)
    {
        let variant = &variant.item;
        let dt = match variant {
            UnnamedItem::Predicated(..) => unreachable!("ordered choice cannot be predicate"),
            UnnamedItem::Repeated(..) | UnnamedItem::Symbol(..) => {
                // in this case `field_pat` is `(#ident)` because it's a newtype variant
                tokenize(parse_quote!(#field_pat), variant, &param)
            }
            UnnamedItem::Block(Braced { value: block, .. }) => {
                let items = match block {
                    // FIXME: This shouldn't panic, rather Err with a useful span
                    Items::OrderedChoice(..) => panic!("choice not allowed here"),
                    Items::Sequence(items) => items,
                };
                let items = items
                    .iter()
                    .filter(|item| make_type(item.unnamed()).is_some())
                    .collect_vec();

                let mut dt = TokenStream::new();
                assert_eq!(fields.len(), items.len());
                for (field, item) in zip(fields, items) {
                    dt.extend(tokenize(parse_quote!(#field), item.unnamed(), &param));
                }
                dt
            }
        };
        to_tokens.extend(quote! {
            #name :: #variant_name #field_pat => { #dt },
        })
    }

    // TODO: impl Parse
    let is_variant = (&variants[..variants.len() - 1])
        .iter()
        .map(|item| match &item.item {
            UnnamedItem::Predicated(..) => unreachable!("checked before"),
            UnnamedItem::Repeated(..) => {
                unreachable!("cannot start non-last choice with repetition")
            }
            UnnamedItem::Block(Braced { value: block, .. }) => {
                let items = match block {
                    // FIXME: This shouldn't panic, rather Err with a useful span
                    Items::OrderedChoice(..) => panic!("choice not allowed here"),
                    Items::Sequence(items) => items,
                };
                block_la(&items, &param)
            }
            UnnamedItem::Symbol(symbol) => {
                let ty = make_symbol_type(symbol);
                quote!(#param.peek(#ty))
            }
        })
        .chain(Some(quote!(true)))
        .collect_vec();
    let parse: Vec<TokenStream> = zip(&*variants, &*field_names)
        .map(|(variant, fields)| {
            match &variant.item {
                UnnamedItem::Predicated(..) => TokenStream::new(), // skip
                item @ UnnamedItem::Symbol(_) | item @ UnnamedItem::Repeated(..) => {
                    assert_eq!(fields.len(), 1);
                    let field = &fields[0];
                    detokenize(parse_quote!(#field), item, &param)
                }
                UnnamedItem::Block(Braced { value: block, .. }) => {
                    let items = match block {
                        // FIXME: This shouldn't panic, rather Err with a useful span
                        Items::OrderedChoice(..) => panic!("choice not allowed here"),
                        Items::Sequence(items) => items,
                    };
                    let items = items
                        .iter()
                        .filter(|item| !matches!(item.unnamed(), UnnamedItem::Predicated(..)))
                        .collect_vec();
                    assert_eq!(items.len(), fields.len());
                    let mut dt = TokenStream::new();
                    for (name, item) in zip(fields, items) {
                        dt.extend(detokenize(parse_quote!(#name), item.unnamed(), &param));
                    }
                    dt
                }
            }
        })
        .collect_vec();

    Ok(quote! {
        // deriving all is fairly safe since we depend on syn:extra-traits
        // so long as the runtime syn is 0.15 we'll have syn:extra-traits on
        // if not, something else is likely to break anyway
        // FIXME: Use pegcel::runtime::external::* to use known version
        #[derive(Clone, Debug, Eq, PartialEq, Hash)]
        pub enum #name {
            #(#variant_names #variant_fields,)*
        }

        impl ::syn::parse::Parse for #name {
            fn parse(#param: ::syn::parse::ParseStream) -> ::syn::parse::Result<Self> {
                #(if #is_variant {
                    #(let mut #field_names;)*
                    #parse
                    Ok(#name_ :: #variant_names #field_pats)
                } else)* {
                    // FIXME: Allow versions that don't assume the last is unpeekable
                    // i.e. do something like syn::parse::Lookahead1 for better errors when possible
                    unreachable!("pegcel ordered choice fallthrough case reached")
                }
            }
        }

        impl ::quote::ToTokens for #name {
            fn to_tokens(&self, #param: &mut ::proc_macro2::TokenStream) {
                use ::quote::{ToTokens, TokenStreamExt};
                match self {
                    #to_tokens
                }
            }
        }
    })
}

fn make_struct_production_def(name: Ident, members: Vec<Item>) -> Result<TokenStream> {
    let fields = make_fields(&members, parse_quote!(pub));
    let param = new_uuidv4_ident();

    // impl ToTokens

    let member_names: Vec<syn::Member> = match fields {
        syn::Fields::Named(_) => fields
            .iter()
            .map(|field| syn::Member::Named(field.ident.as_ref().unwrap().clone()))
            .collect(),
        syn::Fields::Unnamed(_) => fields
            .iter()
            .enumerate()
            .map(|(i, field)| {
                syn::Member::Unnamed(syn::Index {
                    index: i as u32,
                    span: field.span(),
                })
            })
            .collect(),
        syn::Fields::Unit => unreachable!("Unexpected Unit type {:?}", name),
    };
    let member_names = &*member_names;

    let mut to_tokens = quote! {
        use ::quote::{ToTokens, TokenStreamExt};
    };

    for (name, item) in zip(member_names, &members) {
        to_tokens.extend(tokenize(
            parse_quote!((&self.#name)),
            item.unnamed(),
            &param,
        ));
    }

    // impl Parse

    let field_names: Vec<Ident> = member_names
        .iter()
        .map(|member| match member {
            syn::Member::Named(name) => name.clone(),
            syn::Member::Unnamed(_) => new_uuidv4_ident(),
        })
        .collect();
    let field_names = &*field_names;

    let mut parse = TokenStream::new();

    for (name, item) in zip(field_names, &members) {
        parse.extend(detokenize(parse_quote!(#name), item.unnamed(), &param));
    }

    let make_struct = match fields {
        syn::Fields::Named(_) => quote!(#name { #(#field_names),* }),
        syn::Fields::Unnamed(_) => quote!(#name( #(#field_names),* )),
        syn::Fields::Unit => unreachable!("Unexpected Unit type {:?}", name),
    };

    // add semi only for tuple struct
    let fields = match fields {
        syn::Fields::Unnamed(fields) => quote!(#fields;),
        syn::Fields::Named(fields) => quote!(#fields),
        syn::Fields::Unit => unreachable!("Unexpected Unit type {:?}", name),
    };

    Ok(quote! {
        // deriving all is fairly safe since we depend on syn:extra-traits
        // so long as the runtime syn is 0.15 we'll have syn:extra-traits on
        // if not, something else is likely to break anyway
        // FIXME: Use pegcel::runtime::external::* to use known version
        #[derive(Clone, Debug, Eq, PartialEq, Hash)]
        pub struct #name #fields

        impl ::syn::parse::Parse for #name {
            fn parse(#param: ::syn::parse::ParseStream) -> ::syn::parse::Result<Self> {
                #(let mut #field_names;)*
                #parse
                Ok(#make_struct)
            }
        }

        impl ::quote::ToTokens for #name {
            fn to_tokens(&self, #param: &mut ::proc_macro2::TokenStream) {
                #to_tokens
            }
        }
    })
}

fn tokenize(place: syn::Expr, item: &UnnamedItem, param: &Ident) -> TokenStream {
    match item {
        UnnamedItem::Predicated(..) => TokenStream::new(), // skip
        UnnamedItem::Symbol(_) => quote!(#place.to_tokens(#param);),
        UnnamedItem::Block(Braced { value: block, .. }) => {
            let items = match block {
                Items::OrderedChoice(..) => unreachable!("errored earlier"),
                Items::Sequence(items) => items
                    .iter()
                    .map(|item| match item {
                        Item::Named(_) => unreachable!("errored earlier"),
                        Item::Unnamed(item) => item,
                    })
                    .filter(|item| !matches!(item, UnnamedItem::Predicated(..)))
                    .collect_vec(),
            };
            let names = items.iter().map(|_| new_uuidv4_ident()).collect_vec();
            let tokenization = zip(&names, &items)
                .map(|(name, item)| tokenize(parse_quote!(#name), item, param))
                .collect_vec();

            quote! {
                let (#(#names),*) = #place;
                #(#tokenization)*
            }
        }
        UnnamedItem::Repeated(item, repetition) => match repetition {
            Repetition::ZeroOne(_) => quote!(#place.to_tokens(#param);),
            Repetition::ZeroPlus(_, None) | Repetition::OnePlus(_, None) => {
                let ident = new_uuidv4_ident();
                let sub_tokenize = tokenize(parse_quote!(#ident), &item, param);
                quote! {
                    for #ident in #place {
                        #sub_tokenize
                    }
                }
            }
            Repetition::ZeroPlus(_, Some(_)) | Repetition::OnePlus(_, Some(_)) => {
                let pair = new_uuidv4_ident();
                let value = new_uuidv4_ident();
                let punct = new_uuidv4_ident();
                let sub_tokenize = tokenize(parse_quote!(#value), &item, param);
                quote! {
                    for #pair in #place.pairs() {
                        let #value = #pair.value();
                        #sub_tokenize

                        if let Some(#punct) = #pair.punct() {
                            #punct.to_tokens(#param);
                        }
                    }
                }
            }
        },
    }
}

#[derive(Copy, Clone, Debug, Ord, PartialOrd, Eq, PartialEq)]
#[allow(clippy::enum_variant_names)] // rust-lang/rust-clippy#739
enum Peek {
    Peek1,
    Peek2,
    Peek3,
}

impl Peek {
    fn succ(self) -> Option<Peek> {
        match self {
            Peek::Peek1 => Some(Peek::Peek2),
            Peek::Peek2 => Some(Peek::Peek3),
            Peek::Peek3 => None,
        }
    }
    fn succn(self, n: usize) -> Option<Peek> {
        if n > 0 {
            self.succ().and_then(|p| p.succn(n - 1))
        } else {
            Some(self)
        }
    }
}

fn la(parity: bool, peek: Peek, item: &UnnamedItem, param: &Ident) -> TokenStream {
    let not = if !parity { quote!(!) } else { quote!() };
    let f = match peek {
        Peek::Peek1 => quote!(#not #param.peek),
        Peek::Peek2 => quote!(#not #param.peek2),
        Peek::Peek3 => quote!(#not #param.peek3),
    };
    match item {
        UnnamedItem::Predicated(pred, item) => {
            // +/+ => +
            // +/- & -/+ => -
            // -/- => +
            la(parity == pred.positive(), peek, item, param)
        }
        UnnamedItem::Repeated(..) => unreachable!("predicate cannot contain repetition"),
        UnnamedItem::Symbol(symbol) => {
            let ty = make_symbol_type(symbol);
            quote!(#f(#ty))
        }
        UnnamedItem::Block(Braced { value: block, .. }) => match block {
            Items::OrderedChoice(_, items) => {
                let individual = items
                    .iter()
                    .map(|item| {
                        // FIXME: These should not require names
                        la(parity, peek, &item.item, param)
                    })
                    .collect_vec();
                quote!((#(#individual)||*))
            }
            Items::Sequence(items) => {
                let mut i = 0;
                let i = &mut i;
                let individual = items
                    .iter()
                    .map(|item| {
                        // FIXME: This shouldn't panic, rather Err with a useful span
                        let expr = la(
                            parity,
                            peek.succn(*i).expect("peek distance"),
                            item.unnamed(),
                            param,
                        );
                        if !matches!(item.unnamed(), UnnamedItem::Predicated(..)) {
                            *i += 1;
                        }
                        expr
                    })
                    .collect_vec();
                quote!((#(#individual &&)* true))
            }
        },
    }
}

fn block_la(items: &[Item], param: &Ident) -> TokenStream {
    assert!(!items.is_empty()); // `{}?` already caught as non-advancing
    match items[0].unnamed() {
        UnnamedItem::Block(..) | UnnamedItem::Repeated(..) => {
            // FIXME: This shouldn't panic, rather Err with a useful span
            // FIXME: Used for ordered choice lookahead as well, needs appropriate error
            panic!("cannot start repetition with block or repetition")
        }
        UnnamedItem::Predicated(pred, item) => la(pred.positive(), Peek::Peek1, item, param),
        UnnamedItem::Symbol(symbol) => {
            let ty = make_symbol_type(symbol);
            quote!(#param.peek(#ty))
        }
    }
}

fn detokenize(place: syn::Expr, item: &UnnamedItem, param: &Ident) -> TokenStream {
    match item {
        UnnamedItem::Predicated(pred, item) => {
            let la = la(pred.positive(), Peek::Peek1, item, param);
            quote!(assert!(#la);)
        }
        UnnamedItem::Symbol(_) => quote!(#place = #param.parse()?;),
        UnnamedItem::Repeated(item, rep) => match rep {
            Repetition::ZeroOne(_) => match &**item {
                UnnamedItem::Predicated(..) | UnnamedItem::Repeated(..) => {
                    unreachable!("repetition cannot contain predicate or repetition")
                }
                UnnamedItem::Symbol(_) => quote!(#place = #param.parse()?;),
                UnnamedItem::Block(Braced { value: block, .. }) => {
                    let items = match block {
                        // FIXME: This shouldn't panic, rather Err with a useful span
                        Items::OrderedChoice(..) => panic!("choice not allowed here"),
                        Items::Sequence(items) => items,
                    };
                    let cond = block_la(items, param);
                    let val = new_uuidv4_ident();
                    let dt = detokenize(parse_quote!(#val), item, param);
                    quote! {
                        if #cond {
                            let #val;
                            #dt
                            #place = Some(#val);
                        } else {
                            #place = None;
                        }
                    }
                }
            },
            Repetition::ZeroPlus(_, None) | Repetition::OnePlus(_, None) => {
                let cond = match &**item {
                    UnnamedItem::Predicated(..) | UnnamedItem::Repeated(..) => {
                        unreachable!("repetition cannot contain predicate or repetition")
                    }
                    UnnamedItem::Symbol(symbol) => match symbol {
                        // FIXME: This doesn't allow stopping with a peekable symbol
                        // WORKAROUND: {&syn::Ident syn::Ident}*
                        Symbol::Path(_) => quote!(!#param.is_empty()),
                        Symbol::Literal(_) => {
                            let ty = make_symbol_type(symbol);
                            quote!(#param.peek(#ty))
                        }
                    },
                    UnnamedItem::Block(Braced { value: block, .. }) => {
                        let items = match block {
                            // FIXME: This shouldn't panic, rather Err with a useful span
                            Items::OrderedChoice(..) => panic!("choice not allowed here"),
                            Items::Sequence(items) => items,
                        };
                        // FIXME: This doesn't allow non-peekable blocks
                        block_la(items, param)
                    }
                };
                let val = new_uuidv4_ident();
                let dt = detokenize(parse_quote!(#val), item, param);
                let init = match rep {
                    Repetition::ZeroPlus(..) => quote!(),
                    Repetition::OnePlus(..) => quote!({ let #val; #dt #val }),
                    _ => unreachable!("matched above"),
                };
                quote! {
                    #place = vec![#init];
                    while #cond {
                        let #val;
                        #dt
                        #place.push(#val);
                    }
                }
            }
            Repetition::ZeroPlus(_, Some(Interspersion::Separated(..))) => quote! {
                // FIXME: Do manual parsing to allow peeking to move on
                #place = if #param.is_empty {
                    ::syn::punctuated::Punctuated::new();
                } else {
                    #param.call(::syn::punctuated::Punctuated::parse_separated_nonempty)?;
                }
            },
            Repetition::OnePlus(_, Some(Interspersion::Separated(..))) => quote! {
                // FIXME: Do manual parsing to open possibility of non-symbol separators
                #place = #param.call(::syn::punctuated::Punctuated::parse_separated_nonempty)?;
            },
            Repetition::ZeroPlus(_, Some(Interspersion::Terminated(..))) => quote! {
                // FIXME: Do manual parsing to make this not require ending stream
                #place = #param.call(::syn::punctuated::Punctuated::parse_terminated)?;
            },
            Repetition::OnePlus(_, Some(Interspersion::Terminated(..))) => {
                // FIXME: Do manual parsing to avoid double-parsing the first item
                let fork = new_uuidv4_ident();
                let ty = make_type(item).unwrap();
                quote! {
                    let #fork = #param.fork();
                    let _: #ty = #fork.parse()?;
                    #place = #param.call(::syn::punctuated::Punctuated::parse_terminated)?;
                }
            }
        },
        UnnamedItem::Block(Braced { value: block, .. }) => {
            let items = match block {
                // FIXME: This shouldn't panic, rather Err with a useful span
                Items::OrderedChoice(..) => panic!("choice not allowed in non-root"),
                Items::Sequence(items) => items,
            };
            let mut make = TokenStream::new();
            let mut ret = vec![];
            for item in items {
                let i = new_uuidv4_ident();
                let p = detokenize(parse_quote!(#i), item.unnamed(), param);
                if matches!(item.unnamed(), UnnamedItem::Predicated(..)) {
                    make.extend(p);
                } else {
                    make.extend(quote!(let #i; #p));
                    ret.push(i);
                }
            }
            quote!(#place = { #make (#(#ret),*) };)
        }
    }
}

fn make_fields(items: &[Item], vis: syn::Visibility) -> syn::Fields {
    if items.iter().any(|item| matches!(item, Item::Named(_))) {
        syn::Fields::Named(make_named_fields(items.iter(), vis))
    } else {
        syn::Fields::Unnamed(make_unnamed_fields(
            items.iter().map(|item| match item {
                Item::Unnamed(item) => item,
                Item::Named(_) => unreachable!(),
            }),
            vis,
        ))
    }
}

fn make_named_fields<'a>(
    items: impl Iterator<Item = &'a Item>,
    vis: syn::Visibility,
) -> syn::FieldsNamed {
    syn::FieldsNamed {
        brace_token: Default::default(),
        named: items
            .map(|item| match item {
                Item::Named(item) => Some(syn::Field {
                    attrs: vec![],
                    vis: vis.clone(),
                    ident: Some(item.name.clone()),
                    colon_token: Some(Default::default()),
                    ty: make_type(&item.item)?,
                }),
                Item::Unnamed(item) => Some(syn::Field {
                    attrs: vec![],
                    vis: syn::Visibility::Inherited,
                    ident: Some(new_uuidv4_ident()),
                    colon_token: Some(Default::default()),
                    ty: make_type(item)?,
                }),
            })
            .flatten()
            .collect(),
    }
}

fn make_unnamed_fields<'a>(
    items: impl Iterator<Item = &'a UnnamedItem>,
    vis: syn::Visibility,
) -> syn::FieldsUnnamed {
    syn::FieldsUnnamed {
        paren_token: Default::default(),
        unnamed: items
            .map(|item| {
                Some(syn::Field {
                    attrs: vec![],
                    vis: vis.clone(),
                    ident: None,
                    colon_token: None,
                    ty: make_type(item)?,
                })
            })
            .flatten()
            .collect(),
    }
}

fn make_symbol_type(symbol: &Symbol) -> syn::Type {
    match symbol {
        Symbol::Path(path) => syn::Type::Path(syn::TypePath {
            qself: None,
            path: (*path).clone(),
        }),
        Symbol::Literal(lit) => {
            // FIXME: This shouldn't panic, rather Err with a useful span
            let tok: TokenStream = lit
                .parse()
                .unwrap_or_else(|_| panic!("Failed to parse token literal"));
            parse_quote!(Token![#tok])
        }
    }
}

fn make_type(item: &UnnamedItem) -> Option<syn::Type> {
    match item {
        UnnamedItem::Predicated(_, _) => None,
        UnnamedItem::Repeated(item, repetition) => Some({
            // FIXME: This shouldn't panic, rather Err with a useful span
            let inner_ty = make_type(item).unwrap_or_else(|| panic!("Non-advancing repetition"));
            match repetition {
                Repetition::ZeroOne(_) => parse_quote!(Option<#inner_ty>),
                Repetition::ZeroPlus(_, None) | Repetition::OnePlus(_, None) => {
                    parse_quote!(Vec<#inner_ty>)
                }
                Repetition::ZeroPlus(_, Some(interspersion))
                | Repetition::OnePlus(_, Some(interspersion)) => match interspersion {
                    Interspersion::Terminated(_, symbol) | Interspersion::Separated(_, symbol) => {
                        let symbol_ty = make_symbol_type(symbol);
                        parse_quote!(::syn::punctuated::Punctuated<#inner_ty, #symbol_ty>)
                    }
                },
            }
        }),
        UnnamedItem::Block(Braced { value: items, .. }) => Some({
            match items {
                Items::OrderedChoice(..) => {
                    // FIXME: This shouldn't panic, rather Err with a useful span
                    panic!("Cannot have ordered choice blocks not at root level")
                }
                Items::Sequence(items) => {
                    syn::Type::Tuple(syn::TypeTuple {
                        paren_token: Default::default(),
                        elems: items
                            .iter()
                            .map(|item| match item {
                                Item::Named(_) => {
                                    // FIXME: This shouldn't panic, rather Err with a useful span
                                    panic!("Cannot have named blocks not at root level")
                                }
                                Item::Unnamed(item) => make_type(item),
                            })
                            .flatten()
                            .collect(),
                    })
                }
            }
        }),
        UnnamedItem::Symbol(symbol) => Some(make_symbol_type(symbol)),
    }
}
