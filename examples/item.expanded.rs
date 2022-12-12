//! syn::Item
use syn::{
    ItemConst, ItemEnum, ItemExternCrate, ItemFn, ItemForeignMod, ItemImpl, ItemMacro,
    ItemMacro2, ItemMod, ItemStatic, ItemStruct, ItemTrait, ItemTraitAlias, ItemType,
    ItemUnion, ItemUse,
};
enum Item {
    Const(ItemConst),
    Enum(ItemEnum),
    ExternCrate(ItemExternCrate),
    Fn(ItemFn),
    ForeignMod(ItemForeignMod),
    Impl(ItemImpl),
    Macro(ItemMacro),
    Macro2(ItemMacro2),
    Mod(ItemMod),
    Static(ItemStatic),
    Struct(ItemStruct),
    Trait(ItemTrait),
    TraitAlias(ItemTraitAlias),
    Type(ItemType),
    Union(ItemUnion),
    Use(ItemUse),
}
#[automatically_derived]
impl ::pegcel::std::clone::Clone for Item {
    fn clone(&self) -> Self {
        match self {
            Item::Const(v) => Item::Const(v.clone()),
            Item::Enum(v) => Item::Enum(v.clone()),
            Item::ExternCrate(v) => Item::ExternCrate(v.clone()),
            Item::Fn(v) => Item::Fn(v.clone()),
            Item::ForeignMod(v) => Item::ForeignMod(v.clone()),
            Item::Impl(v) => Item::Impl(v.clone()),
            Item::Macro(v) => Item::Macro(v.clone()),
            Item::Macro2(v) => Item::Macro2(v.clone()),
            Item::Mod(v) => Item::Mod(v.clone()),
            Item::Static(v) => Item::Static(v.clone()),
            Item::Struct(v) => Item::Struct(v.clone()),
            Item::Trait(v) => Item::Trait(v.clone()),
            Item::TraitAlias(v) => Item::TraitAlias(v.clone()),
            Item::Type(v) => Item::Type(v.clone()),
            Item::Union(v) => Item::Union(v.clone()),
            Item::Use(v) => Item::Use(v.clone()),
        }
    }
}
#[automatically_derived]
impl ::pegcel::std::fmt::Debug for Item {
    fn fmt(
        &self,
        f: &mut ::pegcel::std::fmt::Formatter<'_>,
    ) -> ::pegcel::std::fmt::Result {
        match self {
            Item::Const(v) => f.debug_tuple("Const").field(v).finish(),
            Item::Enum(v) => f.debug_tuple("Enum").field(v).finish(),
            Item::ExternCrate(v) => f.debug_tuple("ExternCrate").field(v).finish(),
            Item::Fn(v) => f.debug_tuple("Fn").field(v).finish(),
            Item::ForeignMod(v) => f.debug_tuple("ForeignMod").field(v).finish(),
            Item::Impl(v) => f.debug_tuple("Impl").field(v).finish(),
            Item::Macro(v) => f.debug_tuple("Macro").field(v).finish(),
            Item::Macro2(v) => f.debug_tuple("Macro2").field(v).finish(),
            Item::Mod(v) => f.debug_tuple("Mod").field(v).finish(),
            Item::Static(v) => f.debug_tuple("Static").field(v).finish(),
            Item::Struct(v) => f.debug_tuple("Struct").field(v).finish(),
            Item::Trait(v) => f.debug_tuple("Trait").field(v).finish(),
            Item::TraitAlias(v) => f.debug_tuple("TraitAlias").field(v).finish(),
            Item::Type(v) => f.debug_tuple("Type").field(v).finish(),
            Item::Union(v) => f.debug_tuple("Union").field(v).finish(),
            Item::Use(v) => f.debug_tuple("Use").field(v).finish(),
        }
    }
}
#[automatically_derived]
impl ::pegcel::std::hash::Hash for Item {
    fn hash<H: ::pegcel::std::hash::Hasher>(&self, state: &mut H) {
        match self {
            Item::Const(v) => {
                ::pegcel::std::mem::discriminant(self).hash(state);
                v.hash(state);
            }
            Item::Enum(v) => {
                ::pegcel::std::mem::discriminant(self).hash(state);
                v.hash(state);
            }
            Item::ExternCrate(v) => {
                ::pegcel::std::mem::discriminant(self).hash(state);
                v.hash(state);
            }
            Item::Fn(v) => {
                ::pegcel::std::mem::discriminant(self).hash(state);
                v.hash(state);
            }
            Item::ForeignMod(v) => {
                ::pegcel::std::mem::discriminant(self).hash(state);
                v.hash(state);
            }
            Item::Impl(v) => {
                ::pegcel::std::mem::discriminant(self).hash(state);
                v.hash(state);
            }
            Item::Macro(v) => {
                ::pegcel::std::mem::discriminant(self).hash(state);
                v.hash(state);
            }
            Item::Macro2(v) => {
                ::pegcel::std::mem::discriminant(self).hash(state);
                v.hash(state);
            }
            Item::Mod(v) => {
                ::pegcel::std::mem::discriminant(self).hash(state);
                v.hash(state);
            }
            Item::Static(v) => {
                ::pegcel::std::mem::discriminant(self).hash(state);
                v.hash(state);
            }
            Item::Struct(v) => {
                ::pegcel::std::mem::discriminant(self).hash(state);
                v.hash(state);
            }
            Item::Trait(v) => {
                ::pegcel::std::mem::discriminant(self).hash(state);
                v.hash(state);
            }
            Item::TraitAlias(v) => {
                ::pegcel::std::mem::discriminant(self).hash(state);
                v.hash(state);
            }
            Item::Type(v) => {
                ::pegcel::std::mem::discriminant(self).hash(state);
                v.hash(state);
            }
            Item::Union(v) => {
                ::pegcel::std::mem::discriminant(self).hash(state);
                v.hash(state);
            }
            Item::Use(v) => {
                ::pegcel::std::mem::discriminant(self).hash(state);
                v.hash(state);
            }
        }
    }
}
#[automatically_derived]
impl ::pegcel::std::cmp::PartialEq for Item {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Item::Const(lhs), Item::Const(rhs)) => lhs == rhs,
            (Item::Enum(lhs), Item::Enum(rhs)) => lhs == rhs,
            (Item::ExternCrate(lhs), Item::ExternCrate(rhs)) => lhs == rhs,
            (Item::Fn(lhs), Item::Fn(rhs)) => lhs == rhs,
            (Item::ForeignMod(lhs), Item::ForeignMod(rhs)) => lhs == rhs,
            (Item::Impl(lhs), Item::Impl(rhs)) => lhs == rhs,
            (Item::Macro(lhs), Item::Macro(rhs)) => lhs == rhs,
            (Item::Macro2(lhs), Item::Macro2(rhs)) => lhs == rhs,
            (Item::Mod(lhs), Item::Mod(rhs)) => lhs == rhs,
            (Item::Static(lhs), Item::Static(rhs)) => lhs == rhs,
            (Item::Struct(lhs), Item::Struct(rhs)) => lhs == rhs,
            (Item::Trait(lhs), Item::Trait(rhs)) => lhs == rhs,
            (Item::TraitAlias(lhs), Item::TraitAlias(rhs)) => lhs == rhs,
            (Item::Type(lhs), Item::Type(rhs)) => lhs == rhs,
            (Item::Union(lhs), Item::Union(rhs)) => lhs == rhs,
            (Item::Use(lhs), Item::Use(rhs)) => lhs == rhs,
            _ => false,
        }
    }
}
#[automatically_derived]
impl ::pegcel::std::cmp::Eq for Item {}
#[automatically_derived]
impl From<ItemConst> for Item {
    fn from(v: ItemConst) -> Self {
        Item::Const(v)
    }
}
#[automatically_derived]
impl From<ItemEnum> for Item {
    fn from(v: ItemEnum) -> Self {
        Item::Enum(v)
    }
}
#[automatically_derived]
impl From<ItemExternCrate> for Item {
    fn from(v: ItemExternCrate) -> Self {
        Item::ExternCrate(v)
    }
}
#[automatically_derived]
impl From<ItemFn> for Item {
    fn from(v: ItemFn) -> Self {
        Item::Fn(v)
    }
}
#[automatically_derived]
impl From<ItemForeignMod> for Item {
    fn from(v: ItemForeignMod) -> Self {
        Item::ForeignMod(v)
    }
}
#[automatically_derived]
impl From<ItemImpl> for Item {
    fn from(v: ItemImpl) -> Self {
        Item::Impl(v)
    }
}
#[automatically_derived]
impl From<ItemMacro> for Item {
    fn from(v: ItemMacro) -> Self {
        Item::Macro(v)
    }
}
#[automatically_derived]
impl From<ItemMacro2> for Item {
    fn from(v: ItemMacro2) -> Self {
        Item::Macro2(v)
    }
}
#[automatically_derived]
impl From<ItemMod> for Item {
    fn from(v: ItemMod) -> Self {
        Item::Mod(v)
    }
}
#[automatically_derived]
impl From<ItemStatic> for Item {
    fn from(v: ItemStatic) -> Self {
        Item::Static(v)
    }
}
#[automatically_derived]
impl From<ItemStruct> for Item {
    fn from(v: ItemStruct) -> Self {
        Item::Struct(v)
    }
}
#[automatically_derived]
impl From<ItemTrait> for Item {
    fn from(v: ItemTrait) -> Self {
        Item::Trait(v)
    }
}
#[automatically_derived]
impl From<ItemTraitAlias> for Item {
    fn from(v: ItemTraitAlias) -> Self {
        Item::TraitAlias(v)
    }
}
#[automatically_derived]
impl From<ItemType> for Item {
    fn from(v: ItemType) -> Self {
        Item::Type(v)
    }
}
#[automatically_derived]
impl From<ItemUnion> for Item {
    fn from(v: ItemUnion) -> Self {
        Item::Union(v)
    }
}
#[automatically_derived]
impl From<ItemUse> for Item {
    fn from(v: ItemUse) -> Self {
        Item::Use(v)
    }
}
#[automatically_derived]
impl ::pegcel::syn::parse::Parse for Item {
    fn parse(
        input: ::pegcel::syn::parse::ParseStream,
    ) -> ::pegcel::syn::parse::Result<Self> {
        use ::pegcel::syn::parse::discouraged::Speculative;
        let mut best_guess = ::pegcel::std::option::Option::None;
        let mut best_guess_cursor = input.cursor();
        let mut best_guess_variant = "";
        {
            let fork = input.fork();
            match fork.parse::<ItemConst>() {
                ::pegcel::std::result::Result::Ok(v) => {
                    input.advance_to(&fork);
                    return ::pegcel::std::result::Result::Ok(Item::Const(v));
                }
                ::pegcel::std::result::Result::Err(e) => {
                    let this_guess_cursor = fork.cursor();
                    if this_guess_cursor > best_guess_cursor {
                        best_guess = ::pegcel::std::option::Option::Some(e);
                        best_guess_variant = "Const";
                        best_guess_cursor = this_guess_cursor;
                    } else if this_guess_cursor == best_guess_cursor {
                        if let ::pegcel::std::option::Option::Some(existing)
                            = &mut best_guess {
                            existing.combine(e);
                        } else {
                            best_guess = ::pegcel::std::option::Option::Some(e);
                        }
                    }
                }
            }
        }
        {
            let fork = input.fork();
            match fork.parse::<ItemEnum>() {
                ::pegcel::std::result::Result::Ok(v) => {
                    input.advance_to(&fork);
                    return ::pegcel::std::result::Result::Ok(Item::Enum(v));
                }
                ::pegcel::std::result::Result::Err(e) => {
                    let this_guess_cursor = fork.cursor();
                    if this_guess_cursor > best_guess_cursor {
                        best_guess = ::pegcel::std::option::Option::Some(e);
                        best_guess_variant = "Enum";
                        best_guess_cursor = this_guess_cursor;
                    } else if this_guess_cursor == best_guess_cursor {
                        if let ::pegcel::std::option::Option::Some(existing)
                            = &mut best_guess {
                            existing.combine(e);
                        } else {
                            best_guess = ::pegcel::std::option::Option::Some(e);
                        }
                    }
                }
            }
        }
        {
            let fork = input.fork();
            match fork.parse::<ItemExternCrate>() {
                ::pegcel::std::result::Result::Ok(v) => {
                    input.advance_to(&fork);
                    return ::pegcel::std::result::Result::Ok(Item::ExternCrate(v));
                }
                ::pegcel::std::result::Result::Err(e) => {
                    let this_guess_cursor = fork.cursor();
                    if this_guess_cursor > best_guess_cursor {
                        best_guess = ::pegcel::std::option::Option::Some(e);
                        best_guess_variant = "ExternCrate";
                        best_guess_cursor = this_guess_cursor;
                    } else if this_guess_cursor == best_guess_cursor {
                        if let ::pegcel::std::option::Option::Some(existing)
                            = &mut best_guess {
                            existing.combine(e);
                        } else {
                            best_guess = ::pegcel::std::option::Option::Some(e);
                        }
                    }
                }
            }
        }
        {
            let fork = input.fork();
            match fork.parse::<ItemFn>() {
                ::pegcel::std::result::Result::Ok(v) => {
                    input.advance_to(&fork);
                    return ::pegcel::std::result::Result::Ok(Item::Fn(v));
                }
                ::pegcel::std::result::Result::Err(e) => {
                    let this_guess_cursor = fork.cursor();
                    if this_guess_cursor > best_guess_cursor {
                        best_guess = ::pegcel::std::option::Option::Some(e);
                        best_guess_variant = "Fn";
                        best_guess_cursor = this_guess_cursor;
                    } else if this_guess_cursor == best_guess_cursor {
                        if let ::pegcel::std::option::Option::Some(existing)
                            = &mut best_guess {
                            existing.combine(e);
                        } else {
                            best_guess = ::pegcel::std::option::Option::Some(e);
                        }
                    }
                }
            }
        }
        {
            let fork = input.fork();
            match fork.parse::<ItemForeignMod>() {
                ::pegcel::std::result::Result::Ok(v) => {
                    input.advance_to(&fork);
                    return ::pegcel::std::result::Result::Ok(Item::ForeignMod(v));
                }
                ::pegcel::std::result::Result::Err(e) => {
                    let this_guess_cursor = fork.cursor();
                    if this_guess_cursor > best_guess_cursor {
                        best_guess = ::pegcel::std::option::Option::Some(e);
                        best_guess_variant = "ForeignMod";
                        best_guess_cursor = this_guess_cursor;
                    } else if this_guess_cursor == best_guess_cursor {
                        if let ::pegcel::std::option::Option::Some(existing)
                            = &mut best_guess {
                            existing.combine(e);
                        } else {
                            best_guess = ::pegcel::std::option::Option::Some(e);
                        }
                    }
                }
            }
        }
        {
            let fork = input.fork();
            match fork.parse::<ItemImpl>() {
                ::pegcel::std::result::Result::Ok(v) => {
                    input.advance_to(&fork);
                    return ::pegcel::std::result::Result::Ok(Item::Impl(v));
                }
                ::pegcel::std::result::Result::Err(e) => {
                    let this_guess_cursor = fork.cursor();
                    if this_guess_cursor > best_guess_cursor {
                        best_guess = ::pegcel::std::option::Option::Some(e);
                        best_guess_variant = "Impl";
                        best_guess_cursor = this_guess_cursor;
                    } else if this_guess_cursor == best_guess_cursor {
                        if let ::pegcel::std::option::Option::Some(existing)
                            = &mut best_guess {
                            existing.combine(e);
                        } else {
                            best_guess = ::pegcel::std::option::Option::Some(e);
                        }
                    }
                }
            }
        }
        {
            let fork = input.fork();
            match fork.parse::<ItemMacro>() {
                ::pegcel::std::result::Result::Ok(v) => {
                    input.advance_to(&fork);
                    return ::pegcel::std::result::Result::Ok(Item::Macro(v));
                }
                ::pegcel::std::result::Result::Err(e) => {
                    let this_guess_cursor = fork.cursor();
                    if this_guess_cursor > best_guess_cursor {
                        best_guess = ::pegcel::std::option::Option::Some(e);
                        best_guess_variant = "Macro";
                        best_guess_cursor = this_guess_cursor;
                    } else if this_guess_cursor == best_guess_cursor {
                        if let ::pegcel::std::option::Option::Some(existing)
                            = &mut best_guess {
                            existing.combine(e);
                        } else {
                            best_guess = ::pegcel::std::option::Option::Some(e);
                        }
                    }
                }
            }
        }
        {
            let fork = input.fork();
            match fork.parse::<ItemMacro2>() {
                ::pegcel::std::result::Result::Ok(v) => {
                    input.advance_to(&fork);
                    return ::pegcel::std::result::Result::Ok(Item::Macro2(v));
                }
                ::pegcel::std::result::Result::Err(e) => {
                    let this_guess_cursor = fork.cursor();
                    if this_guess_cursor > best_guess_cursor {
                        best_guess = ::pegcel::std::option::Option::Some(e);
                        best_guess_variant = "Macro2";
                        best_guess_cursor = this_guess_cursor;
                    } else if this_guess_cursor == best_guess_cursor {
                        if let ::pegcel::std::option::Option::Some(existing)
                            = &mut best_guess {
                            existing.combine(e);
                        } else {
                            best_guess = ::pegcel::std::option::Option::Some(e);
                        }
                    }
                }
            }
        }
        {
            let fork = input.fork();
            match fork.parse::<ItemMod>() {
                ::pegcel::std::result::Result::Ok(v) => {
                    input.advance_to(&fork);
                    return ::pegcel::std::result::Result::Ok(Item::Mod(v));
                }
                ::pegcel::std::result::Result::Err(e) => {
                    let this_guess_cursor = fork.cursor();
                    if this_guess_cursor > best_guess_cursor {
                        best_guess = ::pegcel::std::option::Option::Some(e);
                        best_guess_variant = "Mod";
                        best_guess_cursor = this_guess_cursor;
                    } else if this_guess_cursor == best_guess_cursor {
                        if let ::pegcel::std::option::Option::Some(existing)
                            = &mut best_guess {
                            existing.combine(e);
                        } else {
                            best_guess = ::pegcel::std::option::Option::Some(e);
                        }
                    }
                }
            }
        }
        {
            let fork = input.fork();
            match fork.parse::<ItemStatic>() {
                ::pegcel::std::result::Result::Ok(v) => {
                    input.advance_to(&fork);
                    return ::pegcel::std::result::Result::Ok(Item::Static(v));
                }
                ::pegcel::std::result::Result::Err(e) => {
                    let this_guess_cursor = fork.cursor();
                    if this_guess_cursor > best_guess_cursor {
                        best_guess = ::pegcel::std::option::Option::Some(e);
                        best_guess_variant = "Static";
                        best_guess_cursor = this_guess_cursor;
                    } else if this_guess_cursor == best_guess_cursor {
                        if let ::pegcel::std::option::Option::Some(existing)
                            = &mut best_guess {
                            existing.combine(e);
                        } else {
                            best_guess = ::pegcel::std::option::Option::Some(e);
                        }
                    }
                }
            }
        }
        {
            let fork = input.fork();
            match fork.parse::<ItemStruct>() {
                ::pegcel::std::result::Result::Ok(v) => {
                    input.advance_to(&fork);
                    return ::pegcel::std::result::Result::Ok(Item::Struct(v));
                }
                ::pegcel::std::result::Result::Err(e) => {
                    let this_guess_cursor = fork.cursor();
                    if this_guess_cursor > best_guess_cursor {
                        best_guess = ::pegcel::std::option::Option::Some(e);
                        best_guess_variant = "Struct";
                        best_guess_cursor = this_guess_cursor;
                    } else if this_guess_cursor == best_guess_cursor {
                        if let ::pegcel::std::option::Option::Some(existing)
                            = &mut best_guess {
                            existing.combine(e);
                        } else {
                            best_guess = ::pegcel::std::option::Option::Some(e);
                        }
                    }
                }
            }
        }
        {
            let fork = input.fork();
            match fork.parse::<ItemTrait>() {
                ::pegcel::std::result::Result::Ok(v) => {
                    input.advance_to(&fork);
                    return ::pegcel::std::result::Result::Ok(Item::Trait(v));
                }
                ::pegcel::std::result::Result::Err(e) => {
                    let this_guess_cursor = fork.cursor();
                    if this_guess_cursor > best_guess_cursor {
                        best_guess = ::pegcel::std::option::Option::Some(e);
                        best_guess_variant = "Trait";
                        best_guess_cursor = this_guess_cursor;
                    } else if this_guess_cursor == best_guess_cursor {
                        if let ::pegcel::std::option::Option::Some(existing)
                            = &mut best_guess {
                            existing.combine(e);
                        } else {
                            best_guess = ::pegcel::std::option::Option::Some(e);
                        }
                    }
                }
            }
        }
        {
            let fork = input.fork();
            match fork.parse::<ItemTraitAlias>() {
                ::pegcel::std::result::Result::Ok(v) => {
                    input.advance_to(&fork);
                    return ::pegcel::std::result::Result::Ok(Item::TraitAlias(v));
                }
                ::pegcel::std::result::Result::Err(e) => {
                    let this_guess_cursor = fork.cursor();
                    if this_guess_cursor > best_guess_cursor {
                        best_guess = ::pegcel::std::option::Option::Some(e);
                        best_guess_variant = "TraitAlias";
                        best_guess_cursor = this_guess_cursor;
                    } else if this_guess_cursor == best_guess_cursor {
                        if let ::pegcel::std::option::Option::Some(existing)
                            = &mut best_guess {
                            existing.combine(e);
                        } else {
                            best_guess = ::pegcel::std::option::Option::Some(e);
                        }
                    }
                }
            }
        }
        {
            let fork = input.fork();
            match fork.parse::<ItemType>() {
                ::pegcel::std::result::Result::Ok(v) => {
                    input.advance_to(&fork);
                    return ::pegcel::std::result::Result::Ok(Item::Type(v));
                }
                ::pegcel::std::result::Result::Err(e) => {
                    let this_guess_cursor = fork.cursor();
                    if this_guess_cursor > best_guess_cursor {
                        best_guess = ::pegcel::std::option::Option::Some(e);
                        best_guess_variant = "Type";
                        best_guess_cursor = this_guess_cursor;
                    } else if this_guess_cursor == best_guess_cursor {
                        if let ::pegcel::std::option::Option::Some(existing)
                            = &mut best_guess {
                            existing.combine(e);
                        } else {
                            best_guess = ::pegcel::std::option::Option::Some(e);
                        }
                    }
                }
            }
        }
        {
            let fork = input.fork();
            match fork.parse::<ItemUnion>() {
                ::pegcel::std::result::Result::Ok(v) => {
                    input.advance_to(&fork);
                    return ::pegcel::std::result::Result::Ok(Item::Union(v));
                }
                ::pegcel::std::result::Result::Err(e) => {
                    let this_guess_cursor = fork.cursor();
                    if this_guess_cursor > best_guess_cursor {
                        best_guess = ::pegcel::std::option::Option::Some(e);
                        best_guess_variant = "Union";
                        best_guess_cursor = this_guess_cursor;
                    } else if this_guess_cursor == best_guess_cursor {
                        if let ::pegcel::std::option::Option::Some(existing)
                            = &mut best_guess {
                            existing.combine(e);
                        } else {
                            best_guess = ::pegcel::std::option::Option::Some(e);
                        }
                    }
                }
            }
        }
        {
            let fork = input.fork();
            match fork.parse::<ItemUse>() {
                ::pegcel::std::result::Result::Ok(v) => {
                    input.advance_to(&fork);
                    return ::pegcel::std::result::Result::Ok(Item::Use(v));
                }
                ::pegcel::std::result::Result::Err(e) => {
                    let this_guess_cursor = fork.cursor();
                    if this_guess_cursor > best_guess_cursor {
                        best_guess = ::pegcel::std::option::Option::Some(e);
                        best_guess_variant = "Use";
                        best_guess_cursor = this_guess_cursor;
                    } else if this_guess_cursor == best_guess_cursor {
                        if let ::pegcel::std::option::Option::Some(existing)
                            = &mut best_guess {
                            existing.combine(e);
                        } else {
                            best_guess = ::pegcel::std::option::Option::Some(e);
                        }
                    }
                }
            }
        }
        ::pegcel::std::result::Result::Err(
            input
                .error(
                    ::core::fmt::Arguments::new_v1(
                        &[
                            "expected ",
                            " but failed to parse any variant; best attempt was ",
                            " with ",
                        ],
                        &[
                            ::core::fmt::ArgumentV1::new_display(&"Item"),
                            ::core::fmt::ArgumentV1::new_display(&best_guess_variant),
                            ::core::fmt::ArgumentV1::new_display(&best_guess.unwrap()),
                        ],
                    ),
                ),
        )
    }
}
impl ::pegcel::quote::ToTokens for Item {
    fn to_tokens(&self, tokens: &mut ::pegcel::proc_macro2::TokenStream) {
        match self {
            Item::Const(v) => (&v).to_tokens(tokens),
            Item::Enum(v) => (&v).to_tokens(tokens),
            Item::ExternCrate(v) => (&v).to_tokens(tokens),
            Item::Fn(v) => (&v).to_tokens(tokens),
            Item::ForeignMod(v) => (&v).to_tokens(tokens),
            Item::Impl(v) => (&v).to_tokens(tokens),
            Item::Macro(v) => (&v).to_tokens(tokens),
            Item::Macro2(v) => (&v).to_tokens(tokens),
            Item::Mod(v) => (&v).to_tokens(tokens),
            Item::Static(v) => (&v).to_tokens(tokens),
            Item::Struct(v) => (&v).to_tokens(tokens),
            Item::Trait(v) => (&v).to_tokens(tokens),
            Item::TraitAlias(v) => (&v).to_tokens(tokens),
            Item::Type(v) => (&v).to_tokens(tokens),
            Item::Union(v) => (&v).to_tokens(tokens),
            Item::Use(v) => (&v).to_tokens(tokens),
        }
    }
}
fn main() {}
