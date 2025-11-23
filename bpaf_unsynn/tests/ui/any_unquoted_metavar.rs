//! Test that any() with unquoted metavar produces an error

fn check_value(s: &str) -> bool {
    !s.is_empty()
}

#[derive(Debug, Clone, bpaf_unsynn::Bpaf)]
#[bpaf(options)]
struct AnyUnquotedMetavar {
    #[bpaf(any(METAVAR, check_value))]
    value: String,
}

fn main() {}
