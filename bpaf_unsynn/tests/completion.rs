//! Tests for completion-related attributes
//!
//! Covers: complete_shell, group (completion group)
//!
//! Note: These tests verify the attributes compile and generate correct code.
//! Actual completion behavior is tested in bpaf's main test suite.

use bpaf::{Parser, ShellComp};

// =============================================================================
// complete_shell attribute with ShellComp::File
// =============================================================================

#[derive(Debug, Clone, PartialEq, bpaf_unsynn::Bpaf)]
#[bpaf(options)]
struct CompleteShellFile {
    #[bpaf(argument("FILE"), complete_shell(ShellComp::File { mask: None }))]
    file: String,
}

#[test]
fn test_complete_shell_file() {
    let parser = complete_shell_file();
    let r = parser.run_inner(&["--file", "test.txt"]).unwrap();
    assert_eq!(r.file, "test.txt");
}

// =============================================================================
// complete_shell with file mask
// =============================================================================

#[derive(Debug, Clone, PartialEq, bpaf_unsynn::Bpaf)]
#[bpaf(options)]
struct CompleteShellWithMask {
    #[bpaf(argument("RS"), complete_shell(ShellComp::File { mask: Some("*.rs") }))]
    source: String,
}

#[test]
fn test_complete_shell_with_mask() {
    let parser = complete_shell_with_mask();
    let r = parser.run_inner(&["--source", "main.rs"]).unwrap();
    assert_eq!(r.source, "main.rs");
}

// =============================================================================
// complete_shell with optional
// =============================================================================

#[derive(Debug, Clone, PartialEq, bpaf_unsynn::Bpaf)]
#[bpaf(options)]
struct CompleteShellOptional {
    #[bpaf(argument("FILE"), complete_shell(ShellComp::File { mask: None }))]
    config: Option<String>,
}

#[test]
fn complete_shell_with_optional() {
    let parser = complete_shell_optional();

    let r = parser.run_inner(&["--config", "config.toml"]).unwrap();
    assert_eq!(r.config, Some("config.toml".to_string()));

    let r = parser.run_inner(&[]).unwrap();
    assert_eq!(r.config, None);
}

// =============================================================================
// complete_shell with ShellComp::Dir
// =============================================================================

#[derive(Debug, Clone, PartialEq, bpaf_unsynn::Bpaf)]
#[bpaf(options)]
struct CompleteShellDir {
    #[bpaf(argument("DIR"), complete_shell(ShellComp::Dir { mask: None }))]
    directory: String,
}

#[test]
fn test_complete_shell_dir() {
    let parser = complete_shell_dir();
    let r = parser.run_inner(&["--directory", "/tmp"]).unwrap();
    assert_eq!(r.directory, "/tmp");
}

// =============================================================================
// complete_shell on positional
// =============================================================================

#[derive(Debug, Clone, PartialEq, bpaf_unsynn::Bpaf)]
#[bpaf(options)]
struct CompleteShellPositional {
    #[bpaf(positional("FILE"), complete_shell(ShellComp::File { mask: None }))]
    file: String,
}

#[test]
fn complete_shell_on_positional() {
    let parser = complete_shell_positional();
    let r = parser.run_inner(&["myfile.txt"]).unwrap();
    assert_eq!(r.file, "myfile.txt");
}

// =============================================================================
// complete and group attributes - require external for proper implementation
// =============================================================================

fn string_completer(_input: &String) -> Vec<(String, Option<String>)> {
    vec![
        ("option1".to_string(), Some("First option".to_string())),
        ("option2".to_string(), Some("Second option".to_string())),
    ]
}

// complete attribute via external
#[derive(Debug, Clone, PartialEq, bpaf_unsynn::Bpaf)]
#[bpaf(options)]
struct CompleteAttr {
    #[bpaf(external(complete_value_parser))]
    value: String,
}

fn complete_value_parser() -> impl Parser<String> {
    use bpaf::*;
    long("value")
        .argument::<String>("VAL")
        .complete(string_completer)
}

#[test]
fn complete_attribute() {
    let parser = complete_attr();
    let r = parser.run_inner(&["--value", "test"]).unwrap();
    assert_eq!(r.value, "test");
}

