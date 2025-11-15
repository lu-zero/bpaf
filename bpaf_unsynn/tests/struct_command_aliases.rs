//! Tests for struct-level command aliases with short, long, and help attributes

use bpaf::Parser;

// ============================================================================
// Command with short alias
// ============================================================================

#[derive(Debug, Clone, PartialEq, bpaf_unsynn::Bpaf)]
#[bpaf(command, short('b'))]
struct Build {
    #[bpaf(long)]
    verbose: bool,
}

#[test]
fn command_with_short() {
    let parser = Build::parse().to_options();

    // Should work with standard command name
    let result = parser.run_inner(&["build", "--verbose"]);
    assert!(result.is_ok(), "Should parse 'build --verbose': {:?}", result);
    let cmd = result.unwrap();
    assert_eq!(cmd.verbose, true);
}

#[test]
fn command_with_short_alias() {
    let parser = Build::parse().to_options();

    // Should also work with short alias
    let result = parser.run_inner(&["b", "--verbose"]);
    assert!(result.is_ok(), "Should parse 'b --verbose': {:?}", result);
    let cmd = result.unwrap();
    assert_eq!(cmd.verbose, true);
}

// ============================================================================
// Command with long alias
// ============================================================================

#[derive(Debug, Clone, PartialEq, bpaf_unsynn::Bpaf)]
#[bpaf(command, long("compile"))]
struct CompileCmd {
    #[bpaf(long)]
    release: bool,
}

#[test]
fn command_with_long() {
    let parser = CompileCmd::parse().to_options();

    // Should work with standard command name
    let result = parser.run_inner(&["compilecmd", "--release"]);
    assert!(result.is_ok(), "Should parse 'compilecmd --release': {:?}", result);
    let cmd = result.unwrap();
    assert_eq!(cmd.release, true);
}

#[test]
fn command_with_long_alias() {
    let parser = CompileCmd::parse().to_options();

    // Should also work with long alias
    let result = parser.run_inner(&["compile", "--release"]);
    assert!(result.is_ok(), "Should parse 'compile --release': {:?}", result);
    let cmd = result.unwrap();
    assert_eq!(cmd.release, true);
}

// ============================================================================
// Command with both short and long aliases
// ============================================================================

#[derive(Debug, Clone, PartialEq, bpaf_unsynn::Bpaf)]
#[bpaf(command, short('t'), long("check"))]
struct Test {
    #[bpaf(long)]
    all: bool,
}

#[test]
fn command_with_both_using_name() {
    let parser = Test::parse().to_options();

    let result = parser.run_inner(&["test", "--all"]);
    assert!(result.is_ok(), "Should parse 'test --all': {:?}", result);
    let cmd = result.unwrap();
    assert_eq!(cmd.all, true);
}

#[test]
fn command_with_both_using_short() {
    let parser = Test::parse().to_options();

    let result = parser.run_inner(&["t", "--all"]);
    assert!(result.is_ok(), "Should parse 't --all': {:?}", result);
    let cmd = result.unwrap();
    assert_eq!(cmd.all, true);
}

#[test]
fn command_with_both_using_long() {
    let parser = Test::parse().to_options();

    let result = parser.run_inner(&["check", "--all"]);
    assert!(result.is_ok(), "Should parse 'check --all': {:?}", result);
    let cmd = result.unwrap();
    assert_eq!(cmd.all, true);
}

// ============================================================================
// Command with help attribute
// ============================================================================

#[derive(Debug, Clone, PartialEq, bpaf_unsynn::Bpaf)]
#[bpaf(command, help("Run the application"))]
struct Run {
    #[bpaf(long)]
    port: Option<u16>,
}

#[test]
fn command_with_help() {
    let parser = Run::parse().to_options();

    let result = parser.run_inner(&["run", "--port", "8080"]);
    assert!(result.is_ok(), "Should parse 'run --port 8080': {:?}", result);
    let cmd = result.unwrap();
    assert_eq!(cmd.port, Some(8080));
}

// ============================================================================
// Command with all three: short, long, and help
// ============================================================================

#[derive(Debug, Clone, PartialEq, bpaf_unsynn::Bpaf)]
#[bpaf(command, short('d'), long("deploy"), help("Deploy the application"))]
struct Deployment {
    #[bpaf(long)]
    target: String,
}

#[test]
fn command_with_all_attrs_using_name() {
    let parser = Deployment::parse().to_options();

    let result = parser.run_inner(&["deployment", "--target", "production"]);
    assert!(result.is_ok());
    assert_eq!(result.unwrap().target, "production");
}

#[test]
fn command_with_all_attrs_using_short() {
    let parser = Deployment::parse().to_options();

    let result = parser.run_inner(&["d", "--target", "staging"]);
    assert!(result.is_ok());
    assert_eq!(result.unwrap().target, "staging");
}

#[test]
fn command_with_all_attrs_using_long() {
    let parser = Deployment::parse().to_options();

    let result = parser.run_inner(&["deploy", "--target", "development"]);
    assert!(result.is_ok());
    assert_eq!(result.unwrap().target, "development");
}

// ============================================================================
// Command with multiple short and long aliases
// ============================================================================

#[derive(Debug, Clone, PartialEq, bpaf_unsynn::Bpaf)]
#[bpaf(command, short('i'), short('a'), long("install"), long("add"))]
struct Install {
    #[bpaf(positional("PACKAGE"))]
    package: String,
}

#[test]
fn command_with_multiple_shorts() {
    let parser = Install::parse().to_options();

    let result1 = parser.run_inner(&["i", "package1"]);
    assert!(result1.is_ok());
    assert_eq!(result1.unwrap().package, "package1");

    let result2 = parser.run_inner(&["a", "package2"]);
    assert!(result2.is_ok());
    assert_eq!(result2.unwrap().package, "package2");
}

#[test]
fn command_with_multiple_longs() {
    let parser = Install::parse().to_options();

    let result1 = parser.run_inner(&["install", "package1"]);
    assert!(result1.is_ok());
    assert_eq!(result1.unwrap().package, "package1");

    let result2 = parser.run_inner(&["add", "package2"]);
    assert!(result2.is_ok());
    assert_eq!(result2.unwrap().package, "package2");
}
