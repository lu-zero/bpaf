//! Tests for Option B features: Phases 4-5
//! - Phase 4: fallback_with, group_help, debug/display/format_fallback
//! - Phase 5: external, any, req_flag, pure, pure_with

use bpaf::Parser;

// ============================================================================
// Phase 4.1: fallback_with - Dynamic fallback
// ============================================================================

fn get_default_count() -> Result<i32, &'static str> {
    Ok(42)
}

#[derive(Debug, Clone, PartialEq)]
#[derive(bpaf_unsynn::Bpaf)]
struct WithFallbackWith {
    #[bpaf(long, fallback_with(get_default_count))]
    count: i32,
}

#[test]
fn fallback_with_uses_function() {
    let parser = WithFallbackWith::parse().to_options();

    // Without argument, should use fallback function
    let result = parser.run_inner(&[] as &[&str]);
    assert!(result.is_ok(), "Failed to parse: {:?}", result);
    let opts = result.unwrap();
    assert_eq!(opts.count, 42);
}

#[test]
fn fallback_with_overridden() {
    let parser = WithFallbackWith::parse().to_options();

    // With argument, should override fallback
    let result = parser.run_inner(&["--count", "100"]);
    assert!(result.is_ok(), "Failed to parse: {:?}", result);
    let opts = result.unwrap();
    assert_eq!(opts.count, 100);
}

// ============================================================================
// Phase 4.2: group_help - Help for option groups
// ============================================================================

#[derive(Debug, Clone)]
#[derive(bpaf_unsynn::Bpaf)]
#[allow(dead_code)]
struct WithGroupHelp {
    #[bpaf(long, group_help("Output Options"))]
    output: String,
}

#[test]
fn group_help_compiles() {
    let _parser = WithGroupHelp::parse().to_options();
    // Just verify it compiles - group_help affects help output
}

// ============================================================================
// Phase 4.3-4.5: Fallback formatting variants
// ============================================================================

#[derive(Debug, Clone, PartialEq)]
struct CustomType {
    value: i32,
}

impl std::fmt::Display for CustomType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Custom({})", self.value)
    }
}

impl std::str::FromStr for CustomType {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(CustomType { value: s.parse().map_err(|e| format!("{}", e))? })
    }
}

#[derive(Debug, Clone)]
#[derive(bpaf_unsynn::Bpaf)]
#[allow(dead_code)]
struct WithDebugFallback {
    #[bpaf(long, debug_fallback, fallback(CustomType { value: 10 }))]
    custom: CustomType,
}

#[test]
fn debug_fallback_compiles() {
    let _parser = WithDebugFallback::parse().to_options();
    // Just verify it compiles
}

#[derive(Debug, Clone)]
#[derive(bpaf_unsynn::Bpaf)]
#[allow(dead_code)]
struct WithDisplayFallback {
    #[bpaf(long, display_fallback, fallback(CustomType { value: 20 }))]
    custom: CustomType,
}

#[test]
fn display_fallback_compiles() {
    let _parser = WithDisplayFallback::parse().to_options();
    // Just verify it compiles
}

fn format_custom(c: &CustomType, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "formatted: {}", c.value)
}

#[derive(Debug, Clone)]
#[derive(bpaf_unsynn::Bpaf)]
#[allow(dead_code)]
struct WithFormatFallback {
    #[bpaf(long, format_fallback(format_custom), fallback(CustomType { value: 30 }))]
    custom: CustomType,
}

#[test]
fn format_fallback_compiles() {
    let _parser = WithFormatFallback::parse().to_options();
    // Just verify it compiles
}

// ============================================================================
// Phase 5.1: external - Reference external parser
// ============================================================================

fn other_parser() -> impl Parser<String> {
    ::bpaf::long("other").argument::<String>("OTHER")
}

#[derive(Debug, Clone, PartialEq)]
#[derive(bpaf_unsynn::Bpaf)]
struct WithExternal {
    #[bpaf(external(other_parser))]
    other: String,

    #[bpaf(long)]
    regular: bool,
}

#[test]
fn external_parser_works() {
    let parser = WithExternal::parse().to_options();

    let result = parser.run_inner(&["--other", "value", "--regular"]);
    assert!(result.is_ok(), "Failed to parse: {:?}", result);
    let opts = result.unwrap();
    assert_eq!(opts.other, "value");
    assert!(opts.regular);
}

#[test]
fn external_parser_required() {
    let parser = WithExternal::parse().to_options();

    // External parser is required
    let result = parser.run_inner(&["--regular"]);
    assert!(result.is_err(), "Should require external field");
}

// ============================================================================
// Phase 5.2: any - Custom validation with metavar
// ============================================================================

fn check_even(s: String) -> Option<i32> {
    s.parse::<i32>().ok().filter(|n| n % 2 == 0)
}

#[derive(Debug, Clone, PartialEq)]
#[derive(bpaf_unsynn::Bpaf)]
struct WithAny {
    #[bpaf(long, any("EVEN", check_even))]
    even_number: i32,
}

#[test]
fn any_accepts_valid() {
    let parser = WithAny::parse().to_options();

    let result = parser.run_inner(&["--even-number", "42"]);
    assert!(result.is_ok(), "Failed to parse even number: {:?}", result);
    let opts = result.unwrap();
    assert_eq!(opts.even_number, 42);
}

#[test]
fn any_rejects_odd() {
    let parser = WithAny::parse().to_options();

    let result = parser.run_inner(&["--even-number", "43"]);
    assert!(result.is_err(), "Should reject odd number");
}

#[test]
fn any_rejects_invalid() {
    let parser = WithAny::parse().to_options();

    let result = parser.run_inner(&["--even-number", "not_a_number"]);
    assert!(result.is_err(), "Should reject non-number");
}

