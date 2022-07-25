mod syntax {
    pegcel::define_syntax! {
        pub macro_rules! Token;
        mod self = crate::syntax;

        mod kw {
            yeet;
        }

        mod punct {
            Elvis(?:);
        }
    }
}

fn main() {
    // macros don't show up in the expansion, so here's an example:
    let _ = <syntax::Token![yeet]>::default();
    let _ = <syntax::Token![?:]>::default();
    // and syn tokens are still available:
    let _ = <syntax::Token![do]>::default();
    let _ = <syntax::Token![::]>::default();
}
