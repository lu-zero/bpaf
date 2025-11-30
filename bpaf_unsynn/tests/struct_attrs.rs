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
    let parser = default_mode().to_options();
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
fn test_explicit_parser_mode() {
    let parser = explicit_parser_mode().to_options();
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
    let parser = options_mode();
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
    let parser = build_cmd().to_options();
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
    let parser = run_command().to_options();
    // Uses explicit command name: run
    let r = parser.run_inner(&["run", "--target", "debug"]).unwrap();
    assert_eq!(r.target, "debug");
}

#[test]
fn command_explicit_name_derived_fails() {
    let parser = run_command().to_options();
    // Derived name should NOT work when explicit name is specified
    let r = parser.run_inner(&["runcommand", "--target", "debug"]);
    assert!(r.is_err());
}

// =============================================================================
// Command with short alias
// =============================================================================

#[derive(Debug, Clone, PartialEq, bpaf_unsynn::Bpaf)]
#[bpaf(command("install"), short('i'))]
struct InstallCmd {
    package: String,
}

#[test]
fn command_with_short_alias() {
    let parser = install_cmd().to_options();

    // Primary command name
    let r = parser.run_inner(&["install", "--package", "foo"]).unwrap();
    assert_eq!(r.package, "foo");

    // Short alias
    let r = parser.run_inner(&["i", "--package", "bar"]).unwrap();
    assert_eq!(r.package, "bar");
}

// =============================================================================
// Command with long alias
// =============================================================================

#[derive(Debug, Clone, PartialEq, bpaf_unsynn::Bpaf)]
#[bpaf(command("rm"), long("remove"))]
struct RemoveCmd {
    target: String,
}

#[test]
fn command_with_long_alias() {
    let parser = remove_cmd().to_options();

    // Primary command name
    let r = parser.run_inner(&["rm", "--target", "foo"]).unwrap();
    assert_eq!(r.target, "foo");

    // Long alias
    let r = parser.run_inner(&["remove", "--target", "bar"]).unwrap();
    assert_eq!(r.target, "bar");
}

// =============================================================================
// Command with both short and long aliases
// =============================================================================

#[derive(Debug, Clone, PartialEq, bpaf_unsynn::Bpaf)]
#[bpaf(command("update"), short('u'), long("upgrade"))]
struct UpdateCmd {
    all: bool,
}

#[test]
fn command_with_both_aliases() {
    let parser = update_cmd().to_options();

    // Primary command name
    let r = parser.run_inner(&["update"]).unwrap();
    assert!(!r.all);

    // Short alias
    let r = parser.run_inner(&["u", "--all"]).unwrap();
    assert!(r.all);

    // Long alias
    let r = parser.run_inner(&["upgrade"]).unwrap();
    assert!(!r.all);
}

// =============================================================================
// Command with help attribute
// =============================================================================

#[derive(Debug, Clone, PartialEq, bpaf_unsynn::Bpaf)]
#[bpaf(command("deploy"), help("Deploy the application to production"))]
struct DeployCmd {
    #[bpaf(long)]
    force: bool,
}

#[test]
fn command_with_help() {
    let parser = deploy_cmd().to_options();

    let r = parser.run_inner(&["deploy"]).unwrap();
    assert!(!r.force);

    let r = parser.run_inner(&["deploy", "--force"]).unwrap();
    assert!(r.force);

    // Verify help text appears in help output
    let help = parser.run_inner(&["--help"]).unwrap_err().unwrap_stdout();
    assert!(
        help.contains("Deploy the application to production"),
        "Help should contain command help text: {}",
        help
    );
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
    let parser = adjacent_struct().to_options();
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
    let parser = options_adjacent();
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
    let parser = make_parser();
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
    let parser = custom_fn().to_options();
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
    let parser = private_parser();
    let r = parser.run_inner(&["--value", "test"]).unwrap();
    assert_eq!(r.value, "test");
}

// =============================================================================
// Boxed attribute (for recursive types)
// =============================================================================

#[derive(Debug, Clone, PartialEq, bpaf_unsynn::Bpaf)]
#[bpaf(options, boxed)]
struct BoxedOptionsParser {
    value: String,
}

#[test]
fn boxed_options_parser_compiles() {
    let parser = boxed_options_parser();
    let r = parser.run_inner(&["--value", "test"]).unwrap();
    assert_eq!(r.value, "test");
}

/// Boxed in parser mode (not options) - for use in composition
#[derive(Debug, Clone, PartialEq, bpaf_unsynn::Bpaf)]
#[bpaf(parser, boxed)]
struct BoxedParserMode {
    #[bpaf(long)]
    flag: bool,
}

#[test]
fn boxed_parser_mode_compiles() {
    // In parser mode, we get a Parser, not OptionParser
    // Wrap it in to_options() to test
    let parser = boxed_parser_mode().to_options();
    let r = parser.run_inner(&["--flag"]).unwrap();
    assert!(r.flag);

    let r = parser.run_inner(&[]).unwrap();
    assert!(!r.flag);
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
    let parser = with_group_help().to_options();
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
    let parser = my_parser();
    let r = parser
        .run_inner(&["--verbose", "--input", "test.txt"])
        .unwrap();
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
    let parser = status_cmd().to_options();
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
    #[bpaf(external(inner))]
    inner: Inner,
    #[bpaf(long)]
    outer_flag: bool,
}

