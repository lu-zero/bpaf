# bpaf_unsynn Test Checklist

This checklist covers all derive features documented in bpaf.
Tests are organized by feature category with no duplicates.

**Test Summary: 148 tests across 12 files (83% code coverage)**

## Basic Types and Type Inference (basic_types.rs - 10 tests)

- [x] `bool` field -> switch
- [x] `String` field -> argument
- [x] `usize`/numeric field -> argument
- [x] `Option<T>` field -> optional argument
- [x] `Vec<T>` field -> many arguments
- [x] `()` unit field -> req_flag(())
- [x] Nested generics: `Option<Vec<T>>`, `Vec<Option<T>>` (via external)
- [x] Complex generics: `HashMap<K, V>` (via external)
- [x] `PathBuf` type

## Name Attributes (name_attrs.rs - 10 tests)

- [x] `#[bpaf(short)]` - derive from field name
- [x] `#[bpaf(short('x'))]` - explicit char
- [x] `#[bpaf(long)]` - derive from field name (kebab-case)
- [x] `#[bpaf(long("custom-name"))]` - explicit name
- [x] `#[bpaf(short, long)]` - both
- [x] `#[bpaf(env("VAR"))]` - environment variable
- [x] `#[bpaf(env(CONST))]` - env from constant
- [x] `short`, `long`, `env` combinations
- [x] Arguments with short and long

## Consumer Attributes (consumers.rs - 14 tests)

- [x] `#[bpaf(switch)]` - boolean switch
- [x] `#[bpaf(flag(present, absent))]` - flag with values (bool and enum)
- [x] `#[bpaf(argument("META"))]` - argument with metavar
- [x] `#[bpaf(positional("META"))]` - positional argument
- [x] `#[bpaf(req_flag(value))]` - required flag
- [x] `#[bpaf(any("META", check))]` - any parser
- [x] `#[bpaf(any::<Type>("META", check))]` - typed any (advanced.rs)
- [x] `#[bpaf(external)]` - external parser (field name)
- [x] `#[bpaf(external(parser_fn))]` - external with path
- [x] `#[bpaf(pure(value))]` - pure value
- [x] `#[bpaf(pure_with(fn))]` - pure with function

## PostParse Attributes (postparse.rs - 9 tests)

- [x] `#[bpaf(map(fn))]` - transform value
- [x] `#[bpaf(parse(fn))]` - fallible transform (via external)
- [x] `#[bpaf(optional)]` - explicit optional
- [x] `#[bpaf(many)]` - explicit many
- [x] `#[bpaf(some("error"))]` - require at least one
- [x] `#[bpaf(catch)]` - catch parse errors (via external)
- [x] `#[bpaf(collect)]` - collect into container (advanced.rs)
- [x] `#[bpaf(count)]` - count occurrences (via external)
- [x] `#[bpaf(anywhere)]` - parse anywhere (advanced.rs, on any parser)
- [x] `#[bpaf(adjacent)]` - struct-level adjacent (struct_attrs.rs)
- [x] `#[bpaf(strict)]` - strict parsing (via external)
- [x] `#[bpaf(non_strict)]` - non-strict parsing (advanced.rs)
- [x] Chained: map then optional

## PostDecor Attributes (postdecor.rs - 16 tests)

- [x] `#[bpaf(guard(check, "msg"))]` - validation
- [x] `#[bpaf(hide)]` - hide from help
- [x] `#[bpaf(hide_usage)]` - hide from usage
- [x] `#[bpaf(custom_usage("text"))]` - custom usage
- [x] `#[bpaf(fallback(value))]` - fallback value
- [x] `#[bpaf(fallback_with(fn))]` - dynamic fallback
- [x] `#[bpaf(group_help("text"))]` - group help
- [x] `#[bpaf(debug_fallback)]` - debug format fallback
- [x] `#[bpaf(display_fallback)]` - display format fallback
- [x] `#[bpaf(last)]` - use last occurrence
- [x] `#[bpaf(complete(fn))]` - shell completion

## Help and Documentation (help.rs - 10 tests)

