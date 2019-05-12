#[path = "../src/manual_grammar.rs"]
mod manual_grammar;

mod grammar {
    use pegcel_macros::pegcel_syn;
    pegcel_syn! {
        use syn
        use crate::manual_grammar::UnnamedItem

        Grammar: {
            kind: GrammarUse
            uses: {&"use" Use}*
            items: NamedItem*
        }

        GrammarUse: {
            / Syn: {"use" "syn"}
        }

        Use: {
            r#use: "use"
            anchor: "::"?
            tree: syn::UseTree
        }

        Item: {
            / Named: {&{syn::Ident !"::" ":"} NamedItem}
            / Unnamed: UnnamedItem
        }

        NamedItem: {
            name: syn::Ident
            colon: ":"
            item: UnnamedItem
        }

        Items: {
            / OrderedChoice: {"/" NamedItem+ % "/"}
            / Sequence: Item*
        }

        Predicate: {
            / Positive: "&"
            / Negative: "!"
        }

        Repetition: {
            / ZeroPlus: {"*" {&"%" Interspersion}?}
            / OnePlus: {"+" {&"%" Interspersion}?}
            / ZeroOne: "?"
        }

        Interspersion: {
            / Terminated: {"%%" Symbol}
            / Separated: {"%" Symbol}
        }

        Symbol: {
            / Literal: syn::LitStr
            / Path: syn::Path
        }
    }
}

#[test]
fn meta_grammar_compiles() {}
