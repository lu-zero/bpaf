//! Tests for tuple variant support

// ============================================================================
// Basic tuple variant (single field)
// ============================================================================

#[derive(Debug, Clone, PartialEq, bpaf_unsynn::Bpaf)]
#[bpaf(options)]
enum Action {
    /// Add a new item
    #[bpaf(command)]
    Add(String),

    /// Delete an item
    #[bpaf(command)]
    Delete(String),
}

#[test]
fn tuple_variant_basic() {
    let parser = Action::parse();

    // Test Add command with argument
    let result = parser.run_inner(&["add", "item1"]);
    assert!(result.is_ok(), "Should parse add command: {:?}", result);
    assert_eq!(result.unwrap(), Action::Add("item1".to_string()));
}

#[test]
fn tuple_variant_delete() {
    let parser = Action::parse();

    let result = parser.run_inner(&["delete", "item2"]);
    assert!(result.is_ok(), "Should parse delete command: {:?}", result);
    assert_eq!(result.unwrap(), Action::Delete("item2".to_string()));
}

// ============================================================================
// Tuple variant with different types
// ============================================================================

#[derive(Debug, Clone, PartialEq, bpaf_unsynn::Bpaf)]
#[bpaf(options)]
enum Command {
    /// Set count value
    #[bpaf(command)]
    SetCount(usize),

    /// Set name
    #[bpaf(command)]
    SetName(String),
}

#[test]
fn tuple_variant_usize() {
    let parser = Command::parse();

    let result = parser.run_inner(&["set-count", "42"]);
    assert!(result.is_ok(), "Should parse set-count: {:?}", result);
    assert_eq!(result.unwrap(), Command::SetCount(42));
}

#[test]
fn tuple_variant_string() {
    let parser = Command::parse();

    let result = parser.run_inner(&["set-name", "test"]);
    assert!(result.is_ok(), "Should parse set-name: {:?}", result);
    assert_eq!(result.unwrap(), Command::SetName("test".to_string()));
}

// ============================================================================
// Mixed: tuple and unit variants
// ============================================================================

#[derive(Debug, Clone, PartialEq, bpaf_unsynn::Bpaf)]
#[bpaf(options)]
enum MixedCommand {
    /// Initialize project
    #[bpaf(command)]
    Init,

    /// Deploy with target
    #[bpaf(command)]
    Deploy(String),

    /// Show status
    #[bpaf(command)]
    Status,
}

#[test]
fn mixed_tuple_unit_init() {
    let parser = MixedCommand::parse();

    let result = parser.run_inner(&["init"]);
    assert!(result.is_ok(), "Should parse init: {:?}", result);
    assert_eq!(result.unwrap(), MixedCommand::Init);
}

#[test]
fn mixed_tuple_unit_deploy() {
    let parser = MixedCommand::parse();

    let result = parser.run_inner(&["deploy", "production"]);
    assert!(result.is_ok(), "Should parse deploy: {:?}", result);
    assert_eq!(result.unwrap(), MixedCommand::Deploy("production".to_string()));
}

#[test]
fn mixed_tuple_unit_status() {
    let parser = MixedCommand::parse();

    let result = parser.run_inner(&["status"]);
    assert!(result.is_ok(), "Should parse status: {:?}", result);
    assert_eq!(result.unwrap(), MixedCommand::Status);
}

// NOTE: Option<T> and Vec<T> in tuple variants require more complex handling
// and will be added in a future iteration

// ============================================================================
// Tuple variant with fallback
// ============================================================================

#[derive(Debug, Clone, PartialEq, bpaf_unsynn::Bpaf)]
#[bpaf(options, fallback(Priority::Medium("default".to_string())))]
enum Priority {
    /// Low priority task
    #[bpaf(command)]
    Low(String),

    /// Medium priority task
    #[bpaf(command)]
    Medium(String),

    /// High priority task
    #[bpaf(command)]
    High(String),
}

#[test]
fn tuple_variant_with_fallback_explicit() {
    let parser = Priority::parse();

    let result = parser.run_inner(&["high", "urgent"]);
    assert!(result.is_ok(), "Should parse high: {:?}", result);
    assert_eq!(result.unwrap(), Priority::High("urgent".to_string()));
}

#[test]
fn tuple_variant_with_fallback_default() {
    let parser = Priority::parse();

    let result = parser.run_inner(&[] as &[&str]);
    assert!(result.is_ok(), "Should use fallback: {:?}", result);
    assert_eq!(result.unwrap(), Priority::Medium("default".to_string()));
}
