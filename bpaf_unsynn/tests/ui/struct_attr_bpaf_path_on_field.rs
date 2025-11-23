// Test that path attribute used on fields produces proper error

#[derive(Debug, Clone, bpaf_unsynn::Bpaf)]
#[bpaf(options)]
struct Test {
    #[bpaf(path(bpaf))]
    value: String,
}

fn main() {}
