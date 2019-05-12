#![allow(clippy::eval_order_dependence)]

use {
    crate::grammar::*,
    matches::matches,
    proc_macro2::TokenStream,
    quote::*,
    syn::{parse::*, Ident},
};

/*
#[manual_parse]
UnnamedItem: {
    / Predicated: {Predicate {:Block / :Symbol}}
    / Repeated: {{:Block / :Symbol} Repetition}
    / Block: {Braced<Items>}
    / Symbol: Symbol
}
*/

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub enum UnnamedItem {
    Predicated(Predicate, Box<UnnamedItem>),
    Repeated(Box<UnnamedItem>, Repetition),
    Block(Braced<Items>),
    Symbol(Symbol),
}

impl Parse for UnnamedItem {
    fn parse(input: ParseStream) -> Result<Self> {
        let item = input.call(UnnamedItem::parse_without_repeated)?;
        if matches!(item, UnnamedItem::Block(..) | UnnamedItem::Symbol(..)) {
            if input.peek(Token![*]) || input.peek(Token![+]) || input.peek(Token![?]) {
                Ok(UnnamedItem::Repeated(Box::new(item), input.parse()?))
            } else {
                Ok(item)
            }
        } else {
            Ok(item)
        }
    }
}

impl UnnamedItem {
    fn parse_without_repeated(input: ParseStream) -> Result<Self> {
        let la = input.lookahead1();

        if la.peek(Token![&]) | la.peek(Token![!]) {
            Ok(UnnamedItem::Predicated(
                input.parse()?,
                Box::new(input.call(UnnamedItem::parse_just_block_symbol)?),
            ))
        } else if la.peek(syn::token::Brace) {
            Ok(UnnamedItem::Block(input.parse()?))
        } else if la.peek(syn::LitStr) || la.peek(Ident) {
            Ok(UnnamedItem::Symbol(input.parse()?))
        } else {
            Err(la.error())
        }
    }

    fn parse_just_block_symbol(input: ParseStream) -> Result<Self> {
        let la = input.lookahead1();

        if la.peek(syn::token::Brace) {
            Ok(UnnamedItem::Block(input.parse()?))
        } else if la.peek(syn::LitStr) || la.peek(Ident) {
            Ok(UnnamedItem::Symbol(input.parse()?))
        } else {
            Err(la.error())
        }
    }
}

impl ToTokens for UnnamedItem {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            UnnamedItem::Predicated(pred, item) => {
                pred.to_tokens(tokens);
                item.to_tokens(tokens);
            }
            UnnamedItem::Repeated(item, rep) => {
                item.to_tokens(tokens);
                rep.to_tokens(tokens);
            }
            UnnamedItem::Block(block) => {
                block.to_tokens(tokens);
            }
            UnnamedItem::Symbol(symb) => {
                symb.to_tokens(tokens);
            }
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct Braced<T> {
    pub bracket: syn::token::Brace,
    pub value: T,
}

impl<T: Parse> Parse for Braced<T> {
    fn parse(input: ParseStream) -> Result<Self> {
        let content;
        Ok(Braced {
            bracket: syn::braced!(content in input),
            value: content.parse()?,
        })
    }
}

impl<T: ToTokens> ToTokens for Braced<T> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.bracket
            .surround(tokens, |tokens| self.value.to_tokens(tokens));
    }
}
