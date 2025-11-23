//! Integration tests for runtime behavior
//!
//! Covers: error handling, complex scenarios, edge cases, and
//! realistic usage patterns

// =============================================================================
// Error handling - missing required arguments
// =============================================================================

#[derive(Debug, Clone, PartialEq, bpaf_unsynn::Bpaf)]
#[bpaf(options)]
struct Required {
    #[bpaf(long)]
    name: String,
}

#[test]
fn missing_required_fails() {
    let parser = Required::parse();
    let r = parser.run_inner(&[]);
    assert!(r.is_err());
}

#[test]
fn required_with_value_succeeds() {
    let parser = Required::parse();
    let r = parser.run_inner(&["--name", "test"]).unwrap();
    assert_eq!(r.name, "test");
}

// =============================================================================
// Error handling - invalid values
// =============================================================================

#[derive(Debug, Clone, PartialEq, bpaf_unsynn::Bpaf)]
#[bpaf(options)]
struct NumericParsing {
    #[bpaf(long)]
    count: i32,
}

#[test]
fn invalid_number_fails() {
    let parser = NumericParsing::parse();
    let r = parser.run_inner(&["--count", "not-a-number"]);
    assert!(r.is_err());
}

#[test]
fn valid_number_succeeds() {
    let parser = NumericParsing::parse();
    let r = parser.run_inner(&["--count", "42"]).unwrap();
    assert_eq!(r.count, 42);
}

// =============================================================================
// Complex argument patterns
// =============================================================================

#[derive(Debug, Clone, PartialEq, bpaf_unsynn::Bpaf)]
#[bpaf(options)]
struct MultipleArgs {
    #[bpaf(short, long)]
    verbose: bool,
    #[bpaf(short('o'), long("output"), argument("FILE"))]
    output: Option<String>,
    #[bpaf(short('i'), long("input"), argument("FILE"), many)]
    inputs: Vec<String>,
    #[bpaf(positional("TARGET"))]
    target: Option<String>,
}

#[test]
fn complex_args_full() {
    let parser = MultipleArgs::parse();
    let r = parser
        .run_inner(&[
            "-v", "-o", "out.txt", "-i", "a.txt", "--input", "b.txt", "build",
        ])
        .unwrap();

    assert!(r.verbose);
    assert_eq!(r.output, Some("out.txt".to_string()));
    assert_eq!(r.inputs, vec!["a.txt", "b.txt"]);
    assert_eq!(r.target, Some("build".to_string()));
}

#[test]
fn complex_args_minimal() {
    let parser = MultipleArgs::parse();
    let r = parser.run_inner(&[]).unwrap();

    assert!(!r.verbose);
    assert_eq!(r.output, None);
    assert!(r.inputs.is_empty());
    assert_eq!(r.target, None);
}

// =============================================================================
// Attached values (--option=value syntax)
// =============================================================================

#[derive(Debug, Clone, PartialEq, bpaf_unsynn::Bpaf)]
#[bpaf(options)]
struct AttachedValues {
    #[bpaf(long, argument("VAL"))]
    config: String,
    #[bpaf(short('n'), argument("NUM"))]
    number: i32,
}

#[test]
fn equals_syntax_works() {
    let parser = AttachedValues::parse();
    let r = parser.run_inner(&["--config=test.cfg", "-n42"]).unwrap();
    assert_eq!(r.config, "test.cfg");
    assert_eq!(r.number, 42);
}

#[test]
fn space_syntax_works() {
    let parser = AttachedValues::parse();
    let r = parser
        .run_inner(&["--config", "test.cfg", "-n", "42"])
        .unwrap();
    assert_eq!(r.config, "test.cfg");
    assert_eq!(r.number, 42);
}

// =============================================================================
// Subcommand patterns
// =============================================================================

#[derive(Debug, Clone, PartialEq, bpaf_unsynn::Bpaf)]
struct BuildOpts {
    #[bpaf(long)]
    release: bool,
    #[bpaf(long, argument("TARGET"))]
    target: Option<String>,
}

#[derive(Debug, Clone, PartialEq, bpaf_unsynn::Bpaf)]
struct RunOpts {
    #[bpaf(long, argument("BIN"))]
    bin: Option<String>,
    #[bpaf(positional("ARGS"), many)]
    args: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, bpaf_unsynn::Bpaf)]
#[bpaf(options)]
enum Cargo {
    #[bpaf(command)]
    Build {
        #[bpaf(external(BuildOpts::parse))]
        opts: BuildOpts,
    },
    #[bpaf(command)]
    Run {
        #[bpaf(external(RunOpts::parse))]
        opts: RunOpts,
    },
}

#[test]
fn subcommand_build() {
    let parser = Cargo::parse();
    let r = parser
        .run_inner(&["build", "--release", "--target", "x86_64"])
        .unwrap();

    match r {
        Cargo::Build { opts } => {
            assert!(opts.release);
            assert_eq!(opts.target, Some("x86_64".to_string()));
        }
        _ => panic!("expected Build"),
    }
}

#[test]
fn subcommand_run() {
    let parser = Cargo::parse();
    let r = parser
        .run_inner(&["run", "--bin", "myapp", "arg1", "arg2"])
        .unwrap();

    match r {
        Cargo::Run { opts } => {
            assert_eq!(opts.bin, Some("myapp".to_string()));
            assert_eq!(opts.args, vec!["arg1", "arg2"]);
        }
        _ => panic!("expected Run"),
    }
}

