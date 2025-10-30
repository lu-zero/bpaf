//! Tests for advanced PostParse features: many, some, anywhere, adjacent, strict, non_strict

use bpaf::Parser;

// ============================================================================
// many - Parse multiple occurrences
// ============================================================================

#[derive(Debug, Clone, PartialEq)]
#[derive(bpaf_unsynn::Bpaf)]
struct WithMany {
    #[bpaf(long, argument("FILE"), many)]
    files: Vec<String>,
}

#[test]
fn many_multiple_values() {
    let parser = WithMany::parse().to_options();

    let result = parser.run_inner(&["--files", "a.txt", "--files", "b.txt", "--files", "c.txt"]);
    assert!(result.is_ok(), "Failed to parse: {:?}", result);
    let opts = result.unwrap();
    assert_eq!(opts.files, vec!["a.txt", "b.txt", "c.txt"]);
}

#[test]
fn many_single_value() {
    let parser = WithMany::parse().to_options();

    let result = parser.run_inner(&["--files", "single.txt"]);
    assert!(result.is_ok(), "Failed to parse: {:?}", result);
    let opts = result.unwrap();
    assert_eq!(opts.files, vec!["single.txt"]);
}

#[test]
fn many_no_values() {
    let parser = WithMany::parse().to_options();

    let result = parser.run_inner(&[] as &[&str]);
    // many should succeed with empty vec when no values provided
    assert!(result.is_ok(), "Should succeed with empty vec: {:?}", result);
    let opts = result.unwrap();
    assert_eq!(opts.files, Vec::<String>::new());
}

// ============================================================================
// some - Require at least one occurrence
// ============================================================================

#[derive(Debug, Clone, PartialEq)]
#[derive(bpaf_unsynn::Bpaf)]
struct WithSome {
    /// important argument
    #[bpaf(argument("ARG"), some("want at least one argument"), ignore_rustdoc)]
    argument: Vec<u32>,

    /// some switch
    #[bpaf(long("switch"), req_flag(true), some("want at least one switch"), ignore_rustdoc)]
    switches: Vec<bool>,
}

#[test]
fn some_with_values() {
    let parser = WithSome::parse().to_options();

    let result = parser.run_inner(&["--argument", "1", "--argument", "2", "--switch", "--switch"]);
    assert!(result.is_ok(), "Failed to parse: {:?}", result);
    let opts = result.unwrap();
    assert_eq!(opts.argument, vec![1, 2]);
    assert_eq!(opts.switches, vec![true, true]);
}

#[test]
fn some_single_value() {
    let parser = WithSome::parse().to_options();

    let result = parser.run_inner(&["--argument", "42", "--switch"]);
    assert!(result.is_ok(), "Failed to parse: {:?}", result);
    let opts = result.unwrap();
    assert_eq!(opts.argument, vec![42]);
    assert_eq!(opts.switches, vec![true]);
}

#[test]
fn some_missing_argument_fails() {
    let parser = WithSome::parse().to_options();

    let result = parser.run_inner(&["--switch"]);
    // Should fail because 'some' requires at least one argument
    assert!(result.is_err(), "Should fail when required 'some' argument is missing");
}

#[test]
fn some_missing_switch_fails() {
    let parser = WithSome::parse().to_options();

    let result = parser.run_inner(&["--argument", "1"]);
    // Should fail because 'some' requires at least one switch
    assert!(result.is_err(), "Should fail when required 'some' switch is missing");
}

#[test]
fn some_no_values_fails() {
    let parser = WithSome::parse().to_options();

    let result = parser.run_inner(&[] as &[&str]);
    // Should fail because 'some' requires at least one of each
    assert!(result.is_err(), "Should fail with no values");
}

// ============================================================================
// anywhere - Parse argument anywhere
// ============================================================================
// Note: `anywhere` is typically used with `any` consumer
// See tests/any_consumer.rs for comprehensive `any` + `anywhere` tests

// ============================================================================
// strict - Strict positional parsing (requires -- separator)
// ============================================================================

#[derive(Debug, Clone, PartialEq)]
#[derive(bpaf_unsynn::Bpaf)]
struct WithStrict {
    #[bpaf(positional("ARG"), strict, many)]
    args: Vec<String>,
}

