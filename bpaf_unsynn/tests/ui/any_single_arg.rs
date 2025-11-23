//! Test that any() with only 1 argument produces a clear error message

#[derive(Debug, Clone, bpaf_unsynn::Bpaf)]
#[bpaf(options)]
struct AnySingleArg {
    #[bpaf(any("METAVAR"))]
    value: String,
}

fn main() {}
