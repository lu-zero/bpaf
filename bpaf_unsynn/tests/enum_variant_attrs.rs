//! Tests for variant-level attributes (command, skip, doc comments)

use bpaf::Parser;

// ============================================================================
// Basic enum with #[bpaf(command)] on variants
// ============================================================================

#[derive(Debug, Clone, PartialEq, bpaf_unsynn::Bpaf)]
#[bpaf(options)]
enum Action {
    /// Run the application
    #[bpaf(command)]
    Run,

    /// Stop the application
    #[bpaf(command)]
    Stop,
}

#[test]
fn enum_with_command_variants_compiles() {
    let parser = Action::parse();

    // Test that it compiles and can parse commands
    let result = parser.run_inner(&["run"]);
    assert!(result.is_ok(), "Should parse run command: {:?}", result);
    assert_eq!(result.unwrap(), Action::Run);
}

#[test]
fn enum_command_stop() {
    let parser = Action::parse();

    let result = parser.run_inner(&["stop"]);
    assert!(result.is_ok(), "Should parse stop command: {:?}", result);
    assert_eq!(result.unwrap(), Action::Stop);
}

// ============================================================================
// Enum with skip attribute
// ============================================================================

#[derive(Debug, Clone, PartialEq, bpaf_unsynn::Bpaf)]
enum Status {
    Active,
    #[allow(dead_code)] // Skip variant is intentionally not constructed
    #[bpaf(skip)]
    Internal,
    Pending,
}

#[test]
fn enum_with_skip_compiles() {
    let parser = Status::parse().to_options();

    // Should parse Active (without #[bpaf(command)], variants are flags)
    let result = parser.run_inner(&["--active"]);
    assert!(result.is_ok(), "Should parse active: {:?}", result);
    assert_eq!(result.unwrap(), Status::Active);
}

#[test]
fn enum_skip_variant_not_available() {
    let parser = Status::parse().to_options();

    // Internal should not be available as a flag
    let result = parser.run_inner(&["--internal"]);
    assert!(result.is_err(), "Should not parse skipped variant");
}

// ============================================================================
// Enum with doc comments on variants
// ============================================================================

#[derive(Debug, Clone, PartialEq, bpaf_unsynn::Bpaf)]
#[bpaf(options)]
enum Operation {
    /// Add a new item to the database
    #[bpaf(command)]
    Add,

    /// Remove an item from the database
    #[bpaf(command)]
    Remove,

    /// List all items in the database
    #[bpaf(command)]
    List,
}

#[test]
fn enum_with_doc_comments_compiles() {
    let parser = Operation::parse();

    // Just test that it compiles and can parse
    let result = parser.run_inner(&["add"]);
    assert!(result.is_ok(), "Should parse add: {:?}", result);
    assert_eq!(result.unwrap(), Operation::Add);
}

#[test]
fn enum_doc_comments_all_variants() {
    let parser = Operation::parse();

    // Test all variants
    let result = parser.run_inner(&["remove"]);
    assert!(result.is_ok(), "Should parse remove: {:?}", result);
    assert_eq!(result.unwrap(), Operation::Remove);

    let result = parser.run_inner(&["list"]);
    assert!(result.is_ok(), "Should parse list: {:?}", result);
    assert_eq!(result.unwrap(), Operation::List);
}

// ============================================================================
// Mixed: command + doc comments + skip
// ============================================================================

#[derive(Debug, Clone, PartialEq, bpaf_unsynn::Bpaf)]
#[bpaf(options)]
enum Command {
    /// Start the service
    #[bpaf(command)]
    Start,

    /// Stop the service
    #[bpaf(command)]
    Stop,

    #[allow(dead_code)] // Skip variant is intentionally not constructed
    #[bpaf(skip)]
    Internal,

    /// Restart the service
    #[bpaf(command)]
    Restart,
}

#[test]
fn mixed_attributes_start() {
    let parser = Command::parse();

    let result = parser.run_inner(&["start"]);
    assert!(result.is_ok(), "Should parse start: {:?}", result);
    assert_eq!(result.unwrap(), Command::Start);
}

#[test]
fn mixed_attributes_restart() {
    let parser = Command::parse();

    let result = parser.run_inner(&["restart"]);
    assert!(result.is_ok(), "Should parse restart: {:?}", result);
    assert_eq!(result.unwrap(), Command::Restart);
}

#[test]
fn mixed_attributes_skip_not_available() {
    let parser = Command::parse();

    // Internal should not be available
    let result = parser.run_inner(&["internal"]);
    assert!(result.is_err(), "Skipped variant should not be available");
}

// ============================================================================
// Multi-line doc comments
// ============================================================================

#[derive(Debug, Clone, PartialEq, bpaf_unsynn::Bpaf)]
#[bpaf(options)]
enum Task {
    /// Build the project
    /// This will compile all source files
    #[bpaf(command)]
    Build,

    /// Run tests
    /// Executes the test suite
    #[bpaf(command)]
    Test,
}

#[test]
fn multiline_doc_comments_build() {
    let parser = Task::parse();

    let result = parser.run_inner(&["build"]);
    assert!(result.is_ok(), "Should parse build: {:?}", result);
    assert_eq!(result.unwrap(), Task::Build);
}

#[test]
fn multiline_doc_comments_test() {
    let parser = Task::parse();

    let result = parser.run_inner(&["test"]);
    assert!(result.is_ok(), "Should parse test: {:?}", result);
    assert_eq!(result.unwrap(), Task::Test);
}
