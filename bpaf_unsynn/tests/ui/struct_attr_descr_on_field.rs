// Test that descr attribute used on fields produces proper error

#[derive(Debug, Clone, bpaf_unsynn::Bpaf)]
#[bpaf(options)]
struct Test {
    #[bpaf(descr("test"))]
    value: String,
}

fn main() {}