// complete with optional via external
/// Newtype to avoid double-wrapping
#[derive(Debug, Clone, PartialEq)]
struct OptConfig(Option<String>);

#[derive(Debug, Clone, PartialEq, bpaf_unsynn::Bpaf)]
#[bpaf(options)]
struct CompleteOptional {
    #[bpaf(external(complete_optional_parser))]
    config: OptConfig,
}

fn complete_optional_parser() -> impl Parser<OptConfig> {
    use bpaf::*;
    long("config")
        .argument::<String>("N")
        .optional()
        .complete(|opt: &Option<String>| {
            match opt {
                Some(_) => vec![],
                None => vec![
                    ("value1".to_string(), Some("First value".to_string())),
                    ("value2".to_string(), None),
                ],
            }
        })
        .map(OptConfig)
}

#[test]
fn complete_with_optional() {
    let parser = complete_optional();

    let r = parser.run_inner(&["--config", "value"]).unwrap();
    assert_eq!(r.config.0, Some("value".to_string()));

    let r = parser.run_inner(&[]).unwrap();
    assert_eq!(r.config.0, None);
}

// group attribute - must come after complete
#[derive(Debug, Clone, PartialEq, bpaf_unsynn::Bpaf)]
#[bpaf(options)]
struct GroupWithComplete {
    #[bpaf(external(group_input_parser))]
    input: String,
}

fn group_input_parser() -> impl Parser<String> {
    use bpaf::*;
    long("input")
        .argument::<String>("VAL")
        .complete(string_completer)
        .group("inputs")
}

#[test]
fn test_group_with_complete() {
    let parser = group_with_complete();
    let r = parser.run_inner(&["--input", "value"]).unwrap();
    assert_eq!(r.input, "value");
}

// Multiple fields with same group
/// Newtype wrappers to avoid double-wrapping
#[derive(Debug, Clone, PartialEq)]
struct OptInput(Option<String>);

#[derive(Debug, Clone, PartialEq)]
struct OptOutput(Option<String>);

#[derive(Debug, Clone, PartialEq, bpaf_unsynn::Bpaf)]
#[bpaf(options)]
struct MultipleFieldsSameGroup {
    #[bpaf(external(group_input_parser2))]
    input: OptInput,
    #[bpaf(external(group_output_parser))]
    output: OptOutput,
}

fn group_input_parser2() -> impl Parser<OptInput> {
    use bpaf::*;
    long("input")
        .argument::<String>("VAL")
        .optional()
        .complete(|opt: &Option<String>| match opt {
            Some(_) => vec![],
            None => string_completer(&String::new()),
        })
        .group("files")
        .map(OptInput)
}

fn group_output_parser() -> impl Parser<OptOutput> {
    use bpaf::*;
    long("output")
        .argument::<String>("VAL")
        .optional()
        .complete(|opt: &Option<String>| match opt {
            Some(_) => vec![],
            None => string_completer(&String::new()),
        })
        .group("files")
        .map(OptOutput)
}

#[test]
fn test_multiple_fields_same_group() {
    let parser = multiple_fields_same_group();

    let r = parser
        .run_inner(&["--input", "in.txt", "--output", "out.txt"])
        .unwrap();
    assert_eq!(r.input.0, Some("in.txt".to_string()));
    assert_eq!(r.output.0, Some("out.txt".to_string()));

    let r = parser.run_inner(&[]).unwrap();
    assert_eq!(r.input.0, None);
    assert_eq!(r.output.0, None);
}

// =============================================================================
// complete_shell with many
// =============================================================================

#[derive(Debug, Clone, PartialEq, bpaf_unsynn::Bpaf)]
#[bpaf(options)]
struct CompleteShellMany {
    #[bpaf(argument("FILE"), complete_shell(ShellComp::File { mask: Some("*.txt") }), many)]
    files: Vec<String>,
}

#[test]
fn complete_shell_with_many() {
    let parser = complete_shell_many();

    let r = parser.run_inner(&["--files", "a.txt", "--files", "b.txt"]).unwrap();
    assert_eq!(r.files, vec!["a.txt", "b.txt"]);

    let r = parser.run_inner(&[]).unwrap();
    assert!(r.files.is_empty());
}
