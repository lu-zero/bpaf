//! Tests for PostParse attributes (type-changing)
//!
//! Covers: map, parse, optional, many, some, catch, collect, count,
//! anywhere, adjacent, strict, non_strict

use bpaf::Parser;

// =============================================================================
// Map attribute
// =============================================================================

#[derive(Debug, Clone, PartialEq, bpaf_unsynn::Bpaf)]
#[bpaf(options)]
struct MapAttr {
    #[bpaf(long, argument("NUM"), map(|x: i32| x * 2))]
    value: i32,
}

#[test]
fn map_transforms_value() {
    let parser = map_attr();

    let r = parser.run_inner(&["--value", "21"]).unwrap();
    assert_eq!(r.value, 42);
}

// =============================================================================
// Parse attribute
// =============================================================================

fn parse_hex(s: String) -> Result<u32, String> {
    u32::from_str_radix(&s, 16).map_err(|e| e.to_string())
}

#[derive(Debug, Clone, PartialEq, bpaf_unsynn::Bpaf)]
#[bpaf(options)]
struct ParseAttr {
    #[bpaf(external(hex_value))]
    value: u32,
}

fn hex_value() -> impl Parser<u32> {
    bpaf::long("value")
        .argument::<String>("HEX")
        .parse(parse_hex)
}

#[test]
fn parse_with_custom_parser() {
    let parser = parse_attr();

    let r = parser.run_inner(&["--value", "ff"]).unwrap();
    assert_eq!(r.value, 255);
}

// =============================================================================
// Optional attribute (explicit)
// =============================================================================

#[derive(Debug, Clone, PartialEq, bpaf_unsynn::Bpaf)]
#[bpaf(options)]
struct OptionalExplicit {
    #[bpaf(long, argument("VAL"), optional)]
    value: Option<String>,
}

#[test]
fn test_optional_explicit() {
    let parser = optional_explicit();

    let r = parser.run_inner(&[]).unwrap();
    assert_eq!(r.value, None);

    let r = parser.run_inner(&["--value", "test"]).unwrap();
    assert_eq!(r.value, Some("test".to_string()));
}

// =============================================================================
// Many attribute (explicit)
// =============================================================================

#[derive(Debug, Clone, PartialEq, bpaf_unsynn::Bpaf)]
#[bpaf(options)]
struct ManyExplicit {
    #[bpaf(long, argument("VAL"), many)]
    values: Vec<String>,
}

#[test]
fn test_many_explicit() {
    let parser = many_explicit();

    let r = parser.run_inner(&[]).unwrap();
    assert!(r.values.is_empty());

    let r = parser
        .run_inner(&["--values", "a", "--values", "b"])
        .unwrap();
    assert_eq!(r.values, vec!["a", "b"]);
}

// =============================================================================
// Some attribute
// =============================================================================

#[derive(Debug, Clone, PartialEq, bpaf_unsynn::Bpaf)]
#[bpaf(options)]
struct SomeAttr {
    #[bpaf(long, argument("VAL"), some("need at least one value"))]
    values: Vec<String>,
}

#[test]
fn some_requires_at_least_one() {
    let parser = some_attr();

    // Fails without values
    let r = parser.run_inner(&[]);
    assert!(r.is_err());

    // Succeeds with values
    let r = parser
        .run_inner(&["--values", "a", "--values", "b"])
        .unwrap();
    assert_eq!(r.values, vec!["a", "b"]);
}

// =============================================================================
// Catch attribute - compilation test
// catch() is best used with external parser to avoid double-wrapping
// =============================================================================

/// Newtype to avoid auto-optional wrapping
#[derive(Debug, Clone, PartialEq)]
struct CatchResult(Option<i32>);

#[derive(Debug, Clone, PartialEq, bpaf_unsynn::Bpaf)]
struct CatchAttr {
    #[bpaf(external(catch_value))]
    value: CatchResult,
}

fn catch_value() -> impl Parser<CatchResult> {
    bpaf::long("value")
        .argument::<i32>("NUM")
        .optional()
        .catch()
        .map(CatchResult)
}

