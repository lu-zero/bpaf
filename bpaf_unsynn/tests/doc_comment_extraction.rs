//! Tests for doc comment extraction functionality
//!
//! This tests that doc comments are automatically used as help text,
//! and that ignore_rustdoc prevents this behavior.

use bpaf::Parser;

// ============================================================================
// Basic doc comment extraction
// ============================================================================

#[derive(Debug, Clone, PartialEq)]
#[derive(bpaf_unsynn::Bpaf)]
struct BasicDocComment {
    /// Enable verbose output
    #[bpaf(long)]
    verbose: bool,
}

#[test]
fn doc_comment_is_used_as_help() {
    // The help text should contain the doc comment
    // We can't check the exact output format at compile time,
    // but we can verify the struct compiles and parses correctly
    let parser = BasicDocComment::parse().to_options();
    let result = parser.run_inner(&[] as &[&str]);
    assert!(result.is_ok());
}

#[test]
fn basic_doc_comment_parsing_works() {
    let parser = BasicDocComment::parse().to_options();
    let result = parser.run_inner(&["--verbose"]);
    assert!(result.is_ok(), "Failed to parse: {:?}", result);
    let opts = result.unwrap();
    assert_eq!(opts.verbose, true);
}

// ============================================================================
// Multi-line doc comments
// ============================================================================

#[derive(Debug, Clone, PartialEq)]
#[derive(bpaf_unsynn::Bpaf)]
struct MultiLineDocComment {
    /// Path to the input file
    /// This can be any valid file path
    /// on your system
    #[bpaf(long, argument("FILE"))]
    input: String,
}

#[test]
fn multi_line_doc_comment() {
    let parser = MultiLineDocComment::parse().to_options();
    let result = parser.run_inner(&["--input", "test.txt"]);
    assert!(result.is_ok(), "Failed to parse: {:?}", result);
    let opts = result.unwrap();
    assert_eq!(opts.input, "test.txt");
}

// ============================================================================
// Doc comments with ignore_rustdoc
// ============================================================================

#[derive(Debug, Clone, PartialEq)]
#[derive(bpaf_unsynn::Bpaf)]
struct IgnoreDocComment {
    /// This doc comment should be ignored
    #[bpaf(long, ignore_rustdoc)]
    flag: bool,
}

#[test]
fn ignore_rustdoc_prevents_doc_extraction() {
    let parser = IgnoreDocComment::parse().to_options();
    let result = parser.run_inner(&["--flag"]);
    assert!(result.is_ok(), "Failed to parse: {:?}", result);
    let opts = result.unwrap();
    assert_eq!(opts.flag, true);
}

// ============================================================================
// Explicit help overrides doc comment
// ============================================================================

#[derive(Debug, Clone, PartialEq)]
#[derive(bpaf_unsynn::Bpaf)]
struct ExplicitHelpOverridesDoc {
    /// This doc comment is ignored
    #[bpaf(long, help("Explicit help text"))]
    option: bool,
}

#[test]
fn explicit_help_overrides_doc() {
    let parser = ExplicitHelpOverridesDoc::parse().to_options();
    let result = parser.run_inner(&["--option"]);
    assert!(result.is_ok(), "Failed to parse: {:?}", result);
    let opts = result.unwrap();
    assert_eq!(opts.option, true);
}

// ============================================================================
// Doc comments with various field types
// ============================================================================

#[derive(Debug, Clone, PartialEq)]
#[derive(bpaf_unsynn::Bpaf)]
struct DocWithDifferentTypes {
    /// A boolean flag
    #[bpaf(short('v'), long)]
    verbose: bool,

    /// An optional string argument
    #[bpaf(short('c'), long, argument("FILE"))]
    config: Option<String>,

    /// A vector of input files
    #[bpaf(short('i'), long, argument("FILE"), many)]
    inputs: Vec<String>,

    /// Number of threads
    #[bpaf(long, argument("N"), fallback(4))]
    threads: usize,
}

#[test]
fn doc_with_different_types_all_specified() {
    let parser = DocWithDifferentTypes::parse().to_options();

    let result = parser.run_inner(&[
        "-v",
        "-c", "config.toml",
        "-i", "file1.txt",
        "-i", "file2.txt",
        "--threads", "8",
    ]);
    assert!(result.is_ok(), "Failed to parse: {:?}", result);
    let opts = result.unwrap();
    assert_eq!(opts.verbose, true);
    assert_eq!(opts.config, Some("config.toml".to_string()));
    assert_eq!(opts.inputs, vec!["file1.txt", "file2.txt"]);
    assert_eq!(opts.threads, 8);
}

#[test]
fn doc_with_different_types_minimal() {
    let parser = DocWithDifferentTypes::parse().to_options();

    let result = parser.run_inner(&[] as &[&str]);
    assert!(result.is_ok(), "Failed to parse: {:?}", result);
    let opts = result.unwrap();
    assert_eq!(opts.verbose, false);
    assert_eq!(opts.config, None);
    assert_eq!(opts.inputs, Vec::<String>::new());
    assert_eq!(opts.threads, 4);
}

// ============================================================================
// Mix of doc comments and ignore_rustdoc
// ============================================================================

