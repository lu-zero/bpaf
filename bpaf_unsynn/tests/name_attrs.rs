//! Tests for name attributes: short, long, env
//!
//! Covers: #[bpaf(short)], #[bpaf(short('x'))], #[bpaf(long)],
//! #[bpaf(long("name"))], #[bpaf(env("VAR"))], and combinations.

// =============================================================================
// Short attribute
// =============================================================================

/// short - derive from field name
#[derive(Debug, Clone, PartialEq, bpaf_unsynn::Bpaf)]
#[bpaf(options)]
struct ShortDerived {
    #[bpaf(short)]
    verbose: bool,
}

#[test]
fn short_derived_from_field_name() {
    let parser = short_derived();

    let r = parser.run_inner(&["-v"]).unwrap();
    assert!(r.verbose);
}

/// short('x') - explicit char
#[derive(Debug, Clone, PartialEq, bpaf_unsynn::Bpaf)]
#[bpaf(options)]
struct ShortExplicit {
    #[bpaf(short('q'))]
    quiet: bool,
}

#[test]
fn short_explicit_char() {
    let parser = short_explicit();

    let r = parser.run_inner(&["-q"]).unwrap();
    assert!(r.quiet);
}

// =============================================================================
// Long attribute
// =============================================================================

/// long - derive from field name (kebab-case)
#[derive(Debug, Clone, PartialEq, bpaf_unsynn::Bpaf)]
#[bpaf(options)]
struct LongDerived {
    #[bpaf(long)]
    my_option: bool,
}

#[test]
fn long_derived_kebab_case() {
    let parser = long_derived();

    // Converted to kebab-case
    let r = parser.run_inner(&["--my-option"]).unwrap();
    assert!(r.my_option);
}

/// long("custom-name") - explicit name
#[derive(Debug, Clone, PartialEq, bpaf_unsynn::Bpaf)]
#[bpaf(options)]
struct LongExplicit {
    #[bpaf(long("custom-flag"))]
    flag: bool,
}

#[test]
fn long_explicit_name() {
    let parser = long_explicit();

    let r = parser.run_inner(&["--custom-flag"]).unwrap();
    assert!(r.flag);
}

// =============================================================================
// Short and Long combined
// =============================================================================

#[derive(Debug, Clone, PartialEq, bpaf_unsynn::Bpaf)]
#[bpaf(options)]
struct ShortLongCombined {
    #[bpaf(short, long)]
    verbose: bool,
}

#[test]
fn short_and_long_both_work() {
    let parser = short_long_combined();

    // Short works
    let r = parser.run_inner(&["-v"]).unwrap();
    assert!(r.verbose);

    // Long works
    let r = parser.run_inner(&["--verbose"]).unwrap();
    assert!(r.verbose);
}

#[derive(Debug, Clone, PartialEq, bpaf_unsynn::Bpaf)]
#[bpaf(options)]
struct ShortLongExplicit {
    #[bpaf(short('d'), long("debug-mode"))]
    debug: bool,
}

#[test]
fn short_and_long_explicit() {
    let parser = short_long_explicit();

    let r = parser.run_inner(&["-d"]).unwrap();
    assert!(r.debug);

    let r = parser.run_inner(&["--debug-mode"]).unwrap();
    assert!(r.debug);
}

// =============================================================================
// Env attribute
// =============================================================================

#[derive(Debug, Clone, PartialEq, bpaf_unsynn::Bpaf)]
#[bpaf(options)]
struct EnvString {
    #[bpaf(env("MY_VAR"), long)]
    value: Option<String>,
}

#[test]
fn env_variable_fallback() {
    // Set env var
    std::env::set_var("MY_VAR", "from_env");

    let parser = env_string();

    // Without arg, uses env
    let r = parser.run_inner(&[]).unwrap();
    assert_eq!(r.value, Some("from_env".to_string()));

    // With arg, arg takes precedence
    let r = parser.run_inner(&["--value", "from_arg"]).unwrap();
    assert_eq!(r.value, Some("from_arg".to_string()));

    // Clean up
    std::env::remove_var("MY_VAR");
}

/// env from constant expression
const ENV_VAR_NAME: &str = "TEST_CONST_VAR";

#[derive(Debug, Clone, PartialEq, bpaf_unsynn::Bpaf)]
#[bpaf(options)]
struct EnvConst {
    #[bpaf(env(ENV_VAR_NAME), long)]
    value: Option<String>,
}

#[test]
fn env_from_constant() {
    std::env::set_var(ENV_VAR_NAME, "const_value");

    let parser = env_const();
    let r = parser.run_inner(&[]).unwrap();
    assert_eq!(r.value, Some("const_value".to_string()));

    std::env::remove_var(ENV_VAR_NAME);
}

// =============================================================================
// Env-only fields (no short/long)
// =============================================================================

/// Field with only env attribute - no command line option, only environment variable
#[derive(Debug, Clone, PartialEq, bpaf_unsynn::Bpaf)]
#[bpaf(options)]
struct EnvOnlySet {
    #[bpaf(env("ENV_ONLY_SET_VAR"))]
    value: Option<String>,
}

#[test]
fn env_only_reads_from_environment() {
    std::env::set_var("ENV_ONLY_SET_VAR", "env_value");

    let parser = env_only_set();
    let r = parser.run_inner(&[]).unwrap();
    assert_eq!(r.value, Some("env_value".to_string()));

    std::env::remove_var("ENV_ONLY_SET_VAR");
}

