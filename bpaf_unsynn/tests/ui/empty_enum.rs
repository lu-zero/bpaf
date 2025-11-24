//! Test that empty enums produce a clear compile-time error

#[derive(Debug, Clone, bpaf_unsynn::Bpaf)]
enum EmptyEnum {}

fn main() {}