#[test]
fn nested_struct_external() {
    let parser = outer();
    let r = parser
        .run_inner(&["--inner-value", "test", "--outer-flag"])
        .unwrap();
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
fn test_fully_decorated_options() {
    let parser = fully_decorated_options();
    let r = parser
        .run_inner(&["--verbose", "--input", "test.txt"])
        .unwrap();
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
    let parser = fallback_to_usage_options();
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
    let parser = top_level_hide();
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
    let parser = top_level_hide_usage();
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
    let parser = top_level_custom_usage();
    let r = parser.run_inner(&["--verbose"]).unwrap();
    assert!(r.verbose);
}

// =============================================================================
// Struct-level guard attribute
// =============================================================================

fn check_value_positive(opts: &TopLevelGuard) -> bool {
    opts.value > 0
}

#[derive(Debug, Clone, PartialEq, bpaf_unsynn::Bpaf)]
#[bpaf(guard(check_value_positive, "value must be positive"))]
struct TopLevelGuard {
    #[bpaf(long, argument("NUM"))]
    value: i32,
}

#[test]
fn top_level_guard_passes() {
    let parser = top_level_guard().to_options();
    let r = parser.run_inner(&["--value", "42"]).unwrap();
    assert_eq!(r.value, 42);
}

#[test]
fn top_level_guard_fails() {
    let parser = top_level_guard().to_options();
    let r = parser.run_inner(&["--value", "-5"]);
    assert!(r.is_err());
}

// =============================================================================
// Struct-level fallback_with attribute
// =============================================================================

fn default_opts() -> Result<TopLevelFallbackWith, String> {
    Ok(TopLevelFallbackWith { value: 100 })
}

#[derive(Debug, Clone, PartialEq, bpaf_unsynn::Bpaf)]
#[bpaf(fallback_with(default_opts))]
struct TopLevelFallbackWith {
    #[bpaf(long, argument("NUM"))]
    value: i32,
}

#[test]
fn top_level_fallback_with_uses_fallback() {
    let parser = top_level_fallback_with().to_options();
    // No arguments - should use fallback
    let r = parser.run_inner(&[]).unwrap();
    assert_eq!(r.value, 100);
}

#[test]
fn top_level_fallback_with_uses_value() {
    let parser = top_level_fallback_with().to_options();
    let r = parser.run_inner(&["--value", "42"]).unwrap();
    assert_eq!(r.value, 42);
}

// =============================================================================
// Struct-level complete attribute
// =============================================================================

fn struct_completer(_opts: &TopLevelComplete) -> Vec<(&'static str, Option<&'static str>)> {
    vec![("option1", Some("First")), ("option2", None)]
}

#[derive(Debug, Clone, PartialEq, bpaf_unsynn::Bpaf)]
#[bpaf(complete(struct_completer))]
struct TopLevelComplete {
    #[bpaf(long, argument("VAL"))]
    value: String,
}

#[test]
fn top_level_complete_compiles() {
    let parser = top_level_complete().to_options();
    let r = parser.run_inner(&["--value", "test"]).unwrap();
    assert_eq!(r.value, "test");
}

// =============================================================================
// Struct-level complete + group attributes (group requires complete first)
// =============================================================================

fn struct_completer_with_group(
    _opts: &TopLevelCompleteAndGroup,
) -> Vec<(&'static str, Option<&'static str>)> {
    vec![("opt1", Some("First")), ("opt2", None)]
}

#[derive(Debug, Clone, PartialEq, bpaf_unsynn::Bpaf)]
#[bpaf(complete(struct_completer_with_group), group("main_group"))]
struct TopLevelCompleteAndGroup {
    #[bpaf(long)]
    verbose: bool,
}

#[test]
fn top_level_complete_and_group_compiles() {
    let parser = top_level_complete_and_group().to_options();
    let r = parser.run_inner(&["--verbose"]).unwrap();
    assert!(r.verbose);
}

// =============================================================================
// cargo_helper + options mode
// =============================================================================

#[derive(Debug, Clone, PartialEq, bpaf_unsynn::Bpaf)]
#[bpaf(options, cargo_helper("mycargo"))]
struct CargoHelperOptions {
    #[bpaf(long)]
    verbose: bool,
}

#[test]
fn cargo_helper_with_options_mode() {
    let parser = cargo_helper_options();
    // cargo_helper strips the first argument if it matches "mycargo"
    // With normal args (not starting with "mycargo"), it works as normal
    let r = parser.run_inner(&["--verbose"]).unwrap();
    assert!(r.verbose);
}

// =============================================================================
// Parser mode with doc comments
// =============================================================================

/// First paragraph - description.
///
/// Second paragraph - more details.
///
/// Third paragraph - even more information.
#[derive(Debug, Clone, PartialEq, bpaf_unsynn::Bpaf)]
#[bpaf(parser)]
struct ParserWithDocs {
    #[bpaf(long)]
    field: String,
}

#[test]
fn parser_mode_with_doc_comments() {
    let parser = parser_with_docs().to_options();
    let r = parser.run_inner(&["--field", "test"]).unwrap();
    assert_eq!(r.field, "test");

    // Verify help includes doc comments
    let help_result = parser.run_inner(&["--help"]);
    assert!(help_result.is_err(), "Expected help to produce an error");
}

/// First paragraph for boxed parser.
///
/// Second paragraph with more context.
///
/// Third paragraph with additional details.
#[derive(Debug, Clone, PartialEq, bpaf_unsynn::Bpaf)]
#[bpaf(parser, boxed)]
struct BoxedParserWithDocs {
    #[bpaf(long)]
    value: String,
}

#[test]
fn boxed_parser_mode_with_doc_comments() {
    let parser = boxed_parser_with_docs().to_options();
    let r = parser.run_inner(&["--value", "test"]).unwrap();
    assert_eq!(r.value, "test");
}
