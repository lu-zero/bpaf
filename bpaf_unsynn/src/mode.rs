//! Mode configuration structures
//!
//! This module defines the mode-specific configuration structs that control
//! how parsers are generated for different modes (Options, Command, Parser).

use proc_macro2::TokenStream;

/// Help text stored as a token stream (either a string literal or custom expression)
pub type Help = TokenStream;

/// Configuration for Command mode
///
/// Used when `#[bpaf(command, ...)]` is specified at the top level.
#[derive(Debug, Clone, Default)]
pub struct CommandCfg {
    /// Custom command name (if specified with `command("name")`)
    pub name: Option<String>,
    /// Long aliases for the command (from `long("alias")`)
    pub long: Vec<String>,
    /// Short aliases for the command (from `short('a')`)
    pub short: Vec<char>,
    /// Help text for the command (from `help(...)`)
    pub help: Option<Help>,
}

/// Configuration for Options mode
///
/// Used when `#[bpaf(options, ...)]` or `#[bpaf(command, ...)]` is specified.
/// Options mode adds `.to_options()` wrapper which provides help/version handling.
#[derive(Debug, Default, Clone)]
pub struct OptionsCfg {
    /// Cargo helper name (from `cargo_helper("name")`)
    /// Enables cargo-style subcommand parsing
    pub cargo_helper: Option<String>,
    /// Parser description (from `descr(...)`)
    pub descr: Option<Help>,
    /// Footer text (from `footer(...)`)
    pub footer: Option<Help>,
    /// Header text (from `header(...)`)
    pub header: Option<Help>,
    /// Custom usage line (from `usage(...)`)
    pub usage: Option<TokenStream>,
    /// Version information (from `version(...)`)
    pub version: Option<TokenStream>,
    /// Maximum help text width (from `max_width(...)`)
    pub max_width: Option<TokenStream>,
    /// Show usage on parse errors (from `fallback_to_usage`)
    pub fallback_usage: bool,
}

/// Configuration for Parser mode
///
/// Used when neither `options` nor `command` is specified (default mode).
/// Parser mode generates `impl Parser<T>` without `.to_options()` wrapper.
#[derive(Debug, Default, Clone)]
pub struct ParserCfg {
    /// Group help text (from `group_help(...)`)
    pub group_help: Option<Help>,
}

/// Parser generation mode
///
/// Determines what kind of parser function is generated and what
/// configuration options are available.
#[derive(Debug, Clone)]
pub enum Mode {
    /// Command mode: generates a command parser with options support
    ///
    /// Example: `#[bpaf(command)]` or `#[bpaf(command("build"))]`
    /// Generated code: `.to_options().command("name")`
    Command {
        /// Command-specific configuration
        command: CommandCfg,
        /// Options configuration (commands include options support)
        options: OptionsCfg,
    },
    /// Options mode: generates an options parser with help/version
    ///
    /// Example: `#[bpaf(options)]`
    /// Generated code: `.to_options()`
    Options {
        /// Options configuration
        options: OptionsCfg,
    },
    /// Parser mode: generates a basic parser without options wrapper
    ///
    /// Example: `#[bpaf]` or `#[bpaf(parser)]` (default)
    /// Generated code: returns `impl Parser<T>` directly
    Parser {
        /// Parser configuration
        parser: ParserCfg,
    },
}

impl Default for Mode {
    fn default() -> Self {
        Mode::Parser {
            parser: ParserCfg::default(),
        }
    }
}
