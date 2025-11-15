//! Tests for top-level attributes: generate, private, boxed

use bpaf_unsynn::Bpaf;

// Test generate(name) attribute
#[test]
fn generate_custom_name() {
    #[derive(Bpaf)]
    #[bpaf(options, generate(my_parser))]
    struct Options {
        #[bpaf(long)]
        verbose: bool,
    }

    // Should generate my_parser() instead of parse()
    let parser = Options::my_parser();
    let opts = parser.run_inner(&["--verbose"]).unwrap();
    assert!(opts.verbose);
}

#[test]
fn generate_with_options_mode() {
    #[derive(Bpaf)]
    #[bpaf(options, generate(custom_options))]
    struct Config {
        #[bpaf(long, argument("FILE"))]
        file: String,
    }

    let parser = Config::custom_options();
    let cfg = parser.run_inner(&["--file", "test.txt"]).unwrap();
    assert_eq!(cfg.file, "test.txt");
}

// Test private attribute
#[test]
fn private_function() {
    #[derive(Bpaf)]
    #[bpaf(options, private)]
    struct PrivateOptions {
        #[bpaf(long)]
        verbose: bool,
    }

    // The generated parse() function should be private (not pub)
    // We can't directly test visibility from outside, but we can verify it compiles
    let _ = std::marker::PhantomData::<PrivateOptions>;
}

#[test]
fn private_with_generate() {
    #[derive(Bpaf)]
    #[bpaf(options, private, generate(internal_parser))]
    struct InternalOptions {
        #[bpaf(long)]
        debug: bool,
    }

    let _ = std::marker::PhantomData::<InternalOptions>;
}

// Test boxed attribute
#[test]
fn boxed_parser() {
    #[derive(Bpaf)]
    #[bpaf(options, boxed)]
    struct BoxedOptions {
        #[bpaf(long)]
        verbose: bool,
    }

    // With boxed, the parser is wrapped with .boxed()
    let parser = BoxedOptions::parse();
    let opts = parser.run_inner(&["--verbose"]).unwrap();
    assert!(opts.verbose);
}

#[test]
fn boxed_with_parser_mode() {
    #[derive(Bpaf)]
    #[bpaf(boxed)]
    struct BoxedParser {
        #[bpaf(long)]
        value: String,
    }

    // boxed changes return type from impl Parser to OptionParser
    let _ = BoxedParser::parse();
}

// Test combinations
#[test]
fn all_three_combined() {
    #[derive(Bpaf)]
    #[bpaf(options, generate(custom_fn), private, boxed)]
    struct CombinedOptions {
        #[bpaf(long)]
        verbose: bool,
    }

    let _ = std::marker::PhantomData::<CombinedOptions>;
}

#[test]
fn generate_and_boxed() {
    #[derive(Bpaf)]
    #[bpaf(options, generate(build_parser), boxed)]
    struct BuildOptions {
        #[bpaf(long, argument("TARGET"))]
        target: String,
    }

    let parser = BuildOptions::build_parser();
    let opts = parser.run_inner(&["--target", "x86_64"]).unwrap();
    assert_eq!(opts.target, "x86_64");
}

#[test]
fn generate_with_path() {
    #[derive(Bpaf)]
    #[bpaf(options, generate(my_fn), path(::bpaf))]
    struct PathOptions {
        #[bpaf(long)]
        verbose: bool,
    }

    let parser = PathOptions::my_fn();
    let opts = parser.run_inner(&["--verbose"]).unwrap();
    assert!(opts.verbose);
}

#[test]
fn all_attributes_combined() {
    #[derive(Bpaf)]
    #[bpaf(options, generate(create_parser), private, boxed, path(::bpaf))]
    struct AllOptions {
        #[bpaf(long)]
        debug: bool,
    }

    let _ = std::marker::PhantomData::<AllOptions>;
}

// ============================================================================
// Tests for top-level PostDecor attributes
// ============================================================================

// Test guard attribute at top level
#[test]
fn top_level_guard() {
    #[derive(Bpaf, Debug, Clone)]
    #[bpaf(options, guard(|o: &GuardedOptions| o.value > 0, "Value must be positive"))]
    struct GuardedOptions {
        #[bpaf(long, argument("N"))]
        value: i32,
    }

    let parser = GuardedOptions::parse();

    // Valid value should pass
    let result = parser.run_inner(&["--value", "5"]);
    assert!(result.is_ok(), "Expected success for positive value");

    // Invalid value should fail
    let result = parser.run_inner(&["--value", "-1"]);
    assert!(result.is_err(), "Expected failure for negative value");
}

// Test hide attribute at top level
#[test]
fn top_level_hide() {
    #[derive(Bpaf)]
    #[bpaf(options, hide)]
    struct HiddenOptions {
        #[bpaf(long)]
        verbose: bool,
    }

    // Parser should still work, just hidden in help
    let parser = HiddenOptions::parse();
    let opts = parser.run_inner(&["--verbose"]).unwrap();
    assert!(opts.verbose);
}

// Test hide_usage attribute at top level
#[test]
fn top_level_hide_usage() {
    #[derive(Bpaf)]
    #[bpaf(options, hide_usage)]
    struct HideUsageOptions {
        #[bpaf(long)]
        verbose: bool,
    }

    let parser = HideUsageOptions::parse();
    let opts = parser.run_inner(&["--verbose"]).unwrap();
    assert!(opts.verbose);
}

// Test custom_usage attribute at top level
#[test]
fn top_level_custom_usage() {
    #[derive(Bpaf)]
    #[bpaf(options, custom_usage("myapp [OPTIONS] <files>..."))]
    struct CustomUsageOptions {
        #[bpaf(long)]
        verbose: bool,
    }

    let parser = CustomUsageOptions::parse();
    let opts = parser.run_inner(&["--verbose"]).unwrap();
    assert!(opts.verbose);
}

// Note: fallback_with, complete, and group attributes are more complex
// and may require specific bpaf API features. Skipping for now.

// Test combining PostDecor attributes
#[test]
fn top_level_multiple_postdecor() {
    #[derive(Bpaf)]
    #[bpaf(options, hide_usage, custom_usage("myapp [FLAGS]"))]
    struct MultiplePostDecor {
        #[bpaf(long)]
        verbose: bool,
    }

    let parser = MultiplePostDecor::parse();
    let opts = parser.run_inner(&["--verbose"]).unwrap();
    assert!(opts.verbose);
}

// Test PostDecor with other top-level attributes
#[test]
fn postdecor_with_generate() {
    #[derive(Bpaf)]
    #[bpaf(options, generate(my_parser), hide_usage)]
    struct PostDecorWithGenerate {
        #[bpaf(long)]
        debug: bool,
    }

    let parser = PostDecorWithGenerate::my_parser();
    let opts = parser.run_inner(&["--debug"]).unwrap();
    assert!(opts.debug);
}