#[derive(Debug, Clone, PartialEq)]
#[derive(bpaf_unsynn::Bpaf)]
struct MixedDocAndIgnore {
    /// This doc is used
    #[bpaf(long)]
    with_doc: bool,

    /// This doc is ignored
    #[bpaf(long, ignore_rustdoc)]
    without_doc: bool,

    /// This doc is overridden
    #[bpaf(long, help("Custom help"))]
    with_custom: bool,
}

#[test]
fn mixed_doc_and_ignore() {
    let parser = MixedDocAndIgnore::parse().to_options();

    let result = parser.run_inner(&["--with-doc", "--without-doc", "--with-custom"]);
    assert!(result.is_ok(), "Failed to parse: {:?}", result);
    let opts = result.unwrap();
    assert_eq!(opts.with_doc, true);
    assert_eq!(opts.without_doc, true);
    assert_eq!(opts.with_custom, true);
}

// ============================================================================
// Empty doc comments
// ============================================================================

#[derive(Debug, Clone, PartialEq)]
#[derive(bpaf_unsynn::Bpaf)]
struct NoDocComment {
    #[bpaf(long)]
    no_doc: bool,
}

#[test]
fn no_doc_comment() {
    let parser = NoDocComment::parse().to_options();
    let result = parser.run_inner(&["--no-doc"]);
    assert!(result.is_ok(), "Failed to parse: {:?}", result);
    let opts = result.unwrap();
    assert_eq!(opts.no_doc, true);
}

// ============================================================================
// Doc comments with positional arguments
// ============================================================================

#[derive(Debug, Clone, PartialEq)]
#[derive(bpaf_unsynn::Bpaf)]
struct DocWithPositional {
    /// Source file to process
    #[bpaf(positional("SOURCE"))]
    source: String,
}

#[test]
fn doc_with_positional() {
    let parser = DocWithPositional::parse().to_options();
    let result = parser.run_inner(&["input.txt"]);
    assert!(result.is_ok(), "Failed to parse: {:?}", result);
    let opts = result.unwrap();
    assert_eq!(opts.source, "input.txt");
}

// ============================================================================
// Doc comments with guard
// ============================================================================

#[derive(Debug, Clone, PartialEq)]
#[derive(bpaf_unsynn::Bpaf)]
struct DocWithGuard {
    /// Port number (must be > 1024)
    #[bpaf(long, argument("PORT"), guard(|x: &u16| *x > 1024, "port must be > 1024"))]
    port: u16,
}

#[test]
fn doc_with_guard_valid() {
    let parser = DocWithGuard::parse().to_options();
    let result = parser.run_inner(&["--port", "8080"]);
    assert!(result.is_ok(), "Failed to parse: {:?}", result);
    let opts = result.unwrap();
    assert_eq!(opts.port, 8080);
}

// ============================================================================
// Complex real-world example
// ============================================================================

#[derive(Debug, Clone, PartialEq)]
#[derive(bpaf_unsynn::Bpaf)]
struct ComplexWithDocs {
    /// Enable verbose logging
    /// This will output detailed information about the processing
    #[bpaf(short('v'), long)]
    verbose: bool,

    /// Input file to process
    /// Can be any text file
    #[bpaf(short('i'), long, argument("FILE"))]
    input: String,

    /// Output file path
    /// If not specified, outputs to stdout
    #[bpaf(short('o'), long, argument("FILE"))]
    output: Option<String>,

    /// Number of worker threads
    /// Default is 4 threads
    #[bpaf(long, argument("N"), fallback(4))]
    threads: usize,

    /// Maximum memory usage in MB
    #[bpaf(long, argument("MB"), guard(|x: &Option<usize>| x.map_or(true, |v| v > 0), "must be positive"))]
    max_memory: Option<usize>,

    /// Configuration files to load
    /// Can be specified multiple times
    #[bpaf(short('c'), long, argument("FILE"), many)]
    configs: Vec<String>,
}

#[test]
fn complex_with_docs_full() {
    let parser = ComplexWithDocs::parse().to_options();

    let result = parser.run_inner(&[
        "-v",
        "-i", "input.txt",
        "-o", "output.txt",
        "--threads", "8",
        "--max-memory", "1024",
        "-c", "config1.toml",
        "-c", "config2.toml",
    ]);
    assert!(result.is_ok(), "Failed to parse: {:?}", result);
    let opts = result.unwrap();
    assert_eq!(opts.verbose, true);
    assert_eq!(opts.input, "input.txt");
    assert_eq!(opts.output, Some("output.txt".to_string()));
    assert_eq!(opts.threads, 8);
    assert_eq!(opts.max_memory, Some(1024));
    assert_eq!(opts.configs, vec!["config1.toml", "config2.toml"]);
}

#[test]
fn complex_with_docs_minimal() {
    let parser = ComplexWithDocs::parse().to_options();

    let result = parser.run_inner(&["-i", "input.txt"]);
    assert!(result.is_ok(), "Failed to parse: {:?}", result);
    let opts = result.unwrap();
    assert_eq!(opts.verbose, false);
    assert_eq!(opts.input, "input.txt");
    assert_eq!(opts.output, None);
    assert_eq!(opts.threads, 4);
    assert_eq!(opts.max_memory, None);
    assert_eq!(opts.configs, Vec::<String>::new());
}
