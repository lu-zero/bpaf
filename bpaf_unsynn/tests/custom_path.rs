//! Tests for custom bpaf crate path support
//!
//! Note: We use `::bpaf` as the custom path in these tests since that's the actual
//! crate available. The important thing is that the generated code uses the custom
//! path instead of the default `::bpaf`.

use bpaf_unsynn::Bpaf;

#[test]
fn custom_path_works() {
    // Use ::bpaf as the custom path (which happens to be the default, but tests the mechanism)
    #[derive(Bpaf)]
    #[bpaf(options, path(::bpaf))]
    struct Options {
        #[bpaf(short, long)]
        verbose: bool,
    }

    // Should compile and work
    let parser = Options::parse();
    let opts = parser.run_inner(&["--verbose"]).unwrap();
    assert!(opts.verbose);
}

#[test]
fn custom_path_with_parser_mode() {
    #[derive(Bpaf)]
    #[bpaf(options, path(::bpaf))]
    struct Items {
        #[bpaf(long)]
        verbose: bool,
    }

    let parser = Items::parse();
    let items = parser.run_inner(&["--verbose"]).unwrap();
    assert!(items.verbose);
}

#[test]
fn custom_path_with_adjacent() {
    #[derive(Bpaf)]
    #[bpaf(options, path(::bpaf), adjacent)]
    struct Config {
        #[bpaf(long, argument("HOST"))]
        host: String,
        #[bpaf(long, argument("PORT"))]
        port: u16,
    }

    let parser = Config::parse();
    let cfg = parser.run_inner(&["--host", "localhost", "--port", "8080"]).unwrap();
    assert_eq!(cfg.host, "localhost");
    assert_eq!(cfg.port, 8080);
}

#[test]
fn custom_path_enum() {
    #[derive(Bpaf, Clone, Debug, PartialEq)]
    #[bpaf(options, path(::bpaf))]
    enum Command {
        #[bpaf(command)]
        Run,
        #[bpaf(command)]
        Build,
    }

    let parser = Command::parse();
    let cmd = parser.run_inner(&["run"]).unwrap();
    assert_eq!(cmd, Command::Run);
}
