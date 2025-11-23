//! Tests for help-related attributes
//!
//! Covers: doc comments, help attribute, version, header, footer, usage,
//! descr (top-level), and help formatting

// =============================================================================
// Doc comments -> help text
// =============================================================================

/// A simple command with doc comments
#[derive(Debug, Clone, PartialEq, bpaf_unsynn::Bpaf)]
#[bpaf(options)]
struct DocComments {
    /// Enable verbose output
    verbose: bool,
    /// The input file to process
    input: String,
}

#[test]
fn doc_comments_compile() {
    let parser = DocComments::parse();

    let r = parser.run_inner(&["--input", "test.txt"]).unwrap();
    assert_eq!(r.input, "test.txt");
}

// =============================================================================
// Help attribute (explicit)
// =============================================================================

#[derive(Debug, Clone, PartialEq, bpaf_unsynn::Bpaf)]
#[bpaf(options)]
struct HelpExplicit {
    #[bpaf(long, help("Enable verbose mode for detailed output"))]
    verbose: bool,
}

#[test]
fn help_explicit_compiles() {
    let parser = HelpExplicit::parse();
    let r = parser.run_inner(&["--verbose"]).unwrap();
    assert!(r.verbose);
}

// =============================================================================
// Doc comment with help attribute combined
// =============================================================================

#[derive(Debug, Clone, PartialEq, bpaf_unsynn::Bpaf)]
#[bpaf(options)]
struct DocAndHelp {
    /// This is the doc comment
    #[bpaf(long, help("This is explicit help"))]
    option: bool,
}

#[test]
fn doc_and_help_combined() {
    let parser = DocAndHelp::parse();
    let r = parser.run_inner(&["--option"]).unwrap();
    assert!(r.option);
}

// =============================================================================
// Version attribute
// =============================================================================

#[derive(Debug, Clone, PartialEq, bpaf_unsynn::Bpaf)]
#[bpaf(options, version)]
struct WithVersion {
    name: String,
}

#[test]
fn version_compiles() {
    let parser = WithVersion::parse();
    let r = parser.run_inner(&["--name", "test"]).unwrap();
    assert_eq!(r.name, "test");
}

/// Explicit version string
#[derive(Debug, Clone, PartialEq, bpaf_unsynn::Bpaf)]
#[bpaf(options, version("1.2.3"))]
struct ExplicitVersion {
    name: String,
}

#[test]
fn explicit_version_compiles() {
    let parser = ExplicitVersion::parse();
    let r = parser.run_inner(&["--name", "test"]).unwrap();
    assert_eq!(r.name, "test");
}

// =============================================================================
// Header and footer
// =============================================================================

#[derive(Debug, Clone, PartialEq, bpaf_unsynn::Bpaf)]
#[bpaf(
    options,
    header("This appears before help"),
    footer("This appears after help")
)]
struct HeaderFooter {
    value: String,
}

#[test]
fn header_footer_compile() {
    let parser = HeaderFooter::parse();
    let r = parser.run_inner(&["--value", "test"]).unwrap();
    assert_eq!(r.value, "test");
}

// =============================================================================
// Usage attribute
// =============================================================================

#[derive(Debug, Clone, PartialEq, bpaf_unsynn::Bpaf)]
#[bpaf(options, usage("CUSTOM USAGE STRING"))]
struct CustomUsage {
    value: String,
}

#[test]
fn custom_usage_compiles() {
    let parser = CustomUsage::parse();
    let r = parser.run_inner(&["--value", "test"]).unwrap();
    assert_eq!(r.value, "test");
}

// =============================================================================
// Descr attribute (top-level description)
// =============================================================================

#[derive(Debug, Clone, PartialEq, bpaf_unsynn::Bpaf)]
#[bpaf(options, descr("This is the program description"))]
struct WithDescr {
    value: String,
}

