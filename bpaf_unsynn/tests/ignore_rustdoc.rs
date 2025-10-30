//! Tests for the `ignore_rustdoc` attribute
//!
//! The `ignore_rustdoc` attribute is a compatibility flag from bpaf_derive.
//! In bpaf_derive, it prevents doc comments from being used as help text.
//! In bpaf_unsynn, this is currently a no-op since we don't extract doc comments,
//! but we support the attribute for compatibility.

use bpaf::Parser;

// ============================================================================
// Basic ignore_rustdoc usage
// ============================================================================

#[derive(Debug, Clone, PartialEq)]
#[derive(bpaf_unsynn::Bpaf)]
struct BasicIgnoreRustdoc {
    /// This is a doc comment that would normally be used as help
    /// But ignore_rustdoc prevents it
    #[bpaf(long, ignore_rustdoc)]
    flag: bool,
}

#[test]
fn basic_ignore_rustdoc() {
    let parser = BasicIgnoreRustdoc::parse().to_options();

    let result = parser.run_inner(&["--flag"]);
    assert!(result.is_ok(), "Failed to parse: {:?}", result);
    let opts = result.unwrap();
    assert_eq!(opts.flag, true);
}

#[test]
fn basic_ignore_rustdoc_not_specified() {
    let parser = BasicIgnoreRustdoc::parse().to_options();

    let result = parser.run_inner(&[] as &[&str]);
    assert!(result.is_ok(), "Failed to parse: {:?}", result);
    let opts = result.unwrap();
    assert_eq!(opts.flag, false);
}

// ============================================================================
// ignore_rustdoc with argument
// ============================================================================

#[derive(Debug, Clone, PartialEq)]
#[derive(bpaf_unsynn::Bpaf)]
struct IgnoreRustdocWithArgument {
    /// Path to the input file
    #[bpaf(long, argument("FILE"), ignore_rustdoc)]
    input: String,
}

#[test]
fn ignore_rustdoc_with_argument() {
    let parser = IgnoreRustdocWithArgument::parse().to_options();

    let result = parser.run_inner(&["--input", "test.txt"]);
    assert!(result.is_ok(), "Failed to parse: {:?}", result);
    let opts = result.unwrap();
    assert_eq!(opts.input, "test.txt");
}

// ============================================================================
// ignore_rustdoc with short flag
// ============================================================================

#[derive(Debug, Clone, PartialEq)]
#[derive(bpaf_unsynn::Bpaf)]
struct IgnoreRustdocWithShort {
    /// Verbose output
    #[bpaf(short('v'), long, ignore_rustdoc)]
    verbose: bool,
}

#[test]
fn ignore_rustdoc_with_short() {
    let parser = IgnoreRustdocWithShort::parse().to_options();

    let result = parser.run_inner(&["-v"]);
    assert!(result.is_ok(), "Failed to parse: {:?}", result);
    let opts = result.unwrap();
    assert_eq!(opts.verbose, true);
}

// ============================================================================
// ignore_rustdoc with help attribute (explicit help overrides)
// ============================================================================

#[derive(Debug, Clone, PartialEq)]
#[derive(bpaf_unsynn::Bpaf)]
struct IgnoreRustdocWithHelp {
    /// This doc comment is ignored
    #[bpaf(long, ignore_rustdoc, help("Explicit help text"))]
    option: bool,
}

#[test]
fn ignore_rustdoc_with_help() {
    let parser = IgnoreRustdocWithHelp::parse().to_options();

    let result = parser.run_inner(&["--option"]);
    assert!(result.is_ok(), "Failed to parse: {:?}", result);
    let opts = result.unwrap();
    assert_eq!(opts.option, true);
}

// ============================================================================
// ignore_rustdoc with fallback
// ============================================================================

#[derive(Debug, Clone, PartialEq)]
#[derive(bpaf_unsynn::Bpaf)]
struct IgnoreRustdocWithFallback {
    /// Default value documentation
    #[bpaf(long, argument("NUM"), fallback(42), ignore_rustdoc)]
    count: i32,
}

#[test]
fn ignore_rustdoc_with_fallback_specified() {
    let parser = IgnoreRustdocWithFallback::parse().to_options();

    let result = parser.run_inner(&["--count", "100"]);
    assert!(result.is_ok(), "Failed to parse: {:?}", result);
    let opts = result.unwrap();
    assert_eq!(opts.count, 100);
}

#[test]
fn ignore_rustdoc_with_fallback_default() {
    let parser = IgnoreRustdocWithFallback::parse().to_options();

    let result = parser.run_inner(&[] as &[&str]);
    assert!(result.is_ok(), "Failed to parse: {:?}", result);
    let opts = result.unwrap();
    assert_eq!(opts.count, 42);
}

// ============================================================================
// ignore_rustdoc with optional
// ============================================================================

#[derive(Debug, Clone, PartialEq)]
#[derive(bpaf_unsynn::Bpaf)]
struct IgnoreRustdocWithOptional {
    /// Optional configuration file
    #[bpaf(long, argument("FILE"), ignore_rustdoc)]
    config: Option<String>,
}

#[test]
fn ignore_rustdoc_with_optional_some() {
    let parser = IgnoreRustdocWithOptional::parse().to_options();

    let result = parser.run_inner(&["--config", "app.toml"]);
    assert!(result.is_ok(), "Failed to parse: {:?}", result);
    let opts = result.unwrap();
    assert_eq!(opts.config, Some("app.toml".to_string()));
}

#[test]
fn ignore_rustdoc_with_optional_none() {
    let parser = IgnoreRustdocWithOptional::parse().to_options();

    let result = parser.run_inner(&[] as &[&str]);
    assert!(result.is_ok(), "Failed to parse: {:?}", result);
    let opts = result.unwrap();
    assert_eq!(opts.config, None);
}

