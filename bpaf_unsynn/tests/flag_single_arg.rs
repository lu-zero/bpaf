//! Test to compare flag behavior with single argument
//! between bpaf_unsynn and bpaf_derive
//!
//! NOTE: Compile-fail tests for error messages are now handled by trybuild.
//! See tests/ui/flag_single_arg.rs and tests/compile_fail.rs
//!
//! This file contains a passing test demonstrating correct flag usage.

// =============================================================================
// Correct usage (2 arguments) - this test passes
// =============================================================================

#[derive(Debug, Clone, PartialEq, bpaf_unsynn::Bpaf)]
#[bpaf(options)]
struct CorrectFlagUsage {
    #[bpaf(long, flag(true, false))]
    enabled: bool,
}

#[test]
fn test_correct_flag_usage() {
    let parser = correct_flag_usage();

    let r = parser.run_inner(&[]).unwrap();
    assert_eq!(r.enabled, false); // absent value

    let r = parser.run_inner(&["--enabled"]).unwrap();
    assert_eq!(r.enabled, true); // present value
}
