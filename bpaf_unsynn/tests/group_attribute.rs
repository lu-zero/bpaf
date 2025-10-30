//! Tests for the `group` attribute
//!
//! The `group` attribute is used for shell completion grouping.
//! It must be used together with `complete` or `complete_shell` attributes.
//! The `group` method is only available on `ParseComp` which is returned by `.complete()`.

use bpaf::Parser;

// ============================================================================
// Basic group attribute usage with complete
// ============================================================================

#[derive(Debug, Clone, PartialEq)]
#[derive(bpaf_unsynn::Bpaf)]
struct BasicGroupWithComplete {
    /// File argument with completion group
    #[bpaf(long, argument("FILE"), complete(|s: &String| vec![(s.clone(), None)]), group("files"))]
    input: String,
}

#[test]
fn group_with_complete_basic() {
    let parser = BasicGroupWithComplete::parse().to_options();

    let result = parser.run_inner(&["--input", "test.txt"]);
    assert!(result.is_ok(), "Failed to parse: {:?}", result);
    let opts = result.unwrap();
    assert_eq!(opts.input, "test.txt");
}

// ============================================================================
// Multiple fields with different groups and complete
// ============================================================================

#[derive(Debug, Clone, PartialEq)]
#[derive(bpaf_unsynn::Bpaf)]
struct MultipleGroupsWithComplete {
    /// Input files group
    #[bpaf(short('i'), long, argument("FILE"), complete(|s: &String| vec![(s.clone(), None)]), group("input-files"), many)]
    inputs: Vec<String>,

    /// Output file group
    #[bpaf(short('o'), long, argument("FILE"), complete(|s: &String| vec![(s.clone(), None)]), group("output-files"))]
    output: String,

    /// Configuration file group (optional, no group)
    #[bpaf(short('c'), long, argument("FILE"))]
    config: Option<String>,
}

#[test]
fn multiple_groups_all_specified() {
    let parser = MultipleGroupsWithComplete::parse().to_options();

    let result = parser.run_inner(&[
        "-i", "in1.txt",
        "-i", "in2.txt",
        "-o", "out.txt",
        "-c", "config.toml",
    ]);
    assert!(result.is_ok(), "Failed to parse: {:?}", result);
    let opts = result.unwrap();
    assert_eq!(opts.inputs, vec!["in1.txt", "in2.txt"]);
    assert_eq!(opts.output, "out.txt");
    assert_eq!(opts.config, Some("config.toml".to_string()));
}

#[test]
fn multiple_groups_minimal() {
    let parser = MultipleGroupsWithComplete::parse().to_options();

    // Only required field
    let result = parser.run_inner(&["-o", "out.txt"]);
    assert!(result.is_ok(), "Failed to parse: {:?}", result);
    let opts = result.unwrap();
    assert_eq!(opts.inputs, Vec::<String>::new());
    assert_eq!(opts.output, "out.txt");
    assert_eq!(opts.config, None);
}

// ============================================================================
// Group with fallback and complete
// ============================================================================

#[derive(Debug, Clone, PartialEq)]
#[derive(bpaf_unsynn::Bpaf)]
struct GroupWithFallbackComplete {
    #[bpaf(long, argument("FILE"), fallback(String::from("default.txt")), complete(|s: &String| vec![(s.clone(), None)]), group("files"))]
    config: String,
}

#[test]
fn group_with_fallback_not_specified() {
    let parser = GroupWithFallbackComplete::parse().to_options();

    let result = parser.run_inner(&[] as &[&str]);
    assert!(result.is_ok(), "Failed to parse: {:?}", result);
    let opts = result.unwrap();
    assert_eq!(opts.config, "default.txt");
}

#[test]
fn group_with_fallback_specified() {
    let parser = GroupWithFallbackComplete::parse().to_options();

    let result = parser.run_inner(&["--config", "custom.txt"]);
    assert!(result.is_ok(), "Failed to parse: {:?}", result);
    let opts = result.unwrap();
    assert_eq!(opts.config, "custom.txt");
}

// ============================================================================
// Group with help text and complete
// ============================================================================

#[derive(Debug, Clone, PartialEq)]
#[derive(bpaf_unsynn::Bpaf)]
struct GroupWithHelpComplete {
    /// Input file path
    #[bpaf(long, argument("FILE"), complete(|s: &String| vec![(s.clone(), None)]), group("input-files"), help("Path to input file"))]
    input: String,
}

