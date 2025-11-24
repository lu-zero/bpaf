// Test to verify trailing comma handling in tuple variants and structs
// Exercises line 155-156 in src/top.rs: "Skip empty types (trailing comma case)"
//
// Rust allows trailing commas in tuple definitions:
//   - Variant(String,)  <- trailing comma
//   - Variant(String)   <- no trailing comma
// Both should parse correctly. The code handles empty token streams that may
// result from parsing trailing commas.

// =============================================================================
// Tuple variant with trailing comma
// =============================================================================

#[derive(Debug, Clone, PartialEq, bpaf_unsynn::Bpaf)]
#[bpaf(options)]
enum ActionWithTrailing {
    /// Copy a file (single field with trailing comma)
    #[bpaf(command)]
    Copy(
        String,  // <- trailing comma here
    ),

    /// Move files (multiple fields with trailing comma)
    #[bpaf(command)]
    Move(
        String,
        String,  // <- trailing comma here
    ),

    /// Delete with attributes and trailing comma
    #[bpaf(command)]
    Delete(
        #[bpaf(positional("FILE"))]
        String,  // <- trailing comma here
    ),
}

#[test]
fn single_field_with_trailing_comma() {
    let parser = ActionWithTrailing::parse();

    let r = parser.run_inner(&["copy", "file.txt"]).unwrap();
    assert_eq!(r, ActionWithTrailing::Copy("file.txt".to_string()));
}

#[test]
fn multi_field_with_trailing_comma() {
    let parser = ActionWithTrailing::parse();

    let r = parser.run_inner(&["move", "src.txt", "dst.txt"]).unwrap();
    assert_eq!(r, ActionWithTrailing::Move("src.txt".to_string(), "dst.txt".to_string()));
}

#[test]
fn attributed_field_with_trailing_comma() {
    let parser = ActionWithTrailing::parse();

    let r = parser.run_inner(&["delete", "file.txt"]).unwrap();
    assert_eq!(r, ActionWithTrailing::Delete("file.txt".to_string()));
}

// =============================================================================
// Mixed: some with trailing commas, some without
// =============================================================================

#[derive(Debug, Clone, PartialEq, bpaf_unsynn::Bpaf)]
#[bpaf(options)]
enum MixedCommas {
    #[bpaf(command)]
    WithComma(String,),  // with trailing comma

    #[bpaf(command)]
    WithoutComma(String),  // no trailing comma

    #[bpaf(command)]
    MultiWithComma(String, u32,),  // with trailing comma

    #[bpaf(command)]
    MultiNoComma(String, u32),  // no trailing comma
}

#[test]
fn mixed_trailing_commas_work() {
    let parser = MixedCommas::parse();

    let r = parser.run_inner(&["with-comma", "test"]).unwrap();
    assert_eq!(r, MixedCommas::WithComma("test".to_string()));

    let r = parser.run_inner(&["without-comma", "test"]).unwrap();
    assert_eq!(r, MixedCommas::WithoutComma("test".to_string()));

    let r = parser.run_inner(&["multi-with-comma", "test", "42"]).unwrap();
    assert_eq!(r, MixedCommas::MultiWithComma("test".to_string(), 42));

    let r = parser.run_inner(&["multi-no-comma", "test", "42"]).unwrap();
    assert_eq!(r, MixedCommas::MultiNoComma("test".to_string(), 42));
}

// =============================================================================
// Tuple variant with attributes on multiple fields and trailing comma
// =============================================================================

#[derive(Debug, Clone, PartialEq, bpaf_unsynn::Bpaf)]
#[bpaf(options)]
enum ComplexTrailing {
    #[bpaf(command)]
    Process(
        #[bpaf(positional("INPUT"))]
        String,
        #[bpaf(positional("OUTPUT"))]
        String,  // <- trailing comma with attributes
    ),
}

#[test]
fn complex_attributes_with_trailing_comma() {
    let parser = ComplexTrailing::parse();

    let r = parser.run_inner(&["process", "in.txt", "out.txt"]).unwrap();
    assert_eq!(r, ComplexTrailing::Process("in.txt".to_string(), "out.txt".to_string()));
}
