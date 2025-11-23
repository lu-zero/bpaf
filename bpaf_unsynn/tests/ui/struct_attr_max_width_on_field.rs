// Test that max_width attribute used on fields produces proper error

#[derive(Debug, Clone, bpaf_unsynn::Bpaf)]
#[bpaf(options)]
struct Test {
    #[bpaf(max_width(80))]
    value: String,
}

fn main() {}
