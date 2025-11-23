//! Tests for enum support
//!
//! Covers: unit variants, tuple variants, struct variants, command variants,
//! default variants, explicit commands, aliases, and mixed combinations

// =============================================================================
// Basic unit variant enum (flags)
// =============================================================================

#[derive(Debug, Clone, PartialEq, bpaf_unsynn::Bpaf)]
#[bpaf(options)]
enum LogLevel {
    #[bpaf(long)]
    Debug,
    #[bpaf(long)]
    Info,
    #[bpaf(long)]
    Warn,
}

#[test]
fn unit_variants_as_flags() {
    let parser = LogLevel::parse();

    let r = parser.run_inner(&["--debug"]).unwrap();
    assert_eq!(r, LogLevel::Debug);

    let r = parser.run_inner(&["--info"]).unwrap();
    assert_eq!(r, LogLevel::Info);

    let r = parser.run_inner(&["--warn"]).unwrap();
    assert_eq!(r, LogLevel::Warn);
}

// =============================================================================
// Enum with command variants
// =============================================================================

#[derive(Debug, Clone, PartialEq, bpaf_unsynn::Bpaf)]
#[bpaf(options)]
enum Command {
    /// Build the project
    #[bpaf(command)]
    Build,
    /// Run tests
    #[bpaf(command)]
    Test,
    /// Run the project
    #[bpaf(command)]
    Run,
}

#[test]
fn command_variants() {
    let parser = Command::parse();

    let r = parser.run_inner(&["build"]).unwrap();
    assert_eq!(r, Command::Build);

    let r = parser.run_inner(&["test"]).unwrap();
    assert_eq!(r, Command::Test);

    let r = parser.run_inner(&["run"]).unwrap();
    assert_eq!(r, Command::Run);
}

// =============================================================================
// Enum with tuple variants
// =============================================================================

#[derive(Debug, Clone, PartialEq, bpaf_unsynn::Bpaf)]
#[bpaf(options)]
enum Action {
    #[bpaf(command)]
    Copy(String),
    #[bpaf(command)]
    Delete(String),
}

#[test]
fn tuple_variant_single_field() {
    let parser = Action::parse();

    let r = parser.run_inner(&["copy", "file.txt"]).unwrap();
    assert_eq!(r, Action::Copy("file.txt".to_string()));

    let r = parser.run_inner(&["delete", "file.txt"]).unwrap();
    assert_eq!(r, Action::Delete("file.txt".to_string()));
}

// =============================================================================
// Enum with struct variants
// =============================================================================

#[derive(Debug, Clone, PartialEq, bpaf_unsynn::Bpaf)]
#[bpaf(options)]
enum FileOp {
    #[bpaf(command)]
    Move { source: String, dest: String },
    #[bpaf(command)]
    Copy {
        source: String,
        dest: String,
        #[bpaf(long)]
        recursive: bool,
    },
}

#[test]
fn struct_variant_fields() {
    let parser = FileOp::parse();

    let r = parser
        .run_inner(&["move", "--source", "a.txt", "--dest", "b.txt"])
        .unwrap();
    assert_eq!(
        r,
        FileOp::Move {
            source: "a.txt".to_string(),
            dest: "b.txt".to_string()
        }
    );

    let r = parser
        .run_inner(&[
            "copy",
            "--source",
            "a.txt",
            "--dest",
            "b.txt",
            "--recursive",
        ])
        .unwrap();
    assert_eq!(
        r,
        FileOp::Copy {
            source: "a.txt".to_string(),
            dest: "b.txt".to_string(),
            recursive: true
        }
    );
}

// =============================================================================
// Enum with explicit command names
// =============================================================================

#[derive(Debug, Clone, PartialEq, bpaf_unsynn::Bpaf)]
#[bpaf(options)]
enum NamedCmd {
    /// Compile command
    #[bpaf(command("compile"))]
    Build,
    /// Execute command
    #[bpaf(command("exec"))]
    Run,
}

#[test]
fn explicit_command_names() {
    let parser = NamedCmd::parse();

    let r = parser.run_inner(&["compile"]).unwrap();
    assert_eq!(r, NamedCmd::Build);

    let r = parser.run_inner(&["exec"]).unwrap();
    assert_eq!(r, NamedCmd::Run);
}

