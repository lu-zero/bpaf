//! Test that we can parse different field types

#[allow(dead_code)]
#[derive(bpaf_unsynn::Bpaf)]
struct AllTypes {
    // Bool - should be detected as Shape::Bool
    verbose: bool,

    // Direct types
    name: String,
    count: i32,

    // Option
    optional: Option<String>,

    // Vec
    items: Vec<String>,
}

#[test]
fn all_types_compile() {
    // If this compiles, type analysis worked
}
