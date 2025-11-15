//! Test enum support

use bpaf::Parser;

// Test simple enum with unit variants
#[derive(Debug, Clone, PartialEq)]
#[derive(bpaf_unsynn::Bpaf)]
enum Command {
    #[bpaf(command)]
    Build,
    #[bpaf(command)]
    Test,
    #[bpaf(command)]
    Run,
}

#[test]
fn simple_enum_works() {
    let parser = Command::parse().to_options();

    // Test build command
    let result = parser.run_inner(&["build"]);
    assert!(result.is_ok());
    let cmd = result.unwrap();
    assert_eq!(cmd, Command::Build);

    // Test test command
    let result = parser.run_inner(&["test"]);
    assert!(result.is_ok());
    let cmd = result.unwrap();
    assert_eq!(cmd, Command::Test);

    // Test run command
    let result = parser.run_inner(&["run"]);
    assert!(result.is_ok());
    let cmd = result.unwrap();
    assert_eq!(cmd, Command::Run);
}

// Test enum with fields
#[derive(Debug, Clone, PartialEq)]
#[derive(bpaf_unsynn::Bpaf)]
enum Action {
    #[bpaf(command)]
    Build {
        #[bpaf(long)]
        release: bool,
    },
    #[bpaf(command)]
    Run {
        #[bpaf(long)]
        file: String,
    },
}

#[test]
fn enum_with_fields_works() {
    let parser = Action::parse().to_options();

    // Test build with release flag
    let result = parser.run_inner(&["build", "--release"]);
    assert!(result.is_ok());
    let action = result.unwrap();
    assert_eq!(action, Action::Build { release: true });

    // Test build without release flag
    let result = parser.run_inner(&["build"]);
    assert!(result.is_ok());
    let action = result.unwrap();
    assert_eq!(action, Action::Build { release: false });

    // Test run with file
    let result = parser.run_inner(&["run", "--file", "main.rs"]);
    assert!(result.is_ok());
    let action = result.unwrap();
    assert_eq!(action, Action::Run { file: "main.rs".to_string() });
}

// Test mixed enum
#[derive(Debug, Clone, PartialEq)]
#[derive(bpaf_unsynn::Bpaf)]
enum MixedCommand {
    #[bpaf(command)]
    Init,
    #[bpaf(command)]
    Deploy {
        #[bpaf(long)]
        target: String,
        #[bpaf(long)]
        verbose: bool,
    },
    #[bpaf(command)]
    Status,
}

#[test]
fn mixed_enum_works() {
    let parser = MixedCommand::parse().to_options();

    // Test unit variant
    let result = parser.run_inner(&["init"]);
    assert!(result.is_ok());
    let cmd = result.unwrap();
    assert_eq!(cmd, MixedCommand::Init);

    // Test variant with fields
    let result = parser.run_inner(&["deploy", "--target", "prod", "--verbose"]);
    assert!(result.is_ok());
    let cmd = result.unwrap();
    assert_eq!(cmd, MixedCommand::Deploy {
        target: "prod".to_string(),
        verbose: true
    });

    // Test another unit variant
    let result = parser.run_inner(&["status"]);
    assert!(result.is_ok());
    let cmd = result.unwrap();
    assert_eq!(cmd, MixedCommand::Status);
}
