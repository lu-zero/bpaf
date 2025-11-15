//! Compare bpaf_unsynn with bpaf_derive for command aliases

// Test 1: Simple command with alias (using bpaf_unsynn)
#[derive(Debug, Clone, PartialEq, bpaf_unsynn::Bpaf)]
#[bpaf(options)]
enum CommandUnsynn {
    #[bpaf(command, long("top-alias"))]
    Command,
}

#[test]
fn unsynn_command_alias() {
    let parser = CommandUnsynn::parse();

    // Should work with command name
    let result = parser.run_inner(&["command"]);
    assert!(result.is_ok(), "Should parse 'command': {:?}", result);

    // Should work with alias
    let result = parser.run_inner(&["top-alias"]);
    assert!(result.is_ok(), "Should parse 'top-alias': {:?}", result);
}

// Test 2: The exact same test as bpaf_derive uses (from tests/params.rs)
#[derive(Debug, Clone, PartialEq, bpaf_unsynn::Bpaf)]
#[bpaf(options)]
enum Groups {
    #[bpaf(command, long("top-alias"))]
    Command,
}

#[test]
fn unsynn_exact_bpaf_derive_test() {
    // This is the exact test from bpaf/tests/params.rs:107-117
    // but using bpaf_unsynn instead of Bpaf

    // Note: bpaf_derive uses command("top") which specifies custom command name
    // We only support unnamed command (which uses kebab-case of variant name)
    // So we expect "command" instead of "top"

    let parser = Groups::parse();
    parser.run_inner(&["command"]).unwrap();
    parser.run_inner(&["top-alias"]).unwrap();
}
