//! Test that long after positional produces a clear error message

#[derive(Debug, Clone, bpaf_unsynn::Bpaf)]
#[bpaf(options)]
struct LongAfterPositional {
    #[bpaf(positional("FILE"), long)]
    input: String,
}

fn main() {}
