//! Tests for advanced features
//!
//! Covers: typed any, collect, anywhere (on any), non_strict,
//! ignore_rustdoc, path, multi-field tuple variants, cargo_helper
//!
//! Note: Some features have complex API requirements - tested with external parsers

use bpaf::Parser;
use std::collections::HashSet;

// =============================================================================
// Typed any: any::<Type>("META", check)
// =============================================================================

fn check_pathbuf(p: std::path::PathBuf) -> Option<std::path::PathBuf> {
    if p.to_string_lossy().starts_with('-') {
        None
    } else {
        Some(p)
    }
}

#[derive(Debug, Clone, PartialEq, bpaf_unsynn::Bpaf)]
#[bpaf(options)]
struct TypedAny {
    #[bpaf(any::<std::path::PathBuf>("FILE", check_pathbuf))]
    file: std::path::PathBuf,
}

#[test]
fn typed_any_works() {
    let parser = typed_any();
    let r = parser.run_inner(&["file.txt"]).unwrap();
    assert_eq!(r.file, std::path::PathBuf::from("file.txt"));
}

// =============================================================================
// Collect attribute - collects Vec into other container
// =============================================================================

/// Newtype to avoid auto-wrapping
#[derive(Debug, Clone, PartialEq)]
struct Tags(HashSet<String>);

#[derive(Debug, Clone, PartialEq, bpaf_unsynn::Bpaf)]
#[bpaf(options)]
struct CollectAttr {
    #[bpaf(external(tags_parser))]
    tags: Tags,
}

fn tags_parser() -> impl Parser<Tags> {
    bpaf::long("tag")
        .argument::<String>("TAG")
        .collect::<HashSet<_>>()
        .map(Tags)
}

#[test]
fn collect_into_hashset() {
    let parser = collect_attr();
    let r = parser
        .run_inner(&["--tag", "a", "--tag", "b", "--tag", "a"])
        .unwrap();
    // HashSet deduplicates
    assert_eq!(r.tags.0.len(), 2);
    assert!(r.tags.0.contains("a"));
    assert!(r.tags.0.contains("b"));
}

// =============================================================================
// Anywhere attribute - on any parser
// =============================================================================

fn check_anywhere(s: String) -> Option<String> {
    if s.starts_with('-') {
        None
    } else {
        Some(s)
    }
}

#[derive(Debug, Clone, PartialEq, bpaf_unsynn::Bpaf)]
#[bpaf(options)]
struct AnywhereAttr {
    #[bpaf(any("ARG", check_anywhere), anywhere, optional)]
    arg: Option<String>,
}

#[test]
fn anywhere_on_any() {
    let parser = anywhere_attr();
    let r = parser.run_inner(&["hello"]).unwrap();
    assert_eq!(r.arg, Some("hello".to_string()));
}

// =============================================================================
// Non-strict attribute
// =============================================================================

#[derive(Debug, Clone, PartialEq, bpaf_unsynn::Bpaf)]
struct NonStrictPositional {
    #[bpaf(positional("ARG"), non_strict)]
    arg: String,
}

#[derive(Debug, Clone, PartialEq, bpaf_unsynn::Bpaf)]
#[bpaf(options)]
struct NonStrictOpts {
    #[bpaf(external(non_strict_positional))]
    pos: NonStrictPositional,
    #[bpaf(long)]
    verbose: bool,
}

#[test]
fn non_strict_allows_flags_after() {
    let parser = non_strict_opts();
    // In non-strict mode, positional doesn't consume everything
    let r = parser.run_inner(&["myarg", "--verbose"]).unwrap();
    assert_eq!(r.pos.arg, "myarg");
    assert!(r.verbose);
}

// =============================================================================
// Ignore rustdoc attribute
// =============================================================================

/// This doc comment should be ignored
#[derive(Debug, Clone, PartialEq, bpaf_unsynn::Bpaf)]
#[bpaf(options, ignore_rustdoc)]
struct IgnoreRustdocAttr {
    /// This field doc should also be ignored
    #[bpaf(long, help("Explicit help text"))]
    value: bool,
}

#[test]
fn ignore_rustdoc_compiles() {
    let parser = ignore_rustdoc_attr();
    let r = parser.run_inner(&["--value"]).unwrap();
    assert!(r.value);
}

// =============================================================================
// Ignores non-bpaf attributes
// =============================================================================

/// Struct with various non-bpaf attributes that should be ignored
#[derive(Debug, Clone, PartialEq, bpaf_unsynn::Bpaf)]
#[bpaf(options)]
#[allow(dead_code)]
#[cfg(all())]
struct WithOtherAttrs {
    #[allow(unused)]
    #[bpaf(long)]
    verbose: bool,

    #[cfg(any())]
    #[bpaf(long)]
    config: Option<String>,
}

