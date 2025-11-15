//! Tests for complex visibility modifiers (pub(crate), pub(super), etc.)

use bpaf::Parser;

#[derive(Debug, Clone, bpaf_unsynn::Bpaf)]
#[bpaf(options)]
struct TestPubCrate {
    pub(crate) field1: bool,
    #[bpaf(long)]
    pub(crate) field2: String,
}

#[test]
fn pub_crate_visibility() {
    let parser = TestPubCrate::parse();
    let result = parser.run_inner(&["--field2", "test"]);
    assert!(result.is_ok(), "Should parse pub(crate) fields: {:?}", result);
}

// Test pub(super) - needs to be in a submodule
mod outer {
    use bpaf::Parser;

    #[derive(Debug, Clone, bpaf_unsynn::Bpaf)]
    #[bpaf(options)]
    pub struct TestPubSuper {
        pub(super) field1: bool,
        #[bpaf(long)]
        pub(super) field2: u32,
    }

    #[test]
    fn pub_super_visibility() {
        let parser = TestPubSuper::parse();
        let result = parser.run_inner(&["--field2", "42"]);
        assert!(result.is_ok(), "Should parse pub(super) fields: {:?}", result);
    }
}

mod inner {
    #[derive(Debug, Clone, bpaf_unsynn::Bpaf)]
    #[bpaf(options)]
    pub struct TestPubInPath {
        pub(in crate) field1: bool,
        #[bpaf(long)]
        pub(in crate) field2: String,
    }
}

#[test]
fn pub_in_path_visibility() {
    use bpaf::Parser;
    let parser = inner::TestPubInPath::parse();
    let result = parser.run_inner(&["--field2", "test"]);
    assert!(result.is_ok(), "Should parse pub(in path) fields: {:?}", result);
}

mod self_test {
    #[derive(Debug, Clone, bpaf_unsynn::Bpaf)]
    #[bpaf(options)]
    pub struct TestPubSelf {
        pub(self) field1: bool,
        #[bpaf(long)]
        pub(self) field2: String,
    }

    #[test]
    fn pub_self_visibility() {
        use bpaf::Parser;
        let parser = TestPubSelf::parse();
        let result = parser.run_inner(&["--field2", "test"]);
        assert!(result.is_ok(), "Should parse pub(self) fields: {:?}", result);
    }
}
