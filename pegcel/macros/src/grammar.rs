extern crate proc_macro2;
extern crate quote;
extern crate syn;

use crate::manual_grammar::UnnamedItem;

pub mod kw {
    ::syn::custom_keyword!(syn);
}

pub mod punct {
    ::syn::custom_punctuation!( _3c0cae222c3740729cb899e4e5651a74 , % % );
}

#[doc(hidden)]
macro_rules! _9a4e10db71554eda87aabfc42ead4c0d {
    ( syn ) => { self::kw::syn };
    ( % % ) => { self::punct::_3c0cae222c3740729cb899e4e5651a74 };
    ( $( $tt:tt )* ) => { ::syn::Token! [ $( $tt )* ] };
}
pub(crate) use _9a4e10db71554eda87aabfc42ead4c0d as Token;

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct Grammar {
    pub kind: GrammarUse,
    pub uses: Vec<(Use)>,
    pub items: Vec<NamedItem>,
}

impl ::syn::parse::Parse for Grammar {
    fn parse(
        _3470c1ed32e7431c8e8a9be1e8556106: ::syn::parse::ParseStream,
    ) -> ::syn::parse::Result<Self> {
        let mut kind;
        let mut uses;
        let mut items;
        kind = _3470c1ed32e7431c8e8a9be1e8556106.parse()?;
        uses = vec![];
        while _3470c1ed32e7431c8e8a9be1e8556106.peek(Token![use]) {
            let _ec4cfc98d98a47cba13c5fe60c8c9fad;
            _ec4cfc98d98a47cba13c5fe60c8c9fad = {
                assert!(_3470c1ed32e7431c8e8a9be1e8556106.peek(Token![use]));
                let _c64ff9ee181c4c79ba04605f7715a896;
                _c64ff9ee181c4c79ba04605f7715a896 = _3470c1ed32e7431c8e8a9be1e8556106.parse()?;
                (_c64ff9ee181c4c79ba04605f7715a896)
            };
            uses.push(_ec4cfc98d98a47cba13c5fe60c8c9fad);
        }
        items = vec![];
        while !_3470c1ed32e7431c8e8a9be1e8556106.is_empty() {
            let _5b1b355a811a4caf84a7035f93e3cffd;
            _5b1b355a811a4caf84a7035f93e3cffd = _3470c1ed32e7431c8e8a9be1e8556106.parse()?;
            items.push(_5b1b355a811a4caf84a7035f93e3cffd);
        }
        Ok(Grammar { kind, uses, items })
    }
}

