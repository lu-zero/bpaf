//! Tests for enum command aliases with #[bpaf(long("alias"))]

// ============================================================================
// Basic command with alias
// ============================================================================

#[derive(Debug, Clone, PartialEq, bpaf_unsynn::Bpaf)]
#[bpaf(options)]
enum Command {
    /// Run the build
    #[bpaf(command, long("compile"))]
    Build,

    /// Run tests
    #[bpaf(command)]
    Test,
}

#[test]
fn command_with_alias_using_command_name() {
    let parser = Command::parse();

    // Should work with standard command name
    let result = parser.run_inner(&["build"]);
    assert!(result.is_ok(), "Should parse 'build': {:?}", result);
    assert_eq!(result.unwrap(), Command::Build);
}

#[test]
fn command_with_alias_using_alias() {
    let parser = Command::parse();

    // Should also work with alias (no dashes - it's an alternative command name)
    let result = parser.run_inner(&["compile"]);
    assert!(result.is_ok(), "Should parse 'compile': {:?}", result);
    assert_eq!(result.unwrap(), Command::Build);
}

#[test]
fn command_without_alias() {
    let parser = Command::parse();

    // Command without alias should work normally
    let result = parser.run_inner(&["test"]);
    assert!(result.is_ok(), "Should parse 'test': {:?}", result);
    assert_eq!(result.unwrap(), Command::Test);
}

// ============================================================================
// Tuple variant with alias
// ============================================================================

#[derive(Debug, Clone, PartialEq, bpaf_unsynn::Bpaf)]
#[bpaf(options)]
enum Action {
    /// Add a new item
    #[bpaf(command, long("insert"))]
    Add(String),

    /// Remove an item
    #[bpaf(command, long("remove"))]
    Delete(String),
}

#[test]
fn tuple_variant_with_alias_using_command() {
    let parser = Action::parse();

    let result = parser.run_inner(&["add", "item1"]);
    assert!(result.is_ok(), "Should parse 'add item1': {:?}", result);
    assert_eq!(result.unwrap(), Action::Add("item1".to_string()));
}

#[test]
fn tuple_variant_with_alias_using_alias() {
    let parser = Action::parse();

    let result = parser.run_inner(&["insert", "item2"]);
    assert!(result.is_ok(), "Should parse 'insert item2': {:?}", result);
    assert_eq!(result.unwrap(), Action::Add("item2".to_string()));
}

// ============================================================================
// Struct variant with alias
// ============================================================================

#[derive(Debug, Clone, PartialEq, bpaf_unsynn::Bpaf)]
#[bpaf(options)]
enum Task {
    /// Deploy to target
    #[bpaf(command, long("release"))]
    Deploy {
        #[bpaf(long)]
        target: String,
    },

    /// Show status
    #[bpaf(command)]
    Status,
}

#[test]
fn struct_variant_with_alias_using_command() {
    let parser = Task::parse();

    let result = parser.run_inner(&["deploy", "--target", "production"]);
    assert!(result.is_ok(), "Should parse deploy command: {:?}", result);
    assert_eq!(result.unwrap(), Task::Deploy { target: "production".to_string() });
}

#[test]
fn struct_variant_with_alias_using_alias() {
    let parser = Task::parse();

    let result = parser.run_inner(&["release", "--target", "staging"]);
    assert!(result.is_ok(), "Should parse release alias: {:?}", result);
    assert_eq!(result.unwrap(), Task::Deploy { target: "staging".to_string() });
}

// ============================================================================
// Mixed variants with multiple aliases
// ============================================================================

#[derive(Debug, Clone, PartialEq, bpaf_unsynn::Bpaf)]
#[bpaf(options)]
enum Operation {
    /// Initialize project
    #[bpaf(command, long("init-project"))]
    Init,

    /// Build project
    #[bpaf(command, long("compile"))]
    Build(String),

    /// Clean artifacts
    #[bpaf(command, long("clear"))]
    Clean {
        #[bpaf(long)]
        all: bool,
    },

    /// Show help
    #[bpaf(command)]
    Help,
}

#[test]
fn multiple_aliases_init() {
    let parser = Operation::parse();

    let result = parser.run_inner(&["init"]);
    assert_eq!(result.unwrap(), Operation::Init);

    let result = parser.run_inner(&["init-project"]);
    assert_eq!(result.unwrap(), Operation::Init);
}

#[test]
fn multiple_aliases_build() {
    let parser = Operation::parse();

    let result = parser.run_inner(&["build", "debug"]);
    assert_eq!(result.unwrap(), Operation::Build("debug".to_string()));

    let result = parser.run_inner(&["compile", "release"]);
    assert_eq!(result.unwrap(), Operation::Build("release".to_string()));
}

#[test]
fn multiple_aliases_clean() {
    let parser = Operation::parse();

    let result = parser.run_inner(&["clean", "--all"]);
    assert_eq!(result.unwrap(), Operation::Clean { all: true });

    let result = parser.run_inner(&["clear", "--all"]);
    assert_eq!(result.unwrap(), Operation::Clean { all: true });
}

#[test]
fn no_alias_command() {
    let parser = Operation::parse();

    let result = parser.run_inner(&["help"]);
    assert_eq!(result.unwrap(), Operation::Help);
}
