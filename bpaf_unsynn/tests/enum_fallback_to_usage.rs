// Tests for fallback_to_usage on enum variants

#[derive(Debug, Clone, PartialEq, bpaf_unsynn::Bpaf)]
#[bpaf(options)]
enum Command {
    /// Build the project
    #[bpaf(command)]
    Build { verbose: bool },

    /// Test the project (shows usage on error)
    #[bpaf(command, fallback_to_usage)]
    Test { pattern: String },

    /// Deploy (also shows usage on error)
    #[bpaf(command, fallback_to_usage)]
    Deploy { target: String },
}

#[test]
fn command_without_fallback_to_usage_works() {
    let parser = Command::parse();
    let result = parser.run_inner(&["build", "--verbose"]);
    assert_eq!(result.unwrap(), Command::Build { verbose: true });
}

#[test]
fn command_with_fallback_to_usage_works() {
    let parser = Command::parse();
    let result = parser.run_inner(&["test", "--pattern", "integration"]);
    assert_eq!(
        result.unwrap(),
        Command::Test {
            pattern: "integration".to_string()
        }
    );
}

#[test]
fn fallback_to_usage_on_deploy_works() {
    let parser = Command::parse();
    let result = parser.run_inner(&["deploy", "--target", "production"]);
    assert_eq!(
        result.unwrap(),
        Command::Deploy {
            target: "production".to_string()
        }
    );
}

// Test with unit variants
#[derive(Debug, Clone, PartialEq, bpaf_unsynn::Bpaf)]
#[bpaf(options)]
enum Action {
    #[bpaf(command)]
    Init,

    #[bpaf(command, fallback_to_usage)]
    Clean,

    #[bpaf(command, fallback_to_usage)]
    Check,
}

#[test]
fn unit_variant_without_fallback_to_usage_works() {
    let parser = Action::parse();
    let result = parser.run_inner(&["init"]);
    assert_eq!(result.unwrap(), Action::Init);
}

#[test]
fn unit_variant_with_fallback_to_usage_works() {
    let parser = Action::parse();
    let result = parser.run_inner(&["clean"]);
    assert_eq!(result.unwrap(), Action::Clean);
}

#[test]
fn another_unit_variant_with_fallback_to_usage() {
    let parser = Action::parse();
    let result = parser.run_inner(&["check"]);
    assert_eq!(result.unwrap(), Action::Check);
}

// Test combination with other attributes
#[derive(Debug, Clone, PartialEq, bpaf_unsynn::Bpaf)]
#[bpaf(options)]
enum ComplexCommand {
    #[bpaf(command("do-build"), fallback_to_usage)]
    Build,

    #[bpaf(command, long("run-tests"), fallback_to_usage)]
    Test { verbose: bool },

    #[bpaf(command, short('d'), fallback_to_usage, hide)]
    Debug,
}

#[test]
fn fallback_to_usage_with_custom_name() {
    let parser = ComplexCommand::parse();
    let result = parser.run_inner(&["do-build"]);
    assert_eq!(result.unwrap(), ComplexCommand::Build);
}

#[test]
fn fallback_to_usage_with_long_alias() {
    let parser = ComplexCommand::parse();
    let result = parser.run_inner(&["run-tests", "--verbose"]);
    assert_eq!(
        result.unwrap(),
        ComplexCommand::Test { verbose: true }
    );
}

#[test]
fn fallback_to_usage_with_short_alias() {
    let parser = ComplexCommand::parse();
    let result = parser.run_inner(&["d"]);
    assert_eq!(result.unwrap(), ComplexCommand::Debug);
}

#[test]
fn fallback_to_usage_with_hide_works() {
    // Even though it's hidden, it should still work when invoked
    let parser = ComplexCommand::parse();
    let result = parser.run_inner(&["d"]);
    assert_eq!(result.unwrap(), ComplexCommand::Debug);
}

// Test with tuple variants
#[derive(Debug, Clone, PartialEq, bpaf_unsynn::Bpaf)]
#[bpaf(options)]
enum TupleCommand {
    #[bpaf(command, fallback_to_usage)]
    Add(String),

    #[bpaf(command, fallback_to_usage)]
    Copy(String, String),

    #[bpaf(command)]
    Delete(String),
}

#[test]
fn tuple_variant_single_field_with_fallback_to_usage() {
    let parser = TupleCommand::parse();
    let result = parser.run_inner(&["add", "item"]);
    assert_eq!(result.unwrap(), TupleCommand::Add("item".to_string()));
}

#[test]
fn tuple_variant_multi_field_with_fallback_to_usage() {
    let parser = TupleCommand::parse();
    let result = parser.run_inner(&["copy", "src", "dst"]);
    assert_eq!(
        result.unwrap(),
        TupleCommand::Copy("src".to_string(), "dst".to_string())
    );
}

#[test]
fn tuple_variant_without_fallback_to_usage() {
    let parser = TupleCommand::parse();
    let result = parser.run_inner(&["delete", "file"]);
    assert_eq!(result.unwrap(), TupleCommand::Delete("file".to_string()));
}

// Test with complex types in tuples
#[derive(Debug, Clone, PartialEq, bpaf_unsynn::Bpaf)]
#[bpaf(options)]
enum ComplexTupleCommand {
    #[bpaf(command, fallback_to_usage)]
    Run(Vec<String>),

    #[bpaf(command, fallback_to_usage)]
    Build(String, Option<String>),
}

#[test]
fn complex_tuple_vec_with_fallback_to_usage() {
    let parser = ComplexTupleCommand::parse();
    let result = parser.run_inner(&["run", "arg1", "arg2"]);
    assert_eq!(
        result.unwrap(),
        ComplexTupleCommand::Run(vec!["arg1".to_string(), "arg2".to_string()])
    );
}

#[test]
fn complex_tuple_with_option_some() {
    let parser = ComplexTupleCommand::parse();
    let result = parser.run_inner(&["build", "target", "output"]);
    assert_eq!(
        result.unwrap(),
        ComplexTupleCommand::Build("target".to_string(), Some("output".to_string()))
    );
}

#[test]
fn complex_tuple_with_option_none() {
    let parser = ComplexTupleCommand::parse();
    let result = parser.run_inner(&["build", "target"]);
    assert_eq!(
        result.unwrap(),
        ComplexTupleCommand::Build("target".to_string(), None)
    );
}
