//! Test that flag() with no arguments produces a clear error message

#[derive(Debug, Clone, bpaf_unsynn::Bpaf)]
#[bpaf(options)]
struct FlagNoArgs {
    #[bpaf(long, flag())]
    enabled: bool,
}

fn main() {}
