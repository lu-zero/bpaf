//! Test help attribute on enum variant without command

#[derive(Debug, Clone, PartialEq, bpaf_unsynn::Bpaf)]
#[bpaf(options)]
enum Foo {
    #[bpaf(help("custom help for Bar"))]
    Bar,
}

#[test]
fn enum_help_no_command() {
    let parser = foo();

    // Should be able to use the variant
    let r = parser.run_inner(&["--bar"]).unwrap();
    assert_eq!(r, Foo::Bar);
}

#[test]
fn enum_help_appears_in_output() {
    let parser = foo();

    // Verify help text appears in --help output
    let help = parser.run_inner(&["--help"]).unwrap_err().unwrap_stdout();
    assert!(
        help.contains("custom help for Bar"),
        "Help should contain custom help text: {}",
        help
    );
}

// Test with both doc comment and help attribute
#[derive(Debug, Clone, PartialEq, bpaf_unsynn::Bpaf)]
#[bpaf(options)]
enum Bar {
    /// This doc comment should be overridden
    #[bpaf(help("explicit help wins"))]
    First,

    /// This doc comment is used
    Second,
}

#[test]
fn help_overrides_doc_comment_non_command() {
    let parser = bar();

    let help = parser.run_inner(&["--help"]).unwrap_err().unwrap_stdout();

    // Explicit help should appear
    assert!(
        help.contains("explicit help wins"),
        "Should contain explicit help: {}",
        help
    );

    // Doc comment should NOT appear for First
    assert!(
        !help.contains("should be overridden"),
        "Should NOT contain overridden doc: {}",
        help
    );

    // Doc comment SHOULD appear for Second
    assert!(
        help.contains("This doc comment is used"),
        "Should contain doc comment for Second: {}",
        help
    );
}
