//! Test that non-command variants with fields in mixed enums produce a clear error

#[derive(Debug, Clone, bpaf_unsynn::Bpaf)]
enum MixedEnum {
    #[bpaf(command)]
    Cmd1 {
        #[bpaf(long)]
        arg: String,
    },
    // This should fail: non-command variant with fields in an enum that has command variants
    Variant2 {
        field: String,
    },
}

fn main() {}
