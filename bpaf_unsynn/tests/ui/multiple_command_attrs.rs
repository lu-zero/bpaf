//! Test that multiple command() attributes produce a clear error message

#[derive(Debug, Clone, bpaf_unsynn::Bpaf)]
#[bpaf(options)]
enum Command {
    #[bpaf(command, command("foo"), command("bar"))]
    Test,
}

fn main() {}
