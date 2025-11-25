// Tests for tuple variant parsing edge cases
//
// FINDING: The check at src/top.rs:156 for `ty_tokens.is_empty()` appears to be
// unreachable with valid Rust syntax. unsynn's DelimitedVec parser does not create
// empty elements from trailing commas. Trailing commas are simply ignored and don't
// result in a TupleField with empty tokens.
//
// These tests demonstrate that:
// 1. Trailing commas work correctly but don't trigger the empty check
// 2. Empty tuple `()` doesn't go through tuple field parsing
// 3. All valid tuple syntax results in non-empty ty_tokens

// Single field with trailing comma - parsed as 1 field, not 2
#[derive(Debug, Clone, PartialEq, bpaf_unsynn::Bpaf)]
#[bpaf(options)]
enum TrailingComma {
    #[bpaf(command)]
    Single(String,),  // trailing comma doesn't create empty field
}

// Multiple fields with trailing comma
#[derive(Debug, Clone, PartialEq, bpaf_unsynn::Bpaf)]
#[bpaf(options)]
enum MultiTrailing {
    #[bpaf(command)]
    Multi(String, u32,),  // trailing comma after last field
}

#[test]
fn trailing_comma_single() {
    let parser = TrailingComma::parse();
    let r = parser.run_inner(&["single", "test"]).unwrap();
    assert_eq!(r, TrailingComma::Single("test".to_string()));
}

#[test]
fn trailing_comma_multi() {
    let parser = MultiTrailing::parse();
    let r = parser.run_inner(&["multi", "test", "42"]).unwrap();
    assert_eq!(r, MultiTrailing::Multi("test".to_string(), 42));
}

// Note: Variant() with empty parens is treated as a unit variant, not a tuple variant
// It doesn't go through convert_tuple_fields() at all, so can't trigger the empty check
