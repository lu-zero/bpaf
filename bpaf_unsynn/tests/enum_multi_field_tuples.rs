// Tests for multi-field tuple variants

#[derive(Debug, Clone, PartialEq, bpaf_unsynn::Bpaf)]
#[bpaf(options)]
enum Command {
    /// Deploy to a target with timeout
    #[bpaf(command)]
    Deploy(String, usize),

    /// Build with output directory and verbosity
    #[bpaf(command)]
    Build(String, bool),

    /// Single field for comparison
    #[bpaf(command)]
    Test(String),
}

#[test]
fn two_field_tuple_string_usize() {
    let parser = Command::parse();
    let result = parser.run_inner(&["deploy", "production", "300"]);
    assert_eq!(
        result.unwrap(),
        Command::Deploy("production".to_string(), 300)
    );
}

#[test]
fn two_field_tuple_string_bool() {
    let parser = Command::parse();
    let result = parser.run_inner(&["build", "dist", "true"]);
    assert_eq!(
        result.unwrap(),
        Command::Build("dist".to_string(), true)
    );
}

#[test]
fn single_field_tuple_still_works() {
    let parser = Command::parse();
    let result = parser.run_inner(&["test", "unit"]);
    assert_eq!(result.unwrap(), Command::Test("unit".to_string()));
}

// Test with three fields
#[derive(Debug, Clone, PartialEq, bpaf_unsynn::Bpaf)]
#[bpaf(options)]
enum Action {
    #[bpaf(command)]
    Copy(String, String, bool),
}

#[test]
fn three_field_tuple() {
    let parser = Action::parse();
    let result = parser.run_inner(&["copy", "src", "dest", "true"]);
    assert_eq!(
        result.unwrap(),
        Action::Copy("src".to_string(), "dest".to_string(), true)
    );
}

// Test with numeric types
#[derive(Debug, Clone, PartialEq, bpaf_unsynn::Bpaf)]
#[bpaf(options)]
enum NumericCommand {
    #[bpaf(command)]
    Range(i32, i32),

    #[bpaf(command)]
    Point(f64, f64),
}

#[test]
fn two_i32_fields() {
    let parser = NumericCommand::parse();
    let result = parser.run_inner(&["range", "10", "20"]);
    assert_eq!(result.unwrap(), NumericCommand::Range(10, 20));
}

#[test]
fn two_f64_fields() {
    let parser = NumericCommand::parse();
    let result = parser.run_inner(&["point", "3.14", "2.71"]);
    assert_eq!(result.unwrap(), NumericCommand::Point(3.14, 2.71));
}

// Test with mixed types
#[derive(Debug, Clone, PartialEq, bpaf_unsynn::Bpaf)]
#[bpaf(options)]
enum MixedCommand {
    #[bpaf(command)]
    Create(String, usize, bool, i32),
}

#[test]
fn four_field_mixed_types() {
    let parser = MixedCommand::parse();
    let result = parser.run_inner(&["create", "name", "42", "true", "-10"]);
    assert_eq!(
        result.unwrap(),
        MixedCommand::Create("name".to_string(), 42, true, -10)
    );
}

// Test with command aliases
#[derive(Debug, Clone, PartialEq, bpaf_unsynn::Bpaf)]
#[bpaf(options)]
enum AliasedCommand {
    #[bpaf(command, long("install-pkg"))]
    Install(String, String),

    #[bpaf(command, short('r'))]
    Remove(String, bool),
}

#[test]
fn multi_field_with_long_alias() {
    let parser = AliasedCommand::parse();
    let result = parser.run_inner(&["install-pkg", "package", "1.0.0"]);
    assert_eq!(
        result.unwrap(),
        AliasedCommand::Install("package".to_string(), "1.0.0".to_string())
    );
}

#[test]
fn multi_field_with_short_alias() {
    let parser = AliasedCommand::parse();
    let result = parser.run_inner(&["r", "package", "true"]);
    assert_eq!(
        result.unwrap(),
        AliasedCommand::Remove("package".to_string(), true)
    );
}

