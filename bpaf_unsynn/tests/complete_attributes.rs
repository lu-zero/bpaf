//! Tests for the `complete` and `complete_shell` attributes
//!
//! These attributes provide shell completion functionality.
//! - `complete` takes a function that generates completion suggestions
//! - `complete_shell` takes a ShellComp expression for shell-specific completion

use bpaf::Parser;

// ============================================================================
// Basic complete attribute usage
// ============================================================================

#[derive(Debug, Clone, PartialEq)]
#[derive(bpaf_unsynn::Bpaf)]
struct BasicComplete {
    /// File argument with completion function
    #[bpaf(long, argument("FILE"), complete(|_: &String| vec![("test.txt".to_string(), None)]))]
    input: String,
}

#[test]
fn complete_basic_single_value() {
    let parser = BasicComplete::parse().to_options();

    let result = parser.run_inner(&["--input", "test.txt"]);
    assert!(result.is_ok(), "Failed to parse: {:?}", result);
    let opts = result.unwrap();
    assert_eq!(opts.input, "test.txt");
}

// ============================================================================
// Complete with custom completion function
// ============================================================================

fn file_completions(_s: &String) -> Vec<(String, Option<String>)> {
    vec![
        ("file1.txt".to_string(), Some("First file".to_string())),
        ("file2.txt".to_string(), Some("Second file".to_string())),
    ]
}

#[derive(Debug, Clone, PartialEq)]
#[derive(bpaf_unsynn::Bpaf)]
struct CompleteWithFunction {
    #[bpaf(long, argument("FILE"), complete(file_completions))]
    file: String,
}

#[test]
fn complete_with_function_works() {
    let parser = CompleteWithFunction::parse().to_options();

    let result = parser.run_inner(&["--file", "test.txt"]);
    assert!(result.is_ok(), "Failed to parse: {:?}", result);
    let opts = result.unwrap();
    assert_eq!(opts.file, "test.txt");
}

// ============================================================================
// Complete with closure
// ============================================================================

#[derive(Debug, Clone, PartialEq)]
#[derive(bpaf_unsynn::Bpaf)]
struct CompleteWithClosure {
    #[bpaf(long, argument("NAME"), complete(|s: &String| {
        vec![
            (format!("{}_1", s), None),
            (format!("{}_2", s), None),
        ]
    }))]
    name: String,
}

#[test]
fn complete_with_closure_works() {
    let parser = CompleteWithClosure::parse().to_options();

    let result = parser.run_inner(&["--name", "test"]);
    assert!(result.is_ok(), "Failed to parse: {:?}", result);
    let opts = result.unwrap();
    assert_eq!(opts.name, "test");
}

// ============================================================================
// Complete with different types
// ============================================================================

#[derive(Debug, Clone, PartialEq)]
#[derive(bpaf_unsynn::Bpaf)]
struct CompleteWithNumber {
    #[bpaf(long, argument("NUM"), complete(|n: &usize| vec![(n.to_string(), None)]))]
    count: usize,
}

#[test]
fn complete_with_number_type() {
    let parser = CompleteWithNumber::parse().to_options();

    let result = parser.run_inner(&["--count", "42"]);
    assert!(result.is_ok(), "Failed to parse: {:?}", result);
    let opts = result.unwrap();
    assert_eq!(opts.count, 42);
}

// ============================================================================
// Complete with Vec/many
// ============================================================================

#[derive(Debug, Clone, PartialEq)]
#[derive(bpaf_unsynn::Bpaf)]
struct CompleteWithMany {
    #[bpaf(long, argument("FILE"), complete(|_: &String| vec![("file.txt".to_string(), None)]), many)]
    files: Vec<String>,
}

#[test]
fn complete_with_many_empty() {
    let parser = CompleteWithMany::parse().to_options();

    let result = parser.run_inner(&[] as &[&str]);
    assert!(result.is_ok(), "Failed to parse: {:?}", result);
    let opts = result.unwrap();
    assert_eq!(opts.files, Vec::<String>::new());
}

#[test]
fn complete_with_many_multiple() {
    let parser = CompleteWithMany::parse().to_options();

    let result = parser.run_inner(&["--files", "one.txt", "--files", "two.txt"]);
    assert!(result.is_ok(), "Failed to parse: {:?}", result);
    let opts = result.unwrap();
    assert_eq!(opts.files, vec!["one.txt", "two.txt"]);
}