// =============================================================================
// Enum with default variant
// =============================================================================

#[derive(Debug, Clone, PartialEq, bpaf_unsynn::Bpaf)]
#[bpaf(options, fallback(Mode::Normal))]
enum Mode {
    #[bpaf(long)]
    Verbose,
    #[bpaf(long)]
    Quiet,
    #[bpaf(skip)]
    Normal,
}

#[test]
fn default_variant_fallback() {
    let parser = Mode::parse();

    // Without any flag, uses fallback
    let r = parser.run_inner(&[]).unwrap();
    assert_eq!(r, Mode::Normal);

    let r = parser.run_inner(&["--verbose"]).unwrap();
    assert_eq!(r, Mode::Verbose);
}

// =============================================================================
// Mixed unit and tuple variants
// =============================================================================

#[derive(Debug, Clone, PartialEq, bpaf_unsynn::Bpaf)]
#[bpaf(options)]
enum MixedAction {
    #[bpaf(command)]
    Init,
    #[bpaf(command)]
    Deploy(String),
    #[bpaf(command)]
    Status,
}

#[test]
fn mixed_unit_and_tuple() {
    let parser = MixedAction::parse();

    let r = parser.run_inner(&["init"]).unwrap();
    assert_eq!(r, MixedAction::Init);

    let r = parser.run_inner(&["deploy", "production"]).unwrap();
    assert_eq!(r, MixedAction::Deploy("production".to_string()));

    let r = parser.run_inner(&["status"]).unwrap();
    assert_eq!(r, MixedAction::Status);
}

// =============================================================================
// Enum commands with short/long aliases
// =============================================================================

#[derive(Debug, Clone, PartialEq, bpaf_unsynn::Bpaf)]
#[bpaf(options)]
enum Format {
    /// JSON output
    #[bpaf(command, long("json-format"))]
    Json,
    /// XML output
    #[bpaf(command, long("xml-format"))]
    Xml,
}

#[test]
fn command_with_long_alias() {
    let parser = Format::parse();

    // Primary command name
    let r = parser.run_inner(&["json"]).unwrap();
    assert_eq!(r, Format::Json);

    // Long alias
    let r = parser.run_inner(&["json-format"]).unwrap();
    assert_eq!(r, Format::Json);

    let r = parser.run_inner(&["xml-format"]).unwrap();
    assert_eq!(r, Format::Xml);
}

// =============================================================================
// Enum with hidden variants
// =============================================================================

#[derive(Debug, Clone, PartialEq, bpaf_unsynn::Bpaf)]
#[bpaf(options)]
enum Feature {
    #[bpaf(long)]
    Stable,
    #[bpaf(long, hide)]
    Experimental,
}

#[test]
fn hidden_variant_still_works() {
    let parser = Feature::parse();

    let r = parser.run_inner(&["--stable"]).unwrap();
    assert_eq!(r, Feature::Stable);

    // Hidden but still parseable
    let r = parser.run_inner(&["--experimental"]).unwrap();
    assert_eq!(r, Feature::Experimental);
}

// =============================================================================
// Enum with doc comments
// =============================================================================

/// Operation to perform
#[derive(Debug, Clone, PartialEq, bpaf_unsynn::Bpaf)]
#[bpaf(options)]
enum Op {
    /// Add items
    #[bpaf(command)]
    Add,
    /// Remove items
    #[bpaf(command)]
    Remove,
}

#[test]
fn enum_with_docs() {
    let parser = Op::parse();

    let r = parser.run_inner(&["add"]).unwrap();
    assert_eq!(r, Op::Add);
}

// =============================================================================
// Enum with req_flag on unit variants
// =============================================================================

#[derive(Debug, Clone, PartialEq, bpaf_unsynn::Bpaf)]
#[bpaf(options)]
enum Required {
    #[bpaf(long, req_flag(Required::Create))]
    Create,
    #[bpaf(long, req_flag(Required::Delete))]
    Delete,
}

#[test]
fn req_flag_variants() {
    let parser = Required::parse();

    let r = parser.run_inner(&["--create"]).unwrap();
    assert_eq!(r, Required::Create);

    let r = parser.run_inner(&["--delete"]).unwrap();
    assert_eq!(r, Required::Delete);
}

