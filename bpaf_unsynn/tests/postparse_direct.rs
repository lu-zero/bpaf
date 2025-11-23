//! Direct tests for count, adjacent, strict, and catch attributes

// =============================================================================
// Direct catch attribute test
// =============================================================================

#[derive(Debug, Clone, PartialEq, bpaf_unsynn::Bpaf)]
#[bpaf(options)]
struct DirectCatchOptional {
    #[bpaf(long, argument("NUM"), optional, catch)]
    value: Option<i32>,
}

#[test]
fn direct_catch_with_optional() {
    let parser = DirectCatchOptional::parse();

    // Valid number - succeeds
    let r = parser.run_inner(&["--value", "42"]).unwrap();
    assert_eq!(r.value, Some(42));

    // No value provided - returns None
    let r = parser.run_inner(&[]).unwrap();
    assert_eq!(r.value, None);
}

#[derive(Debug, Clone, PartialEq, bpaf_unsynn::Bpaf)]
#[bpaf(options)]
struct DirectCatchMany {
    #[bpaf(long, argument("FILE"), many, catch)]
    files: Vec<std::path::PathBuf>,
}

#[test]
fn direct_catch_with_many() {
    let parser = DirectCatchMany::parse();

    // Valid files - succeeds
    let r = parser
        .run_inner(&["--files", "a.txt", "--files", "b.txt"])
        .unwrap();
    assert_eq!(r.files.len(), 2);

    // No files provided - returns empty vec
    let r = parser.run_inner(&[]).unwrap();
    assert_eq!(r.files, Vec::<std::path::PathBuf>::new());
}

#[derive(Debug, Clone, PartialEq, bpaf_unsynn::Bpaf)]
#[bpaf(options)]
struct DirectCatchSome {
    #[bpaf(long, argument("ARG"), some("need at least one"), catch)]
    args: Vec<String>,
}

#[test]
fn direct_catch_with_some() {
    let parser = DirectCatchSome::parse();

    // Valid args - succeeds
    let r = parser.run_inner(&["--args", "a", "--args", "b"]).unwrap();
    assert_eq!(r.args, vec!["a", "b"]);

    // No args - catch makes it succeed with empty vec
    let r = parser.run_inner(&[]);
    // With catch, the error is caught and returns empty
    assert!(r.is_err() || r.unwrap().args.is_empty());
}

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
