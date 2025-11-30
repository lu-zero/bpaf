// Test to verify that consumers that don't need names still compile
// These test cases would trigger the expect at src/top.rs:1315 if the logic were broken

fn get_value() -> Result<i32, &'static str> {
    Ok(42)
}

fn my_parser() -> impl bpaf::Parser<String> {
    bpaf::long("test").argument("TEST")
}

// Pure without explicit names - should get default long (wasteful but safe)
#[derive(Debug, Clone, bpaf_unsynn::Bpaf)]
#[bpaf(options)]
struct PureNoNames {
    #[bpaf(pure(()))]
    unit_field: (),
    name: String,
}

// PureWith without explicit names - should get default long (wasteful but safe)
#[derive(Debug, Clone, bpaf_unsynn::Bpaf)]
#[bpaf(options)]
struct PureWithNoNames {
    #[bpaf(pure_with(get_value))]
    value: i32,
    name: String,
}

// External without explicit names - should get default long (wasteful but safe)
#[derive(Debug, Clone, bpaf_unsynn::Bpaf)]
#[bpaf(options)]
struct ExternalNoNames {
    #[bpaf(external(my_parser))]
    field: String,
    name: String,
}

#[test]
fn pure_no_names_compiles() {
    let parser = pure_no_names();
    let r = parser.run_inner(&["--name", "test"]).unwrap();
    assert_eq!(r.unit_field, ());
    assert_eq!(r.name, "test");
}

#[test]
fn pure_with_no_names_compiles() {
    let parser = pure_with_no_names();
    let r = parser.run_inner(&["--name", "test"]).unwrap();
    assert_eq!(r.value, 42);
    assert_eq!(r.name, "test");
}

#[test]
fn external_no_names_compiles() {
    let parser = external_no_names();
    let r = parser.run_inner(&["--test", "value", "--name", "test"]).unwrap();
    assert_eq!(r.field, "value");
    assert_eq!(r.name, "test");
}
