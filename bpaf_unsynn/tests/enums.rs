//! Tests for enum support
//!
//! Covers: unit variants, tuple variants, struct variants, command variants,
//! default variants, explicit commands, aliases, and mixed combinations

use bpaf::Parser;

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

// =============================================================================
// Enum with cargo_helper
// =============================================================================

#[derive(Debug, Clone, PartialEq, bpaf_unsynn::Bpaf)]
#[bpaf(cargo_helper("myenum"))]
enum CargoCmd {
    #[bpaf(command)]
    Build,
    #[bpaf(command)]
    Test,
}

#[test]
fn enum_with_cargo_helper() {
    // cargo_helper returns impl Parser, needs .to_options()
    let parser = CargoCmd::parse().to_options();

    let r = parser.run_inner(&["build"]).unwrap();
    assert_eq!(r, CargoCmd::Build);

    let r = parser.run_inner(&["test"]).unwrap();
    assert_eq!(r, CargoCmd::Test);
}

// =============================================================================
// Command variant with doc comment help
// =============================================================================

#[derive(Debug, Clone, PartialEq, bpaf_unsynn::Bpaf)]
#[bpaf(options)]
enum CmdWithHelp {
    /// First command help from doc comment
    #[bpaf(command)]
    First,
    /// Second command help from doc comment
    #[bpaf(command)]
    Second,
}

#[test]
fn command_with_doc_comment_help() {
    let parser = CmdWithHelp::parse();

    let r = parser.run_inner(&["first"]).unwrap();
    assert_eq!(r, CmdWithHelp::First);

    let r = parser.run_inner(&["second"]).unwrap();
    assert_eq!(r, CmdWithHelp::Second);

    // Verify help contains the doc comment help text
    let help = parser.run_inner(&["--help"]).unwrap_err().unwrap_stdout();
    assert!(
        help.contains("First command help from doc comment"),
        "Help should contain first command help: {}",
        help
    );
    assert!(
        help.contains("Second command help from doc comment"),
        "Help should contain second command help: {}",
        help
    );
}

// =============================================================================
// Command variant with explicit help attribute
// =============================================================================

#[derive(Debug, Clone, PartialEq, bpaf_unsynn::Bpaf)]
#[bpaf(options)]
enum CmdWithExplicitHelp {
    /// Doc comment (should be overridden)
    #[bpaf(command, help("Explicit help overrides doc comment"))]
    Override,
    #[bpaf(command, help("Help without doc comment"))]
    NoDoc,
}

#[test]
fn command_with_explicit_help_attribute() {
    let parser = CmdWithExplicitHelp::parse();

    let r = parser.run_inner(&["override"]).unwrap();
    assert_eq!(r, CmdWithExplicitHelp::Override);

    let r = parser.run_inner(&["no-doc"]).unwrap();
    assert_eq!(r, CmdWithExplicitHelp::NoDoc);

    // Verify help contains the explicit help text, not doc comment
    let help = parser.run_inner(&["--help"]).unwrap_err().unwrap_stdout();
    assert!(
        help.contains("Explicit help overrides doc comment"),
        "Help should contain explicit help text: {}",
        help
    );
    assert!(
        !help.contains("should be overridden"),
        "Help should NOT contain overridden doc comment: {}",
        help
    );
    assert!(
        help.contains("Help without doc comment"),
        "Help should contain help for variant without doc: {}",
        help
    );
}

// =============================================================================
// Enum with both short and long aliases on command
// =============================================================================

#[derive(Debug, Clone, PartialEq, bpaf_unsynn::Bpaf)]
#[bpaf(options)]
enum CmdWithBothAliases {
    #[bpaf(command, short('i'), long("inst"))]
    Install,
    #[bpaf(command, short('u'), long("upgrade"))]
    Update,
}

#[test]
fn command_with_short_and_long_alias() {
    let parser = CmdWithBothAliases::parse();

    // Primary command name
    let r = parser.run_inner(&["install"]).unwrap();
    assert_eq!(r, CmdWithBothAliases::Install);

    // Short alias
    let r = parser.run_inner(&["i"]).unwrap();
    assert_eq!(r, CmdWithBothAliases::Install);

    // Long alias
    let r = parser.run_inner(&["inst"]).unwrap();
    assert_eq!(r, CmdWithBothAliases::Install);

    // All three for Update
    let r = parser.run_inner(&["update"]).unwrap();
    assert_eq!(r, CmdWithBothAliases::Update);

    let r = parser.run_inner(&["u"]).unwrap();
    assert_eq!(r, CmdWithBothAliases::Update);

    let r = parser.run_inner(&["upgrade"]).unwrap();
    assert_eq!(r, CmdWithBothAliases::Update);
}

// =============================================================================
// Non-command enum variants with fields (flag-based with struct fields)
// =============================================================================

