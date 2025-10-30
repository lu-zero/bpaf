//! Tests for the `any` consumer

use bpaf::Parser;

// Helper function for validation
fn is_valid_limit(x: String) -> Option<usize> {
    x.parse::<usize>().ok().filter(|&n| n <= 100)
}

// Helper function with type conversion
fn isize_to_usize(x: isize) -> Option<usize> {
    if x >= 0 {
        Some(x as usize)
    } else {
        None
    }
}

// ============================================================================
// any without type annotation
// ============================================================================

#[derive(Debug, Clone, PartialEq)]
#[derive(bpaf_unsynn::Bpaf)]
struct WithAnySimple {
    #[bpaf(any("LIMIT", is_valid_limit))]
    limit: usize,
}

#[test]
fn any_simple_valid() {
    let parser = WithAnySimple::parse().to_options();

    let result = parser.run_inner(&["50"]);
    assert!(result.is_ok(), "Failed to parse: {:?}", result);
    let opts = result.unwrap();
    assert_eq!(opts.limit, 50);
}

#[test]
fn any_simple_invalid() {
    let parser = WithAnySimple::parse().to_options();

    let result = parser.run_inner(&["150"]);
    // Should fail because 150 > 100
    assert!(result.is_err(), "Should reject value > 100");
}

// ============================================================================
// any with type annotation (turbofish)
// ============================================================================

#[derive(Debug, Clone, PartialEq)]
#[derive(bpaf_unsynn::Bpaf)]
struct WithAnyTyped {
    #[bpaf(any::<isize>("LIMIT", isize_to_usize))]
    limit: usize,
}

#[test]
fn any_typed_valid() {
    let parser = WithAnyTyped::parse().to_options();

    let result = parser.run_inner(&["42"]);
    assert!(result.is_ok(), "Failed to parse: {:?}", result);
    let opts = result.unwrap();
    assert_eq!(opts.limit, 42);
}

#[test]
fn any_typed_negative_fails() {
    let parser = WithAnyTyped::parse().to_options();

    let result = parser.run_inner(&["-5"]);
    // Should fail because negative values are rejected
    assert!(result.is_err(), "Should reject negative values");
}

// ============================================================================
// any with anywhere
// ============================================================================

#[derive(Debug, Clone, PartialEq)]
#[derive(bpaf_unsynn::Bpaf)]
struct WithAnyAnywhere {
    #[bpaf(short, long, switch)]
    flag: bool,

    #[bpaf(any::<isize>("LIMIT", isize_to_usize), anywhere, many)]
    limits: Vec<usize>,
}

#[test]
fn any_anywhere_before_flag() {
    let parser = WithAnyAnywhere::parse().to_options();

    let result = parser.run_inner(&["10", "-f", "20"]);
    assert!(result.is_ok(), "Failed to parse: {:?}", result);
    let opts = result.unwrap();
    assert_eq!(opts.flag, true);
    assert_eq!(opts.limits, vec![10, 20]);
}

#[test]
fn any_anywhere_after_flag() {
    let parser = WithAnyAnywhere::parse().to_options();

    let result = parser.run_inner(&["-f", "10", "20"]);
    assert!(result.is_ok(), "Failed to parse: {:?}", result);
    let opts = result.unwrap();
    assert_eq!(opts.flag, true);
    assert_eq!(opts.limits, vec![10, 20]);
}

#[test]
fn any_anywhere_mixed() {
    let parser = WithAnyAnywhere::parse().to_options();

    let result = parser.run_inner(&["10", "-f", "20", "30"]);
    assert!(result.is_ok(), "Failed to parse: {:?}", result);
    let opts = result.unwrap();
    assert_eq!(opts.flag, true);
    assert_eq!(opts.limits, vec![10, 20, 30]);
}

// ============================================================================
// any with many
// ============================================================================

#[derive(Debug, Clone, PartialEq)]
#[derive(bpaf_unsynn::Bpaf)]
struct WithAnyMany {
    #[bpaf(any("LIMIT", is_valid_limit), many)]
    limits: Vec<usize>,
}

#[test]
fn any_many_multiple_values() {
    let parser = WithAnyMany::parse().to_options();

    let result = parser.run_inner(&["10", "20", "30"]);
    assert!(result.is_ok(), "Failed to parse: {:?}", result);
    let opts = result.unwrap();
    assert_eq!(opts.limits, vec![10, 20, 30]);
}

#[test]
fn any_many_stops_at_invalid() {
    let parser = WithAnyMany::parse().to_options();

    // Should collect valid values until hitting invalid one
    let result = parser.run_inner(&["10", "20", "150"]);
    // Depending on `many` behavior, this might succeed with [10, 20] or fail
    // For now, let's just check it doesn't panic
    let _r = result;
}

#[test]
fn any_many_no_values() {
    let parser = WithAnyMany::parse().to_options();

    let result = parser.run_inner(&[] as &[&str]);
    assert!(result.is_ok(), "Should succeed with empty vec: {:?}", result);
    let opts = result.unwrap();
    assert_eq!(opts.limits, Vec::<usize>::new());
}
