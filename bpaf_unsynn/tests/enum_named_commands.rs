//! Tests for enum named commands with #[bpaf(command("custom-name"))]

// ============================================================================
// Basic command with custom name
// ============================================================================

#[derive(Debug, Clone, PartialEq, bpaf_unsynn::Bpaf)]
#[bpaf(options)]
enum Command {
    /// Build the project
    #[bpaf(command("compile"))]
    Build,

    /// Run tests
    #[bpaf(command)]
    Test,
}

#[test]
fn named_command_using_custom_name() {
    let parser = Command::parse();

    // Should work with custom command name
    let result = parser.run_inner(&["compile"]);
    assert!(result.is_ok(), "Should parse 'compile': {:?}", result);
    assert_eq!(result.unwrap(), Command::Build);
}

#[test]
fn named_command_original_name_should_not_work() {
    let parser = Command::parse();

    // Original variant name should NOT work when custom name is specified
    let result = parser.run_inner(&["build"]);
    assert!(result.is_err(), "Should not parse 'build' when custom name is set");
}

#[test]
fn unnamed_command_works_normally() {
    let parser = Command::parse();

    // Command without custom name should use kebab-case variant name
    let result = parser.run_inner(&["test"]);
    assert!(result.is_ok(), "Should parse 'test': {:?}", result);
    assert_eq!(result.unwrap(), Command::Test);
}

// ============================================================================
// Command with custom name and aliases
// ============================================================================

#[derive(Debug, Clone, PartialEq, bpaf_unsynn::Bpaf)]
#[bpaf(options)]
enum Action {
    /// Add a new item
    #[bpaf(command("create"), long("insert"), short('a'))]
    Add(String),

    /// Remove an item
    #[bpaf(command("remove"))]
    Delete(String),
}

#[test]
fn custom_name_with_aliases_using_custom_name() {
    let parser = Action::parse();

    let result = parser.run_inner(&["create", "item1"]);
    assert!(result.is_ok(), "Should parse 'create item1': {:?}", result);
    assert_eq!(result.unwrap(), Action::Add("item1".to_string()));
}

#[test]
fn custom_name_with_aliases_using_long() {
    let parser = Action::parse();

    let result = parser.run_inner(&["insert", "item2"]);
    assert!(result.is_ok(), "Should parse 'insert item2': {:?}", result);
    assert_eq!(result.unwrap(), Action::Add("item2".to_string()));
}

#[test]
fn custom_name_with_aliases_using_short() {
    let parser = Action::parse();

    let result = parser.run_inner(&["a", "item3"]);
    assert!(result.is_ok(), "Should parse 'a item3': {:?}", result);
    assert_eq!(result.unwrap(), Action::Add("item3".to_string()));
}

#[test]
fn custom_name_with_aliases_original_should_not_work() {
    let parser = Action::parse();

    // Original variant name should not work
    let result = parser.run_inner(&["add", "item4"]);
    assert!(result.is_err(), "Should not parse 'add' when custom name is 'create'");
}

#[test]
fn custom_name_delete() {
    let parser = Action::parse();

    let result = parser.run_inner(&["remove", "item5"]);
    assert!(result.is_ok(), "Should parse 'remove item5': {:?}", result);
    assert_eq!(result.unwrap(), Action::Delete("item5".to_string()));
}

// ============================================================================
// Struct variant with custom command name
// ============================================================================

#[derive(Debug, Clone, PartialEq, bpaf_unsynn::Bpaf)]
#[bpaf(options)]
enum Task {
    /// Deploy to target
    #[bpaf(command("publish"))]
    Deploy {
        #[bpaf(long)]
        target: String,
    },

    /// Show status
    #[bpaf(command("info"))]
    Status,
}

#[test]
fn struct_variant_custom_name() {
    let parser = Task::parse();

    let result = parser.run_inner(&["publish", "--target", "production"]);
    assert!(result.is_ok(), "Should parse custom name: {:?}", result);
    assert_eq!(result.unwrap(), Task::Deploy { target: "production".to_string() });
}

#[test]
fn unit_variant_custom_name() {
    let parser = Task::parse();

    let result = parser.run_inner(&["info"]);
    assert!(result.is_ok(), "Should parse custom name: {:?}", result);
    assert_eq!(result.unwrap(), Task::Status);
}

// ============================================================================
// Multiple variants with mixed custom and default names
// ============================================================================

#[derive(Debug, Clone, PartialEq, bpaf_unsynn::Bpaf)]
#[bpaf(options)]
enum Operation {
    /// Initialize project
    #[bpaf(command("setup"))]
    Init,

    /// Build project (uses default kebab-case)
    #[bpaf(command)]
    BuildProject(String),

    /// Clean artifacts
    #[bpaf(command("purge"))]
    Clean {
        #[bpaf(long)]
        all: bool,
    },

    /// Show help (uses default kebab-case)
    #[bpaf(command)]
    Help,
}

#[test]
fn mixed_names_custom_init() {
    let parser = Operation::parse();

    let result = parser.run_inner(&["setup"]);
    assert_eq!(result.unwrap(), Operation::Init);

    // Original name should not work
    let result = parser.run_inner(&["init"]);
    assert!(result.is_err());
}

#[test]
fn mixed_names_default_build() {
    let parser = Operation::parse();

    // Should use kebab-case of variant name
    let result = parser.run_inner(&["build-project", "debug"]);
    assert_eq!(result.unwrap(), Operation::BuildProject("debug".to_string()));
}

#[test]
fn mixed_names_custom_clean() {
    let parser = Operation::parse();

    let result = parser.run_inner(&["purge", "--all"]);
    assert_eq!(result.unwrap(), Operation::Clean { all: true });

    // Original name should not work
    let result = parser.run_inner(&["clean", "--all"]);
    assert!(result.is_err());
}

#[test]
fn mixed_names_default_help() {
    let parser = Operation::parse();

    // Should use kebab-case of variant name
    let result = parser.run_inner(&["help"]);
    assert_eq!(result.unwrap(), Operation::Help);
}

// ============================================================================
// Edge cases: names with special characters
// ============================================================================

#[derive(Debug, Clone, PartialEq, bpaf_unsynn::Bpaf)]
#[bpaf(options)]
enum Special {
    /// Command with dash
    #[bpaf(command("do-something"))]
    DoSomething,

    /// Command with underscore (uncommon but valid)
    #[bpaf(command("run_test"))]
    RunTest,
}

#[test]
fn custom_name_with_dash() {
    let parser = Special::parse();

    let result = parser.run_inner(&["do-something"]);
    assert!(result.is_ok(), "Should parse 'do-something': {:?}", result);
    assert_eq!(result.unwrap(), Special::DoSomething);
}

#[test]
fn custom_name_with_underscore() {
    let parser = Special::parse();

    let result = parser.run_inner(&["run_test"]);
    assert!(result.is_ok(), "Should parse 'run_test': {:?}", result);
    assert_eq!(result.unwrap(), Special::RunTest);
}
