//! Test that we correctly filter attributes:
//! - #[bpaf(...)] is processed
//! - #[doc = "..."] is extracted
//! - Everything else is ignored

use bpaf::Parser;

// ============================================================================
// Mix of different attribute types
// ============================================================================

#[derive(Debug, Clone, PartialEq)]
#[derive(bpaf_unsynn::Bpaf)]
struct MixedAttributes {
    /// This is a doc comment
    #[allow(dead_code)]
    #[bpaf(long)]
    #[cfg_attr(test, allow(unused))]
    verbose: bool,

    #[bpaf(long, argument("FILE"))]
    #[allow(dead_code)]
    /// Another doc comment
    #[cfg_attr(debug_assertions, allow(unused))]
    input: String,
}

#[test]
fn mixed_attributes_work() {
    let parser = MixedAttributes::parse().to_options();

    let result = parser.run_inner(&["--verbose", "--input", "test.txt"]);
    assert!(result.is_ok(), "Failed to parse: {:?}", result);
    let opts = result.unwrap();
    assert_eq!(opts.verbose, true);
    assert_eq!(opts.input, "test.txt");
}

// ============================================================================
// Derive macros should be ignored
// ============================================================================

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[derive(bpaf_unsynn::Bpaf)]
struct WithDerive {
    #[bpaf(short, long)]
    flag: bool,
}

#[test]
fn derive_attributes_ignored() {
    let parser = WithDerive::parse().to_options();

    let result = parser.run_inner(&["-f"]);
    assert!(result.is_ok(), "Failed to parse: {:?}", result);
    let opts = result.unwrap();
    assert_eq!(opts.flag, true);
}

// ============================================================================
// Allow/deny attributes should be ignored
// ============================================================================

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
#[derive(bpaf_unsynn::Bpaf)]
struct WithLints {
    #[allow(dead_code)]
    #[bpaf(long)]
    option: bool,
}

#[test]
fn lint_attributes_ignored() {
    let parser = WithLints::parse().to_options();

    let result = parser.run_inner(&["--option"]);
    assert!(result.is_ok(), "Failed to parse: {:?}", result);
    let opts = result.unwrap();
    assert_eq!(opts.option, true);
}

// ============================================================================
// Doc comments mixed with other attributes
// ============================================================================

#[derive(Debug, Clone, PartialEq)]
#[derive(bpaf_unsynn::Bpaf)]
struct DocWithOtherAttrs {
    /// Enable verbose mode
    /// This is a multi-line
    /// doc comment
    #[allow(dead_code)]
    #[bpaf(short('v'), long)]
    verbose: bool,

    #[allow(dead_code)]
    /// Input file path
    #[bpaf(long, argument("FILE"))]
    input: String,
}

#[test]
fn doc_with_other_attrs() {
    let parser = DocWithOtherAttrs::parse().to_options();

    let result = parser.run_inner(&["-v", "--input", "file.txt"]);
    assert!(result.is_ok(), "Failed to parse: {:?}", result);
    let opts = result.unwrap();
    assert_eq!(opts.verbose, true);
    assert_eq!(opts.input, "file.txt");
}

// ============================================================================
// Multiple derives before bpaf derive
// ============================================================================

#[derive(Debug)]
#[derive(Clone)]
#[derive(PartialEq)]
#[derive(bpaf_unsynn::Bpaf)]
struct MultipleDerives {
    #[bpaf(long)]
    flag: bool,
}

#[test]
fn multiple_derives_before_bpaf() {
    let parser = MultipleDerives::parse().to_options();

    let result = parser.run_inner(&["--flag"]);
    assert!(result.is_ok(), "Failed to parse: {:?}", result);
    let opts = result.unwrap();
    assert_eq!(opts.flag, true);
}

// ============================================================================
// Cfg attributes should be ignored
// ============================================================================

#[derive(Debug, Clone, PartialEq)]
#[derive(bpaf_unsynn::Bpaf)]
#[cfg_attr(test, derive(Default))]
struct WithCfgAttr {
    #[cfg_attr(debug_assertions, allow(dead_code))]
    #[bpaf(long)]
    /// Debug flag
    debug: bool,
}

#[test]
fn cfg_attributes_ignored() {
    let parser = WithCfgAttr::parse().to_options();

    let result = parser.run_inner(&["--debug"]);
    assert!(result.is_ok(), "Failed to parse: {:?}", result);
    let opts = result.unwrap();
    assert_eq!(opts.debug, true);
}

// ============================================================================
// Complex real-world case
// ============================================================================

#[derive(Debug, Clone, PartialEq)]
#[derive(bpaf_unsynn::Bpaf)]
#[allow(dead_code)]
struct ComplexAttributes {
    /// Verbose output
    #[allow(dead_code)]
    #[bpaf(short('v'), long)]
    #[cfg_attr(test, allow(unused))]
    verbose: bool,

    #[bpaf(long, argument("FILE"))]
    #[allow(dead_code)]
    /// Input file
    #[cfg_attr(debug_assertions, allow(unused))]
    input: String,

    /// Optional output
    #[bpaf(long, argument("FILE"))]
    #[allow(dead_code)]
    output: Option<String>,
}

#[test]
fn complex_attributes_all_specified() {
    let parser = ComplexAttributes::parse().to_options();

    let result = parser.run_inner(&[
        "-v",
        "--input", "in.txt",
        "--output", "out.txt",
    ]);
    assert!(result.is_ok(), "Failed to parse: {:?}", result);
    let opts = result.unwrap();
    assert_eq!(opts.verbose, true);
    assert_eq!(opts.input, "in.txt");
    assert_eq!(opts.output, Some("out.txt".to_string()));
}

#[test]
fn complex_attributes_minimal() {
    let parser = ComplexAttributes::parse().to_options();

    let result = parser.run_inner(&["--input", "in.txt"]);
    assert!(result.is_ok(), "Failed to parse: {:?}", result);
    let opts = result.unwrap();
    assert_eq!(opts.verbose, false);
    assert_eq!(opts.input, "in.txt");
    assert_eq!(opts.output, None);
}
