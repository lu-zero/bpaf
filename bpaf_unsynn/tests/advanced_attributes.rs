//! Test advanced #[bpaf(...)] attributes

use bpaf::Parser;

// Test help attribute
#[derive(Debug, Clone)]
#[derive(bpaf_unsynn::Bpaf)]
#[allow(dead_code)]
struct WithHelp {
    #[bpaf(short, long, help("Enable verbose output"))]
    verbose: bool,
}

#[test]
fn help_attribute_compiles() {
    // Just verify it compiles - help text affects --help output
    let _parser = WithHelp::parse().to_options();
}

// Test fallback attribute
#[derive(Debug, Clone, PartialEq)]
#[derive(bpaf_unsynn::Bpaf)]
struct WithFallback {
    #[bpaf(long, fallback(42))]
    count: i32,

    #[bpaf(long, fallback(String::from("default")))]
    name: String,
}

#[test]
fn fallback_works() {
    let parser = WithFallback::parse().to_options();

    // Test without providing values - should use fallbacks
    let result = parser.run_inner(&[] as &[&str]);
    assert!(result.is_ok());
    let opts = result.unwrap();
    assert_eq!(opts.count, 42);
    assert_eq!(opts.name, "default");

    // Test with provided values - should override fallbacks
    let result = parser.run_inner(&["--count", "100", "--name", "custom"]);
    assert!(result.is_ok());
    let opts = result.unwrap();
    assert_eq!(opts.count, 100);
    assert_eq!(opts.name, "custom");
}

// Test positional attribute
#[derive(Debug, Clone)]
#[derive(bpaf_unsynn::Bpaf)]
struct WithPositional {
    #[bpaf(positional("FILE"))]
    input: String,

    #[bpaf(positional("OUTPUT"))]
    output: String,
}

#[test]
fn positional_works() {
    let parser = WithPositional::parse().to_options();

    // Test positional arguments
    let result = parser.run_inner(&["input.txt", "output.txt"]);
    assert!(result.is_ok());
    let opts = result.unwrap();
    assert_eq!(opts.input, "input.txt");
    assert_eq!(opts.output, "output.txt");
}

// Test explicit argument attribute
#[derive(Debug, Clone)]
#[derive(bpaf_unsynn::Bpaf)]
struct WithExplicitArgument {
    #[bpaf(long, argument)]
    count: i32,
}

#[test]
fn explicit_argument_works() {
    let parser = WithExplicitArgument::parse().to_options();

    let result = parser.run_inner(&["--count", "42"]);
    assert!(result.is_ok());
    let opts = result.unwrap();
    assert_eq!(opts.count, 42);
}

// Test explicit switch attribute
#[derive(Debug, Clone)]
#[derive(bpaf_unsynn::Bpaf)]
struct WithExplicitSwitch {
    #[bpaf(long, switch)]
    flag: bool,
}

#[test]
fn explicit_switch_works() {
    let parser = WithExplicitSwitch::parse().to_options();

    let result = parser.run_inner(&["--flag"]);
    assert!(result.is_ok());
    let opts = result.unwrap();
    assert!(opts.flag);

    let result = parser.run_inner(&[] as &[&str]);
    assert!(result.is_ok());
    let opts = result.unwrap();
    assert!(!opts.flag);
}

// Test combining multiple attributes
#[derive(Debug, Clone, PartialEq)]
#[derive(bpaf_unsynn::Bpaf)]
struct Combined {
    #[bpaf(short('v'), long("verbose"), help("Enable verbose mode"), switch)]
    verbose: bool,

    #[bpaf(short('o'), long("output"), help("Output file"), fallback(String::from("out.txt")))]
    output: String,

    #[bpaf(positional("INPUT"))]
    input: String,
}

#[test]
fn combined_attributes_work() {
    let parser = Combined::parse().to_options();

    // Test with all options
    let result = parser.run_inner(&["-v", "-o", "custom.txt", "input.txt"]);
    assert!(result.is_ok());
    let opts = result.unwrap();
    assert!(opts.verbose);
    assert_eq!(opts.output, "custom.txt");
    assert_eq!(opts.input, "input.txt");

    // Test with minimal args (using fallback)
    let result = parser.run_inner(&["input.txt"]);
    assert!(result.is_ok());
    let opts = result.unwrap();
    assert!(!opts.verbose);
    assert_eq!(opts.output, "out.txt");
    assert_eq!(opts.input, "input.txt");
}
