// Tests for complex types in tuple variants (Option<T>, Vec<T>)
//
// NOTE: For positional arguments (tuples), Option<T> fields should generally be at the end.
// This is because positional arguments are consumed in order, and having optional positionals
// in the middle creates ambiguity. Vec<T> fields should also be at the end for the same reason.

#[derive(Debug, Clone, PartialEq, bpaf_unsynn::Bpaf)]
#[bpaf(options)]
enum Command {
    /// Add item with optional description
    #[bpaf(command)]
    Add(String, Option<String>),

    /// Build with target and optional output
    #[bpaf(command)]
    Build(String, Option<String>),

    /// Run with multiple arguments
    #[bpaf(command)]
    Run(Vec<String>),

    /// Deploy with target and multiple tags
    #[bpaf(command)]
    Deploy(String, Vec<String>),

    /// Complex with all types
    #[bpaf(command)]
    Complex(String, Option<usize>, Vec<String>),
}

#[test]
fn option_as_second_field_some() {
    let parser = Command::parse();
    let result = parser.run_inner(&["add", "item", "description"]);
    assert_eq!(
        result.unwrap(),
        Command::Add("item".to_string(), Some("description".to_string()))
    );
}

#[test]
fn option_as_second_field_none() {
    let parser = Command::parse();
    let result = parser.run_inner(&["add", "item"]);
    assert_eq!(
        result.unwrap(),
        Command::Add("item".to_string(), None)
    );
}

#[test]
fn build_with_option_some() {
    let parser = Command::parse();
    let result = parser.run_inner(&["build", "target", "output.txt"]);
    assert_eq!(
        result.unwrap(),
        Command::Build("target".to_string(), Some("output.txt".to_string()))
    );
}

#[test]
fn build_with_option_none() {
    let parser = Command::parse();
    let result = parser.run_inner(&["build", "target"]);
    assert_eq!(
        result.unwrap(),
        Command::Build("target".to_string(), None)
    );
}

#[test]
fn vec_single_field_empty() {
    let parser = Command::parse();
    let result = parser.run_inner(&["run"]);
    assert_eq!(result.unwrap(), Command::Run(vec![]));
}

#[test]
fn vec_single_field_one_value() {
    let parser = Command::parse();
    let result = parser.run_inner(&["run", "arg1"]);
    assert_eq!(
        result.unwrap(),
        Command::Run(vec!["arg1".to_string()])
    );
}

#[test]
fn vec_single_field_multiple_values() {
    let parser = Command::parse();
    let result = parser.run_inner(&["run", "arg1", "arg2", "arg3"]);
    assert_eq!(
        result.unwrap(),
        Command::Run(vec![
            "arg1".to_string(),
            "arg2".to_string(),
            "arg3".to_string()
        ])
    );
}

#[test]
fn vec_as_second_field_empty() {
    let parser = Command::parse();
    let result = parser.run_inner(&["deploy", "production"]);
    assert_eq!(
        result.unwrap(),
        Command::Deploy("production".to_string(), vec![])
    );
}

#[test]
fn vec_as_second_field_multiple() {
    let parser = Command::parse();
    let result = parser.run_inner(&["deploy", "production", "v1.0", "stable"]);
    assert_eq!(
        result.unwrap(),
        Command::Deploy(
            "production".to_string(),
            vec!["v1.0".to_string(), "stable".to_string()]
        )
    );
}

#[test]
fn complex_all_types_minimal() {
    let parser = Command::parse();
    let result = parser.run_inner(&["complex", "name"]);
    assert_eq!(
        result.unwrap(),
        Command::Complex("name".to_string(), None, vec![])
    );
}

#[test]
fn complex_all_types_with_option() {
    let parser = Command::parse();
    let result = parser.run_inner(&["complex", "name", "42"]);
    assert_eq!(
        result.unwrap(),
        Command::Complex("name".to_string(), Some(42), vec![])
    );
}

#[test]
fn complex_all_types_full() {
    let parser = Command::parse();
    let result = parser.run_inner(&["complex", "name", "42", "tag1", "tag2"]);
    assert_eq!(
        result.unwrap(),
        Command::Complex(
            "name".to_string(),
            Some(42),
            vec!["tag1".to_string(), "tag2".to_string()]
        )
    );
}

// Test with nested Option types
#[derive(Debug, Clone, PartialEq, bpaf_unsynn::Bpaf)]
#[bpaf(options)]
enum NestedCommand {
    #[bpaf(command)]
    Test(Option<usize>, Option<bool>),
}

#[test]
fn nested_options_both_none() {
    let parser = NestedCommand::parse();
    let result = parser.run_inner(&["test"]);
    assert_eq!(result.unwrap(), NestedCommand::Test(None, None));
}

#[test]
fn nested_options_first_some() {
    let parser = NestedCommand::parse();
    let result = parser.run_inner(&["test", "42"]);
    assert_eq!(result.unwrap(), NestedCommand::Test(Some(42), None));
}

#[test]
fn nested_options_both_some() {
    let parser = NestedCommand::parse();
    let result = parser.run_inner(&["test", "42", "true"]);
    assert_eq!(result.unwrap(), NestedCommand::Test(Some(42), Some(true)));
}

// Test with multiple Vec fields
#[derive(Debug, Clone, PartialEq, bpaf_unsynn::Bpaf)]
#[bpaf(options)]
enum MultiVecCommand {
    #[bpaf(command)]
    Copy(Vec<String>, Vec<String>),
}

#[test]
fn multiple_vecs_both_empty() {
    let parser = MultiVecCommand::parse();
    let result = parser.run_inner(&["copy"]);
    assert_eq!(result.unwrap(), MultiVecCommand::Copy(vec![], vec![]));
}