// ============================================================================
// Complete with fallback
// ============================================================================

#[derive(Debug, Clone, PartialEq)]
#[derive(bpaf_unsynn::Bpaf)]
struct CompleteWithFallback {
    #[bpaf(long, argument("FILE"), fallback(String::from("default.txt")), complete(|_: &String| vec![("config.txt".to_string(), None)]))]
    config: String,
}

#[test]
fn complete_with_fallback_not_specified() {
    let parser = CompleteWithFallback::parse().to_options();

    let result = parser.run_inner(&[] as &[&str]);
    assert!(result.is_ok(), "Failed to parse: {:?}", result);
    let opts = result.unwrap();
    assert_eq!(opts.config, "default.txt");
}

#[test]
fn complete_with_fallback_specified() {
    let parser = CompleteWithFallback::parse().to_options();

    let result = parser.run_inner(&["--config", "custom.txt"]);
    assert!(result.is_ok(), "Failed to parse: {:?}", result);
    let opts = result.unwrap();
    assert_eq!(opts.config, "custom.txt");
}

// ============================================================================
// Complete with group
// ============================================================================

#[derive(Debug, Clone, PartialEq)]
#[derive(bpaf_unsynn::Bpaf)]
struct CompleteWithGroup {
    #[bpaf(long, argument("FILE"), complete(|_: &String| vec![("in.txt".to_string(), None)]), group("input-files"))]
    input: String,

    #[bpaf(long, argument("FILE"), complete(|_: &String| vec![("out.txt".to_string(), None)]), group("output-files"))]
    output: String,
}

#[test]
fn complete_with_group_works() {
    let parser = CompleteWithGroup::parse().to_options();

    let result = parser.run_inner(&["--input", "in.txt", "--output", "out.txt"]);
    assert!(result.is_ok(), "Failed to parse: {:?}", result);
    let opts = result.unwrap();
    assert_eq!(opts.input, "in.txt");
    assert_eq!(opts.output, "out.txt");
}

// ============================================================================
// Complete with help
// ============================================================================

#[derive(Debug, Clone, PartialEq)]
#[derive(bpaf_unsynn::Bpaf)]
struct CompleteWithHelp {
    /// Input file with completion
    #[bpaf(long, argument("FILE"), complete(|_: &String| vec![("file.txt".to_string(), None)]), help("Input file path"))]
    input: String,
}

#[test]
fn complete_with_help_works() {
    let parser = CompleteWithHelp::parse().to_options();

    let result = parser.run_inner(&["--input", "test.txt"]);
    assert!(result.is_ok(), "Failed to parse: {:?}", result);
    let opts = result.unwrap();
    assert_eq!(opts.input, "test.txt");
}

// ============================================================================
// Complete with guard
// ============================================================================

#[derive(Debug, Clone, PartialEq)]
#[derive(bpaf_unsynn::Bpaf)]
struct CompleteWithGuard {
    #[bpaf(long, argument("NUM"), complete(|_: &usize| vec![("5".to_string(), None)]), guard(|x: &usize| *x > 0, "must be positive"))]
    count: usize,
}

#[test]
fn complete_with_guard_valid() {
    let parser = CompleteWithGuard::parse().to_options();

    let result = parser.run_inner(&["--count", "5"]);
    assert!(result.is_ok(), "Failed to parse: {:?}", result);
    let opts = result.unwrap();
    assert_eq!(opts.count, 5);
}

// ============================================================================
// complete_shell attribute usage
// ============================================================================

#[derive(Debug, Clone, PartialEq)]
#[derive(bpaf_unsynn::Bpaf)]
struct CompleteShellBasic {
    #[bpaf(long, argument("FILE"), complete_shell(bpaf::ShellComp::File { mask: None }))]
    file: String,
}

#[test]
fn complete_shell_basic_works() {
    let parser = CompleteShellBasic::parse().to_options();

    let result = parser.run_inner(&["--file", "test.txt"]);
    assert!(result.is_ok(), "Failed to parse: {:?}", result);
    let opts = result.unwrap();
    assert_eq!(opts.file, "test.txt");
}

// ============================================================================
// complete_shell with file mask
// ============================================================================

#[derive(Debug, Clone, PartialEq)]
#[derive(bpaf_unsynn::Bpaf)]
struct CompleteShellWithMask {
    #[bpaf(long, argument("CONFIG"), complete_shell(bpaf::ShellComp::File { mask: Some("*.toml") }))]
    config: String,
}

