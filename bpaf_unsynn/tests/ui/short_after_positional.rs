//! Test that short after positional produces a clear error message

#[derive(Debug, Clone, bpaf_unsynn::Bpaf)]
#[bpaf(options)]
struct ShortAfterPositional {
    #[bpaf(positional("FILE"), short)]
    input: String,
}

fn main() {}
