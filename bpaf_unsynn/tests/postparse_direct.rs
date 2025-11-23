//! Direct tests for count, adjacent, and strict attributes
//!
//! Note: catch requires external() for proper type handling.
//! See postparse.rs for catch tests.

// =============================================================================
// Direct count attribute test
// =============================================================================

#[derive(Debug, Clone, PartialEq, bpaf_unsynn::Bpaf)]
#[bpaf(options)]
struct DirectCount {
    #[bpaf(short('v'), long("verbose"), req_flag(()), count)]
    verbosity: usize,
}

#[test]
fn direct_count_attribute() {
    let parser = DirectCount::parse();

    // One flag
    let r = parser.run_inner(&["-v"]).unwrap();
    assert_eq!(r.verbosity, 1);

    // Multiple flags
    let r = parser.run_inner(&["-v", "-v", "--verbose"]).unwrap();
    assert_eq!(r.verbosity, 3);

    // Stacked short flags
    let r = parser.run_inner(&["-vvv"]).unwrap();
    assert_eq!(r.verbosity, 3);
}

// =============================================================================
// Direct adjacent attribute test
// =============================================================================

#[derive(Debug, Clone, PartialEq, bpaf_unsynn::Bpaf)]
#[bpaf(options)]
struct DirectAdjacent {
    #[bpaf(long("key"), argument("KEY"), adjacent)]
    key: String,
    #[bpaf(long("value"), argument("VAL"))]
    value: String,
}

#[test]
fn direct_adjacent_attribute() {
    let parser = DirectAdjacent::parse();

    // Adjacent at field level requires = syntax
    let r = parser.run_inner(&["--key=foo", "--value", "bar"]).unwrap();
    assert_eq!(r.key, "foo");
    assert_eq!(r.value, "bar");
}

// =============================================================================
// Direct strict attribute test
// =============================================================================

#[derive(Debug, Clone, PartialEq, bpaf_unsynn::Bpaf)]
#[bpaf(options)]
struct DirectStrict {
    #[bpaf(positional("ARG"), strict, many)]
    args: Vec<String>,
}

#[test]
fn direct_strict_attribute() {
    let parser = DirectStrict::parse();

    // Strict requires -- before positional arguments
    let r = parser.run_inner(&["--", "arg1", "arg2"]).unwrap();
    assert_eq!(r.args, vec!["arg1", "arg2"]);
}

// =============================================================================
// Direct non_strict attribute test
// =============================================================================

#[derive(Debug, Clone, PartialEq, bpaf_unsynn::Bpaf)]
#[bpaf(options)]
struct DirectNonStrict {
    #[bpaf(positional("ARG"), non_strict)]
    arg: String,
    #[bpaf(short, long, switch)]
    flag: bool,
}

#[test]
fn direct_non_strict_attribute() {
    let parser = DirectNonStrict::parse();

    // In non-strict mode, flags can appear after positionals
    let r = parser.run_inner(&["value", "--flag"]).unwrap();
    assert_eq!(r.arg, "value");
    assert_eq!(r.flag, true);

    // Also works in normal order
    let r = parser.run_inner(&["--flag", "value"]).unwrap();
    assert_eq!(r.arg, "value");
    assert_eq!(r.flag, true);
}
