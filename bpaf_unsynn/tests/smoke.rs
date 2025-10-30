//! Smoke test - verifies the derive macro compiles

#[allow(dead_code)]
#[derive(bpaf_unsynn::Bpaf)]
struct Options {
    verbose: bool,
}

#[test]
fn smoke_test_compiles() {
    // If this compiles, the test passes
}
