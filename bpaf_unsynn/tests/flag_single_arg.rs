//! Test to compare flag behavior with single argument
//! between bpaf_unsynn and bpaf_derive
//!
//! This file tests what happens when flag() is called with only 1 argument
//! instead of the required 2 arguments (present and absent values).
//!
//! To run this test and see the errors:
//! 1. Uncomment the structs below one at a time
//! 2. Run `cargo test --test flag_single_arg`
//! 3. Compare the error messages

#[allow(unused_imports)]
use bpaf::Parser;

// =============================================================================
// Test 1: bpaf_unsynn with single argument to flag
// =============================================================================

// TEST bpaf_unsynn ERROR - Custom validation provides clear error message!
// Error: Parse error: flag() requires exactly 2 comma-separated arguments (present value, absent value)
//
// Uncomment to see the error:
// #[derive(Debug, Clone, PartialEq, bpaf_unsynn::Bpaf)]
// #[bpaf(options)]
// struct UnsynnSingleArgFlag {
//     #[bpaf(long, flag(true))]
//     enabled: bool,
// }
//
// #[test]
// fn test_unsynn_single_arg_flag() {
//     let parser = UnsynnSingleArgFlag::parse();
//     let r = parser.run_inner(&[]).unwrap();
//     assert_eq!(r.enabled, true);
// }

// =============================================================================
// Test 2: bpaf_derive with single argument to flag (for comparison)
// =============================================================================

// TEST bpaf_derive ERROR - Tested manually in bpaf_derive/tests/
//
// MANUAL TEST RESULTS (tested separately):
// Error: expected `,`
// Location: bpaf_derive/tests/flag_single_arg_test.rs:8:27
// Message: error: expected `,`
//          --> bpaf_derive/tests/flag_single_arg_test.rs:8:27
//          |
//        8 |     #[bpaf(long, flag(true))]
//          |                           ^
//
// Quality: syn parsing error - catches at macro parse time
//
// COMPARISON:
// - bpaf_unsynn: Produces custom validation error with clear guidance
//   "Parse error: flag() requires exactly 2 comma-separated arguments (present value, absent value)"
//   Points to the derive macro location
//
// - bpaf_derive: Produces syn parse error (expected comma)
//   "expected `,`"
//   Points to the closing paren in the attribute
//
// ANALYSIS:
// - bpaf_derive (using syn): Parses attributes with syn, expects comma-separated
//   values, catches error during attribute parsing
// - bpaf_unsynn (using unsynn): Parses arguments with unsynn, then validates them
//   with custom error messages during attribute validation
//
// Both errors are clear and actionable:
// - bpaf_derive: Concise syn parsing error
// - bpaf_unsynn: Descriptive custom validation error with purpose explanation
//
// #[derive(Debug, Clone, PartialEq, bpaf_derive::Bpaf)]
// #[bpaf(options)]
// struct DeriveSingleArgFlag {
//     #[bpaf(long, flag(true))]
//     enabled: bool,
// }
//
// #[test]
// fn test_derive_single_arg_flag() {
//     let parser = DeriveSingleArgFlag::parse();
//     let r = parser.run_inner(&[]).unwrap();
//     assert_eq!(r.enabled, true);
// }

// =============================================================================
// Test 3: Correct usage (2 arguments) for reference
// =============================================================================

#[derive(Debug, Clone, PartialEq, bpaf_unsynn::Bpaf)]
#[bpaf(options)]
struct CorrectFlagUsage {
    #[bpaf(long, flag(true, false))]
    enabled: bool,
}

#[test]
fn test_correct_flag_usage() {
    let parser = CorrectFlagUsage::parse();

    let r = parser.run_inner(&[]).unwrap();
    assert_eq!(r.enabled, false); // absent value

    let r = parser.run_inner(&["--enabled"]).unwrap();
    assert_eq!(r.enabled, true); // present value
}

// =============================================================================
// Instructions for testing:
// =============================================================================
//
// To reproduce bpaf_unsynn error:
//   1. Uncomment lines 23-35 (UnsynnSingleArgFlag struct and test)
//   2. Run: cargo test --test flag_single_arg
//   3. Observe error: "Parse error: flag() requires exactly 2 comma-separated arguments (present value, absent value)"
//
// To reproduce bpaf_derive error:
//   1. See bpaf_derive/tests/flag_single_arg_test.rs
//   2. Run: cd ../bpaf_derive && cargo test --test flag_single_arg_test
//   3. Observe error: "expected `,`" (syn parsing error)
//
// Full comparison documented in: FLAG_ERROR_COMPARISON.md
//
// CONCLUSION:
//   Both produce clear, actionable errors with different approaches:
//   - bpaf_derive: syn parsing error (concise)
//   - bpaf_unsynn: custom validation error (descriptive and actionable)