/// Enum variant without #[bpaf(command)] but with struct fields.
/// This exercises the code path for flag-based variants with fields.
#[derive(Debug, Clone, PartialEq, bpaf_unsynn::Bpaf)]
enum FlagVariantWithFields {
    /// Variant with fields but no command
    Enabled {
        #[bpaf(long, argument("LEVEL"))]
        level: u32,
    },
}

#[test]
fn flag_variant_with_struct_fields() {
    let parser = FlagVariantWithFields::parse().to_options();

    let r = parser.run_inner(&["--level", "5"]).unwrap();
    assert_eq!(r, FlagVariantWithFields::Enabled { level: 5 });
}

/// Enum with multiple non-command struct variants
#[derive(Debug, Clone, PartialEq, bpaf_unsynn::Bpaf)]
enum MultiVariantNoCommand {
    /// First option
    First {
        #[bpaf(long, argument("A"))]
        alpha: String,
    },
    /// Second option
    Second {
        #[bpaf(long, argument("B"))]
        beta: i32,
    },
}

#[test]
fn multi_variant_without_command() {
    let parser = MultiVariantNoCommand::parse().to_options();

    let r = parser.run_inner(&["--alpha", "test"]).unwrap();
    assert_eq!(
        r,
        MultiVariantNoCommand::First {
            alpha: "test".to_string()
        }
    );

    let r = parser.run_inner(&["--beta", "42"]).unwrap();
    assert_eq!(r, MultiVariantNoCommand::Second { beta: 42 });
}

// =============================================================================
// Unit enum variant (non-command) with help text via doc comments
// =============================================================================

/// Enum with unit variants that have help text via doc comments
#[derive(Debug, Clone, PartialEq, bpaf_unsynn::Bpaf)]
enum UnitVariantWithHelp {
    /// Enable verbose mode
    Verbose,
    /// Enable quiet mode
    Quiet,
}

#[test]
fn unit_variant_with_help_text() {
    let parser = UnitVariantWithHelp::parse().to_options();

    let r = parser.run_inner(&["--verbose"]).unwrap();
    assert_eq!(r, UnitVariantWithHelp::Verbose);

    let r = parser.run_inner(&["--quiet"]).unwrap();
    assert_eq!(r, UnitVariantWithHelp::Quiet);

    // Check help output has the doc comments
    let help = parser.run_inner(&["--help"]).unwrap_err().unwrap_stdout();
    assert!(
        help.contains("verbose mode"),
        "Help should contain 'verbose mode': {}",
        help
    );
}

// =============================================================================
// Boxed command mode (struct level)
// =============================================================================

#[derive(Debug, Clone, PartialEq, bpaf_unsynn::Bpaf)]
#[bpaf(command, boxed)]
struct BoxedCmd {
    #[bpaf(long)]
    verbose: bool,
}

#[test]
fn boxed_command_mode() {
    let parser = BoxedCmd::parse().to_options();

    let r = parser.run_inner(&["boxedcmd"]).unwrap();
    assert!(!r.verbose);

    let r = parser.run_inner(&["boxedcmd", "--verbose"]).unwrap();
    assert!(r.verbose);
}

// =============================================================================
// Tuple variants - additional tests
// =============================================================================

/// Tuple variant with multiple types (all positional by default)
#[derive(Debug, Clone, PartialEq, bpaf_unsynn::Bpaf)]
#[bpaf(options)]
enum TupleMultiType {
    #[bpaf(command)]
    Transfer(String, u32),
    #[bpaf(command)]
    Query(String),
}

#[test]
fn tuple_variant_multiple_types() {
    let parser = TupleMultiType::parse();

    let r = parser.run_inner(&["transfer", "account", "100"]).unwrap();
    assert_eq!(
        r,
        TupleMultiType::Transfer("account".to_string(), 100)
    );

    let r = parser.run_inner(&["query", "status"]).unwrap();
    assert_eq!(r, TupleMultiType::Query("status".to_string()));
}

/// Tuple variant with optional type
#[derive(Debug, Clone, PartialEq, bpaf_unsynn::Bpaf)]
#[bpaf(options)]
enum TupleWithOption {
    #[bpaf(command)]
    Fetch(String, Option<u32>),
}

#[test]
fn tuple_variant_with_option_type() {
    let parser = TupleWithOption::parse();

    // With optional value
    let r = parser.run_inner(&["fetch", "data", "5"]).unwrap();
    assert_eq!(
        r,
        TupleWithOption::Fetch("data".to_string(), Some(5))
    );

    // Without optional value
    let r = parser.run_inner(&["fetch", "data"]).unwrap();
    assert_eq!(r, TupleWithOption::Fetch("data".to_string(), None));
}

/// Tuple variant with Vec type
#[derive(Debug, Clone, PartialEq, bpaf_unsynn::Bpaf)]
#[bpaf(options)]
enum TupleWithVec {
    #[bpaf(command)]
    Process(String, Vec<String>),
}

