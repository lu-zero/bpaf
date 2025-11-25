//! Test that positional() with long produces a clear error message

#[derive(Debug, Clone, bpaf_unsynn::Bpaf)]
#[bpaf(options)]
struct PositionalWithLong {
    #[bpaf(long, positional("FILE"))]
    input: String,
}

fn main() {}
