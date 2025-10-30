//! Test that #[bpaf(...)] attributes work correctly

use bpaf::Parser;

#[derive(Debug, Clone)]
#[derive(bpaf_unsynn::Bpaf)]
struct WithShortLong {
    #[bpaf(short, long)]
    verbose: bool,
}

#[test]
fn short_and_long_work() {
    let parser = WithShortLong::parse().to_options();

    // Test with short flag
    let result = parser.run_inner(&["-v"]);
    assert!(result.is_ok());
    let opts = result.unwrap();
    assert!(opts.verbose);

    // Test with long flag
    let result = parser.run_inner(&["--verbose"]);
    assert!(result.is_ok());
    let opts = result.unwrap();
    assert!(opts.verbose);

    // Test without flag
    let result = parser.run_inner(&[] as &[&str]);
    assert!(result.is_ok());
    let opts = result.unwrap();
    assert!(!opts.verbose);
}

#[derive(Debug, Clone)]
#[derive(bpaf_unsynn::Bpaf)]
struct WithCustomNames {
    #[bpaf(short('d'), long("debug"))]
    verbose: bool,

    #[bpaf(long("output-file"))]
    output: String,
}

#[test]
fn custom_names_work() {
    let parser = WithCustomNames::parse().to_options();

    // Test custom short
    let result = parser.run_inner(&["-d", "--output-file", "test.txt"]);
    assert!(result.is_ok());
    let opts = result.unwrap();
    assert!(opts.verbose);
    assert_eq!(opts.output, "test.txt");

    // Test custom long
    let result = parser.run_inner(&["--debug", "--output-file", "test.txt"]);
    assert!(result.is_ok());
    let opts = result.unwrap();
    assert!(opts.verbose);
    assert_eq!(opts.output, "test.txt");
}

#[derive(Debug, Clone)]
#[derive(bpaf_unsynn::Bpaf)]
struct OnlyShort {
    #[bpaf(short)]
    flag: bool,
}

#[test]
fn only_short_works() {
    let parser = OnlyShort::parse().to_options();

    // Test with short flag (should default to 'f' from field name)
    let result = parser.run_inner(&["-f"]);
    assert!(result.is_ok());
    let opts = result.unwrap();
    assert!(opts.flag);
}

#[derive(Debug, Clone)]
#[derive(bpaf_unsynn::Bpaf)]
struct OnlyLong {
    #[bpaf(long)]
    my_flag: bool,
}

#[test]
fn only_long_works() {
    let parser = OnlyLong::parse().to_options();

    // Test with long flag (should default to "my-flag" from field name)
    let result = parser.run_inner(&["--my-flag"]);
    assert!(result.is_ok());
    let opts = result.unwrap();
    assert!(opts.my_flag);
}
