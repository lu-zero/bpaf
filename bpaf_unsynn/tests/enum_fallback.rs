//! Tests for enum-level fallback attribute

use bpaf::Parser;

// ============================================================================
// Basic enum with fallback
// ============================================================================

#[derive(Debug, Clone, PartialEq, bpaf_unsynn::Bpaf)]
#[bpaf(fallback(Decision::No))]
enum Decision {
    #[bpaf(command)]
    Yes,
    #[bpaf(command)]
    No,
}

#[test]
fn enum_fallback_basic() {
    let parser = Decision::parse().to_options();

    // When no arguments provided, should use fallback
    let result = parser.run_inner(&[] as &[&str]);
    assert!(result.is_ok(), "Should succeed with fallback: {:?}", result);
    assert_eq!(result.unwrap(), Decision::No);
}

#[test]
fn enum_fallback_explicit_value() {
    let parser = Decision::parse().to_options();

    // When explicit value provided, should use it (enum variants are commands, not flags)
    let result = parser.run_inner(&["yes"]);
    assert!(result.is_ok(), "Should parse explicit value: {:?}", result);
    assert_eq!(result.unwrap(), Decision::Yes);
}

// ============================================================================
// Enum with fallback and options mode
// ============================================================================

#[derive(Debug, Clone, PartialEq, bpaf_unsynn::Bpaf)]
#[bpaf(options, fallback(Action::Help))]
enum Action {
    #[bpaf(command)]
    Run,
    #[bpaf(command)]
    Help,
}

#[test]
fn enum_fallback_with_options() {
    let parser = Action::parse();

    // When no arguments provided, should use fallback
    let result = parser.run_inner(&[] as &[&str]);
    assert!(result.is_ok(), "Should succeed with fallback: {:?}", result);
    assert_eq!(result.unwrap(), Action::Help);
}

#[test]
fn enum_fallback_with_options_explicit() {
    let parser = Action::parse();

    // When explicit value provided, should use it (enum variants are commands, not flags)
    let result = parser.run_inner(&["run"]);
    assert!(result.is_ok(), "Should parse explicit value: {:?}", result);
    assert_eq!(result.unwrap(), Action::Run);
}

// ============================================================================
// Complex enum with fallback
// ============================================================================

#[derive(Debug, Clone, PartialEq, bpaf_unsynn::Bpaf)]
#[bpaf(fallback(Status::Pending))]
enum Status {
    #[bpaf(command)]
    Active,
    #[bpaf(command)]
    Pending,
    #[bpaf(command)]
    Completed,
}

#[test]
fn enum_fallback_complex() {
    let parser = Status::parse().to_options();

    // Default to Pending
    let result = parser.run_inner(&[] as &[&str]);
    assert!(result.is_ok(), "Should succeed with fallback: {:?}", result);
    assert_eq!(result.unwrap(), Status::Pending);
}

#[test]
fn enum_fallback_complex_active() {
    let parser = Status::parse().to_options();

    let result = parser.run_inner(&["active"]);
    assert!(result.is_ok(), "Should parse Active: {:?}", result);
    assert_eq!(result.unwrap(), Status::Active);
}

#[test]
fn enum_fallback_complex_completed() {
    let parser = Status::parse().to_options();

    let result = parser.run_inner(&["completed"]);
    assert!(result.is_ok(), "Should parse Completed: {:?}", result);
    assert_eq!(result.unwrap(), Status::Completed);
}

// ============================================================================
// Struct with fallback (should also work)
// ============================================================================

#[derive(Debug, Clone, PartialEq, bpaf_unsynn::Bpaf)]
#[bpaf(fallback(Config { verbose: false }))]
struct Config {
    #[bpaf(short, long, switch)]
    verbose: bool,
}

#[test]
fn struct_fallback_basic() {
    let parser = Config::parse().to_options();

    // When no arguments provided, should use fallback
    let result = parser.run_inner(&[] as &[&str]);
    assert!(result.is_ok(), "Should succeed with fallback: {:?}", result);
    assert_eq!(result.unwrap(), Config { verbose: false });
}

#[test]
fn struct_fallback_explicit() {
    let parser = Config::parse().to_options();

    // When explicit value provided, should use it
    let result = parser.run_inner(&["-v"]);
    assert!(result.is_ok(), "Should parse explicit value: {:?}", result);
    assert_eq!(result.unwrap(), Config { verbose: true });
}

// ============================================================================
// Fallback with complex expression
// ============================================================================

#[derive(Debug, Clone, PartialEq, bpaf_unsynn::Bpaf)]
#[bpaf(fallback(Priority::from_default()))]
enum Priority {
    #[bpaf(command)]
    Low,
    #[bpaf(command)]
    Medium,
    #[bpaf(command)]
    High,
}

impl Priority {
    fn from_default() -> Self {
        Priority::Medium
    }
}

#[test]
fn enum_fallback_with_function() {
    let parser = Priority::parse().to_options();

    // Should call Priority::from_default()
    let result = parser.run_inner(&[] as &[&str]);
    assert!(result.is_ok(), "Should succeed with fallback: {:?}", result);
    assert_eq!(result.unwrap(), Priority::Medium);
}

#[test]
fn enum_fallback_with_function_explicit() {
    let parser = Priority::parse().to_options();

    let result = parser.run_inner(&["high"]);
    assert!(result.is_ok(), "Should parse High: {:?}", result);
    assert_eq!(result.unwrap(), Priority::High);
}
