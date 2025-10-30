//! Tests for the `last` attribute
//!
//! The `last` attribute makes the parser use the last occurrence of an option
//! when it appears multiple times, instead of failing or collecting all values.

use bpaf::Parser;

// ============================================================================
// Basic last attribute usage
// ============================================================================

#[derive(Debug, Clone, PartialEq)]
#[derive(bpaf_unsynn::Bpaf)]
struct BasicLast {
    /// Use the last value if specified multiple times
    #[bpaf(long, argument("VALUE"), last)]
    output: String,
}

#[test]
fn last_basic_single_value() {
    let parser = BasicLast::parse().to_options();

    let result = parser.run_inner(&["--output", "file.txt"]);
    assert!(result.is_ok(), "Failed to parse: {:?}", result);
    let opts = result.unwrap();
    assert_eq!(opts.output, "file.txt");
}

#[test]
fn last_basic_multiple_values() {
    let parser = BasicLast::parse().to_options();

    // When multiple values are provided, use the last one
    let result = parser.run_inner(&["--output", "first.txt", "--output", "second.txt", "--output", "last.txt"]);
    assert!(result.is_ok(), "Failed to parse: {:?}", result);
    let opts = result.unwrap();
    assert_eq!(opts.output, "last.txt");
}

#[test]
fn last_basic_two_values() {
    let parser = BasicLast::parse().to_options();

    let result = parser.run_inner(&["--output", "first.txt", "--output", "last.txt"]);
    assert!(result.is_ok(), "Failed to parse: {:?}", result);
    let opts = result.unwrap();
    assert_eq!(opts.output, "last.txt");
}

// ============================================================================
// Last with short options
// ============================================================================

#[derive(Debug, Clone, PartialEq)]
#[derive(bpaf_unsynn::Bpaf)]
struct LastWithShort {
    #[bpaf(short('o'), long, argument("FILE"), last)]
    output: String,
}

#[test]
fn last_with_short_option() {
    let parser = LastWithShort::parse().to_options();

    let result = parser.run_inner(&["-o", "first.txt", "-o", "second.txt", "-o", "third.txt"]);
    assert!(result.is_ok(), "Failed to parse: {:?}", result);
    let opts = result.unwrap();
    assert_eq!(opts.output, "third.txt");
}

#[test]
fn last_with_mixed_short_long() {
    let parser = LastWithShort::parse().to_options();

    // Mix short and long options
    let result = parser.run_inner(&["-o", "first.txt", "--output", "second.txt", "-o", "last.txt"]);
    assert!(result.is_ok(), "Failed to parse: {:?}", result);
    let opts = result.unwrap();
    assert_eq!(opts.output, "last.txt");
}

// ============================================================================
// Last with different types
// ============================================================================

#[derive(Debug, Clone, PartialEq)]
#[derive(bpaf_unsynn::Bpaf)]
struct LastWithNumber {
    #[bpaf(short('n'), long, argument("NUM"), last)]
    number: usize,
}

#[test]
fn last_with_numeric_type() {
    let parser = LastWithNumber::parse().to_options();

    let result = parser.run_inner(&["-n", "10", "-n", "20", "--number", "30"]);
    assert!(result.is_ok(), "Failed to parse: {:?}", result);
    let opts = result.unwrap();
    assert_eq!(opts.number, 30);
}

// ============================================================================
// Last with fallback
// ============================================================================

#[derive(Debug, Clone, PartialEq)]
#[derive(bpaf_unsynn::Bpaf)]
struct LastWithFallback {
    #[bpaf(long, argument("VALUE"), fallback(String::from("default")), last)]
    config: String,
}

#[test]
fn last_with_fallback_not_specified() {
    let parser = LastWithFallback::parse().to_options();

    let result = parser.run_inner(&[] as &[&str]);
    assert!(result.is_ok(), "Failed to parse: {:?}", result);
    let opts = result.unwrap();
    assert_eq!(opts.config, "default");
}

#[test]
fn last_with_fallback_specified_once() {
    let parser = LastWithFallback::parse().to_options();

    let result = parser.run_inner(&["--config", "custom"]);
    assert!(result.is_ok(), "Failed to parse: {:?}", result);
    let opts = result.unwrap();
    assert_eq!(opts.config, "custom");
}

#[test]
fn last_with_fallback_specified_multiple() {
    let parser = LastWithFallback::parse().to_options();

    let result = parser.run_inner(&["--config", "first", "--config", "second", "--config", "third"]);
    assert!(result.is_ok(), "Failed to parse: {:?}", result);
    let opts = result.unwrap();
    assert_eq!(opts.config, "third");
}

// ============================================================================
// Last with Optional
// ============================================================================

#[derive(Debug, Clone, PartialEq)]
#[derive(bpaf_unsynn::Bpaf)]
struct LastWithOptional {
    #[bpaf(long, argument("VALUE"), last)]
    value: Option<String>,
}

#[test]
fn last_with_optional_none() {
    let parser = LastWithOptional::parse().to_options();

    let result = parser.run_inner(&[] as &[&str]);
    assert!(result.is_ok(), "Failed to parse: {:?}", result);
    let opts = result.unwrap();
    assert_eq!(opts.value, None);
}

