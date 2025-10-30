//! Test that generated parsers actually work with bpaf

use bpaf::Parser;

#[derive(Debug, Clone)]
#[derive(bpaf_unsynn::Bpaf)]
struct Options {
    verbose: bool,
}

#[test]
fn bool_parser_works() {
    let parser = Options::parse().to_options();

    // Test parsing with --verbose flag
    let result = parser.run_inner(&["--verbose"]);
    assert!(result.is_ok());
    let opts = result.unwrap();
    assert!(opts.verbose);

    // Test parsing without flag
    let result = parser.run_inner(&[] as &[&str]);
    assert!(result.is_ok());
    let opts = result.unwrap();
    assert!(!opts.verbose);
}

#[derive(Debug, Clone)]
#[derive(bpaf_unsynn::Bpaf)]
struct WithString {
    name: String,
}

#[test]
fn string_parser_works() {
    let parser = WithString::parse().to_options();

    // Test parsing with --name argument
    let result = parser.run_inner(&["--name", "test"]);
    assert!(result.is_ok());
    let opts = result.unwrap();
    assert_eq!(opts.name, "test");
}

#[derive(Debug, Clone)]
#[derive(bpaf_unsynn::Bpaf)]
struct WithOption {
    optional: Option<String>,
}

#[test]
fn option_parser_works() {
    let parser = WithOption::parse().to_options();

    // Test parsing with --optional argument
    let result = parser.run_inner(&["--optional", "value"]);
    assert!(result.is_ok());
    let opts = result.unwrap();
    assert_eq!(opts.optional, Some("value".to_string()));

    // Test parsing without argument
    let result = parser.run_inner(&[] as &[&str]);
    assert!(result.is_ok());
    let opts = result.unwrap();
    assert_eq!(opts.optional, None);
}

#[derive(Debug, Clone)]
#[derive(bpaf_unsynn::Bpaf)]
struct WithVec {
    items: Vec<String>,
}

#[test]
fn vec_parser_works() {
    let parser = WithVec::parse().to_options();

    // Test parsing with multiple --items arguments
    let result = parser.run_inner(&["--items", "one", "--items", "two", "--items", "three"]);
    assert!(result.is_ok());
    let opts = result.unwrap();
    assert_eq!(opts.items, vec!["one", "two", "three"]);

    // Test parsing with no arguments
    let result = parser.run_inner(&[] as &[&str]);
    assert!(result.is_ok());
    let opts = result.unwrap();
    assert_eq!(opts.items, Vec::<String>::new());
}
