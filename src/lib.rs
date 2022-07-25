#[doc(hidden)]
#[cfg(feature = "printing")]
pub use quote;
#[doc(hidden)]
pub use {paste::paste, proc_macro2, syn};

#[macro_export]
macro_rules! define_syntax {
    {
        $(#[$struct_attr:meta])*
        $struct_vis:vis struct $Name:ident {
            $(
                $(#[$field_attr:meta])*
                $field_vis:vis $field:ident: $ty:ty
                $(as $parse_fn:expr)?
            ),* $(,)?
        }
    } => {
        $(#[$struct_attr])*
        $struct_vis struct $Name {
            $(
                $(#[$field_attr])*
                $field_vis $field: $ty
            ),*
        }

        $crate::__clone_impls! {
            #[automatically_derived]
            impl ::std::clone::Clone for $Name {
                fn clone(&self) -> Self {
                    Self {
                        $(
                            $field: self.$field.clone(),
                        )*
                    }
                }
            }
        }

        $crate::__extra_traits! {
            #[automatically_derived]
            impl ::std::fmt::Debug for $Name {
                fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                    f.debug_struct(::std::stringify!($Name))
                        $(
                            .field(::std::stringify!($field), &self.$field)
                        )*
                        .finish()
                }
            }

            #[automatically_derived]
            impl ::std::hash::Hash for $Name {
                fn hash<H: ::std::hash::Hasher>(&self, state: &mut H) {
                    $(
                        self.$field.hash(state);
                    )*
                }
            }

            #[automatically_derived]
            impl ::std::cmp::PartialEq for $Name {
                fn eq(&self, other: &Self) -> bool {
                    $(
                        self.$field == other.$field
                    )&&*
                }
            }

            #[automatically_derived]
            impl ::std::cmp::Eq for $Name {
            }
        }

        #[automatically_derived]
        impl $crate::syn::parse::Parse for $Name {
            fn parse(input: $crate::syn::parse::ParseStream) -> $crate::syn::parse::Result<Self> {
                ::std::result::Result::Ok(Self {
                    $(
                        $field: $crate::__switch! {
                            if { $($parse_fn)? }
                            do { input.call($($parse_fn)?)? }
                            else { input.parse()? }
                        },
                    )*
                })
            }
        }

        $crate::__printing! {
            impl $crate::quote::ToTokens for $Name {
                fn to_tokens(&self, tokens: &mut $crate::proc_macro2::TokenStream) {
                    use $crate::__IterableToTokens;
                    $(
                        self.$field.to_tokens(tokens);
                    )*
                }
            }
        }
    };

    {
        $(#[$enum_attr:meta])*
        $enum_vis:vis enum $Name:ident {
            $($Variant:ident $(($NameVariant:ty))?),+ $(,)?
        }
    } => {
        $crate::paste! {
            $(#[$enum_attr])*
            $enum_vis enum $Name {
                $($Variant($crate::__switch!{
                    if { $($NameVariant)? }
                    do { $($NameVariant)? }
                    else { [<$Name $Variant>] }
                })),*
            }

            $crate::__clone_impls! {
                #[automatically_derived]
                impl ::std::clone::Clone for $Name {
                    fn clone(&self) -> Self {
                        match self {
                            $(
                                $Name::$Variant(v) => $Name::$Variant(v.clone()),
                            )*
                        }
                    }
                }
            }

            $crate::__extra_traits! {
                #[automatically_derived]
                impl ::std::fmt::Debug for $Name {
                    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                        match self {
                            $(
                                $Name::$Variant(v) =>
                                    f.debug_tuple(::std::stringify!($Variant))
                                        .field(v)
                                        .finish(),
                            )*
                        }
                    }
                }

                #[automatically_derived]
                impl ::std::hash::Hash for $Name {
                    fn hash<H: ::std::hash::Hasher>(&self, state: &mut H) {
                        match self {
                            $(
                                $Name::$Variant(v) => {
                                    ::std::mem::discriminant(self).hash(state);
                                    v.hash(state);
                                },
                            )*
                        }
                    }
                }

                #[automatically_derived]
                impl ::std::cmp::PartialEq for $Name {
                    fn eq(&self, other: &Self) -> bool {
                        match (self, other) {
                            $(
                                ($Name::$Variant(lhs), $Name::$Variant(rhs)) => lhs == rhs,
                            )*
                            _ => false,
                        }
                    }
                }

                #[automatically_derived]
                impl ::std::cmp::Eq for $Name {}
            }

            $(
                #[automatically_derived]
                impl From<$crate::__switch! {
                    if { $($NameVariant)? }
                    do { $($NameVariant)? }
                    else { [<$Name $Variant>] }
                }> for $Name {
                    fn from(
                        v: $crate::__switch! {
                            if { $($NameVariant)? }
                            do { $($NameVariant)? }
                            else { [<$Name $Variant>] }
                        },
                    ) -> Self {
                        $Name::$Variant(v)
                    }
                }
            )*

            #[automatically_derived]
            impl $crate::syn::parse::Parse for $Name {
                fn parse(input: $crate::syn::parse::ParseStream) -> $crate::syn::parse::Result<Self> {
                    use $crate::syn::parse::discouraged::Speculative;
                    let mut best_guess = ::std::option::Option::None;
                    let mut best_guess_variant = "";
                    let mut best_guess_remaining = ::std::primitive::usize::MAX;
                    $({
                        let fork = input.fork();
                        match fork.parse::<$crate::__switch! {
                            if { $($NameVariant)? }
                            do { $($NameVariant)? }
                            else { [<$Name $Variant>] }
                        }>() {
                            ::std::result::Result::Ok(v) => {
                                input.advance_to(&fork);
                                return ::std::result::Result::Ok($Name::$Variant(v));
                            }
                            ::std::result::Result::Err(e) => {
                                // TODO: ask syn for a Cursor::len to optimize this
                                let this_guess_remaining =
                                    fork.cursor().token_stream().into_iter().count();
                                if this_guess_remaining < best_guess_remaining {
                                    best_guess = ::std::option::Option::Some(e);
                                    best_guess_variant = ::std::stringify!($Variant);
                                    best_guess_remaining = this_guess_remaining;
                                }
                            },
                        }
                    })*
                    ::std::result::Result::Err(input.error(format_args!(
                        "expected {} but failed to parse any variant; best attempt was {} with {}",
                        ::std::stringify!($Name),
                        best_guess_variant,
                        best_guess.unwrap(),
                    )))
                }
            }

            $crate::__printing! {
                impl $crate::quote::ToTokens for $Name {
                    fn to_tokens(&self, tokens: &mut $crate::proc_macro2::TokenStream) {
                        match self {
                            $(
                                $Name::$Variant(v) => v.to_tokens(tokens),
                            )*
                        }
                    }
                }
            }
        }
    };

    {
        $vis:vis macro_rules! $Macro:ident;
        mod self = $krate:tt $(::$self_path:ident)*;
        $(
            mod kw {
                $($keyword:ident;)*
            }
        )?
        $(
            mod punct {
                $($punct:ident($($tt:tt)+);)*
            }
        )?
    } => {
        $crate::define_syntax! {
            (@$) $vis macro_rules! $Macro;
            mod self = $krate $(::$self_path)*;
            $(
                mod kw {
                    $($keyword;)*
                }
            )?
            $(
                mod punct {
                    $($punct($($tt)+);)*
                }
            )?
        }
    };
    {
        (@$d:tt) $vis:vis macro_rules! $Macro:ident;
        mod self = $krate:tt $(::$self_path:ident)*;
        $(
            mod kw {
                $($keyword:ident;)*
            }
        )?
        $(
            mod punct {
                $($punct:ident($($tt:tt)+);)*
            }
        )?
    } => {
        $($vis mod kw {
            $($crate::syn::custom_keyword!($keyword);)*
        })?
        $($vis mod punct {
            $($crate::syn::custom_punctuation!($punct, $($tt)+);)*
        })?

        $crate::paste! {
            // this is a macro generated macro generated macro; my head hurts
            macro_rules! [< __pegcel_generate_token_macro__ $Macro >] {
                (($d d:tt) macro = $d macro:path) => {
                    // NB: avoid using $$crate: rust-lang/rust#99035
                    #[macro_export]
                    macro_rules! [< __pegcel_generated_token_macro__ $Macro >] {
                        { __pegcel_generated_token_macro__ kw $d d keyword:ident } => { $d $krate $(:: $self_path)* :: kw :: $d d keyword };
                        { __pegcel_generated_token_macro__ punct $d d punct:ident } => { $d $krate $(:: $self_path)* :: punct :: $d d punct };

                        $($( [$keyword] => { $d macro ! { __pegcel_generated_token_macro__ kw $keyword } }; )*)?
                        $($( [$($tt)+] => { $d macro ! { __pegcel_generated_token_macro__ punct $punct } }; )*)?
                        [$d d ( $d d fallback:tt )*] => { $crate::syn::Token! { $d d ( $d d fallback )* } };
                    }
                }
            }

            [< __pegcel_generate_token_macro__ $Macro >]!(($) macro = $krate $(:: $self_path)* :: $Macro);
            $vis use [< __pegcel_generated_token_macro__ $Macro >] as $Macro;
        }
    };
}

#[doc(hidden)]
#[cfg(feature = "printing")]
/// Used to "specialize" `$field.to_tokens()` for iterables quoted with `#()*`.
pub trait __IterableToTokens {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream);
}

#[cfg(feature = "printing")]
impl<T> __IterableToTokens for T
where
    for<'a> &'a T: IntoIterator,
    for<'a> <&'a T as IntoIterator>::Item: quote::ToTokens,
{
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        use quote::TokenStreamExt;
        tokens.append_all(self)
    }
}

#[doc(hidden)]
#[macro_export]
macro_rules! __switch {
    {
        if {
        }
        do {
            $($do:tt)*
        }
        $(else {
            $($else:tt)*
        })?
    } => {
        $($($else)*)?
    };
    {
        if {
            $($cond:tt)+
        }
        do {
            $($do:tt)*
        }
        $(else {
            $($else:tt)*
        })?
    } => {
        $($do)*
    };
}

#[doc(hidden)]
#[macro_export]
#[cfg(feature = "clone-impls")]
macro_rules! __clone_impls {
    ($($tt:tt)*) => ($($tt)*);
}

#[doc(hidden)]
#[macro_export]
#[cfg(not(feature = "clone-impls"))]
macro_rules! __clone_impls {
    ($($tt:tt)*) => {};
}

#[doc(hidden)]
#[macro_export]
#[cfg(feature = "extra-traits")]
macro_rules! __extra_traits {
    ($($tt:tt)*) => ($($tt)*);
}

#[doc(hidden)]
#[macro_export]
#[cfg(not(feature = "extra-traits"))]
macro_rules! __extra_traits {
    ($($tt:tt)*) => {};
}

#[doc(hidden)]
#[macro_export]
#[cfg(feature = "printing")]
macro_rules! __printing {
    ($($tt:tt)*) => ($($tt)*);
}

#[doc(hidden)]
#[macro_export]
#[cfg(not(feature = "printing"))]
macro_rules! __printing {
    ($($tt:tt)*) => {};
}
