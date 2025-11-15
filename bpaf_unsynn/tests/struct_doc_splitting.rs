//! Test struct-level doc comment splitting into descr/header/footer

use bpaf::Parser;

/// First paragraph becomes description
///
/// Second paragraph becomes header
///
/// Third paragraph
/// becomes the footer
#[derive(Debug, Clone, bpaf_unsynn::Bpaf)]
#[bpaf(options)]
struct TestSplit {
    #[bpaf(long)]
    flag: bool,
}

#[test]
fn struct_doc_split_works() {
    let parser = TestSplit::parse();
    // Just verify it compiles and runs
    let result = parser.run_inner(&["--flag"]);
    assert!(result.is_ok());
}

/// Only one paragraph
#[derive(Debug, Clone, bpaf_unsynn::Bpaf)]
#[bpaf(options)]
struct TestOneParagraph {
    #[bpaf(long)]
    flag: bool,
}

#[test]
fn single_paragraph_works() {
    let parser = TestOneParagraph::parse();
    let result = parser.run_inner(&["--flag"]);
    assert!(result.is_ok());
}

/// This is ignored
#[derive(Debug, Clone, bpaf_unsynn::Bpaf)]
#[bpaf(options, ignore_rustdoc)]
struct TestIgnoreRustdoc {
    #[bpaf(long)]
    flag: bool,
}

#[test]
fn ignore_rustdoc_struct_level() {
    let parser = TestIgnoreRustdoc::parse();
    let result = parser.run_inner(&["--flag"]);
    assert!(result.is_ok());
}

/// Doc comment for parser mode
#[derive(Debug, Clone, bpaf_unsynn::Bpaf)]
struct TestParserMode {
    #[bpaf(long)]
    flag: bool,
}

#[test]
fn parser_mode_with_docs() {
    let parser = TestParserMode::parse().to_options();
    let result = parser.run_inner(&["--flag"]);
    assert!(result.is_ok());
}
