//! Compile-fail tests using trybuild
//!
//! These tests verify that the macro produces the correct error messages
//! when given invalid input.

#[test]
fn ui() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/ui/*.rs");
}
