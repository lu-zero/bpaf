// Test that footer attribute used on fields produces proper error

#[derive(Debug, Clone, bpaf_unsynn::Bpaf)]
#[bpaf(options)]
struct Test {
    #[bpaf(footer("test"))]
    value: String,
}

fn main() {}