#[test]
fn ignores_non_bpaf_attributes() {
    let parser = with_other_attrs();
    let r = parser.run_inner(&["--verbose"]).unwrap();
    assert!(r.verbose);
}

/// Struct with doc attributes on fields (common pattern with other derives)
#[derive(Debug, Clone, PartialEq, bpaf_unsynn::Bpaf)]
#[bpaf(options)]
#[repr(C)]
struct WithMixedAttrs {
    /// Output file path
    #[bpaf(long("output-file"))]
    output: String,

    /// Enable quiet mode
    #[bpaf(short, long)]
    quiet: bool,
}

#[test]
fn ignores_other_field_attributes() {
    let parser = with_mixed_attrs();
    let r = parser.run_inner(&["--output-file", "test.txt"]).unwrap();
    assert_eq!(r.output, "test.txt");
    assert!(!r.quiet);
}

// =============================================================================
// Path attribute (uses ::bpaf path explicitly)
// =============================================================================

#[derive(Debug, Clone, PartialEq, bpaf_unsynn::Bpaf)]
#[bpaf(options, path(::bpaf))]
struct CustomPathAttr {
    #[bpaf(long)]
    verbose: bool,
}

#[test]
fn custom_path_compiles() {
    let parser = custom_path_attr();
    let r = parser.run_inner(&["--verbose"]).unwrap();
    assert!(r.verbose);
}

// =============================================================================
// Multi-field tuple variants
// =============================================================================

#[derive(Debug, Clone, PartialEq, bpaf_unsynn::Bpaf)]
#[bpaf(options)]
enum MultiFieldTuple {
    #[bpaf(command)]
    Copy(String, String),
    #[bpaf(command)]
    Move(String, String, String),
}

#[test]
fn multi_field_tuple_two_fields() {
    let parser = multi_field_tuple();
    let r = parser.run_inner(&["copy", "src.txt", "dst.txt"]).unwrap();
    assert_eq!(
        r,
        MultiFieldTuple::Copy("src.txt".to_string(), "dst.txt".to_string())
    );
}

#[test]
fn multi_field_tuple_three_fields() {
    let parser = multi_field_tuple();
    // All three fields are positional
    let r = parser
        .run_inner(&["move", "src.txt", "dst.txt", "backup.txt"])
        .unwrap();
    assert_eq!(
        r,
        MultiFieldTuple::Move(
            "src.txt".to_string(),
            "dst.txt".to_string(),
            "backup.txt".to_string()
        )
    );
}

// =============================================================================
// Cargo helper - requires parser mode (not options)
// =============================================================================

#[derive(Debug, Clone, PartialEq, bpaf_unsynn::Bpaf)]
#[bpaf(cargo_helper("myapp"))]
struct CargoHelperAttr {
    #[bpaf(long)]
    verbose: bool,
}

#[test]
fn cargo_helper_compiles() {
    // cargo_helper returns impl Parser, needs .to_options()
    let parser = cargo_helper_attr().to_options();
    let r = parser.run_inner(&["--verbose"]).unwrap();
    assert!(r.verbose);
}

// =============================================================================
// Field-level adjacent via struct-level (working example)
// =============================================================================

#[derive(Debug, Clone, PartialEq, bpaf_unsynn::Bpaf)]
#[bpaf(adjacent)]
struct AdjacentPair {
    #[bpaf(long)]
    key: String,
    #[bpaf(long)]
    value: String,
}

/// Newtype to avoid auto-wrapping
#[derive(Debug, Clone, PartialEq)]
struct OptPair(Option<AdjacentPair>);

#[derive(Debug, Clone, PartialEq, bpaf_unsynn::Bpaf)]
#[bpaf(options)]
struct AdjacentFieldTest {
    #[bpaf(external(test_adjacent_pair))]
    pair: OptPair,
}

fn test_adjacent_pair() -> impl Parser<OptPair> {
    adjacent_pair().optional().map(OptPair)
}

#[test]
fn adjacent_via_struct() {
    let parser = adjacent_field_test();
    let r = parser.run_inner(&["--key", "k", "--value", "v"]).unwrap();
    assert!(r.pair.0.is_some());
    let pair = r.pair.0.unwrap();
    assert_eq!(pair.key, "k");
    assert_eq!(pair.value, "v");
}

// =============================================================================
// Complete attribute (already tested in postdecor.rs, but verify here)
// =============================================================================

fn complete_fn(_input: &String) -> Vec<(&'static str, Option<&'static str>)> {
    vec![("opt1", Some("Option 1")), ("opt2", Some("Option 2"))]
}

#[derive(Debug, Clone, PartialEq, bpaf_unsynn::Bpaf)]
struct CompleteTest {
    #[bpaf(long, argument("VAL"), complete(complete_fn))]
    value: String,
}

#[test]
fn complete_attribute_works() {
    let parser = complete_test().to_options();
    let r = parser.run_inner(&["--value", "test"]).unwrap();
    assert_eq!(r.value, "test");
}
