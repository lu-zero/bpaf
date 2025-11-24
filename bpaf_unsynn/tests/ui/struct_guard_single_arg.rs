//! Test that struct-level guard() with only 1 argument produces a clear error message

fn check_valid(opts: &StructGuardSingleArg) -> bool {
    opts.count > 0
}

#[derive(Debug, Clone, bpaf_unsynn::Bpaf)]
#[bpaf(guard(check_valid))]
struct StructGuardSingleArg {
    #[bpaf(long, argument("NUM"))]
    count: u32,
}

fn main() {}