// ============================================================================
// Phase 5.3: req_flag - Required flag
// ============================================================================

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
enum Mode {
    Fast,
    Safe,
}

#[derive(Debug, Clone, PartialEq)]
#[derive(bpaf_unsynn::Bpaf)]
struct WithReqFlag {
    #[bpaf(long("fast"), req_flag(Mode::Fast))]
    mode: Mode,
}

#[test]
fn req_flag_sets_value() {
    let parser = WithReqFlag::parse().to_options();

    let result = parser.run_inner(&["--fast"]);
    assert!(result.is_ok(), "Failed to parse: {:?}", result);
    let opts = result.unwrap();
    assert_eq!(opts.mode, Mode::Fast);
}

#[test]
fn req_flag_required() {
    let parser = WithReqFlag::parse().to_options();

    // req_flag is required
    let result = parser.run_inner(&[] as &[&str]);
    assert!(result.is_err(), "Should require flag");
}

// ============================================================================
// Phase 5.4: pure - Pure value (no parsing)
// ============================================================================

#[derive(Debug, Clone, PartialEq)]
#[derive(bpaf_unsynn::Bpaf)]
struct WithPure {
    #[bpaf(long)]
    name: String,

    #[bpaf(pure(42))]
    constant: i32,
}

#[test]
fn pure_provides_constant() {
    let parser = WithPure::parse().to_options();

    let result = parser.run_inner(&["--name", "test"]);
    assert!(result.is_ok(), "Failed to parse: {:?}", result);
    let opts = result.unwrap();
    assert_eq!(opts.name, "test");
    assert_eq!(opts.constant, 42);
}

// ============================================================================
// Phase 5.5: pure_with - Pure with function
// ============================================================================

fn get_timestamp() -> Result<i64, &'static str> {
    Ok(1234567890)
}

#[derive(Debug, Clone, PartialEq)]
#[derive(bpaf_unsynn::Bpaf)]
struct WithPureWith {
    #[bpaf(long)]
    name: String,

    #[bpaf(pure_with(get_timestamp))]
    timestamp: i64,
}

#[test]
fn pure_with_calls_function() {
    let parser = WithPureWith::parse().to_options();

    let result = parser.run_inner(&["--name", "test"]);
    assert!(result.is_ok(), "Failed to parse: {:?}", result);
    let opts = result.unwrap();
    assert_eq!(opts.name, "test");
    assert_eq!(opts.timestamp, 1234567890);
}

// ============================================================================
// Combined tests: Multiple Phase 4-5 attributes
// ============================================================================

#[derive(Debug, Clone, PartialEq)]
#[derive(bpaf_unsynn::Bpaf)]
struct CombinedPhase45 {
    #[bpaf(long, fallback_with(get_default_count))]
    count: i32,

    #[bpaf(long, any("EVEN", check_even))]
    even: i32,

    #[bpaf(pure(String::from("constant")))]
    constant: String,
}

#[test]
fn combined_phase_45_works() {
    let parser = CombinedPhase45::parse().to_options();

    // Use fallback for count, provide even
    let result = parser.run_inner(&["--even", "100"]);
    assert!(result.is_ok(), "Failed to parse: {:?}", result);
    let opts = result.unwrap();
    assert_eq!(opts.count, 42); // fallback
    assert_eq!(opts.even, 100);
    assert_eq!(opts.constant, "constant");
}

#[test]
fn combined_override_fallback() {
    let parser = CombinedPhase45::parse().to_options();

    // Override fallback
    let result = parser.run_inner(&["--count", "99", "--even", "200"]);
    assert!(result.is_ok(), "Failed to parse: {:?}", result);
    let opts = result.unwrap();
    assert_eq!(opts.count, 99);
    assert_eq!(opts.even, 200);
    assert_eq!(opts.constant, "constant");
}

// ============================================================================
// Integration with Option A features
// ============================================================================

#[derive(Debug, Clone, PartialEq)]
#[derive(bpaf_unsynn::Bpaf)]
struct IntegratedAllFeatures {
    // Option A: guard, map
    #[bpaf(long, map(|s: String| s.to_uppercase()), guard(|s: &String| !s.is_empty(), "cannot be empty"))]
    name: String,

    // Option B: fallback_with
    #[bpaf(long, fallback_with(get_default_count))]
    count: i32,

    // Option B: any
    #[bpaf(long, any("EVEN", check_even))]
    even: i32,

    // Option B: pure
    #[bpaf(pure(true))]
    flag: bool,
}

#[test]
fn integrated_all_features() {
    let parser = IntegratedAllFeatures::parse().to_options();

    let result = parser.run_inner(&["--name", "hello", "--even", "50"]);
    assert!(result.is_ok(), "Failed to parse: {:?}", result);
    let opts = result.unwrap();
    assert_eq!(opts.name, "HELLO"); // mapped to uppercase
    assert_eq!(opts.count, 42); // fallback_with
    assert_eq!(opts.even, 50); // any validation
    assert!(opts.flag); // pure
}

#[test]
fn integrated_guard_rejects() {
    let parser = IntegratedAllFeatures::parse().to_options();

    // Guard should reject empty string
    let result = parser.run_inner(&["--name", "", "--even", "50"]);
    assert!(result.is_err(), "Should reject empty name");
}

#[test]
fn integrated_any_rejects() {
    let parser = IntegratedAllFeatures::parse().to_options();

    // any should reject odd numbers
    let result = parser.run_inner(&["--name", "hello", "--even", "51"]);
    assert!(result.is_err(), "Should reject odd number");
}
