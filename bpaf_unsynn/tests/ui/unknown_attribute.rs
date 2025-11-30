// This test verifies that unknown attributes are rejected

#[derive(Debug, Clone, bpaf_unsynn::Bpaf)]
#[bpaf(options)]
struct UnknownAttr {
    #[bpaf(foobar_unknown)]
    value: String,
}

fn main() {}