#[test]
fn catch_compiles() {
    // Verify the derive compiles with catch
    let _ = catch_attr();
}

// =============================================================================
// Count attribute - compilation test
// count() requires external parser setup
// =============================================================================

#[derive(Debug, Clone, PartialEq, bpaf_unsynn::Bpaf)]
struct CountAttr {
    #[bpaf(external(count_verbose))]
    verbosity: usize,
}

fn count_verbose() -> impl Parser<usize> {
    bpaf::short('v').long("verbose").req_flag(()).count()
}

#[test]
fn count_compiles_and_works() {
    let parser = count_attr().to_options();

    let r = parser.run_inner(&[]).unwrap();
    assert_eq!(r.verbosity, 0);

    let r = parser.run_inner(&["-v"]).unwrap();
    assert_eq!(r.verbosity, 1);

    let r = parser.run_inner(&["-v", "-v", "-v"]).unwrap();
    assert_eq!(r.verbosity, 3);
}

// =============================================================================
// Adjacent attribute (struct-level - tested in struct_attrs.rs)
// =============================================================================

// Adjacent is primarily a struct-level attribute.
// See struct_attrs.rs for comprehensive adjacent tests.

// =============================================================================
// Strict attribute (requires external for proper usage)
// =============================================================================

#[derive(Debug, Clone, PartialEq, bpaf_unsynn::Bpaf)]
#[bpaf(options)]
struct StrictAttr {
    #[bpaf(external(strict_args), many)]
    args: Vec<String>,
}

fn strict_args() -> impl Parser<String> {
    bpaf::positional::<String>("ARG").strict()
}

#[test]
fn strict_positional() {
    let parser = strict_attr();

    // After --, everything is positional
    let r = parser.run_inner(&["--", "-a", "-b"]).unwrap();
    assert_eq!(r.args, vec!["-a", "-b"]);
}

// =============================================================================
// Chained PostParse attributes
// =============================================================================

#[derive(Debug, Clone, PartialEq, bpaf_unsynn::Bpaf)]
#[bpaf(options)]
struct ChainedPostParse {
    #[bpaf(long, argument("NUM"), map(|x: i32| x * 2), optional)]
    value: Option<i32>,
}

#[test]
fn chained_map_then_optional() {
    let parser = chained_post_parse();

    let r = parser.run_inner(&[]).unwrap();
    assert_eq!(r.value, None);

    let r = parser.run_inner(&["--value", "10"]).unwrap();
    assert_eq!(r.value, Some(20)); // 10 * 2
}

// =============================================================================
// Direct parse attribute (not via external)
// =============================================================================

fn validate_positive(n: u32) -> Result<u32, String> {
    if n > 0 {
        Ok(n)
    } else {
        Err("must be positive".to_string())
    }
}

#[derive(Debug, Clone, PartialEq, bpaf_unsynn::Bpaf)]
#[bpaf(options)]
struct DirectParse {
    #[bpaf(long, argument("NUM"), parse(validate_positive))]
    count: u32,
}

#[test]
fn direct_parse_attribute() {
    let parser = direct_parse();

    let r = parser.run_inner(&["--count", "42"]).unwrap();
    assert_eq!(r.count, 42);

    // Invalid value (zero is rejected by validator)
    let r = parser.run_inner(&["--count", "0"]);
    assert!(r.is_err());
}

// =============================================================================
// Direct collect attribute (not via external)
// =============================================================================

#[derive(Debug, Clone, PartialEq, bpaf_unsynn::Bpaf)]
#[bpaf(options)]
struct DirectCollect {
    #[bpaf(long, argument("VAL"), collect)]
    items: Vec<String>,
}

#[test]
fn direct_collect_attribute() {
    let parser = direct_collect();

    let r = parser.run_inner(&[]).unwrap();
    assert!(r.items.is_empty());

    let r = parser
        .run_inner(&["--items", "a", "--items", "b", "--items", "c"])
        .unwrap();
    assert_eq!(r.items, vec!["a", "b", "c"]);
}