#[test]
fn last_with_optional_single() {
    let parser = LastWithOptional::parse().to_options();

    let result = parser.run_inner(&["--value", "present"]);
    assert!(result.is_ok(), "Failed to parse: {:?}", result);
    let opts = result.unwrap();
    assert_eq!(opts.value, Some("present".to_string()));
}

#[test]
fn last_with_optional_multiple() {
    let parser = LastWithOptional::parse().to_options();

    let result = parser.run_inner(&["--value", "first", "--value", "last"]);
    assert!(result.is_ok(), "Failed to parse: {:?}", result);
    let opts = result.unwrap();
    assert_eq!(opts.value, Some("last".to_string()));
}

// ============================================================================
// Complex struct with multiple fields, one using last
// ============================================================================

#[derive(Debug, Clone, PartialEq)]
#[derive(bpaf_unsynn::Bpaf)]
struct ComplexWithLast {
    #[bpaf(short, long)]
    verbose: bool,

    /// Can be specified multiple times, last wins
    #[bpaf(short('o'), long, argument("FILE"), last)]
    output: String,

    /// Multiple inputs are collected
    #[bpaf(short('i'), long, argument("FILE"), many)]
    inputs: Vec<String>,

    #[bpaf(short('j'), long, argument("N"), fallback(1))]
    jobs: usize,
}

#[test]
fn complex_with_last_basic() {
    let parser = ComplexWithLast::parse().to_options();

    let result = parser.run_inner(&[
        "-v",
        "-o", "out1.txt",
        "-i", "in1.txt",
        "-o", "out2.txt",
        "-i", "in2.txt",
        "-o", "final.txt",
    ]);
    assert!(result.is_ok(), "Failed to parse: {:?}", result);
    let opts = result.unwrap();
    assert_eq!(opts.verbose, true);
    assert_eq!(opts.output, "final.txt"); // Last value
    assert_eq!(opts.inputs, vec!["in1.txt", "in2.txt"]); // All values collected
    assert_eq!(opts.jobs, 1); // Default
}

#[test]
fn complex_with_last_interleaved() {
    let parser = ComplexWithLast::parse().to_options();

    let result = parser.run_inner(&[
        "--output", "first.txt",
        "--inputs", "a.txt",
        "-j", "4",
        "--output", "second.txt",
        "--inputs", "b.txt",
        "--output", "last.txt",
    ]);
    assert!(result.is_ok(), "Failed to parse: {:?}", result);
    let opts = result.unwrap();
    assert_eq!(opts.verbose, false);
    assert_eq!(opts.output, "last.txt"); // Last output value
    assert_eq!(opts.inputs, vec!["a.txt", "b.txt"]);
    assert_eq!(opts.jobs, 4); // jobs specified once
}

// ============================================================================
// Last with help text
// ============================================================================

#[derive(Debug, Clone, PartialEq)]
#[derive(bpaf_unsynn::Bpaf)]
struct LastWithHelp {
    /// Output file (last value wins if specified multiple times)
    #[bpaf(long, argument("FILE"), last, help("Output file path"))]
    output: String,
}

#[test]
fn last_with_help_works() {
    let parser = LastWithHelp::parse().to_options();

    let result = parser.run_inner(&["--output", "a.txt", "--output", "b.txt"]);
    assert!(result.is_ok(), "Failed to parse: {:?}", result);
    let opts = result.unwrap();
    assert_eq!(opts.output, "b.txt");
}

// ============================================================================
// Last with custom usage
// ============================================================================

#[derive(Debug, Clone, PartialEq)]
#[derive(bpaf_unsynn::Bpaf)]
struct LastWithCustomUsage {
    #[bpaf(long, argument("FILE"), last, custom_usage("--output FILE"))]
    output: String,
}

#[test]
fn last_with_custom_usage_works() {
    let parser = LastWithCustomUsage::parse().to_options();

    let result = parser.run_inner(&["--output", "first.txt", "--output", "last.txt"]);
    assert!(result.is_ok(), "Failed to parse: {:?}", result);
    let opts = result.unwrap();
    assert_eq!(opts.output, "last.txt");
}

// ============================================================================
// Last combined with guard
// ============================================================================

#[derive(Debug, Clone, PartialEq)]
#[derive(bpaf_unsynn::Bpaf)]
struct LastWithGuard {
    #[bpaf(long, argument("NUM"), last, guard(|x: &usize| *x > 0, "must be positive"))]
    count: usize,
}

#[test]
fn last_with_guard_valid() {
    let parser = LastWithGuard::parse().to_options();

    let result = parser.run_inner(&["--count", "5", "--count", "10"]);
    assert!(result.is_ok(), "Failed to parse: {:?}", result);
    let opts = result.unwrap();
    assert_eq!(opts.count, 10);
}

#[test]
fn last_with_guard_invalid_last() {
    let parser = LastWithGuard::parse().to_options();

    // The last value fails the guard
    let result = parser.run_inner(&["--count", "5", "--count", "0"]);
    assert!(result.is_err(), "Should fail guard validation");
}

#[test]
fn last_with_guard_valid_last() {
    let parser = LastWithGuard::parse().to_options();

    // Even though first value is invalid, last is valid
    let result = parser.run_inner(&["--count", "10", "--count", "20"]);
    assert!(result.is_ok(), "Failed to parse: {:?}", result);
    let opts = result.unwrap();
    assert_eq!(opts.count, 20);
}