- [x] Doc comments as help text
- [x] `#[bpaf(help("text"))]` - explicit help
- [x] `#[bpaf(ignore_rustdoc)]` - ignore doc comments (advanced.rs)
- [x] Multi-line doc comments

## Struct-level Attributes (struct_attrs.rs - 21 tests)

- [x] `#[bpaf(options)]` - options mode
- [x] `#[bpaf(command)]` - command mode (derived name)
- [x] `#[bpaf(command("name"))]` - named command (BUG FIXED)
- [x] `#[bpaf(parser)]` - parser mode (default)
- [x] `#[bpaf(adjacent)]` - adjacent struct
- [x] `#[bpaf(generate(name))]` - custom function name
- [x] `#[bpaf(private)]` - private function
- [x] `#[bpaf(boxed)]` - boxed parser
- [x] `#[bpaf(path(::bpaf))]` - custom path (advanced.rs)
- [x] `#[bpaf(group_help("..."))]` - group help at struct level
- [x] `#[bpaf(cargo_helper("name"))]` - cargo subcommand (advanced.rs)

## Options Mode Attributes (help.rs + struct_attrs.rs)

- [x] `#[bpaf(descr("..."))]` - description
- [x] `#[bpaf(footer("..."))]` - footer text
- [x] `#[bpaf(header("..."))]` - header text
- [x] `#[bpaf(usage("..."))]` - custom usage
- [x] `#[bpaf(version)]` - version info
- [x] `#[bpaf(version("..."))]` - custom version
- [x] `#[bpaf(max_width(N))]` - max help width
- [x] `#[bpaf(fallback_to_usage)]` - show usage on error
- [x] `#[bpaf(hide)]` - top-level hide
- [x] `#[bpaf(hide_usage)]` - top-level hide_usage
- [x] `#[bpaf(custom_usage("..."))]` - top-level custom_usage

## Enum Support (enums.rs - 13 tests)

### Basic Enums
- [x] Unit variants (flag-based)
- [x] Struct variants with fields
- [x] Tuple variants (single field)
- [x] Mixed variant types
- [x] Multi-field tuple variants (advanced.rs)

### Enum with Commands
- [x] `#[bpaf(command)]` on variants
- [x] `#[bpaf(command("name"))]` custom name
- [x] Command with fields
- [x] `#[bpaf(skip)]` - skip variant (used with fallback)
- [x] `#[bpaf(hide)]` - hide variant
- [x] `#[bpaf(long("alias"))]` - command alias
- [x] `#[bpaf(req_flag(Enum::Variant))]` - req_flag on variants

### Enum Variant Doc Comments
- [x] Doc comments as command help
- [x] Nested external in variant

## Integration Tests (integration.rs - 16 tests)

- [x] Parsing succeeds with valid input
- [x] Parsing fails with missing required
- [x] Parsing fails with invalid values
- [x] Environment variable fallback works (tested in name_attrs.rs)
- [x] Default/fallback values work
- [x] Command dispatch works
- [x] Complex argument patterns
- [x] Attached values (--option=value, -nVAL)
- [x] Subcommand patterns
- [x] Mutually exclusive options
- [x] Flat struct composition
- [x] Multiple positional arguments
- [x] Real-world CLI example
- [x] PathBuf and stdlib types
- [x] Multiple short flags

## Advanced Features (advanced.rs - 11 tests)

- [x] `#[bpaf(any::<Type>("META", check))]` - typed any
- [x] `#[bpaf(collect)]` - collect into container
- [x] `#[bpaf(anywhere)]` - parse anywhere (on any parser)
- [x] `#[bpaf(non_strict)]` - non-strict positional
- [x] `#[bpaf(ignore_rustdoc)]` - ignore doc comments
- [x] `#[bpaf(path(::bpaf))]` - custom bpaf path
- [x] `#[bpaf(cargo_helper("name"))]` - cargo subcommand
- [x] Multi-field tuple variants
- [x] Adjacent via struct composition
- [x] Complete attribute

## Compatibility Tests (bpaf_derive_compat.rs - 5 tests)

- [x] Help output verification
- [x] Command with fallback
- [x] Pure with optional
- [x] Command parsing
- [x] Fallback behavior