// ============================================================================
// ignore_rustdoc with many
// ============================================================================

#[derive(Debug, Clone, PartialEq)]
#[derive(bpaf_unsynn::Bpaf)]
struct IgnoreRustdocWithMany {
    /// Input files to process
    #[bpaf(long, argument("FILE"), ignore_rustdoc, many)]
    files: Vec<String>,
}

#[test]
fn ignore_rustdoc_with_many_empty() {
    let parser = IgnoreRustdocWithMany::parse().to_options();

    let result = parser.run_inner(&[] as &[&str]);
    assert!(result.is_ok(), "Failed to parse: {:?}", result);
    let opts = result.unwrap();
    assert_eq!(opts.files, Vec::<String>::new());
}

#[test]
fn ignore_rustdoc_with_many_multiple() {
    let parser = IgnoreRustdocWithMany::parse().to_options();

    let result = parser.run_inner(&["--files", "a.txt", "--files", "b.txt"]);
    assert!(result.is_ok(), "Failed to parse: {:?}", result);
    let opts = result.unwrap();
    assert_eq!(opts.files, vec!["a.txt", "b.txt"]);
}

// ============================================================================
// ignore_rustdoc with guard
// ============================================================================

#[derive(Debug, Clone, PartialEq)]
#[derive(bpaf_unsynn::Bpaf)]
struct IgnoreRustdocWithGuard {
    /// Port number (must be > 1024)
    #[bpaf(long, argument("PORT"), ignore_rustdoc, guard(|x: &u16| *x > 1024, "port must be > 1024"))]
    port: u16,
}

#[test]
fn ignore_rustdoc_with_guard_valid() {
    let parser = IgnoreRustdocWithGuard::parse().to_options();

    let result = parser.run_inner(&["--port", "8080"]);
    assert!(result.is_ok(), "Failed to parse: {:?}", result);
    let opts = result.unwrap();
    assert_eq!(opts.port, 8080);
}

// ============================================================================
// ignore_rustdoc with positional
// ============================================================================

#[derive(Debug, Clone, PartialEq)]
#[derive(bpaf_unsynn::Bpaf)]
struct IgnoreRustdocWithPositional {
    /// Source file to process
    #[bpaf(positional("SOURCE"), ignore_rustdoc)]
    source: String,
}

#[test]
fn ignore_rustdoc_with_positional() {
    let parser = IgnoreRustdocWithPositional::parse().to_options();

    let result = parser.run_inner(&["input.txt"]);
    assert!(result.is_ok(), "Failed to parse: {:?}", result);
    let opts = result.unwrap();
    assert_eq!(opts.source, "input.txt");
}

// ============================================================================
// Complex struct with multiple ignore_rustdoc attributes
// ============================================================================

#[derive(Debug, Clone, PartialEq)]
#[derive(bpaf_unsynn::Bpaf)]
struct ComplexIgnoreRustdoc {
    /// Verbose mode documentation
    #[bpaf(short('v'), long, ignore_rustdoc)]
    verbose: bool,

    /// Input file documentation
    #[bpaf(short('i'), long, argument("FILE"), ignore_rustdoc)]
    input: String,

    /// Output file documentation
    #[bpaf(short('o'), long, argument("FILE"), ignore_rustdoc)]
    output: Option<String>,

    /// Number of threads documentation
    #[bpaf(long, argument("N"), fallback(4), ignore_rustdoc)]
    threads: usize,
}

#[test]
fn complex_ignore_rustdoc_all_specified() {
    let parser = ComplexIgnoreRustdoc::parse().to_options();

    let result = parser.run_inner(&[
        "-v",
        "-i", "input.txt",
        "-o", "output.txt",
        "--threads", "8",
    ]);
    assert!(result.is_ok(), "Failed to parse: {:?}", result);
    let opts = result.unwrap();
    assert_eq!(opts.verbose, true);
    assert_eq!(opts.input, "input.txt");
    assert_eq!(opts.output, Some("output.txt".to_string()));
    assert_eq!(opts.threads, 8);
}

#[test]
fn complex_ignore_rustdoc_minimal() {
    let parser = ComplexIgnoreRustdoc::parse().to_options();

    let result = parser.run_inner(&["-i", "input.txt"]);
    assert!(result.is_ok(), "Failed to parse: {:?}", result);
    let opts = result.unwrap();
    assert_eq!(opts.verbose, false);
    assert_eq!(opts.input, "input.txt");
    assert_eq!(opts.output, None);
    assert_eq!(opts.threads, 4);
}

// ============================================================================
// ignore_rustdoc with env
// ============================================================================

#[derive(Debug, Clone, PartialEq)]
#[derive(bpaf_unsynn::Bpaf)]
struct IgnoreRustdocWithEnv {
    /// API key from environment
    #[bpaf(long, env("API_KEY"), ignore_rustdoc)]
    api_key: Option<String>,
}

#[test]
fn ignore_rustdoc_with_env_from_arg() {
    let parser = IgnoreRustdocWithEnv::parse().to_options();

    let result = parser.run_inner(&["--api-key", "secret123"]);
    assert!(result.is_ok(), "Failed to parse: {:?}", result);
    let opts = result.unwrap();
    assert_eq!(opts.api_key, Some("secret123".to_string()));
}

#[test]
fn ignore_rustdoc_with_env_none() {
    let parser = IgnoreRustdocWithEnv::parse().to_options();

    let result = parser.run_inner(&[] as &[&str]);
    assert!(result.is_ok(), "Failed to parse: {:?}", result);
    let _opts = result.unwrap();
    // Will be None if API_KEY env var is not set
    // We can't reliably test env var reading without setting it
}
