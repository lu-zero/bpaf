//! Tests for struct-level adjacent attribute

use bpaf::Parser;

#[derive(Debug, Clone, PartialEq)]
#[derive(bpaf_unsynn::Bpaf)]
#[bpaf(adjacent)]
struct Rect {
    #[bpaf(short('r'), long, req_flag(()))]
    rect: (),

    #[bpaf(short, long, argument("PX"))]
    width: usize,

    #[bpaf(short, long, argument("PX"))]
    height: usize,
}

#[test]
fn struct_adjacent_works() {
    let parser = Rect::parse().to_options();

    // All fields adjacent
    let result = parser.run_inner(&["-r", "-w", "100", "-h", "50"]);
    assert!(result.is_ok(), "Failed to parse: {:?}", result);
    let opts = result.unwrap();
    assert_eq!(opts.width, 100);
    assert_eq!(opts.height, 50);
}

#[test]
fn struct_adjacent_with_long() {
    let parser = Rect::parse().to_options();

    let result = parser.run_inner(&["--rect", "--width", "200", "--height", "100"]);
    assert!(result.is_ok(), "Failed to parse: {:?}", result);
    let opts = result.unwrap();
    assert_eq!(opts.width, 200);
    assert_eq!(opts.height, 100);
}
