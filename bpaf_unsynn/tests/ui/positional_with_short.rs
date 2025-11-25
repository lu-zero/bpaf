//! Test that positional() with short produces a clear error message

#[derive(Debug, Clone, bpaf_unsynn::Bpaf)]
#[bpaf(options)]
struct PositionalWithShort {
    #[bpaf(short, positional("FILE"))]
    input: String,
}

fn main() {}
