// Test that cargo_helper attribute used on fields produces proper error

#[derive(Debug, Clone, bpaf_unsynn::Bpaf)]
#[bpaf(options)]
struct Test {
    #[bpaf(cargo_helper("test"))]
    value: String,
}

fn main() {}
