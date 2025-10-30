//! Tests for bug fixes: positional metavar and flag consumer

use bpaf::Parser;

// Test 1: Positional with custom metavar
#[derive(Debug, Clone)]
#[derive(bpaf_unsynn::Bpaf)]
struct WithCustomMetavar {
    #[bpaf(positional("INPUT_FILE"))]
    input: String,
}

#[test]
fn positional_custom_metavar_works() {
    let parser = WithCustomMetavar::parse().to_options();

    // Test parsing positional argument
    let result = parser.run_inner(&["file.txt"]);
    assert!(result.is_ok(), "Failed to parse: {:?}", result);
    let opts = result.unwrap();
    assert_eq!(opts.input, "file.txt");

    // Check help contains custom metavar (this is a compile test mainly)
    // The metavar should be used in the generated code
}

// Test 2: Positional without metavar (should default to "ARG")
#[derive(Debug, Clone)]
#[derive(bpaf_unsynn::Bpaf)]
struct WithDefaultMetavar {
    #[bpaf(positional)]
    output: String,
}

#[test]
fn positional_default_metavar_works() {
    let parser = WithDefaultMetavar::parse().to_options();

    // Test parsing positional argument
    let result = parser.run_inner(&["output.txt"]);
    assert!(result.is_ok(), "Failed to parse: {:?}", result);
    let opts = result.unwrap();
    assert_eq!(opts.output, "output.txt");
}

// Test 3: Multiple positionals with different metavars
#[derive(Debug, Clone)]
#[derive(bpaf_unsynn::Bpaf)]
struct WithMultiplePositionals {
    #[bpaf(positional("SOURCE"))]
    source: String,

    #[bpaf(positional("DEST"))]
    destination: String,
}

#[test]
fn multiple_positionals_work() {
    let parser = WithMultiplePositionals::parse().to_options();

    let result = parser.run_inner(&["src.txt", "dest.txt"]);
    assert!(result.is_ok(), "Failed to parse: {:?}", result);
    let opts = result.unwrap();
    assert_eq!(opts.source, "src.txt");
    assert_eq!(opts.destination, "dest.txt");
}

// Test 4: Flag with present/absent values (bool)
#[derive(Debug, Clone, PartialEq)]
#[derive(bpaf_unsynn::Bpaf)]
struct WithFlag {
    #[bpaf(long, flag(true, false))]
    verbose: bool,
}

#[test]
fn flag_bool_works() {
    let parser = WithFlag::parse().to_options();

    // Test with flag present
    let result = parser.run_inner(&["--verbose"]);
    assert!(result.is_ok(), "Failed to parse with flag: {:?}", result);
    let opts = result.unwrap();
    assert_eq!(opts.verbose, true);

    // Test with flag absent
    let result = parser.run_inner(&[] as &[&str]);
    assert!(result.is_ok(), "Failed to parse without flag: {:?}", result);
    let opts = result.unwrap();
    assert_eq!(opts.verbose, false);
}

// Test 5: Flag with custom present/absent values (non-bool)
#[derive(Debug, Clone, PartialEq)]
#[derive(bpaf_unsynn::Bpaf)]
struct WithCustomFlag {
    #[bpaf(long("color"), flag(String::from("always"), String::from("never")))]
    color: String,
}

#[test]
fn flag_custom_values_work() {
    let parser = WithCustomFlag::parse().to_options();

    // Test with flag present
    let result = parser.run_inner(&["--color"]);
    assert!(result.is_ok(), "Failed to parse with flag: {:?}", result);
    let opts = result.unwrap();
    assert_eq!(opts.color, "always");

    // Test with flag absent
    let result = parser.run_inner(&[] as &[&str]);
    assert!(result.is_ok(), "Failed to parse without flag: {:?}", result);
    let opts = result.unwrap();
    assert_eq!(opts.color, "never");
}

// Test 6: Flag with numeric values
#[derive(Debug, Clone, PartialEq)]
#[derive(bpaf_unsynn::Bpaf)]
struct WithNumericFlag {
    #[bpaf(short('v'), long, flag(1, 0))]
    verbosity: i32,
}

#[test]
fn flag_numeric_works() {
    let parser = WithNumericFlag::parse().to_options();

    // Test with flag present (short)
    let result = parser.run_inner(&["-v"]);
    assert!(result.is_ok(), "Failed to parse with -v: {:?}", result);
    let opts = result.unwrap();
    assert_eq!(opts.verbosity, 1);

    // Test with flag present (long)
    let result = parser.run_inner(&["--verbosity"]);
    assert!(result.is_ok(), "Failed to parse with --verbosity: {:?}", result);
    let opts = result.unwrap();
    assert_eq!(opts.verbosity, 1);

    // Test with flag absent
    let result = parser.run_inner(&[] as &[&str]);
    assert!(result.is_ok(), "Failed to parse without flag: {:?}", result);
    let opts = result.unwrap();
    assert_eq!(opts.verbosity, 0);
}

// Test 7: Combined: flags and positionals together
#[derive(Debug, Clone, PartialEq)]
#[derive(bpaf_unsynn::Bpaf)]
struct CombinedFlagPositional {
    #[bpaf(short('q'), long, flag(true, false))]
    quiet: bool,

    #[bpaf(short('v'), long, flag(true, false))]
    verbose: bool,

    #[bpaf(positional("INPUT"))]
    input: String,

    #[bpaf(positional("OUTPUT"))]
    output: String,
}

#[test]
fn combined_flags_positionals_work() {
    let parser = CombinedFlagPositional::parse().to_options();

    // Test with all options
    let result = parser.run_inner(&["-v", "in.txt", "out.txt"]);
    assert!(result.is_ok(), "Failed to parse: {:?}", result);
    let opts = result.unwrap();
    assert_eq!(opts.quiet, false);
    assert_eq!(opts.verbose, true);
    assert_eq!(opts.input, "in.txt");
    assert_eq!(opts.output, "out.txt");

    // Test with no flags
    let result = parser.run_inner(&["in.txt", "out.txt"]);
    assert!(result.is_ok(), "Failed to parse: {:?}", result);
    let opts = result.unwrap();
    assert_eq!(opts.quiet, false);
    assert_eq!(opts.verbose, false);
    assert_eq!(opts.input, "in.txt");
    assert_eq!(opts.output, "out.txt");

    // Test with both flags
    let result = parser.run_inner(&["-q", "-v", "in.txt", "out.txt"]);
    assert!(result.is_ok(), "Failed to parse: {:?}", result);
    let opts = result.unwrap();
    assert_eq!(opts.quiet, true);
    assert_eq!(opts.verbose, true);
    assert_eq!(opts.input, "in.txt");
    assert_eq!(opts.output, "out.txt");
}