#[derive(Debug, Clone, PartialEq, bpaf_unsynn::Bpaf)]
#[bpaf(options)]
struct EnvOnlyUnset {
    #[bpaf(env("ENV_ONLY_UNSET_VAR"))]
    value: Option<String>,
}

#[test]
fn env_only_returns_none_when_unset() {
    // Make sure the var is not set
    std::env::remove_var("ENV_ONLY_UNSET_VAR");

    let parser = env_only_unset();
    let r = parser.run_inner(&[]).unwrap();
    assert_eq!(r.value, None);
}

/// Env-only with a required field (uses fallback)
#[derive(Debug, Clone, PartialEq, bpaf_unsynn::Bpaf)]
#[bpaf(options)]
struct EnvOnlyRequiredSet {
    #[bpaf(env("ENV_REQUIRED_SET_VAR"), fallback("default_value".to_string()))]
    value: String,
}

#[test]
fn env_only_required_uses_env() {
    std::env::set_var("ENV_REQUIRED_SET_VAR", "from_env");

    let parser = env_only_required_set();
    let r = parser.run_inner(&[]).unwrap();
    assert_eq!(r.value, "from_env");

    std::env::remove_var("ENV_REQUIRED_SET_VAR");
}

#[derive(Debug, Clone, PartialEq, bpaf_unsynn::Bpaf)]
#[bpaf(options)]
struct EnvOnlyRequiredUnset {
    #[bpaf(env("ENV_REQUIRED_UNSET_VAR"), fallback("default_value".to_string()))]
    value: String,
}

#[test]
fn env_only_required_uses_fallback() {
    std::env::remove_var("ENV_REQUIRED_UNSET_VAR");

    let parser = env_only_required_unset();
    let r = parser.run_inner(&[]).unwrap();
    assert_eq!(r.value, "default_value");
}

/// Env-only with numeric type
#[derive(Debug, Clone, PartialEq, bpaf_unsynn::Bpaf)]
#[bpaf(options)]
struct EnvOnlyNumeric {
    #[bpaf(env("ENV_PORT"))]
    port: Option<u16>,
}

#[test]
fn env_only_numeric_parses() {
    std::env::set_var("ENV_PORT", "8080");

    let parser = env_only_numeric();
    let r = parser.run_inner(&[]).unwrap();
    assert_eq!(r.port, Some(8080));

    std::env::remove_var("ENV_PORT");
}

/// Multiple env-only fields
#[derive(Debug, Clone, PartialEq, bpaf_unsynn::Bpaf)]
#[bpaf(options)]
struct MultipleEnvOnly {
    #[bpaf(env("DB_HOST"))]
    host: Option<String>,
    #[bpaf(env("DB_PORT"))]
    port: Option<u16>,
    #[bpaf(env("DB_NAME"))]
    database: Option<String>,
}

#[test]
fn multiple_env_only_fields() {
    std::env::set_var("DB_HOST", "localhost");
    std::env::set_var("DB_PORT", "5432");
    std::env::set_var("DB_NAME", "mydb");

    let parser = multiple_env_only();
    let r = parser.run_inner(&[]).unwrap();
    assert_eq!(r.host, Some("localhost".to_string()));
    assert_eq!(r.port, Some(5432));
    assert_eq!(r.database, Some("mydb".to_string()));

    std::env::remove_var("DB_HOST");
    std::env::remove_var("DB_PORT");
    std::env::remove_var("DB_NAME");
}

// =============================================================================
// All three combined: short, long, env
// =============================================================================

#[derive(Debug, Clone, PartialEq, bpaf_unsynn::Bpaf)]
#[bpaf(options)]
struct AllThree {
    #[bpaf(short('c'), long("config"), env("CONFIG_PATH"))]
    config: Option<String>,
}

#[test]
fn short_long_env_all_work() {
    let parser = all_three();

    // Short
    let r = parser.run_inner(&["-c", "short.cfg"]).unwrap();
    assert_eq!(r.config, Some("short.cfg".to_string()));

    // Long
    let r = parser.run_inner(&["--config", "long.cfg"]).unwrap();
    assert_eq!(r.config, Some("long.cfg".to_string()));

    // Env
    std::env::set_var("CONFIG_PATH", "env.cfg");
    let r = parser.run_inner(&[]).unwrap();
    assert_eq!(r.config, Some("env.cfg".to_string()));
    std::env::remove_var("CONFIG_PATH");
}

// =============================================================================
// Arguments with short/long
// =============================================================================

#[derive(Debug, Clone, PartialEq, bpaf_unsynn::Bpaf)]
#[bpaf(options)]
struct ArgumentWithNames {
    #[bpaf(short('n'), long("name"), argument("NAME"))]
    name: String,
}

#[test]
fn argument_with_short_and_long() {
    let parser = argument_with_names();

    // Short with value
    let r = parser.run_inner(&["-n", "Alice"]).unwrap();
    assert_eq!(r.name, "Alice");

    // Short with attached value
    let r = parser.run_inner(&["-nBob"]).unwrap();
    assert_eq!(r.name, "Bob");

    // Long with value
    let r = parser.run_inner(&["--name", "Charlie"]).unwrap();
    assert_eq!(r.name, "Charlie");

    // Long with = value
    let r = parser.run_inner(&["--name=Dave"]).unwrap();
    assert_eq!(r.name, "Dave");
}
