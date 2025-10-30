//! Tests for struct-level mode attributes: options, command, parser

use bpaf::Parser;

// ============================================================================
// Parser mode (default) - generates impl Parser<T>
// ============================================================================

#[derive(Debug, Clone, PartialEq)]
#[derive(bpaf_unsynn::Bpaf)]
struct DefaultMode {
    #[bpaf(short, long)]
    verbose: bool,
}

#[test]
fn default_parser_mode() {
    let parser = DefaultMode::parse().to_options();

    let result = parser.run_inner(&["--verbose"]);
    assert!(result.is_ok(), "Failed to parse: {:?}", result);
    let opts = result.unwrap();
    assert_eq!(opts.verbose, true);
}

// ============================================================================
// Parser mode (explicit) - generates impl Parser<T>
// ============================================================================

#[derive(Debug, Clone, PartialEq)]
#[derive(bpaf_unsynn::Bpaf)]
#[bpaf(parser)]
struct ExplicitParserMode {
    #[bpaf(short, long)]
    verbose: bool,
}

#[test]
fn explicit_parser_mode() {
    let parser = ExplicitParserMode::parse().to_options();

    let result = parser.run_inner(&["--verbose"]);
    assert!(result.is_ok(), "Failed to parse: {:?}", result);
    let opts = result.unwrap();
    assert_eq!(opts.verbose, true);
}

// ============================================================================
// Options mode - generates OptionParser<T>
// ============================================================================

#[derive(Debug, Clone, PartialEq)]
#[derive(bpaf_unsynn::Bpaf)]
#[bpaf(options)]
struct OptionsMode {
    #[bpaf(short, long)]
    verbose: bool,
}

#[test]
fn options_mode_basic() {
    let parser = OptionsMode::parse();

    let result = parser.run_inner(&["--verbose"]);
    assert!(result.is_ok(), "Failed to parse: {:?}", result);
    let opts = result.unwrap();
    assert_eq!(opts.verbose, true);
}

#[test]
fn options_mode_help() {
    let parser = OptionsMode::parse();

    // OptionParser provides built-in help/version handling
    // This should fail with help output request
    let result = parser.run_inner(&["--help"]);
    assert!(result.is_err(), "Should request help");
}

// ============================================================================
// Command mode - generates impl Parser<T> with .command()
// ============================================================================

#[derive(Debug, Clone, PartialEq)]
#[derive(bpaf_unsynn::Bpaf)]
#[bpaf(command)]
struct CommandMode {
    #[bpaf(short, long)]
    verbose: bool,
}

#[test]
fn command_mode_basic() {
    let parser = CommandMode::parse().to_options();

    // Command mode requires the command name as first argument
    let result = parser.run_inner(&["commandmode", "--verbose"]);
    assert!(result.is_ok(), "Failed to parse: {:?}", result);
    let opts = result.unwrap();
    assert_eq!(opts.verbose, true);
}

#[test]
fn command_mode_without_command_fails() {
    let parser = CommandMode::parse().to_options();

    // Without the command name, parsing should fail
    let result = parser.run_inner(&["--verbose"]);
    assert!(result.is_err(), "Should fail without command name");
}

// ============================================================================
// Combined modes with other attributes
// ============================================================================

#[derive(Debug, Clone, PartialEq)]
#[derive(bpaf_unsynn::Bpaf)]
#[bpaf(options, adjacent)]
struct OptionsWithAdjacent {
    #[bpaf(short('r'), long, req_flag(()))]
    rect: (),

    #[bpaf(short, long, argument("PX"))]
    width: usize,
}

#[test]
fn options_with_adjacent() {
    let parser = OptionsWithAdjacent::parse();

    let result = parser.run_inner(&["-r", "-w", "100"]);
    assert!(result.is_ok(), "Failed to parse: {:?}", result);
    let opts = result.unwrap();
    assert_eq!(opts.width, 100);
}

#[derive(Debug, Clone, PartialEq)]
#[derive(bpaf_unsynn::Bpaf)]
#[bpaf(command, adjacent)]
struct CommandWithAdjacent {
    #[bpaf(short('r'), long, req_flag(()))]
    rect: (),

    #[bpaf(short, long, argument("PX"))]
    width: usize,
}

#[test]
fn command_with_adjacent() {
    let parser = CommandWithAdjacent::parse().to_options();

    let result = parser.run_inner(&["commandwithadjacent", "-r", "-w", "100"]);
    assert!(result.is_ok(), "Failed to parse: {:?}", result);
    let opts = result.unwrap();
    assert_eq!(opts.width, 100);
}

// ============================================================================
// Complex struct with options mode
// ============================================================================

#[derive(Debug, Clone, PartialEq)]
#[derive(bpaf_unsynn::Bpaf)]
#[bpaf(options)]
struct ComplexOptions {
    /// Enable verbose output
    #[bpaf(short, long)]
    verbose: bool,

    /// Input file
    #[bpaf(short, long, argument("FILE"))]
    input: Option<String>,

    /// Output files
    #[bpaf(short, long, argument("FILE"), many)]
    output: Vec<String>,

    /// Number of threads
    #[bpaf(short('j'), long, argument("N"), fallback(1))]
    jobs: usize,
}

#[test]
fn complex_options_full() {
    let parser = ComplexOptions::parse();

    let result = parser.run_inner(&[
        "-v",
        "--input", "in.txt",
        "-o", "out1.txt",
        "--output", "out2.txt",
        "-j", "4"
    ]);
    assert!(result.is_ok(), "Failed to parse: {:?}", result);
    let opts = result.unwrap();
    assert_eq!(opts.verbose, true);
    assert_eq!(opts.input, Some("in.txt".to_string()));
    assert_eq!(opts.output, vec!["out1.txt", "out2.txt"]);
    assert_eq!(opts.jobs, 4);
}

#[test]
fn complex_options_minimal() {
    let parser = ComplexOptions::parse();

    // Only required fields (none in this case)
    let result = parser.run_inner(&[] as &[&str]);
    assert!(result.is_ok(), "Failed to parse: {:?}", result);
    let opts = result.unwrap();
    assert_eq!(opts.verbose, false);
    assert_eq!(opts.input, None);
    assert_eq!(opts.output, Vec::<String>::new());
    assert_eq!(opts.jobs, 1); // fallback value
}
