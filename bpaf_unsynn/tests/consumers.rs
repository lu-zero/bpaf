//! Tests for consumer attributes
//!
//! Covers: switch, flag, argument, positional, req_flag, any, external, pure, pure_with

use bpaf::Parser;

// =============================================================================
// Switch consumer
// =============================================================================

#[derive(Debug, Clone, PartialEq, bpaf_unsynn::Bpaf)]
#[bpaf(options)]
struct SwitchConsumer {
    #[bpaf(long, switch)]
    verbose: bool,
}

#[test]
fn switch_explicit() {
    let parser = switch_consumer();

    let r = parser.run_inner(&[]).unwrap();
    assert!(!r.verbose);

    let r = parser.run_inner(&["--verbose"]).unwrap();
    assert!(r.verbose);
}

// =============================================================================
// Flag consumer (with present/absent values)
// =============================================================================

#[derive(Debug, Clone, PartialEq, bpaf_unsynn::Bpaf)]
#[bpaf(options)]
struct FlagConsumer {
    #[bpaf(long, flag(true, false))]
    enabled: bool,
}

#[test]
fn flag_with_values() {
    let parser = flag_consumer();

    let r = parser.run_inner(&[]).unwrap();
    assert!(!r.enabled); // absent value

    let r = parser.run_inner(&["--enabled"]).unwrap();
    assert!(r.enabled); // present value
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Mode {
    Fast,
    Slow,
}

#[derive(Debug, Clone, PartialEq, bpaf_unsynn::Bpaf)]
#[bpaf(options)]
struct FlagEnum {
    #[bpaf(long, flag(Mode::Fast, Mode::Slow))]
    mode: Mode,
}

#[test]
fn flag_with_enum_values() {
    let parser = flag_enum();

    let r = parser.run_inner(&[]).unwrap();
    assert_eq!(r.mode, Mode::Slow);

    let r = parser.run_inner(&["--mode"]).unwrap();
    assert_eq!(r.mode, Mode::Fast);
}

#[derive(Debug, Clone, PartialEq, bpaf_unsynn::Bpaf)]
#[bpaf(options)]
struct FlagString {
    #[bpaf(long("color"), flag(String::from("always"), String::from("never")))]
    color: String,
}

#[test]
fn flag_with_string_values() {
    let parser = flag_string();

    let r = parser.run_inner(&[]).unwrap();
    assert_eq!(r.color, "never");

    let r = parser.run_inner(&["--color"]).unwrap();
    assert_eq!(r.color, "always");
}

#[derive(Debug, Clone, PartialEq, bpaf_unsynn::Bpaf)]
#[bpaf(options)]
struct FlagNumeric {
    #[bpaf(short('v'), long, flag(1, 0))]
    verbosity: i32,
}

#[test]
fn flag_with_numeric_values() {
    let parser = flag_numeric();

    let r = parser.run_inner(&[]).unwrap();
    assert_eq!(r.verbosity, 0);

    let r = parser.run_inner(&["-v"]).unwrap();
    assert_eq!(r.verbosity, 1);

    let r = parser.run_inner(&["--verbosity"]).unwrap();
    assert_eq!(r.verbosity, 1);
}

// =============================================================================
// Argument consumer
// =============================================================================

#[derive(Debug, Clone, PartialEq, bpaf_unsynn::Bpaf)]
#[bpaf(options)]
struct ArgumentConsumer {
    #[bpaf(long, argument("FILE"))]
    input: String,
}

#[test]
fn argument_with_metavar() {
    let parser = argument_consumer();

    let r = parser.run_inner(&["--input", "file.txt"]).unwrap();
    assert_eq!(r.input, "file.txt");
}

// =============================================================================
// Positional consumer
// =============================================================================

#[derive(Debug, Clone, PartialEq, bpaf_unsynn::Bpaf)]
#[bpaf(options)]
struct PositionalConsumer {
    #[bpaf(positional("FILE"))]
    file: String,
}

#[test]
fn positional_argument() {
    let parser = positional_consumer();

    let r = parser.run_inner(&["myfile.txt"]).unwrap();
    assert_eq!(r.file, "myfile.txt");
}

#[derive(Debug, Clone, PartialEq, bpaf_unsynn::Bpaf)]
#[bpaf(options)]
struct MultiplePositionals {
    #[bpaf(positional("SRC"))]
    source: String,
    #[bpaf(positional("DST"))]
    dest: String,
}

#[test]
fn multiple_positional_arguments() {
    let parser = multiple_positionals();

    let r = parser.run_inner(&["src.txt", "dst.txt"]).unwrap();
    assert_eq!(r.source, "src.txt");
    assert_eq!(r.dest, "dst.txt");
}

// =============================================================================
// ReqFlag consumer
// =============================================================================

#[derive(Debug, Clone, PartialEq, bpaf_unsynn::Bpaf)]
#[bpaf(options)]
struct ReqFlagConsumer {
    #[bpaf(long("create"), req_flag(()))]
    create: (),
    name: String,
}

#[test]
fn req_flag_required() {
    let parser = req_flag_consumer();

    // With required flag
    let r = parser.run_inner(&["--create", "--name", "test"]).unwrap();
    assert_eq!(r.create, ());
    assert_eq!(r.name, "test");

    // Without required flag - fails
    let r = parser.run_inner(&["--name", "test"]);
    assert!(r.is_err());
}

// =============================================================================
// Any consumer
// =============================================================================

#[derive(Debug, Clone, PartialEq, bpaf_unsynn::Bpaf)]
#[bpaf(options)]
struct AnyConsumer {
    #[bpaf(any("FILE", check_file))]
    file: String,
}

fn check_file(s: String) -> Option<String> {
    if s.starts_with('-') {
        None
    } else {
        Some(s)
    }
}

#[test]
fn any_with_check() {
    let parser = any_consumer();

    let r = parser.run_inner(&["file.txt"]).unwrap();
    assert_eq!(r.file, "file.txt");
}

#[derive(Debug, Clone, PartialEq, bpaf_unsynn::Bpaf)]
#[bpaf(options)]
struct NamedAnyConsumer {
    #[bpaf(short, long, any("INPUT", check_input))]
    value: String,
}

fn check_input(s: String) -> Option<String> {
    if !s.is_empty() {
        Some(s)
    } else {
        None
    }
}

#[test]
fn test_named_any_consumer() {
    let parser = named_any_consumer();

    // Test with short flag
    let r = parser.run_inner(&["-v", "test"]).unwrap();
    assert_eq!(r.value, "test");

    // Test with long flag
    let r = parser.run_inner(&["--value", "data"]).unwrap();
    assert_eq!(r.value, "data");
}

// =============================================================================
// External consumer
// =============================================================================

#[derive(Debug, Clone, PartialEq, bpaf_unsynn::Bpaf)]
#[bpaf(options)]
struct ExternalConsumer {
    #[bpaf(external(custom_parser))]
    value: i32,
}

fn custom_parser() -> impl Parser<i32> {
    bpaf::long("value").argument::<i32>("NUM")
}

#[test]
fn external_parser() {
    let parser = external_consumer();

    let r = parser.run_inner(&["--value", "42"]).unwrap();
    assert_eq!(r.value, 42);
}

/// External with default field name
#[derive(Debug, Clone, PartialEq, bpaf_unsynn::Bpaf)]
#[bpaf(options)]
struct ExternalDefault {
    #[bpaf(external)]
    count: usize,
}

fn count() -> impl Parser<usize> {
    bpaf::long("count").argument::<usize>("N")
}

#[test]
fn external_default_name() {
    let parser = external_default();

    let r = parser.run_inner(&["--count", "10"]).unwrap();
    assert_eq!(r.count, 10);
}

// =============================================================================
// Pure consumer
// =============================================================================

#[derive(Debug, Clone, PartialEq, bpaf_unsynn::Bpaf)]
#[bpaf(options)]
struct PureConsumer {
    #[bpaf(pure(42))]
    value: i32,
    name: String,
}

#[test]
fn pure_value() {
    let parser = pure_consumer();

    let r = parser.run_inner(&["--name", "test"]).unwrap();
    assert_eq!(r.value, 42);
    assert_eq!(r.name, "test");
}

// =============================================================================
// PureWith consumer
// =============================================================================

fn get_default_value() -> Result<i32, &'static str> {
    Ok(100)
}

#[derive(Debug, Clone, PartialEq, bpaf_unsynn::Bpaf)]
#[bpaf(options)]
struct PureWithConsumer {
    #[bpaf(pure_with(get_default_value))]
    value: i32,
    name: String,
}

#[test]
fn pure_with_function() {
    let parser = pure_with_consumer();

    let r = parser.run_inner(&["--name", "test"]).unwrap();
    assert_eq!(r.value, 100);
    assert_eq!(r.name, "test");
}
