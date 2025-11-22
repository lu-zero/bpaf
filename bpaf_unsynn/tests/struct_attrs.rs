//! Tests for struct-level attributes
//!
//! Covers: options, command, parser, adjacent, generate, private, boxed,
//! group_help, and combined attributes

use bpaf::Parser;

// =============================================================================
// Basic struct modes
// =============================================================================

/// Default mode (parser) - generates impl Parser<T>
#[derive(Debug, Clone, PartialEq, bpaf_unsynn::Bpaf)]
struct DefaultMode {
    #[bpaf(long)]
    verbose: bool,
}

#[test]
fn default_mode_creates_parser() {
    let parser = DefaultMode::parse().to_options();
    let r = parser.run_inner(&["--verbose"]).unwrap();
    assert!(r.verbose);
}

/// Explicit parser mode
#[derive(Debug, Clone, PartialEq, bpaf_unsynn::Bpaf)]
#[bpaf(parser)]
struct ExplicitParserMode {
    #[bpaf(long)]
    verbose: bool,
}

#[test]
fn explicit_parser_mode() {
    let parser = ExplicitParserMode::parse().to_options();
    let r = parser.run_inner(&["--verbose"]).unwrap();
    assert!(r.verbose);
}

/// Options mode - generates OptionParser
#[derive(Debug, Clone, PartialEq, bpaf_unsynn::Bpaf)]
#[bpaf(options)]
struct OptionsMode {
    verbose: bool,
}

#[test]
fn options_mode_creates_option_parser() {
    let parser = OptionsMode::parse();
    let r = parser.run_inner(&["--verbose"]).unwrap();
    assert!(r.verbose);
}

/// Command mode - derived from type name (lowercase)
#[derive(Debug, Clone, PartialEq, bpaf_unsynn::Bpaf)]
#[bpaf(command)]
struct BuildCmd {
    name: String,
}

#[test]
fn command_mode_derived_name() {
    let parser = BuildCmd::parse().to_options();
    // Command name is lowercase of type name: buildcmd
    let r = parser.run_inner(&["buildcmd", "--name", "test"]).unwrap();
    assert_eq!(r.name, "test");
}

/// Command with explicit name
#[derive(Debug, Clone, PartialEq, bpaf_unsynn::Bpaf)]
#[bpaf(command("run"))]
struct RunCommand {
    target: String,
}

#[test]
fn command_explicit_name() {
    let parser = RunCommand::parse().to_options();
    // Uses explicit command name: run
    let r = parser.run_inner(&["run", "--target", "debug"]).unwrap();
    assert_eq!(r.target, "debug");
}

#[test]
fn command_explicit_name_derived_fails() {
    let parser = RunCommand::parse().to_options();
    // Derived name should NOT work when explicit name is specified
    let r = parser.run_inner(&["runcommand", "--target", "debug"]);
    assert!(r.is_err());
}

// =============================================================================
// Adjacent attribute
// =============================================================================

/// Adjacent struct - all options must be together
#[derive(Debug, Clone, PartialEq, bpaf_unsynn::Bpaf)]
#[bpaf(adjacent)]
struct AdjacentStruct {
    #[bpaf(short('r'), long, req_flag(()))]
    rect: (),
    #[bpaf(short('w'), long, argument("PX"))]
    width: usize,
}

#[test]
fn adjacent_struct_parses() {
    let parser = AdjacentStruct::parse().to_options();
    let r = parser.run_inner(&["-r", "-w", "100"]).unwrap();
    assert_eq!(r.width, 100);
}

/// Options + adjacent combined
#[derive(Debug, Clone, PartialEq, bpaf_unsynn::Bpaf)]
#[bpaf(options, adjacent)]
struct OptionsAdjacent {
    #[bpaf(short('f'), long, req_flag(()))]
    flag: (),
    #[bpaf(short('v'), long, argument("VAL"))]
    value: i32,
}

#[test]
fn options_with_adjacent() {
    let parser = OptionsAdjacent::parse();
    let r = parser.run_inner(&["-f", "-v", "42"]).unwrap();
    assert_eq!(r.value, 42);
}

// =============================================================================
// Generate attribute - custom method name
// =============================================================================

#[derive(Debug, Clone, PartialEq, bpaf_unsynn::Bpaf)]
#[bpaf(options, generate(make_parser))]
struct WithGenerate {
    value: String,
}

#[test]
fn generate_custom_name() {
    // Generated method is make_parser instead of parse
    let parser = WithGenerate::make_parser();
    let r = parser.run_inner(&["--value", "test"]).unwrap();
    assert_eq!(r.value, "test");
}

#[derive(Debug, Clone, PartialEq, bpaf_unsynn::Bpaf)]
#[bpaf(generate(custom_fn))]
struct GenerateParserMode {
    #[bpaf(long)]
    verbose: bool,
}

#[test]
fn generate_with_parser_mode() {
    let parser = GenerateParserMode::custom_fn().to_options();
    let r = parser.run_inner(&["--verbose"]).unwrap();
    assert!(r.verbose);
}

// =============================================================================
// Private attribute
// =============================================================================

#[derive(Debug, Clone, PartialEq, bpaf_unsynn::Bpaf)]
#[bpaf(options, private)]
struct PrivateParser {
    value: String,
}

#[test]
fn private_parser_compiles() {
    // Private still generates a method, just not pub
    let parser = PrivateParser::parse();
    let r = parser.run_inner(&["--value", "test"]).unwrap();
    assert_eq!(r.value, "test");
}

// =============================================================================
// Boxed attribute (for recursive types)
// =============================================================================