#[test]
fn strict_with_separator() {
    let parser = WithStrict::parse().to_options();

    // With -- separator, positional args are accepted
    let result = parser.run_inner(&["--", "arg1", "arg2", "arg3"]);
    assert!(result.is_ok(), "Failed to parse: {:?}", result);
    let opts = result.unwrap();
    assert_eq!(opts.args, vec!["arg1", "arg2", "arg3"]);
}

#[test]
fn strict_without_separator_fails() {
    let parser = WithStrict::parse().to_options();

    // Without -- separator, strict mode rejects positional args
    let result = parser.run_inner(&["arg1", "arg2", "arg3"]);
    assert!(result.is_err(), "Should fail without -- separator in strict mode");
}

#[test]
fn strict_no_args() {
    let parser = WithStrict::parse().to_options();

    let result = parser.run_inner(&[] as &[&str]);
    assert!(result.is_ok(), "Should succeed with empty vec: {:?}", result);
    let opts = result.unwrap();
    assert_eq!(opts.args, Vec::<String>::new());
}

// ============================================================================
// non_strict - Non-strict positional parsing
// ============================================================================

#[derive(Debug, Clone, PartialEq)]
#[derive(bpaf_unsynn::Bpaf)]
struct WithNonStrict {
    #[bpaf(positional("ARG"), non_strict)]
    arg: String,
}

#[test]
fn non_strict_with_value() {
    let parser = WithNonStrict::parse().to_options();

    let result = parser.run_inner(&["value"]);
    assert!(result.is_ok(), "Failed to parse: {:?}", result);
    let opts = result.unwrap();
    assert_eq!(opts.arg, "value");
}

// ============================================================================
// adjacent - Require adjacent argument values (no space between flag and value)
// ============================================================================

#[derive(Debug, Clone, PartialEq)]
#[derive(bpaf_unsynn::Bpaf)]
struct WithAdjacent {
    #[bpaf(short, long, argument("SPEC"), adjacent)]
    package: String,
}

#[test]
fn adjacent_short_adjacent() {
    let parser = WithAdjacent::parse().to_options();

    // Short option with adjacent value (no space): -pvalue
    let result = parser.run_inner(&["-pmy-package"]);
    assert!(result.is_ok(), "Failed to parse: {:?}", result);
    let opts = result.unwrap();
    assert_eq!(opts.package, "my-package");
}

#[test]
fn adjacent_long_with_equals() {
    let parser = WithAdjacent::parse().to_options();

    // Long option with = separator: --package=value
    let result = parser.run_inner(&["--package=my-package"]);
    assert!(result.is_ok(), "Failed to parse: {:?}", result);
    let opts = result.unwrap();
    assert_eq!(opts.package, "my-package");
}

#[test]
fn adjacent_long_with_space_fails() {
    let parser = WithAdjacent::parse().to_options();

    // Long option with space should fail because adjacent requires =
    let result = parser.run_inner(&["--package", "my-package"]);
    assert!(result.is_err(), "Should fail when value is not adjacent");
}

// ============================================================================
// Combined tests
// ============================================================================

#[derive(Debug, Clone, PartialEq)]
#[derive(bpaf_unsynn::Bpaf)]
struct CombinedAdvanced {
    #[bpaf(long, argument("FILE"), many, catch)]
    files: Vec<String>,

    #[bpaf(long, argument("NUM"), many)]
    numbers: Vec<i32>,
}

#[test]
fn combined_many_with_catch() {
    let parser = CombinedAdvanced::parse().to_options();

    let result = parser.run_inner(&["--numbers", "1", "--numbers", "2", "--files", "a.txt"]);
    assert!(result.is_ok(), "Failed to parse: {:?}", result);
    let opts = result.unwrap();
    assert_eq!(opts.numbers, vec![1, 2]);
    assert_eq!(opts.files, vec!["a.txt"]);
}

#[test]
fn combined_only_many() {
    let parser = CombinedAdvanced::parse().to_options();

    let result = parser.run_inner(&["--numbers", "42"]);
    assert!(result.is_ok(), "Failed to parse: {:?}", result);
    let opts = result.unwrap();
    assert_eq!(opts.numbers, vec![42]);
    assert_eq!(opts.files, Vec::<String>::new());
}
