//! Test that positional fields must come after named fields

#[derive(Debug, Clone, bpaf_unsynn::Bpaf)]
#[bpaf(options)]
struct WrongOrder {
    #[bpaf(positional("X"))]
    x: f64,

    #[bpaf(long)]
    verbose: bool,
}

fn main() {}
