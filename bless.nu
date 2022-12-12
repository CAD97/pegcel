with-env [MACROTEST "overwrite"] {
    cargo +nightly test --test examples -- expand
}
