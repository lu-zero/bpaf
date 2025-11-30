//! Test that multiple short/long aliases work correctly for command variants

#[derive(Clone, Debug, PartialEq, bpaf_unsynn::Bpaf)]
#[bpaf(options)]
enum Command {
    /// Build command with multiple aliases
    #[bpaf(command, short('b'), short('B'), long("build"), long("compile"))]
    Build,

    /// Run command with one short and multiple longs
    #[bpaf(command, short('r'), long("run"), long("execute"), long("start"))]
    Run,
}

#[test]
fn multiple_short_aliases() {
    let parser = command();

    // Test first short alias - 'b' is a command name alias for build
    let cmd = parser.run_inner(&["b"]).unwrap();
    assert_eq!(cmd, Command::Build);

    // Test second short alias - 'B' is also an alias
    let cmd = parser.run_inner(&["B"]).unwrap();
    assert_eq!(cmd, Command::Build);
}

#[test]
fn multiple_long_aliases() {
    let parser = command();

    // Test derived command name
    let cmd = parser.run_inner(&["build"]).unwrap();
    assert_eq!(cmd, Command::Build);

    // Test first long alias - "build" is also explicitly added
    let cmd = parser.run_inner(&["build"]).unwrap();
    assert_eq!(cmd, Command::Build);

    // Test second long alias - "compile" is another name for build command
    let cmd = parser.run_inner(&["compile"]).unwrap();
    assert_eq!(cmd, Command::Build);

    // Test run with different long aliases
    let cmd = parser.run_inner(&["run"]).unwrap();
    assert_eq!(cmd, Command::Run);

    let cmd = parser.run_inner(&["execute"]).unwrap();
    assert_eq!(cmd, Command::Run);

    let cmd = parser.run_inner(&["start"]).unwrap();
    assert_eq!(cmd, Command::Run);
}

#[test]
fn short_and_long_mixed() {
    let parser = command();

    // run command can use short alias 'r'
    let cmd = parser.run_inner(&["r"]).unwrap();
    assert_eq!(cmd, Command::Run);

    // or any of its long aliases
    let cmd = parser.run_inner(&["execute"]).unwrap();
    assert_eq!(cmd, Command::Run);

    let cmd = parser.run_inner(&["start"]).unwrap();
    assert_eq!(cmd, Command::Run);
}
