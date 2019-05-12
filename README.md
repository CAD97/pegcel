# Pegcel: a [PEG] parser generator for [Syn]

[![Chat on Discord](https://img.shields.io/badge/-chat-26262b.svg?style=popout&logo=discord)][Discord]

Pegcel does the boring parts of creating [Syn]-style [Syntax tree types] from a
grammar for you, while still allowing you to have all of the power of manually
implemented parsing.

In short, Pegcel takes input [like this](pegcel/macros/tests/meta.rs):

```
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
```

and [generates](pegcel/macros/src/grammar.rs) all the types described, along
with implementations for `syn::parse::Parse` and `quote::ToTokens`, so you can
use them as if the were normal AST types provided by Syn.

Pegcel is self-hosting, so check out the meta grammar at
[pegcel/macros/tests/meta.rs](pegcel/macros/tests/meta.rs) and the generated
output at [pegcel/macros/src/grammar.rs](pegcel/macros/src/grammar.rs).

## Notes

Pegcel expects you to have `syn:^0.15.34`, `quote:^0.6.12`, and
`proc-macro2:^0.4.27` available as cargo dependencies. They must
not be renamed by cargo. Obviously, `syn` and `quote` become
public dependencies of your crate if you export the AST types.

A `Token` macro is generated by Pegcel to support any custom
tokens used in your grammar as well as those provided by Syn.
Due to current restrictions on `proc-macro` crates, this macro
is `pub(crate)` and cannot be made more public by default. An
opt-in to making it public will be offered in the future.

## Tutorial

Actually, I'm uncertain how to explain how to use Pegcel!
Hit me up at #1 and on [Discord] and ask me questions, and
then we can document it together.

  [Syn]: <https://github.com/dtolnay/syn>
  [Syntax tree types]: <https://docs.rs/syn/0.15/syn/enum.Expr.html#syntax-tree-enums>
  [UseTree]: <https://docs.rs/syn/0.15/syn/enum.UseTree.html>
  [PEG]: <https://en.wikipedia.org/wiki/Parsing_expression_grammar>
  [Discord]: <https://discord.gg/FuPE9JE>