// =============================================================================
// Enum with nested external
// =============================================================================

#[derive(Debug, Clone, PartialEq, bpaf_unsynn::Bpaf)]
struct BuildOpts {
    #[bpaf(long)]
    release: bool,
}

#[derive(Debug, Clone, PartialEq, bpaf_unsynn::Bpaf)]
#[bpaf(options)]
enum CmdWithOpts {
    #[bpaf(command)]
    Build {
        #[bpaf(external(BuildOpts::parse))]
        opts: BuildOpts,
    },
}

#[test]
fn variant_with_nested_struct() {
    let parser = CmdWithOpts::parse();

    let r = parser.run_inner(&["build", "--release"]).unwrap();
    match r {
        CmdWithOpts::Build { opts } => assert!(opts.release),
    }
}

// =============================================================================
// Enum as subcommands with adjacent
// =============================================================================

#[derive(Debug, Clone, PartialEq, bpaf_unsynn::Bpaf)]
#[bpaf(options)]
enum AdjacentCmd {
    #[bpaf(command, adjacent)]
    Config {
        #[bpaf(long)]
        key: String,
        #[bpaf(long)]
        value: String,
    },
}

#[test]
fn adjacent_command_variant() {
    let parser = AdjacentCmd::parse();

    let r = parser
        .run_inner(&["config", "--key", "name", "--value", "test"])
        .unwrap();
    match r {
        AdjacentCmd::Config { key, value } => {
            assert_eq!(key, "name");
            assert_eq!(value, "test");
        }
    }
}

// =============================================================================
// Command with short alias
// =============================================================================

#[derive(Debug, Clone, PartialEq, bpaf_unsynn::Bpaf)]
#[bpaf(options)]
enum ShortCmd {
    /// Install packages
    #[bpaf(command, short('i'))]
    Install,
    /// Remove packages
    #[bpaf(command, short('r'))]
    Remove,
}

#[test]
fn command_with_short_alias() {
    let parser = ShortCmd::parse();

    // Primary command name
    let r = parser.run_inner(&["install"]).unwrap();
    assert_eq!(r, ShortCmd::Install);

    // Short alias
    let r = parser.run_inner(&["i"]).unwrap();
    assert_eq!(r, ShortCmd::Install);

    let r = parser.run_inner(&["r"]).unwrap();
    assert_eq!(r, ShortCmd::Remove);
}

// =============================================================================
// Command with hide
// =============================================================================

#[derive(Debug, Clone, PartialEq, bpaf_unsynn::Bpaf)]
#[bpaf(options)]
enum HiddenCmd {
    /// Public command
    #[bpaf(command)]
    Public,
    /// Internal command (hidden from help)
    #[bpaf(command, hide)]
    Internal,
}

#[test]
fn hidden_command_still_works() {
    let parser = HiddenCmd::parse();

    let r = parser.run_inner(&["public"]).unwrap();
    assert_eq!(r, HiddenCmd::Public);

    // Hidden but still parseable
    let r = parser.run_inner(&["internal"]).unwrap();
    assert_eq!(r, HiddenCmd::Internal);
}

// =============================================================================
// Command with fallback_to_usage
// =============================================================================

#[derive(Debug, Clone, PartialEq, bpaf_unsynn::Bpaf)]
#[bpaf(options)]
enum FallbackCmd {
    /// Show help when args missing
    #[bpaf(command, fallback_to_usage)]
    Help {
        #[bpaf(positional("TOPIC"))]
        topic: String,
    },
    /// Normal command
    #[bpaf(command)]
    Run,
}

#[test]
fn command_with_fallback_to_usage() {
    let parser = FallbackCmd::parse();

    // Normal command works
    let r = parser.run_inner(&["run"]).unwrap();
    assert_eq!(r, FallbackCmd::Run);

    // Command with args works
    let r = parser.run_inner(&["help", "topic"]).unwrap();
    match r {
        FallbackCmd::Help { topic } => assert_eq!(topic, "topic"),
        _ => panic!("Expected Help variant"),
    }

    // fallback_to_usage makes missing args show usage instead of error
    // We can't easily test the usage output, but we verify parsing works
}
