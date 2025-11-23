//! Tests for ignore_rustdoc attribute at field level

#[derive(Debug, Clone, PartialEq, bpaf_unsynn::Bpaf)]
#[bpaf(options)]
struct IgnoreRustdocField {
    /// This doc comment should be ignored
    #[bpaf(long, ignore_rustdoc, help("Explicit help text"))]
    value: bool,
}

#[test]
fn ignore_rustdoc_on_field() {
    let parser = IgnoreRustdocField::parse();
    let r = parser.run_inner(&["--value"]).unwrap();
    assert!(r.value);
}

// Test without explicit help - rustdoc is ignored, no help
#[derive(Debug, Clone, PartialEq, bpaf_unsynn::Bpaf)]
#[bpaf(options)]
struct IgnoreRustdocNoHelp {
    /// This doc comment should be ignored, leaving no help text
    #[bpaf(long, ignore_rustdoc)]
    flag: bool,
}

#[test]
fn ignore_rustdoc_no_explicit_help() {
    let parser = IgnoreRustdocNoHelp::parse();
    let r = parser.run_inner(&["--flag"]).unwrap();
    assert!(r.flag);
}

// Test on multiple fields
#[derive(Debug, Clone, PartialEq, bpaf_unsynn::Bpaf)]
#[bpaf(options)]
struct MultipleIgnoreRustdoc {
    /// Doc for input - ignored
    #[bpaf(long, ignore_rustdoc)]
    input: String,
    /// Doc for output - used
    #[bpaf(long)]
    output: String,
}

#[test]
fn multiple_fields_mixed_rustdoc() {
    let parser = MultipleIgnoreRustdoc::parse();
    let r = parser
        .run_inner(&["--input", "in.txt", "--output", "out.txt"])
        .unwrap();
    assert_eq!(r.input, "in.txt");
    assert_eq!(r.output, "out.txt");
}