#[test]
fn descr_compiles() {
    let parser = WithDescr::parse();
    let r = parser.run_inner(&["--value", "test"]).unwrap();
    assert_eq!(r.value, "test");
}

// =============================================================================
// Multiple help decorations combined
// =============================================================================

/// Program doc comment description
#[derive(Debug, Clone, PartialEq, bpaf_unsynn::Bpaf)]
#[bpaf(
    options,
    version("2.0.0"),
    header("=== Header ==="),
    footer("=== Footer ===")
)]
struct FullyDecorated {
    /// Enable verbose mode
    #[bpaf(short, long)]
    verbose: bool,
    /// Input file path
    #[bpaf(short('i'), long("input"))]
    file: Option<String>,
}

#[test]
fn fully_decorated_works() {
    let parser = FullyDecorated::parse();

    let r = parser.run_inner(&[]).unwrap();
    assert!(!r.verbose);
    assert_eq!(r.file, None);

    let r = parser.run_inner(&["-v", "-i", "test.txt"]).unwrap();
    assert!(r.verbose);
    assert_eq!(r.file, Some("test.txt".to_string()));
}

// =============================================================================
// Multi-line doc comments
// =============================================================================

/// This is a multi-line
/// doc comment that spans
/// several lines
#[derive(Debug, Clone, PartialEq, bpaf_unsynn::Bpaf)]
#[bpaf(options)]
struct MultiLineDoc {
    /// First line of help
    /// Second line of help
    /// Third line of help
    option: bool,
}

#[test]
fn multi_line_doc_compiles() {
    let parser = MultiLineDoc::parse();
    let r = parser.run_inner(&[]).unwrap();
    assert!(!r.option);
}

// =============================================================================
// Multi-paragraph doc comments (descr/header/footer)
// =============================================================================

/// This is the description paragraph.
/// It can span multiple lines.
///
///
/// This is the header paragraph.
/// It appears after a double empty line.
///
///
/// This is the footer paragraph.
/// Everything from the third paragraph onwards
/// becomes the footer.
#[derive(Debug, Clone, PartialEq, bpaf_unsynn::Bpaf)]
#[bpaf(options)]
struct MultiParagraphDoc {
    /// Option help text
    #[bpaf(long)]
    verbose: bool,
}

#[test]
fn multi_paragraph_doc_splits_correctly() {
    let parser = MultiParagraphDoc::parse();

    // Verify parsing works
    let r = parser.run_inner(&["--verbose"]).unwrap();
    assert!(r.verbose);

    // Verify help output has paragraphs in correct positions
    let help = parser
        .run_inner(&["--help"])
        .unwrap_err()
        .unwrap_stdout();

    // Find positions of each section
    let descr_pos = help.find("This is the description paragraph");
    let usage_pos = help.find("Usage:");
    let header_pos = help.find("This is the header paragraph");
    let options_pos = help.find("Available options:");
    let footer_pos = help.find("This is the footer paragraph");

    // All sections must be present
    let descr_pos = descr_pos.expect("Description should be present in help");
    let usage_pos = usage_pos.expect("Usage should be present in help");
    let header_pos = header_pos.expect("Header should be present in help");
    let options_pos = options_pos.expect("Available options should be present in help");
    let footer_pos = footer_pos.expect("Footer should be present in help");

    // Verify correct ordering: descr < usage < header < options < footer
    assert!(
        descr_pos < usage_pos,
        "Description ({}) should come before usage ({})\n{}",
        descr_pos,
        usage_pos,
        help
    );
    assert!(
        usage_pos < header_pos,
        "Usage ({}) should come before header ({})\n{}",
        usage_pos,
        header_pos,
        help
    );
    assert!(
        header_pos < options_pos,
        "Header ({}) should come before options ({})\n{}",
        header_pos,
        options_pos,
        help
    );
    assert!(
        options_pos < footer_pos,
        "Options ({}) should come before footer ({})\n{}",
        options_pos,
        footer_pos,
        help
    );
}
