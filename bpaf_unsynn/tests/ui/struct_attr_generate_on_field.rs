// Test that generate attribute used on fields produces proper error

#[derive(Debug, Clone, bpaf_unsynn::Bpaf)]
#[bpaf(options)]
struct Test {
    #[bpaf(generate(test_parser))]
    value: String,
}

fn main() {}