// Test with custom command names
#[derive(Debug, Clone, PartialEq, bpaf_unsynn::Bpaf)]
#[bpaf(options)]
enum CustomCommand {
    #[bpaf(command("do-copy"))]
    Copy(String, String),

    #[bpaf(command("do-move"))]
    Move(String, String, bool),
}

#[test]
fn multi_field_with_custom_name_two_args() {
    let parser = CustomCommand::parse();
    let result = parser.run_inner(&["do-copy", "src", "dst"]);
    assert_eq!(
        result.unwrap(),
        CustomCommand::Copy("src".to_string(), "dst".to_string())
    );
}

#[test]
fn multi_field_with_custom_name_three_args() {
    let parser = CustomCommand::parse();
    let result = parser.run_inner(&["do-move", "src", "dst", "false"]);
    assert_eq!(
        result.unwrap(),
        CustomCommand::Move("src".to_string(), "dst".to_string(), false)
    );
}

// Test with hide attribute
#[derive(Debug, Clone, PartialEq, bpaf_unsynn::Bpaf)]
#[bpaf(options)]
enum HiddenCommand {
    #[bpaf(command)]
    Visible(String, usize),

    #[bpaf(command, hide)]
    Hidden(String, bool),
}

#[test]
fn multi_field_visible_variant() {
    let parser = HiddenCommand::parse();
    let result = parser.run_inner(&["visible", "value", "42"]);
    assert_eq!(
        result.unwrap(),
        HiddenCommand::Visible("value".to_string(), 42)
    );
}

#[test]
fn multi_field_hidden_variant() {
    let parser = HiddenCommand::parse();
    let result = parser.run_inner(&["hidden", "secret", "true"]);
    assert_eq!(
        result.unwrap(),
        HiddenCommand::Hidden("secret".to_string(), true)
    );
}

// Test mixed struct and multi-field tuple variants
#[derive(Debug, Clone, PartialEq, bpaf_unsynn::Bpaf)]
#[bpaf(options)]
enum MixedVariants {
    #[bpaf(command)]
    Tuple(String, usize),

    #[bpaf(command)]
    Struct { name: String, count: usize },

    #[bpaf(command)]
    Unit,
}

#[test]
fn mixed_tuple_variant() {
    let parser = MixedVariants::parse();
    let result = parser.run_inner(&["tuple", "value", "10"]);
    assert_eq!(
        result.unwrap(),
        MixedVariants::Tuple("value".to_string(), 10)
    );
}

#[test]
fn mixed_struct_variant() {
    let parser = MixedVariants::parse();
    let result = parser.run_inner(&["struct", "--name", "test", "--count", "5"]);
    assert_eq!(
        result.unwrap(),
        MixedVariants::Struct {
            name: "test".to_string(),
            count: 5
        }
    );
}

#[test]
fn mixed_unit_variant() {
    let parser = MixedVariants::parse();
    let result = parser.run_inner(&["unit"]);
    assert_eq!(result.unwrap(), MixedVariants::Unit);
}

// Test error handling - missing arguments
#[test]
fn multi_field_missing_second_arg() {
    let parser = Command::parse();
    let result = parser.run_inner(&["deploy", "production"]);
    assert!(result.is_err()); // Should fail - missing second argument
}

#[test]
fn multi_field_wrong_type() {
    let parser = Command::parse();
    let result = parser.run_inner(&["deploy", "production", "not-a-number"]);
    assert!(result.is_err()); // Should fail - second arg should be usize
}

// Test with pathbuf types
#[derive(Debug, Clone, PartialEq, bpaf_unsynn::Bpaf)]
#[bpaf(options)]
enum PathCommand {
    #[bpaf(command)]
    Copy(std::path::PathBuf, std::path::PathBuf),
}

#[test]
fn two_pathbuf_fields() {
    let parser = PathCommand::parse();
    let result = parser.run_inner(&["copy", "/src/file.txt", "/dst/file.txt"]);
    assert_eq!(
        result.unwrap(),
        PathCommand::Copy(
            std::path::PathBuf::from("/src/file.txt"),
            std::path::PathBuf::from("/dst/file.txt")
        )
    );
}