// =============================================================================
// Mutually exclusive options via enum
// =============================================================================

#[derive(Debug, Clone, PartialEq, bpaf_unsynn::Bpaf)]
#[bpaf(options)]
enum Verbosity {
    #[bpaf(long)]
    Quiet,
    #[bpaf(long)]
    Verbose,
    #[bpaf(long)]
    Debug,
}

#[test]
fn mutually_exclusive_options() {
    let parser = Verbosity::parse();

    let r = parser.run_inner(&["--quiet"]).unwrap();
    assert_eq!(r, Verbosity::Quiet);

    let r = parser.run_inner(&["--verbose"]).unwrap();
    assert_eq!(r, Verbosity::Verbose);

    let r = parser.run_inner(&["--debug"]).unwrap();
    assert_eq!(r, Verbosity::Debug);
}

// =============================================================================
// Flat struct composition
// =============================================================================

#[derive(Debug, Clone, PartialEq, bpaf_unsynn::Bpaf)]
struct CommonOpts {
    #[bpaf(short, long)]
    verbose: bool,
    #[bpaf(long, argument("FILE"))]
    config: Option<String>,
}

#[derive(Debug, Clone, PartialEq, bpaf_unsynn::Bpaf)]
#[bpaf(options)]
struct AppOpts {
    #[bpaf(external(CommonOpts::parse))]
    common: CommonOpts,
    #[bpaf(long)]
    dry_run: bool,
}

#[test]
fn flat_composition() {
    let parser = AppOpts::parse();
    let r = parser
        .run_inner(&["--verbose", "--config", "app.cfg", "--dry-run"])
        .unwrap();

    assert!(r.common.verbose);
    assert_eq!(r.common.config, Some("app.cfg".to_string()));
    assert!(r.dry_run);
}

// =============================================================================
// Multiple positional arguments
// =============================================================================

#[derive(Debug, Clone, PartialEq, bpaf_unsynn::Bpaf)]
#[bpaf(options)]
struct CpCommand {
    #[bpaf(positional("SRC"))]
    source: String,
    #[bpaf(positional("DST"))]
    dest: String,
}

#[test]
fn multiple_positionals() {
    let parser = CpCommand::parse();
    let r = parser.run_inner(&["src.txt", "dst.txt"]).unwrap();

    assert_eq!(r.source, "src.txt");
    assert_eq!(r.dest, "dst.txt");
}

// =============================================================================
// Real-world CLI example
// =============================================================================

#[derive(Debug, Clone, PartialEq, bpaf_unsynn::Bpaf)]
#[bpaf(options, version("1.0.0"), descr("A realistic CLI tool"))]
struct GitLike {
    /// Enable verbose output
    #[bpaf(short, long)]
    verbose: bool,

    /// Working directory
    #[bpaf(short('C'), long("cwd"), argument("DIR"))]
    directory: Option<String>,

    /// Configuration key=value pairs
    #[bpaf(short('c'), long("config"), argument("KEY=VAL"), many)]
    configs: Vec<String>,
}

#[test]
fn realistic_cli() {
    let parser = GitLike::parse();

    // Basic usage
    let r = parser.run_inner(&["-v"]).unwrap();
    assert!(r.verbose);
    assert_eq!(r.directory, None);
    assert!(r.configs.is_empty());

    // Full usage
    let r = parser
        .run_inner(&[
            "-v",
            "-C",
            "/tmp/repo",
            "-c",
            "user.name=Test",
            "--config",
            "user.email=test@example.com",
        ])
        .unwrap();

    assert!(r.verbose);
    assert_eq!(r.directory, Some("/tmp/repo".to_string()));
    assert_eq!(
        r.configs,
        vec!["user.name=Test", "user.email=test@example.com"]
    );
}

// =============================================================================
// PathBuf and other stdlib types
// =============================================================================

use std::path::PathBuf;

#[derive(Debug, Clone, PartialEq, bpaf_unsynn::Bpaf)]
#[bpaf(options)]
struct PathArgs {
    #[bpaf(long, argument("PATH"))]
    input: PathBuf,
    #[bpaf(long, argument("PATH"))]
    output: Option<PathBuf>,
}

#[test]
fn pathbuf_args() {
    let parser = PathArgs::parse();
    let r = parser
        .run_inner(&["--input", "/home/user/file.txt", "--output", "/tmp/out.txt"])
        .unwrap();

    assert_eq!(r.input, PathBuf::from("/home/user/file.txt"));
    assert_eq!(r.output, Some(PathBuf::from("/tmp/out.txt")));
}

// =============================================================================
// Boolean flag combinations
// =============================================================================

#[derive(Debug, Clone, PartialEq, bpaf_unsynn::Bpaf)]
#[bpaf(options)]
struct Flags {
    #[bpaf(short('a'))]
    all: bool,
    #[bpaf(short('l'))]
    long_format: bool,
    #[bpaf(short('h'))]
    human_readable: bool,
}

#[test]
fn multiple_short_flags() {
    let parser = Flags::parse();

    let r = parser.run_inner(&["-a"]).unwrap();
    assert!(r.all);
    assert!(!r.long_format);
    assert!(!r.human_readable);

    let r = parser.run_inner(&["-a", "-l", "-h"]).unwrap();
    assert!(r.all);
    assert!(r.long_format);
    assert!(r.human_readable);
}
