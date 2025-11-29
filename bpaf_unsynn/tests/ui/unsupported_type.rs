//! Test that applying Bpaf to non-struct/non-enum produces a clear error

#[derive(bpaf_unsynn::Bpaf)]
union NotSupported {
    a: u32,
    b: f32,
}

fn main() {}