#[derive(Debug, Clone, PartialEq, bpaf_unsynn::Bpaf)]
#[bpaf(options, boxed)]
struct BoxedParser {
    value: String,
}

#[test]
fn boxed_parser_compiles() {
    let parser = BoxedParser::parse();
    let r = parser.run_inner(&["--value", "test"]).unwrap();
    assert_eq!(r.value, "test");
}

// =============================================================================
// Group help attribute
// =============================================================================

#[derive(Debug, Clone, PartialEq, bpaf_unsynn::Bpaf)]
#[bpaf(parser, group_help("Output settings"))]
struct WithGroupHelp {
    #[bpaf(long)]
    verbose: bool,
}

#[test]
fn group_help_at_struct_level() {
    let parser = WithGroupHelp::parse().to_options();
    let r = parser.run_inner(&["--verbose"]).unwrap();
    assert!(r.verbose);
}

// =============================================================================
// Combining multiple struct attributes
// =============================================================================

#[derive(Debug, Clone, PartialEq, bpaf_unsynn::Bpaf)]
#[bpaf(options, version("1.0.0"), generate(my_parser), boxed)]
struct CombinedAttrs {
    #[bpaf(long)]
    verbose: bool,
    #[bpaf(long, argument("FILE"))]
    input: Option<String>,
}

#[test]
fn combined_struct_attrs() {
    let parser = CombinedAttrs::my_parser();
    let r = parser.run_inner(&["--verbose", "--input", "test.txt"]).unwrap();
    assert!(r.verbose);
    assert_eq!(r.input, Some("test.txt".to_string()));
}

// =============================================================================
// Unit struct (command with no fields)
// =============================================================================

#[derive(Debug, Clone, PartialEq, bpaf_unsynn::Bpaf)]
#[bpaf(command)]
struct StatusCmd;

#[test]
fn unit_struct_command() {
    let parser = StatusCmd::parse().to_options();
    // Command name is lowercase: statuscmd
    let r = parser.run_inner(&["statuscmd"]).unwrap();
    assert_eq!(r, StatusCmd);
}

// =============================================================================
// Nested struct with external
// =============================================================================

#[derive(Debug, Clone, PartialEq, bpaf_unsynn::Bpaf)]
struct Inner {
    #[bpaf(long)]
    inner_value: String,
}

#[derive(Debug, Clone, PartialEq, bpaf_unsynn::Bpaf)]
#[bpaf(options)]
struct Outer {
    #[bpaf(external(Inner::parse))]
    inner: Inner,
    #[bpaf(long)]
    outer_flag: bool,
}

#[test]
fn nested_struct_external() {
    let parser = Outer::parse();
    let r = parser.run_inner(&["--inner-value", "test", "--outer-flag"]).unwrap();
    assert_eq!(r.inner.inner_value, "test");
    assert!(r.outer_flag);
}

// =============================================================================
// Options mode with all help decorations
// =============================================================================

#[derive(Debug, Clone, PartialEq, bpaf_unsynn::Bpaf)]
#[bpaf(
    options,
    descr("Comprehensive test program"),
    header("MyApp - Test Suite"),
    footer("Report bugs to bugs@example.com"),
    version("2.0.0"),
    max_width(100)
)]
struct FullyDecoratedOptions {
    #[bpaf(short, long)]
    verbose: bool,
    #[bpaf(short, long, argument("FILE"))]
    input: Option<String>,
}

#[test]
fn fully_decorated_options() {
    let parser = FullyDecoratedOptions::parse();
    let r = parser.run_inner(&["--verbose", "--input", "test.txt"]).unwrap();
    assert!(r.verbose);
    assert_eq!(r.input, Some("test.txt".to_string()));
}

// =============================================================================
// Fallback to usage
// =============================================================================

#[derive(Debug, Clone, PartialEq, bpaf_unsynn::Bpaf)]
#[bpaf(options, fallback_to_usage)]
struct FallbackToUsageOptions {
    #[bpaf(long)]
    verbose: bool,
}

#[test]
fn fallback_to_usage_compiles() {
    let parser = FallbackToUsageOptions::parse();
    let r = parser.run_inner(&["--verbose"]).unwrap();
    assert!(r.verbose);
}

// =============================================================================
// Top-level PostDecor attributes
// =============================================================================

#[derive(Debug, Clone, PartialEq, bpaf_unsynn::Bpaf)]
#[bpaf(options, hide)]
struct TopLevelHide {
    #[bpaf(long)]
    verbose: bool,
}

#[test]
fn top_level_hide_compiles() {
    let parser = TopLevelHide::parse();
    let r = parser.run_inner(&["--verbose"]).unwrap();
    assert!(r.verbose);
}

#[derive(Debug, Clone, PartialEq, bpaf_unsynn::Bpaf)]
#[bpaf(options, hide_usage)]
struct TopLevelHideUsage {
    #[bpaf(long)]
    verbose: bool,
}

#[test]
fn top_level_hide_usage_compiles() {
    let parser = TopLevelHideUsage::parse();
    let r = parser.run_inner(&["--verbose"]).unwrap();
    assert!(r.verbose);
}

#[derive(Debug, Clone, PartialEq, bpaf_unsynn::Bpaf)]
#[bpaf(options, custom_usage("myapp [FLAGS] <files>..."))]
struct TopLevelCustomUsage {
    #[bpaf(long)]
    verbose: bool,
}

#[test]
fn top_level_custom_usage_compiles() {
    let parser = TopLevelCustomUsage::parse();
    let r = parser.run_inner(&["--verbose"]).unwrap();
    assert!(r.verbose);
}