impl ::quote::ToTokens for Grammar {
    fn to_tokens(&self, _3470c1ed32e7431c8e8a9be1e8556106: &mut ::proc_macro2::TokenStream) {
        use ::quote::{ToTokens, TokenStreamExt};
        (&self.kind).to_tokens(_3470c1ed32e7431c8e8a9be1e8556106);
        for _4426e0e1e24341b4868da89581296926 in (&self.uses) {
            let (_5a239cde035046bd8ae8c39532fcaae4) = _4426e0e1e24341b4868da89581296926;
            _5a239cde035046bd8ae8c39532fcaae4.to_tokens(_3470c1ed32e7431c8e8a9be1e8556106);
        }
        for _ed5285a269a1491094ef55fc67ddf97a in (&self.items) {
            _ed5285a269a1491094ef55fc67ddf97a.to_tokens(_3470c1ed32e7431c8e8a9be1e8556106);
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub enum GrammarUse {
    Syn(Token![use], Token![syn]),
}

impl ::syn::parse::Parse for GrammarUse {
    fn parse(
        _e6dab0493b904d0ca4495fb430a1b953: ::syn::parse::ParseStream,
    ) -> ::syn::parse::Result<Self> {
        if true {
            let mut _4d87bfb20907461aa65cf19e6c8a3d77;
            let mut _98f615577c1a48589a008daa7fc1afd2;
            _4d87bfb20907461aa65cf19e6c8a3d77 = _e6dab0493b904d0ca4495fb430a1b953.parse()?;
            _98f615577c1a48589a008daa7fc1afd2 = _e6dab0493b904d0ca4495fb430a1b953.parse()?;
            Ok(GrammarUse::Syn(
                _4d87bfb20907461aa65cf19e6c8a3d77,
                _98f615577c1a48589a008daa7fc1afd2,
            ))
        } else {
            unreachable!("pegcel ordered choice fallthrough case reached")
        }
    }
}

impl ::quote::ToTokens for GrammarUse {
    fn to_tokens(&self, _e6dab0493b904d0ca4495fb430a1b953: &mut ::proc_macro2::TokenStream) {
        use ::quote::{ToTokens, TokenStreamExt};
        match self {
            GrammarUse::Syn(
                _4d87bfb20907461aa65cf19e6c8a3d77,
                _98f615577c1a48589a008daa7fc1afd2,
            ) => {
                _4d87bfb20907461aa65cf19e6c8a3d77.to_tokens(_e6dab0493b904d0ca4495fb430a1b953);
                _98f615577c1a48589a008daa7fc1afd2.to_tokens(_e6dab0493b904d0ca4495fb430a1b953);
            }
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct Use {
    pub r#use: Token![use],
    pub anchor: Option<Token![ :: ]>,
    pub tree: syn::UseTree,
}

impl ::syn::parse::Parse for Use {
    fn parse(
        _72f18f2f8e3943c4bd8003cccb462ddd: ::syn::parse::ParseStream,
    ) -> ::syn::parse::Result<Self> {
        let mut r#use;
        let mut anchor;
        let mut tree;
        r#use = _72f18f2f8e3943c4bd8003cccb462ddd.parse()?;
        anchor = _72f18f2f8e3943c4bd8003cccb462ddd.parse()?;
        tree = _72f18f2f8e3943c4bd8003cccb462ddd.parse()?;
        Ok(Use {
            r#use,
            anchor,
            tree,
        })
    }
}

impl ::quote::ToTokens for Use {
    fn to_tokens(&self, _72f18f2f8e3943c4bd8003cccb462ddd: &mut ::proc_macro2::TokenStream) {
        use ::quote::{ToTokens, TokenStreamExt};
        (&self.r#use).to_tokens(_72f18f2f8e3943c4bd8003cccb462ddd);
        (&self.anchor).to_tokens(_72f18f2f8e3943c4bd8003cccb462ddd);
        (&self.tree).to_tokens(_72f18f2f8e3943c4bd8003cccb462ddd);
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub enum Item {
    Named(NamedItem),
    Unnamed(UnnamedItem),
}

impl ::syn::parse::Parse for Item {
    fn parse(
        _f9d63e921bfb46f695d04074f63badb8: ::syn::parse::ParseStream,
    ) -> ::syn::parse::Result<Self> {
        if (_f9d63e921bfb46f695d04074f63badb8.peek(syn::Ident)
            && !_f9d63e921bfb46f695d04074f63badb8.peek2(Token![ :: ])
            && _f9d63e921bfb46f695d04074f63badb8.peek2(Token![ : ])
            && true)
        {
            let mut _c15996e620b94538a8f580ee24ab6920;
            _c15996e620b94538a8f580ee24ab6920 = _f9d63e921bfb46f695d04074f63badb8.parse()?;
            Ok(Item::Named(_c15996e620b94538a8f580ee24ab6920))
        } else if true {
            let mut _99113c2a9793432ca8432bf96a4c1fc3;
            _99113c2a9793432ca8432bf96a4c1fc3 = _f9d63e921bfb46f695d04074f63badb8.parse()?;
            Ok(Item::Unnamed(_99113c2a9793432ca8432bf96a4c1fc3))
        } else {
            unreachable!("pegcel ordered choice fallthrough case reached")
        }
    }
}

impl ::quote::ToTokens for Item {
    fn to_tokens(&self, _f9d63e921bfb46f695d04074f63badb8: &mut ::proc_macro2::TokenStream) {
        use ::quote::{ToTokens, TokenStreamExt};
        match self {
            Item::Named(_c15996e620b94538a8f580ee24ab6920) => {
                _c15996e620b94538a8f580ee24ab6920.to_tokens(_f9d63e921bfb46f695d04074f63badb8);
            }
            Item::Unnamed(_99113c2a9793432ca8432bf96a4c1fc3) => {
                (_99113c2a9793432ca8432bf96a4c1fc3).to_tokens(_f9d63e921bfb46f695d04074f63badb8);
            }
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct NamedItem {
    pub name: syn::Ident,
    pub colon: Token![ : ],
    pub item: UnnamedItem,
}

impl ::syn::parse::Parse for NamedItem {
    fn parse(
        _221a00e820cb434cb29a388b7debd900: ::syn::parse::ParseStream,
    ) -> ::syn::parse::Result<Self> {
        let mut name;
        let mut colon;
        let mut item;
        name = _221a00e820cb434cb29a388b7debd900.parse()?;
        colon = _221a00e820cb434cb29a388b7debd900.parse()?;
        item = _221a00e820cb434cb29a388b7debd900.parse()?;
        Ok(NamedItem { name, colon, item })
    }
}

impl ::quote::ToTokens for NamedItem {
    fn to_tokens(&self, _221a00e820cb434cb29a388b7debd900: &mut ::proc_macro2::TokenStream) {
        use ::quote::{ToTokens, TokenStreamExt};
        (&self.name).to_tokens(_221a00e820cb434cb29a388b7debd900);
        (&self.colon).to_tokens(_221a00e820cb434cb29a388b7debd900);
        (&self.item).to_tokens(_221a00e820cb434cb29a388b7debd900);
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub enum Items {
    OrderedChoice(
        Token![ / ],
        ::syn::punctuated::Punctuated<NamedItem, Token![ / ]>,
    ),
    Sequence(Vec<Item>),
}

impl ::syn::parse::Parse for Items {
    fn parse(
        _63e19ad067044144ad9ef3b1e19f8808: ::syn::parse::ParseStream,
    ) -> ::syn::parse::Result<Self> {
        if _63e19ad067044144ad9ef3b1e19f8808.peek(Token![ / ]) {
            let mut _3f95a03242fc486e9236ee9d54d55b76;
            let mut _5e3dd63cde1247b6bcb1535b02614fbb;
            _3f95a03242fc486e9236ee9d54d55b76 = _63e19ad067044144ad9ef3b1e19f8808.parse()?;
            _5e3dd63cde1247b6bcb1535b02614fbb = _63e19ad067044144ad9ef3b1e19f8808
                .call(::syn::punctuated::Punctuated::parse_separated_nonempty)?;
            Ok(Items::OrderedChoice(
                _3f95a03242fc486e9236ee9d54d55b76,
                _5e3dd63cde1247b6bcb1535b02614fbb,
            ))
        } else if true {
            let mut _b1cee978ee844c7289c36973e2911398;
            _b1cee978ee844c7289c36973e2911398 = vec![];
            while !_63e19ad067044144ad9ef3b1e19f8808.is_empty() {
                let _8f4fe4ff9efc43048edafaf0d6f361b4;
                _8f4fe4ff9efc43048edafaf0d6f361b4 = _63e19ad067044144ad9ef3b1e19f8808.parse()?;
                _b1cee978ee844c7289c36973e2911398.push(_8f4fe4ff9efc43048edafaf0d6f361b4);
            }
            Ok(Items::Sequence(_b1cee978ee844c7289c36973e2911398))
        } else {
            unreachable!("pegcel ordered choice fallthrough case reached")
        }
    }
}

impl ::quote::ToTokens for Items {
    fn to_tokens(&self, _63e19ad067044144ad9ef3b1e19f8808: &mut ::proc_macro2::TokenStream) {
        use ::quote::{ToTokens, TokenStreamExt};
        match self {
            Items::OrderedChoice(
                _3f95a03242fc486e9236ee9d54d55b76,
                _5e3dd63cde1247b6bcb1535b02614fbb,
            ) => {
                _3f95a03242fc486e9236ee9d54d55b76.to_tokens(_63e19ad067044144ad9ef3b1e19f8808);
                for _c618f71708dd495395db3bceb9ea7702 in _5e3dd63cde1247b6bcb1535b02614fbb.pairs() {
                    let _7d3138f4dfe84446933568743ac62fca =
                        _c618f71708dd495395db3bceb9ea7702.value();
                    _7d3138f4dfe84446933568743ac62fca.to_tokens(_63e19ad067044144ad9ef3b1e19f8808);
                    if let Some(_a607b2f7d0a242d6ab960727e9207562) =
                        _c618f71708dd495395db3bceb9ea7702.punct()
                    {
                        _a607b2f7d0a242d6ab960727e9207562
                            .to_tokens(_63e19ad067044144ad9ef3b1e19f8808);
                    }
                }
            }
            Items::Sequence(_b1cee978ee844c7289c36973e2911398) => {
                for _4949aeed3100488fbd9e2d812feb4fd6 in (_b1cee978ee844c7289c36973e2911398) {
                    _4949aeed3100488fbd9e2d812feb4fd6.to_tokens(_63e19ad067044144ad9ef3b1e19f8808);
                }
            }
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub enum Predicate {
    Positive(Token![ & ]),
    Negative(Token![!]),
}

impl ::syn::parse::Parse for Predicate {
    fn parse(
        _1e94b2203d3649c48792dbbf401007d0: ::syn::parse::ParseStream,
    ) -> ::syn::parse::Result<Self> {
        if _1e94b2203d3649c48792dbbf401007d0.peek(Token![ & ]) {
            let mut _c338462f9fbf4d67b49a9d081ea7f411;
            _c338462f9fbf4d67b49a9d081ea7f411 = _1e94b2203d3649c48792dbbf401007d0.parse()?;
            Ok(Predicate::Positive(_c338462f9fbf4d67b49a9d081ea7f411))
        } else if true {
            let mut _0cf99098eee04b8c832e38ab8cabf8bf;
            _0cf99098eee04b8c832e38ab8cabf8bf = _1e94b2203d3649c48792dbbf401007d0.parse()?;
            Ok(Predicate::Negative(_0cf99098eee04b8c832e38ab8cabf8bf))
        } else {
            unreachable!("pegcel ordered choice fallthrough case reached")
        }
    }
}

impl ::quote::ToTokens for Predicate {
    fn to_tokens(&self, _1e94b2203d3649c48792dbbf401007d0: &mut ::proc_macro2::TokenStream) {
        use ::quote::{ToTokens, TokenStreamExt};
        match self {
            Predicate::Positive(_c338462f9fbf4d67b49a9d081ea7f411) => {
                (_c338462f9fbf4d67b49a9d081ea7f411).to_tokens(_1e94b2203d3649c48792dbbf401007d0);
            }
            Predicate::Negative(_0cf99098eee04b8c832e38ab8cabf8bf) => {
                (_0cf99098eee04b8c832e38ab8cabf8bf).to_tokens(_1e94b2203d3649c48792dbbf401007d0);
            }
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub enum Repetition {
    ZeroPlus(Token![ * ], Option<(Interspersion)>),
    OnePlus(Token![ + ], Option<(Interspersion)>),
    ZeroOne(Token![ ? ]),
}

impl ::syn::parse::Parse for Repetition {
    fn parse(
        _929bbf92b3c049a29b1c0993a394c769: ::syn::parse::ParseStream,
    ) -> ::syn::parse::Result<Self> {
        if _929bbf92b3c049a29b1c0993a394c769.peek(Token![ * ]) {
            let mut _bd781c83f4054ed3b854f1ef249bc6fd;
            let mut _b52f5fb37343456090f4fdaa88a1d618;
            _bd781c83f4054ed3b854f1ef249bc6fd = _929bbf92b3c049a29b1c0993a394c769.parse()?;
            if _929bbf92b3c049a29b1c0993a394c769.peek(Token![ % ]) {
                let _c125b7df99f34a52a4e459b599f329de;
                _c125b7df99f34a52a4e459b599f329de = {
                    assert!(_929bbf92b3c049a29b1c0993a394c769.peek(Token![ % ]));
                    let _b67eae6c355f4cefb078fc091769f2f4;
                    _b67eae6c355f4cefb078fc091769f2f4 =
                        _929bbf92b3c049a29b1c0993a394c769.parse()?;
                    (_b67eae6c355f4cefb078fc091769f2f4)
                };
                _b52f5fb37343456090f4fdaa88a1d618 = Some(_c125b7df99f34a52a4e459b599f329de);
            } else {
                _b52f5fb37343456090f4fdaa88a1d618 = None;
            }
            Ok(Repetition::ZeroPlus(
                _bd781c83f4054ed3b854f1ef249bc6fd,
                _b52f5fb37343456090f4fdaa88a1d618,
            ))
        } else if _929bbf92b3c049a29b1c0993a394c769.peek(Token![ + ]) {
            let mut _b280794005064275abdf3a1d399c3eb1;
            let mut _0a3ab10db9b94700bd3301fe8d330c8e;
            _b280794005064275abdf3a1d399c3eb1 = _929bbf92b3c049a29b1c0993a394c769.parse()?;
            if _929bbf92b3c049a29b1c0993a394c769.peek(Token![ % ]) {
                let _facfcf12424c4bf99b16d72851e9f4c2;
                _facfcf12424c4bf99b16d72851e9f4c2 = {
                    assert!(_929bbf92b3c049a29b1c0993a394c769.peek(Token![ % ]));
                    let _2a720a7aedad407cabecf76fc0116fa2;
                    _2a720a7aedad407cabecf76fc0116fa2 =
                        _929bbf92b3c049a29b1c0993a394c769.parse()?;
                    (_2a720a7aedad407cabecf76fc0116fa2)
                };
                _0a3ab10db9b94700bd3301fe8d330c8e = Some(_facfcf12424c4bf99b16d72851e9f4c2);
            } else {
                _0a3ab10db9b94700bd3301fe8d330c8e = None;
            }
            Ok(Repetition::OnePlus(
                _b280794005064275abdf3a1d399c3eb1,
                _0a3ab10db9b94700bd3301fe8d330c8e,
            ))
        } else if true {
            let mut _ae5b50a8fa0f4e0789f1b6d35e558545;
            _ae5b50a8fa0f4e0789f1b6d35e558545 = _929bbf92b3c049a29b1c0993a394c769.parse()?;
            Ok(Repetition::ZeroOne(_ae5b50a8fa0f4e0789f1b6d35e558545))
        } else {
            unreachable!("pegcel ordered choice fallthrough case reached")
        }
    }
}

impl ::quote::ToTokens for Repetition {
    fn to_tokens(&self, _929bbf92b3c049a29b1c0993a394c769: &mut ::proc_macro2::TokenStream) {
        use ::quote::{ToTokens, TokenStreamExt};
        match self {
            Repetition::ZeroPlus(
                _bd781c83f4054ed3b854f1ef249bc6fd,
                _b52f5fb37343456090f4fdaa88a1d618,
            ) => {
                _bd781c83f4054ed3b854f1ef249bc6fd.to_tokens(_929bbf92b3c049a29b1c0993a394c769);
                _b52f5fb37343456090f4fdaa88a1d618.to_tokens(_929bbf92b3c049a29b1c0993a394c769);
            }
            Repetition::OnePlus(
                _b280794005064275abdf3a1d399c3eb1,
                _0a3ab10db9b94700bd3301fe8d330c8e,
            ) => {
                _b280794005064275abdf3a1d399c3eb1.to_tokens(_929bbf92b3c049a29b1c0993a394c769);
                _0a3ab10db9b94700bd3301fe8d330c8e.to_tokens(_929bbf92b3c049a29b1c0993a394c769);
            }
            Repetition::ZeroOne(_ae5b50a8fa0f4e0789f1b6d35e558545) => {
                (_ae5b50a8fa0f4e0789f1b6d35e558545).to_tokens(_929bbf92b3c049a29b1c0993a394c769);
            }
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub enum Interspersion {
    Terminated(Token![ % % ], Symbol),
    Separated(Token![ % ], Symbol),
}

impl ::syn::parse::Parse for Interspersion {
    fn parse(
        _3a527dd82f634d6197c59d68a193bc3f: ::syn::parse::ParseStream,
    ) -> ::syn::parse::Result<Self> {
        if _3a527dd82f634d6197c59d68a193bc3f.peek(Token![ % % ]) {
            let mut _70b38bd2ee234d7e830c1eaae095da5a;
            let mut _609af660ec704794acb03e5fe0f3c4ab;
            _70b38bd2ee234d7e830c1eaae095da5a = _3a527dd82f634d6197c59d68a193bc3f.parse()?;
            _609af660ec704794acb03e5fe0f3c4ab = _3a527dd82f634d6197c59d68a193bc3f.parse()?;
            Ok(Interspersion::Terminated(
                _70b38bd2ee234d7e830c1eaae095da5a,
                _609af660ec704794acb03e5fe0f3c4ab,
            ))
        } else if true {
            let mut _defb76a416e54c79ad11b77b636100f2;
            let mut _d25cb6f0cc7e4a97a23dcbaa1dc23e8c;
            _defb76a416e54c79ad11b77b636100f2 = _3a527dd82f634d6197c59d68a193bc3f.parse()?;
            _d25cb6f0cc7e4a97a23dcbaa1dc23e8c = _3a527dd82f634d6197c59d68a193bc3f.parse()?;
            Ok(Interspersion::Separated(
                _defb76a416e54c79ad11b77b636100f2,
                _d25cb6f0cc7e4a97a23dcbaa1dc23e8c,
            ))
        } else {
            unreachable!("pegcel ordered choice fallthrough case reached")
        }
    }
}

impl ::quote::ToTokens for Interspersion {
    fn to_tokens(&self, _3a527dd82f634d6197c59d68a193bc3f: &mut ::proc_macro2::TokenStream) {
        use ::quote::{ToTokens, TokenStreamExt};
        match self {
            Interspersion::Terminated(
                _70b38bd2ee234d7e830c1eaae095da5a,
                _609af660ec704794acb03e5fe0f3c4ab,
            ) => {
                _70b38bd2ee234d7e830c1eaae095da5a.to_tokens(_3a527dd82f634d6197c59d68a193bc3f);
                _609af660ec704794acb03e5fe0f3c4ab.to_tokens(_3a527dd82f634d6197c59d68a193bc3f);
            }
            Interspersion::Separated(
                _defb76a416e54c79ad11b77b636100f2,
                _d25cb6f0cc7e4a97a23dcbaa1dc23e8c,
            ) => {
                _defb76a416e54c79ad11b77b636100f2.to_tokens(_3a527dd82f634d6197c59d68a193bc3f);
                _d25cb6f0cc7e4a97a23dcbaa1dc23e8c.to_tokens(_3a527dd82f634d6197c59d68a193bc3f);
            }
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub enum Symbol {
    Literal(syn::LitStr),
    Path(syn::Path),
}

impl ::syn::parse::Parse for Symbol {
    fn parse(
        _fc2998ee4ec54faead0060d6ae90e084: ::syn::parse::ParseStream,
    ) -> ::syn::parse::Result<Self> {
        if _fc2998ee4ec54faead0060d6ae90e084.peek(syn::LitStr) {
            let mut _801122d88b114a52830c4bf9cebfd9ca;
            _801122d88b114a52830c4bf9cebfd9ca = _fc2998ee4ec54faead0060d6ae90e084.parse()?;
            Ok(Symbol::Literal(_801122d88b114a52830c4bf9cebfd9ca))
        } else if true {
            let mut _b056d6e4cc94452ebc1807bc5bb60787;
            _b056d6e4cc94452ebc1807bc5bb60787 = _fc2998ee4ec54faead0060d6ae90e084.parse()?;
            Ok(Symbol::Path(_b056d6e4cc94452ebc1807bc5bb60787))
        } else {
            unreachable!("pegcel ordered choice fallthrough case reached")
        }
    }
}

impl ::quote::ToTokens for Symbol {
    fn to_tokens(&self, _fc2998ee4ec54faead0060d6ae90e084: &mut ::proc_macro2::TokenStream) {
        use ::quote::{ToTokens, TokenStreamExt};
        match self {
            Symbol::Literal(_801122d88b114a52830c4bf9cebfd9ca) => {
                (_801122d88b114a52830c4bf9cebfd9ca).to_tokens(_fc2998ee4ec54faead0060d6ae90e084);
            }
            Symbol::Path(_b056d6e4cc94452ebc1807bc5bb60787) => {
                (_b056d6e4cc94452ebc1807bc5bb60787).to_tokens(_fc2998ee4ec54faead0060d6ae90e084);
            }
        }
    }
}
