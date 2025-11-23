// Test that usage attribute used on fields produces proper error

#[derive(Debug, Clone, bpaf_unsynn::Bpaf)]
#[bpaf(options)]
struct Test {
    #[bpaf(usage("test"))]
    value: String,
}

fn main() {}
