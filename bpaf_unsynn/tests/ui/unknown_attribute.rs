// This test verifies that unknown attributes are rejected
// Currently bpaf_unsynn silently ignores them - this should be fixed

#[derive(Debug, Clone, bpaf_unsynn::Bpaf)]
#[bpaf(options)]
struct UnknownAttr {
    #[bpaf(foobar_unknown)]
    value: String,
}

fn main() {}
