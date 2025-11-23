// Test that fallback_to_usage attribute used on fields produces proper error

#[derive(Debug, Clone, bpaf_unsynn::Bpaf)]
#[bpaf(options)]
struct Test {
    #[bpaf(fallback_to_usage)]
    value: String,
}

fn main() {}
