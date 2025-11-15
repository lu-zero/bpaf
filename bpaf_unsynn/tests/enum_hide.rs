#[derive(Debug, Clone, PartialEq, bpaf_unsynn::Bpaf)]
#[bpaf(options)]
enum Command {
    /// Build the project
    #[bpaf(command)]
    Build,

    /// Test the project (hidden)
    #[bpaf(command, hide)]
    Test,

    /// Deploy the project
    #[bpaf(command)]
    Deploy { target: String },

    /// Internal debugging command (hidden)
    #[bpaf(command, hide)]
    Debug { verbose: bool },
}

#[test]
fn hidden_unit_variant_still_works() {
    let parser = Command::parse();
    let result = parser.run_inner(&["test"]);
    assert_eq!(result.unwrap(), Command::Test);
}

#[test]
fn visible_unit_variant_works() {
    let parser = Command::parse();
    let result = parser.run_inner(&["build"]);
    assert_eq!(result.unwrap(), Command::Build);
}

#[test]
fn hidden_struct_variant_still_works() {
    let parser = Command::parse();
    let result = parser.run_inner(&["debug", "--verbose"]);
    assert_eq!(
        result.unwrap(),
        Command::Debug { verbose: true }
    );
}

#[test]
fn visible_struct_variant_works() {
    let parser = Command::parse();
    let result = parser.run_inner(&["deploy", "--target", "production"]);
    assert_eq!(
        result.unwrap(),
        Command::Deploy {
            target: "production".to_string()
        }
    );
}

// Test with all hidden variants
#[derive(Debug, Clone, PartialEq, bpaf_unsynn::Bpaf)]
#[bpaf(options)]
enum InternalCommands {
    #[bpaf(command, hide)]
    Secret,

    #[bpaf(command, hide)]
    Internal { code: String },
}

#[test]
fn all_hidden_unit_variant_works() {
    let parser = InternalCommands::parse();
    let result = parser.run_inner(&["secret"]);
    assert_eq!(result.unwrap(), InternalCommands::Secret);
}

#[test]
fn all_hidden_struct_variant_works() {
    let parser = InternalCommands::parse();
    let result = parser.run_inner(&["internal", "--code", "42"]);
    assert_eq!(
        result.unwrap(),
        InternalCommands::Internal {
            code: "42".to_string()
        }
    );
}

// Test hide with command aliases
#[derive(Debug, Clone, PartialEq, bpaf_unsynn::Bpaf)]
#[bpaf(options)]
enum AliasedCommand {
    #[bpaf(command, long("compile"), hide)]
    Build,

    #[bpaf(command, short('t'), hide)]
    Test,
}

#[test]
fn hidden_with_long_alias_works() {
    let parser = AliasedCommand::parse();
    let result = parser.run_inner(&["compile"]);
    assert_eq!(result.unwrap(), AliasedCommand::Build);
}

#[test]
fn hidden_with_short_alias_works() {
    let parser = AliasedCommand::parse();
    let result = parser.run_inner(&["t"]);
    assert_eq!(result.unwrap(), AliasedCommand::Test);
}

// Test hide with custom command name
#[derive(Debug, Clone, PartialEq, bpaf_unsynn::Bpaf)]
#[bpaf(options)]
enum CustomNamedCommand {
    #[bpaf(command("do-build"), hide)]
    Build,

    #[bpaf(command)]
    Test,
}

#[test]
fn hidden_with_custom_name_works() {
    let parser = CustomNamedCommand::parse();
    let result = parser.run_inner(&["do-build"]);
    assert_eq!(result.unwrap(), CustomNamedCommand::Build);
}

#[test]
fn visible_without_hide_works() {
    let parser = CustomNamedCommand::parse();
    let result = parser.run_inner(&["test"]);
    assert_eq!(result.unwrap(), CustomNamedCommand::Test);
}

// Test with tuple variant (single field only, as multi-field not supported yet)
#[derive(Debug, Clone, PartialEq, bpaf_unsynn::Bpaf)]
#[bpaf(options)]
enum TupleCommand {
    #[bpaf(command)]
    Visible(String),

    #[bpaf(command, hide)]
    Hidden(String),
}

#[test]
fn hidden_tuple_variant_works() {
    let parser = TupleCommand::parse();
    let result = parser.run_inner(&["hidden", "value"]);
    assert_eq!(result.unwrap(), TupleCommand::Hidden("value".to_string()));
}

#[test]
fn visible_tuple_variant_works() {
    let parser = TupleCommand::parse();
    let result = parser.run_inner(&["visible", "value"]);
    assert_eq!(result.unwrap(), TupleCommand::Visible("value".to_string()));
}
