//! Tests for PostDecor attributes (behavior-changing)
//!
//! Covers: guard, hide, hide_usage, custom_usage, fallback, fallback_with,
//! group_help, debug_fallback, display_fallback, format_fallback, last, complete

// =============================================================================
// Guard attribute
// =============================================================================

fn positive(x: &i32) -> bool {
    *x > 0
}

#[derive(Debug, Clone, PartialEq, bpaf_unsynn::Bpaf)]
#[bpaf(options)]
struct GuardAttr {
    #[bpaf(long, argument("NUM"), guard(positive, "must be positive"))]
    value: i32,
}

#[test]
fn guard_accepts_valid() {
    let parser = guard_attr();
    let r = parser.run_inner(&["--value", "42"]).unwrap();
    assert_eq!(r.value, 42);
}

#[test]
fn guard_rejects_invalid() {
    let parser = guard_attr();
    let r = parser.run_inner(&["--value", "-1"]);
    assert!(r.is_err());
}

#[test]
fn guard_rejects_zero() {
    let parser = guard_attr();
    let r = parser.run_inner(&["--value", "0"]);
    assert!(r.is_err());
}

// =============================================================================
// Hide attribute
// =============================================================================

#[derive(Debug, Clone, PartialEq, bpaf_unsynn::Bpaf)]
#[bpaf(options)]
struct HideAttr {
    #[bpaf(long, hide)]
    hidden: bool,
    #[bpaf(long)]
    visible: bool,
}

#[test]
fn hide_still_parses() {
    let parser = hide_attr();

    // Hidden option still works
    let r = parser.run_inner(&["--hidden"]).unwrap();
    assert!(r.hidden);
}

// =============================================================================
// Hide usage attribute
// =============================================================================

#[derive(Debug, Clone, PartialEq, bpaf_unsynn::Bpaf)]
#[bpaf(options)]
struct HideUsageAttr {
    #[bpaf(long, hide_usage)]
    internal: bool,
}

#[test]
fn hide_usage_still_parses() {
    let parser = hide_usage_attr();
    let r = parser.run_inner(&["--internal"]).unwrap();
    assert!(r.internal);
}

// =============================================================================
// Fallback attribute
// =============================================================================

#[derive(Debug, Clone, PartialEq, bpaf_unsynn::Bpaf)]
#[bpaf(options)]
struct FallbackAttr {
    #[bpaf(long, fallback(42))]
    value: i32,
}

#[test]
fn fallback_provides_default() {
    let parser = fallback_attr();

    // Without arg, uses fallback
    let r = parser.run_inner(&[]).unwrap();
    assert_eq!(r.value, 42);

    // With arg, uses provided value
    let r = parser.run_inner(&["--value", "100"]).unwrap();
    assert_eq!(r.value, 100);
}

// =============================================================================
// FallbackWith attribute
// =============================================================================

fn get_default() -> Result<i32, &'static str> {
    Ok(99)
}

#[derive(Debug, Clone, PartialEq, bpaf_unsynn::Bpaf)]
#[bpaf(options)]
struct FallbackWithAttr {
    #[bpaf(long, fallback_with(get_default))]
    value: i32,
}

#[test]
fn fallback_with_calls_function() {
    let parser = fallback_with_attr();

    let r = parser.run_inner(&[]).unwrap();
    assert_eq!(r.value, 99);
}

// =============================================================================
// GroupHelp attribute
// =============================================================================

#[derive(Debug, Clone, PartialEq, bpaf_unsynn::Bpaf)]
#[bpaf(options)]
struct GroupHelpAttr {
    #[bpaf(long, group_help("Output options:"))]
    verbose: bool,
}

#[test]
fn group_help_compiles() {
    let parser = group_help_attr();
    let r = parser.run_inner(&["--verbose"]).unwrap();
    assert!(r.verbose);
}

// =============================================================================
// DebugFallback attribute
// =============================================================================

#[derive(Debug, Clone, PartialEq, bpaf_unsynn::Bpaf)]
#[bpaf(options)]
struct DebugFallbackAttr {
    #[bpaf(long, fallback(42), debug_fallback)]
    value: i32,
}

