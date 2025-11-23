// Test that boxed attribute used on fields produces proper error

#[derive(Debug, Clone, bpaf_unsynn::Bpaf)]
#[bpaf(options)]
struct Test {
    #[bpaf(boxed)]
    value: String,
}

fn main() {}
