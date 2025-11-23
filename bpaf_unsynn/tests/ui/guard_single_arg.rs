//! Test that guard() with only 1 argument produces a clear error message

fn check_positive(n: &u32) -> bool {
    *n > 0
}

#[derive(Debug, Clone, bpaf_unsynn::Bpaf)]
#[bpaf(options)]
struct GuardSingleArg {
    #[bpaf(long, argument("NUM"), guard(check_positive))]
    count: u32,
}

fn main() {}
