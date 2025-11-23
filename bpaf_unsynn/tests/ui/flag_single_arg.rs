//! Test that flag() with only 1 argument produces a clear error message

#[derive(Debug, Clone, bpaf_unsynn::Bpaf)]
#[bpaf(options)]
struct FlagSingleArg {
    #[bpaf(long, flag(true))]
    enabled: bool,
}

fn main() {}
