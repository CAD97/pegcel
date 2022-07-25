#[test]
fn expand() {
    macrotest::expand_args(
        "examples/*.rs",
        &[
            "--features",
            "pegcel/clone-impls,pegcel/extra-traits,pegcel/printing",
        ],
    );
}
