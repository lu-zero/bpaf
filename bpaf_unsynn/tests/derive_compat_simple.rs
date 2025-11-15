// Simplified version of derive compatibility tests
// Testing if basic derive patterns from bpaf work with bpaf_unsynn

use bpaf_unsynn::Bpaf;
use bpaf::Parser;

#[test]
fn help_with_default_simple() {
    #[derive(Debug, Clone, Bpaf, PartialEq)]
    #[bpaf(options, fallback(Action::CheckConnection))]
    enum Action {
        /// Add a new TODO item
        #[bpaf(command)]
        Add(String),

        /// Test connection to the server
        #[bpaf(command)]
        CheckConnection,
    }

    let parser = Action::parse();

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
    #[derive(Debug, Clone, Bpaf, PartialEq)]
    #[bpaf(parser)]
    enum Action {
        /// Add a new TODO item
        #[bpaf(command)]
        Add(String),

        /// Does nothing
        /// in two lines
        #[bpaf(command)]
        NoAction,
    }

    let parser = Action::parse().fallback(Action::NoAction).to_options();

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
fn single_unit_command() {
    #[derive(Bpaf, Debug, Clone, Eq, PartialEq)]
    #[bpaf(command)]
    struct One;

    let parser = One::parse().to_options();
    let help = parser.run_inner(&["--help"]).unwrap_err().unwrap_stdout();
    let expected = "\
Usage: COMMAND ...

Available options:
    -h, --help  Prints help information

Available commands:
    one
";
    assert_eq!(help, expected);

    let r = parser.run_inner(&["one"]).unwrap();
    assert_eq!(r, One);
}

#[test]
fn pure_optional() {
    #[derive(Bpaf, Debug, Clone, PartialEq)]
    #[bpaf(options)]
    struct Opts {
        #[bpaf(pure(Default::default()))]
        foo: Option<Vec<u32>>,
    }

    assert_eq!(Opts::parse().run().foo, None);
}

#[test]
fn basic_enum_commands() {
    #[derive(Debug, Clone, PartialEq, Bpaf)]
    #[bpaf(options)]
    enum Command {
        #[bpaf(command)]
        Build,

        #[bpaf(command)]
        Test,
    }

    let parser = Command::parse();
    assert_eq!(parser.run_inner(&["build"]).unwrap(), Command::Build);
    assert_eq!(parser.run_inner(&["test"]).unwrap(), Command::Test);
}
