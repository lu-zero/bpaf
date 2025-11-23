// Test that version attribute used on fields produces proper error

#[derive(Debug, Clone, bpaf_unsynn::Bpaf)]
#[bpaf(options)]
struct Test {
    #[bpaf(version("1.0"))]
    value: String,
}

fn main() {}