#[test]
fn complete_shell_with_mask_works() {
    let parser = CompleteShellWithMask::parse().to_options();

    let result = parser.run_inner(&["--config", "Cargo.toml"]);
    assert!(result.is_ok(), "Failed to parse: {:?}", result);
    let opts = result.unwrap();
    assert_eq!(opts.config, "Cargo.toml");
}

// ============================================================================
// complete_shell with directory completion
// ============================================================================

#[derive(Debug, Clone, PartialEq)]
#[derive(bpaf_unsynn::Bpaf)]
struct CompleteShellDir {
    #[bpaf(long, argument("DIR"), complete_shell(bpaf::ShellComp::Dir { mask: None }))]
    directory: String,
}

#[test]
fn complete_shell_dir_works() {
    let parser = CompleteShellDir::parse().to_options();

    let result = parser.run_inner(&["--directory", "/tmp"]);
    assert!(result.is_ok(), "Failed to parse: {:?}", result);
    let opts = result.unwrap();
    assert_eq!(opts.directory, "/tmp");
}

// ============================================================================
// complete_shell with raw completion
// ============================================================================

#[derive(Debug, Clone, PartialEq)]
#[derive(bpaf_unsynn::Bpaf)]
struct CompleteShellMultiMask {
    #[bpaf(long, argument("FILE"), complete_shell(bpaf::ShellComp::File { mask: Some("*.(rs|toml)") }))]
    source: String,
}

#[test]
fn complete_shell_multi_mask_works() {
    let parser = CompleteShellMultiMask::parse().to_options();

    let result = parser.run_inner(&["--source", "main.rs"]);
    assert!(result.is_ok(), "Failed to parse: {:?}", result);
    let opts = result.unwrap();
    assert_eq!(opts.source, "main.rs");
}

// ============================================================================
// Complex struct with multiple completion types
// ============================================================================

#[derive(Debug, Clone, PartialEq)]
#[derive(bpaf_unsynn::Bpaf)]
struct ComplexWithComplete {
    /// Source files with custom completion
    #[bpaf(short('s'), long("source"), argument("FILE"), complete(|_: &String| vec![("main.rs".to_string(), None)]), many)]
    sources: Vec<String>,

    /// Output file with shell completion
    #[bpaf(short('o'), long("output"), argument("FILE"), complete_shell(bpaf::ShellComp::File { mask: None }))]
    output: String,

    /// Verbose flag
    #[bpaf(short('v'), long("verbose"))]
    verbose: bool,
}

#[test]
fn complex_with_complete_basic() {
    let parser = ComplexWithComplete::parse().to_options();

    let result = parser.run_inner(&[
        "-s", "main.rs",
        "-s", "lib.rs",
        "-o", "output.txt",
    ]);
    assert!(result.is_ok(), "Failed to parse: {:?}", result);
    let opts = result.unwrap();
    assert_eq!(opts.sources, vec!["main.rs", "lib.rs"]);
    assert_eq!(opts.output, "output.txt");
    assert_eq!(opts.verbose, false);
}

#[test]
fn complex_with_complete_with_verbose() {
    let parser = ComplexWithComplete::parse().to_options();

    let result = parser.run_inner(&[
        "-s", "main.rs",
        "-o", "output.txt",
        "-v",
    ]);
    assert!(result.is_ok(), "Failed to parse: {:?}", result);
    let opts = result.unwrap();
    assert_eq!(opts.sources, vec!["main.rs"]);
    assert_eq!(opts.output, "output.txt");
    assert_eq!(opts.verbose, true);
}

// ============================================================================
// Complete with all post-processing attributes combined
// ============================================================================

#[derive(Debug, Clone, PartialEq)]
#[derive(bpaf_unsynn::Bpaf)]
struct CompleteWithAllAttributes {
    #[bpaf(
        long,
        argument("NUM"),
        complete(|_: &usize| vec![("50".to_string(), None)]),
        group("numbers"),
        guard(|x: &usize| *x <= 100, "must be <= 100"),
        help("Number between 1 and 100")
    )]
    count: usize,
}

#[test]
fn complete_with_all_attributes_valid() {
    let parser = CompleteWithAllAttributes::parse().to_options();

    let result = parser.run_inner(&["--count", "50"]);
    assert!(result.is_ok(), "Failed to parse: {:?}", result);
    let opts = result.unwrap();
    assert_eq!(opts.count, 50);
}