#[test]
fn debug_fallback_compiles() {
    let parser = debug_fallback_attr();
    let r = parser.run_inner(&[]).unwrap();
    assert_eq!(r.value, 42);
}

// =============================================================================
// DisplayFallback attribute
// =============================================================================

#[derive(Debug, Clone, PartialEq, bpaf_unsynn::Bpaf)]
#[bpaf(options)]
struct DisplayFallbackAttr {
    #[bpaf(long, fallback(42), display_fallback)]
    value: i32,
}

#[test]
fn display_fallback_compiles() {
    let parser = display_fallback_attr();
    let r = parser.run_inner(&[]).unwrap();
    assert_eq!(r.value, 42);
}

// =============================================================================
// FormatFallback attribute
// =============================================================================

#[derive(Debug, Clone, PartialEq)]
struct CustomType(String);

impl std::str::FromStr for CustomType {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(CustomType(s.to_string()))
    }
}

fn format_custom_type(val: &CustomType, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "custom({})", val.0)
}

#[derive(Debug, Clone, PartialEq, bpaf_unsynn::Bpaf)]
#[bpaf(options)]
struct FormatFallbackAttr {
    #[bpaf(long, argument("VAL"), fallback(CustomType("default".to_string())), format_fallback(format_custom_type))]
    value: CustomType,
}

#[test]
fn format_fallback_compiles() {
    let parser = format_fallback_attr();
    let r = parser.run_inner(&[]).unwrap();
    assert_eq!(r.value.0, "default");
}

// =============================================================================
// Last attribute - uses last occurrence when specified multiple times
// =============================================================================

#[derive(Debug, Clone, PartialEq, bpaf_unsynn::Bpaf)]
#[bpaf(options)]
struct LastAttr {
    #[bpaf(long, argument("VAL"), last)]
    value: String,
}

#[test]
fn last_compiles_and_parses() {
    let parser = last_attr();
    let r = parser.run_inner(&["--value", "test"]).unwrap();
    assert_eq!(r.value, "test");
}

#[test]
fn last_uses_last_occurrence() {
    let parser = last_attr();
    let r = parser
        .run_inner(&["--value", "first", "--value", "second", "--value", "last"])
        .unwrap();
    assert_eq!(r.value, "last");
}

#[derive(Debug, Clone, PartialEq, bpaf_unsynn::Bpaf)]
#[bpaf(options)]
struct LastWithShort {
    #[bpaf(short('o'), long, argument("FILE"), last)]
    output: String,
}

#[test]
fn last_with_mixed_short_long() {
    let parser = last_with_short();
    let r = parser
        .run_inner(&["-o", "first", "--output", "second", "-o", "last"])
        .unwrap();
    assert_eq!(r.output, "last");
}

#[derive(Debug, Clone, PartialEq, bpaf_unsynn::Bpaf)]
#[bpaf(options)]
struct LastNumeric {
    #[bpaf(short('n'), long, argument("NUM"), last)]
    number: usize,
}

#[test]
fn last_with_numeric_type() {
    let parser = last_numeric();
    let r = parser
        .run_inner(&["-n", "10", "-n", "20", "--number", "30"])
        .unwrap();
    assert_eq!(r.number, 30);
}

// =============================================================================
// Complete attribute (compilation test)
// =============================================================================

fn complete_fn(_input: &String) -> Vec<(&'static str, Option<&'static str>)> {
    vec![
        ("option1", Some("First option")),
        ("option2", Some("Second option")),
    ]
}

#[derive(Debug, Clone, PartialEq, bpaf_unsynn::Bpaf)]
struct CompleteAttr {
    #[bpaf(long, argument("VAL"), complete(complete_fn))]
    value: String,
}

#[test]
fn complete_compiles() {
    let _ = complete_attr();
}

// =============================================================================
// CustomUsage attribute
// =============================================================================

#[derive(Debug, Clone, PartialEq, bpaf_unsynn::Bpaf)]
#[bpaf(options)]
struct CustomUsageAttr {
    #[bpaf(long, custom_usage("VALUE"))]
    value: Option<String>,
}

#[test]
fn field_custom_usage_compiles() {
    let parser = custom_usage_attr();
    let r = parser.run_inner(&[]).unwrap();
    assert_eq!(r.value, None);
}