#[test]
fn tuple_variant_with_vec_type() {
    let parser = TupleWithVec::parse();

    let r = parser
        .run_inner(&["process", "task", "arg1", "arg2", "arg3"])
        .unwrap();
    assert_eq!(
        r,
        TupleWithVec::Process(
            "task".to_string(),
            vec!["arg1".to_string(), "arg2".to_string(), "arg3".to_string()]
        )
    );

    // Empty vec
    let r = parser.run_inner(&["process", "task"]).unwrap();
    assert_eq!(
        r,
        TupleWithVec::Process("task".to_string(), vec![])
    );
}

// =============================================================================
// Tuple variants with field attributes
// =============================================================================

/// Tuple variant with explicit field attributes
#[derive(Debug, Clone, PartialEq, bpaf_unsynn::Bpaf)]
#[bpaf(options)]
enum TupleWithFieldAttrs {
    #[bpaf(command)]
    Send(
        #[bpaf(long("to"), argument("ADDR"))]
        String,
        #[bpaf(positional("MESSAGE"))]
        String,
    ),
    #[bpaf(command)]
    Receive(
        #[bpaf(long("from"), argument("ADDR"))]
        String,
    ),
}

#[test]
fn tuple_variant_with_field_attributes() {
    let parser = TupleWithFieldAttrs::parse();

    let r = parser
        .run_inner(&["send", "--to", "alice@example.com", "Hello!"])
        .unwrap();
    assert_eq!(
        r,
        TupleWithFieldAttrs::Send("alice@example.com".to_string(), "Hello!".to_string())
    );

    let r = parser
        .run_inner(&["receive", "--from", "bob@example.com"])
        .unwrap();
    assert_eq!(
        r,
        TupleWithFieldAttrs::Receive("bob@example.com".to_string())
    );
}

/// Tuple variant with mixed named and positional fields
#[derive(Debug, Clone, PartialEq, bpaf_unsynn::Bpaf)]
#[bpaf(options)]
enum TupleMixedAttrs {
    #[bpaf(command)]
    Create(
        #[bpaf(positional("NAME"))]
        String,
        #[bpaf(long("count"), argument("COUNT"))]
        u32,
        #[bpaf(short('v'), long("verbose"))]
        bool,
    ),
}

#[test]
fn tuple_variant_mixed_field_attrs() {
    let parser = TupleMixedAttrs::parse();

    let r = parser
        .run_inner(&["create", "myfile", "--count", "5", "--verbose"])
        .unwrap();
    assert_eq!(r, TupleMixedAttrs::Create("myfile".to_string(), 5, true));

    // Without optional flag
    let r = parser
        .run_inner(&["create", "other", "--count", "10"])
        .unwrap();
    assert_eq!(r, TupleMixedAttrs::Create("other".to_string(), 10, false));
}

/// Tuple variant with optional field via attribute
#[derive(Debug, Clone, PartialEq, bpaf_unsynn::Bpaf)]
#[bpaf(options)]
enum TupleOptionalAttr {
    #[bpaf(command)]
    Run(
        #[bpaf(positional("SCRIPT"))]
        String,
        #[bpaf(long("arg"))]
        Option<String>,
    ),
}

#[test]
fn tuple_variant_optional_field_attr() {
    let parser = TupleOptionalAttr::parse();

    let r = parser.run_inner(&["run", "script.sh"]).unwrap();
    assert_eq!(r, TupleOptionalAttr::Run("script.sh".to_string(), None));

    let r = parser
        .run_inner(&["run", "script.sh", "--arg", "value"])
        .unwrap();
    assert_eq!(
        r,
        TupleOptionalAttr::Run("script.sh".to_string(), Some("value".to_string()))
    );
}

/// Tuple variant with doc comment on field
#[derive(Debug, Clone, PartialEq, bpaf_unsynn::Bpaf)]
#[bpaf(options)]
enum TupleWithDocs {
    #[bpaf(command)]
    Echo(
        /// The message to echo
        #[bpaf(positional("MSG"))]
        String,
    ),
}

#[test]
fn tuple_variant_with_doc_on_field() {
    let parser = TupleWithDocs::parse();

    let r = parser.run_inner(&["echo", "hello"]).unwrap();
    assert_eq!(r, TupleWithDocs::Echo("hello".to_string()));
}

// =============================================================================
// Tuple variant enum (non-command, flag-based)
// =============================================================================

#[derive(Debug, Clone, PartialEq, bpaf_unsynn::Bpaf)]
#[bpaf(options)]
enum TupleVariantEnum {
    #[bpaf(long)]
    First(String),

    #[bpaf(long)]
    Second(String, i32),
}

#[test]
fn tuple_variant_enum_with_fields() {
    let parser = TupleVariantEnum::parse();

    // Test single field tuple variant - fields are positional by default
    let r = parser.run_inner(&["test"]).unwrap();
    assert_eq!(r, TupleVariantEnum::First("test".to_string()));

    // Test multi-field tuple variant - both fields are positional
    let r = parser.run_inner(&["data", "42"]).unwrap();
    assert_eq!(r, TupleVariantEnum::Second("data".to_string(), 42));
}