#[test]
fn group_with_help_works() {
    let parser = GroupWithHelpComplete::parse().to_options();

    let result = parser.run_inner(&["--input", "data.txt"]);
    assert!(result.is_ok(), "Failed to parse: {:?}", result);
    let opts = result.unwrap();
    assert_eq!(opts.input, "data.txt");
}

// ============================================================================
// Group with custom usage and complete
// ============================================================================

#[derive(Debug, Clone, PartialEq)]
#[derive(bpaf_unsynn::Bpaf)]
struct GroupWithCustomUsage {
    #[bpaf(long, argument("FILE"), complete(|s: &String| vec![(s.clone(), None)]), group("files"), custom_usage("<file>"))]
    file: String,
}

#[test]
fn group_with_custom_usage_works() {
    let parser = GroupWithCustomUsage::parse().to_options();

    let result = parser.run_inner(&["--file", "test.txt"]);
    assert!(result.is_ok(), "Failed to parse: {:?}", result);
    let opts = result.unwrap();
    assert_eq!(opts.file, "test.txt");
}

// ============================================================================
// Group with Vec/many and complete
// ============================================================================

#[derive(Debug, Clone, PartialEq)]
#[derive(bpaf_unsynn::Bpaf)]
struct GroupWithManyComplete {
    #[bpaf(long, argument("FILE"), complete(|s: &String| vec![(s.clone(), None)]), group("files"), many)]
    files: Vec<String>,
}

#[test]
fn group_with_many_empty() {
    let parser = GroupWithManyComplete::parse().to_options();

    let result = parser.run_inner(&[] as &[&str]);
    assert!(result.is_ok(), "Failed to parse: {:?}", result);
    let opts = result.unwrap();
    assert_eq!(opts.files, Vec::<String>::new());
}

#[test]
fn group_with_many_multiple() {
    let parser = GroupWithManyComplete::parse().to_options();

    let result = parser.run_inner(&["--files", "one.txt", "--files", "two.txt", "--files", "three.txt"]);
    assert!(result.is_ok(), "Failed to parse: {:?}", result);
    let opts = result.unwrap();
    assert_eq!(opts.files, vec!["one.txt", "two.txt", "three.txt"]);
}

// ============================================================================
// Group with guard and complete
// ============================================================================

#[derive(Debug, Clone, PartialEq)]
#[derive(bpaf_unsynn::Bpaf)]
struct GroupWithGuardComplete {
    #[bpaf(long, argument("NUM"), complete(|n: &usize| vec![(n.to_string(), None)]), group("numbers"), guard(|x: &usize| *x > 0, "must be positive"))]
    count: usize,
}

#[test]
fn group_with_guard_valid() {
    let parser = GroupWithGuardComplete::parse().to_options();

    let result = parser.run_inner(&["--count", "5"]);
    assert!(result.is_ok(), "Failed to parse: {:?}", result);
    let opts = result.unwrap();
    assert_eq!(opts.count, 5);
}

// ============================================================================
// Complex struct with multiple groups and complete
// ============================================================================

#[derive(Debug, Clone, PartialEq)]
#[derive(bpaf_unsynn::Bpaf)]
struct ComplexWithGroupsComplete {
    /// Source files
    #[bpaf(short('s'), long("source"), argument("FILE"), complete(|s: &String| vec![(s.clone(), None)]), group("source-files"), many)]
    sources: Vec<String>,

    /// Output file
    #[bpaf(short('o'), long("output"), argument("FILE"), complete(|s: &String| vec![(s.clone(), None)]), group("output-files"))]
    output: String,

    /// Verbose flag (no group)
    #[bpaf(short('v'), long("verbose"))]
    verbose: bool,
}

#[test]
fn complex_with_groups_basic() {
    let parser = ComplexWithGroupsComplete::parse().to_options();

    let result = parser.run_inner(&[
        "-s", "main.c",
        "-s", "util.c",
        "-o", "program",
        "-v",
    ]);
    assert!(result.is_ok(), "Failed to parse: {:?}", result);
    let opts = result.unwrap();
    assert_eq!(opts.sources, vec!["main.c", "util.c"]);
    assert_eq!(opts.output, "program");
    assert_eq!(opts.verbose, true);
}

#[test]
fn complex_with_groups_minimal() {
    let parser = ComplexWithGroupsComplete::parse().to_options();

    // Only required field
    let result = parser.run_inner(&["-o", "program"]);
    assert!(result.is_ok(), "Failed to parse: {:?}", result);
    let opts = result.unwrap();
    assert_eq!(opts.sources, Vec::<String>::new());
    assert_eq!(opts.output, "program");
    assert_eq!(opts.verbose, false);
}
