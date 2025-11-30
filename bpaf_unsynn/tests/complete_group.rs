//! Tests for complete and group attributes used directly
//!
//! These attributes work on required fields. For optional fields, use external parsers.

use bpaf::ShellComp;

fn string_completer(_s: &String) -> Vec<(String, Option<String>)> {
    vec![
        ("option1".to_string(), Some("First option".to_string())),
        ("option2".to_string(), None),
    ]
}

// =============================================================================
// complete attribute
// =============================================================================

#[derive(Debug, Clone, PartialEq, bpaf_unsynn::Bpaf)]
#[bpaf(options)]
struct CompleteAttr {
    #[bpaf(argument("VAL"), complete(string_completer))]
    value: String,
}

#[test]
fn complete_works() {
    let parser = complete_attr();
    let r = parser.run_inner(&["--value", "test"]).unwrap();
    assert_eq!(r.value, "test");
}

// =============================================================================
// group attribute (must come after complete)
// =============================================================================

#[derive(Debug, Clone, PartialEq, bpaf_unsynn::Bpaf)]
#[bpaf(options)]
struct GroupAttr {
    #[bpaf(argument("VAL"), complete(string_completer), group("inputs"))]
    input: String,
}

#[test]
fn group_works() {
    let parser = group_attr();
    let r = parser.run_inner(&["--input", "value"]).unwrap();
    assert_eq!(r.input, "value");
}

// =============================================================================
// Multiple fields with same group
// =============================================================================

#[derive(Debug, Clone, PartialEq, bpaf_unsynn::Bpaf)]
#[bpaf(options)]
struct MultipleFieldsSameGroup {
    #[bpaf(long, argument("IN"), complete(string_completer), group("files"))]
    input: String,
    #[bpaf(long, argument("OUT"), complete(string_completer), group("files"))]
    output: String,
}

#[test]
fn test_multiple_fields_same_group() {
    let parser = multiple_fields_same_group();
    let r = parser
        .run_inner(&["--input", "in.txt", "--output", "out.txt"])
        .unwrap();
    assert_eq!(r.input, "in.txt");
    assert_eq!(r.output, "out.txt");
}

// =============================================================================
// complete_shell works on required fields
// =============================================================================

#[derive(Debug, Clone, PartialEq, bpaf_unsynn::Bpaf)]
#[bpaf(options)]
struct CompleteShellRequired {
    #[bpaf(argument("FILE"), complete_shell(ShellComp::File { mask: Some("*.rs") }))]
    file: String,
}

#[test]
fn test_complete_shell_required() {
    let parser = complete_shell_required();
    let r = parser.run_inner(&["--file", "main.rs"]).unwrap();
    assert_eq!(r.file, "main.rs");
}
