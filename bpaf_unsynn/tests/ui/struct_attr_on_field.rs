// Test that struct-level attributes used on fields produce proper errors

#[derive(Debug, Clone, bpaf_unsynn::Bpaf)]
#[bpaf(options)]
struct StructAttrOnField {
    #[bpaf(options)]
    value: String,
}

fn main() {}
