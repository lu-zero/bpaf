//! Compatibility tests from bpaf/tests/derive.rs
//! Tests adapted to use bpaf_unsynn instead of bpaf_derive

use bpaf::Parser;

#[test]
fn help_with_default_parse() {
    #[derive(Debug, Clone, bpaf_unsynn::Bpaf)]
    #[bpaf(options, fallback(Action::CheckConnection))]
    enum Action {
        /// Add a new TODO item
        #[bpaf(command)]
        Add(String),

        /// Test connection to the server
        #[bpaf(command)]
        CheckConnection,
    }

    if let Action::Add(value) = Action::Add(String::new()) {
        drop(value);
    }

    let parser = action();

    let help = parser
        .run_inner(&["add", "--help"])
        .unwrap_err()
        .unwrap_stdout();

    let expected_help = "\
Add a new TODO item

Usage: add ARG

Available options:
    -h, --help  Prints help information
";
    assert_eq!(expected_help, help);

    let help = parser.run_inner(&["--help"]).unwrap_err().unwrap_stdout();

    let expected_help = "\
Usage: [COMMAND ...]

Available options:
    -h, --help        Prints help information

Available commands:
    add               Add a new TODO item
    check-connection  Test connection to the server
";
    assert_eq!(expected_help, help);
}

#[test]
fn command_and_fallback() {
    #[derive(Debug, Clone, bpaf_unsynn::Bpaf)]
    enum Action {
        /// Add a new TODO item
        #[bpaf(command)]
        Add(String),

        /// Does nothing
        #[bpaf(command)]
        NoAction,
    }

    if let Action::Add(value) = Action::Add(String::new()) {
        drop(value);
    }

    let parser = action().fallback(Action::NoAction).to_options();

    let help = parser
        .run_inner(&["add", "--help"])
        .unwrap_err()
        .unwrap_stdout();

    let expected_help = "\
Add a new TODO item

Usage: add ARG

Available options:
    -h, --help  Prints help information
";
    assert_eq!(expected_help, help);

    let help = parser.run_inner(&["--help"]).unwrap_err().unwrap_stdout();

    let expected_help = "\
Usage: [COMMAND ...]

Available options:
    -h, --help  Prints help information

Available commands:
    add         Add a new TODO item
    no-action   Does nothing
";
    assert_eq!(expected_help, help);
}

#[test]
fn pure_optional() {
    #[derive(bpaf_unsynn::Bpaf, Debug, Clone)]
    #[bpaf(options)]
    struct Opts {
        #[bpaf(pure(Default::default()))]
        foo: Option<Vec<u32>>,
    }

    // pure(Default::default()) should return None for Option<Vec<u32>>
    // Previously, bpaf_unsynn incorrectly returned Some([]) due to implicit .optional() being applied
    let result = opts().run().foo;
    assert_eq!(result, None, "Expected None, got {:?}", result);
}

// Test that we can parse actual values, not just help
#[test]
fn command_parsing_works() {
    #[derive(Debug, Clone, PartialEq, bpaf_unsynn::Bpaf)]
    #[bpaf(options)]
    enum Action {
        /// Add item
        #[bpaf(command)]
        Add(String),

        /// Delete item
        #[bpaf(command)]
        Delete(String),
    }

    let parser = action();

    let result = parser.run_inner(&["add", "test"]);
    assert!(result.is_ok(), "Should parse add command: {:?}", result);
    assert_eq!(result.unwrap(), Action::Add("test".to_string()));

    let result = parser.run_inner(&["delete", "item"]);
    assert!(result.is_ok(), "Should parse delete command: {:?}", result);
    assert_eq!(result.unwrap(), Action::Delete("item".to_string()));
}

#[test]
fn fallback_works() {
    #[derive(Debug, Clone, PartialEq, bpaf_unsynn::Bpaf)]
    #[bpaf(options, fallback(Action::Help))]
    enum Action {
        #[bpaf(command)]
        Run,
        #[bpaf(command)]
        Help,
    }

    let parser = action();

    // No arguments - should use fallback
    let result = parser.run_inner(&[] as &[&str]);
    assert!(result.is_ok(), "Should use fallback: {:?}", result);
    assert_eq!(result.unwrap(), Action::Help);

    // Explicit command
    let result = parser.run_inner(&["run"]);
    assert!(result.is_ok(), "Should parse run: {:?}", result);
    assert_eq!(result.unwrap(), Action::Run);
}
