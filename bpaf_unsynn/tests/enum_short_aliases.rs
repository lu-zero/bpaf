//! Tests for enum command short aliases with #[bpaf(short('x'))]

// ============================================================================
// Basic command with short alias
// ============================================================================

#[derive(Debug, Clone, PartialEq, bpaf_unsynn::Bpaf)]
#[bpaf(options)]
enum Command {
    /// Build the project
    #[bpaf(command, short('b'))]
    Build,

    /// Run tests
    #[bpaf(command)]
    Test,
}

#[test]
fn command_with_short_using_command_name() {
    let parser = Command::parse();

    // Should work with standard command name
    let result = parser.run_inner(&["build"]);
    assert!(result.is_ok(), "Should parse 'build': {:?}", result);
    assert_eq!(result.unwrap(), Command::Build);
}

#[test]
fn command_with_short_using_alias() {
    let parser = Command::parse();

    // Should also work with short alias (no dashes - it's a command)
    let result = parser.run_inner(&["b"]);
    assert!(result.is_ok(), "Should parse 'b': {:?}", result);
    assert_eq!(result.unwrap(), Command::Build);
}

#[test]
fn command_without_short() {
    let parser = Command::parse();

    // Command without alias should work normally
    let result = parser.run_inner(&["test"]);
    assert!(result.is_ok(), "Should parse 'test': {:?}", result);
    assert_eq!(result.unwrap(), Command::Test);
}

// ============================================================================
// Command with both long and short aliases
// ============================================================================

#[derive(Debug, Clone, PartialEq, bpaf_unsynn::Bpaf)]
#[bpaf(options)]
enum Action {
    /// Add a new item
    #[bpaf(command, long("insert"), short('a'))]
    Add(String),

    /// Remove an item
    #[bpaf(command, long("remove"), short('d'))]
    Delete(String),
}

#[test]
fn both_aliases_using_command() {
    let parser = Action::parse();

    let result = parser.run_inner(&["add", "item1"]);
    assert!(result.is_ok(), "Should parse 'add item1': {:?}", result);
    assert_eq!(result.unwrap(), Action::Add("item1".to_string()));
}

#[test]
fn both_aliases_using_long() {
    let parser = Action::parse();

    let result = parser.run_inner(&["insert", "item2"]);
    assert!(result.is_ok(), "Should parse 'insert item2': {:?}", result);
    assert_eq!(result.unwrap(), Action::Add("item2".to_string()));
}

#[test]
fn both_aliases_using_short() {
    let parser = Action::parse();

    let result = parser.run_inner(&["a", "item3"]);
    assert!(result.is_ok(), "Should parse 'a item3': {:?}", result);
    assert_eq!(result.unwrap(), Action::Add("item3".to_string()));
}

#[test]
fn delete_with_short() {
    let parser = Action::parse();

    let result = parser.run_inner(&["d", "item4"]);
    assert!(result.is_ok(), "Should parse 'd item4': {:?}", result);
    assert_eq!(result.unwrap(), Action::Delete("item4".to_string()));
}

// ============================================================================
// Struct variant with short alias
// ============================================================================

#[derive(Debug, Clone, PartialEq, bpaf_unsynn::Bpaf)]
#[bpaf(options)]
enum Task {
    /// Deploy to target
    #[bpaf(command, short('d'))]
    Deploy {
        #[bpaf(long)]
        target: String,
    },

    /// Show status
    #[bpaf(command, short('s'))]
    Status,
}

#[test]
fn struct_variant_short_alias() {
    let parser = Task::parse();

    let result = parser.run_inner(&["d", "--target", "production"]);
    assert!(result.is_ok(), "Should parse short alias: {:?}", result);
    assert_eq!(result.unwrap(), Task::Deploy { target: "production".to_string() });
}

#[test]
fn unit_variant_short_alias() {
    let parser = Task::parse();

    let result = parser.run_inner(&["s"]);
    assert!(result.is_ok(), "Should parse short alias: {:?}", result);
    assert_eq!(result.unwrap(), Task::Status);
}

// ============================================================================
// Multiple variants with different short aliases
// ============================================================================

#[derive(Debug, Clone, PartialEq, bpaf_unsynn::Bpaf)]
#[bpaf(options)]
enum Operation {
    /// Initialize project
    #[bpaf(command, short('i'))]
    Init,

    /// Build project
    #[bpaf(command, long("compile"), short('b'))]
    Build(String),

    /// Clean artifacts
    #[bpaf(command, long("clear"), short('c'))]
    Clean {
        #[bpaf(long)]
        all: bool,
    },

    /// Show help
    #[bpaf(command, short('h'))]
    Help,
}

#[test]
fn multiple_short_init() {
    let parser = Operation::parse();

    let result = parser.run_inner(&["i"]);
    assert_eq!(result.unwrap(), Operation::Init);

    let result = parser.run_inner(&["init"]);
    assert_eq!(result.unwrap(), Operation::Init);
}

#[test]
fn multiple_short_build() {
    let parser = Operation::parse();

    let result = parser.run_inner(&["b", "debug"]);
    assert_eq!(result.unwrap(), Operation::Build("debug".to_string()));

    let result = parser.run_inner(&["build", "release"]);
    assert_eq!(result.unwrap(), Operation::Build("release".to_string()));

    let result = parser.run_inner(&["compile", "test"]);
    assert_eq!(result.unwrap(), Operation::Build("test".to_string()));
}

#[test]
fn multiple_short_clean() {
    let parser = Operation::parse();

    let result = parser.run_inner(&["c", "--all"]);
    assert_eq!(result.unwrap(), Operation::Clean { all: true });

    let result = parser.run_inner(&["clean", "--all"]);
    assert_eq!(result.unwrap(), Operation::Clean { all: true });

    let result = parser.run_inner(&["clear", "--all"]);
    assert_eq!(result.unwrap(), Operation::Clean { all: true });
}

#[test]
fn multiple_short_help() {
    let parser = Operation::parse();

    let result = parser.run_inner(&["h"]);
    assert_eq!(result.unwrap(), Operation::Help);

    let result = parser.run_inner(&["help"]);
    assert_eq!(result.unwrap(), Operation::Help);
}
