# bpaf_unsynn

[![License: MIT OR Apache-2.0](https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-blue)](LICENSE)
[![bpaf_unsynn on crates.io](https://img.shields.io/crates/v/bpaf_unsynn)](https://crates.io/crates/bpaf_unsynn)
[![Documentation](https://docs.rs/bpaf_unsynn/badge.svg)](https://docs.rs/bpaf_unsynn)

Derive macros for [bpaf](https://crates.io/crates/bpaf) Command Line Argument Parser using `unsynn` instead of `syn`.

## Overview

`bpaf_unsynn` is a drop-in replacement for `bpaf_derive` that provides the same derive macro functionality but uses the lighter-weight `unsynn` parser instead of `syn`. This results in:

## Quick Start

Add to your `Cargo.toml`:

```toml
[dependencies]
bpaf = "0.9"
bpaf_unsynn = "0.1"
```

Use in your code:

```rust
use bpaf::Parser;
use bpaf_unsynn::Bpaf;

#[derive(Debug, Clone, Bpaf)]
#[bpaf(options)]
/// Simple command line calculator
struct Args {
    /// First number
    #[bpaf(positional("X"))]
    x: f64,

    /// Second number
    #[bpaf(positional("Y"))]
    y: f64,

    /// Operation to perform
    #[bpaf(short, long)]
    operation: Operation,
}

#[derive(Debug, Clone, Bpaf)]
enum Operation {
    /// Add two numbers
    Add,
    /// Subtract two numbers
    Sub,
    /// Multiply two numbers
    Mul,
    /// Divide two numbers
    Div,
}

fn main() {
    let args = Args::parse();
    let result = match args.operation {
        Operation::Add => args.x + args.y,
        Operation::Sub => args.x - args.y,
        Operation::Mul => args.x * args.y,
        Operation::Div => args.x / args.y,
    };
    println!("{}", result);
}
```

## Features

### All bpaf Derive Attributes Supported

`bpaf_unsynn` supports all 60+ attributes from `bpaf_derive`:

**Variant/Enum Attributes:**
- `command`, `command("name")` - Define command variants
- `short('c')`, `long("name")` - **Multiple aliases supported!**
- `skip`, `hide`, `fallback_to_usage`, `help("text")`

**Field Attributes:**
- **Consumers**: `switch`, `flag`, `req_flag`, `argument`, `positional`, `any`, `external`, `pure`, `pure_with`
- **Names**: `short`, `long`, `env`
- **Post-parse**: `map`, `parse`, `optional`, `many`, `some`, `catch`, `collect`, `count`, `anywhere`, `adjacent`, `strict`, `non_strict`
- **Post-decor**: `guard`, `hide`, `hide_usage`, `custom_usage`, `fallback`, `fallback_with`, `debug_fallback`, `display_fallback`, `format_fallback`, `group_help`, `last`
- **Completion**: `complete`, `complete_shell`, `group`

**Struct-Level Attributes:**
- `options`, `command`, `parser`, `generate`, `private`, `boxed`
- `descr`, `header`, `footer`, `usage`, `version`, `max_width`
- `bpaf_path`, `cargo_helper`, `ignore_rustdoc`

See [the bpaf documentation](https://docs.rs/bpaf) for detailed attribute documentation.

### Better Error Messages

```rust
#[derive(Debug, Clone, Bpaf)]
#[bpaf(options)]
enum Command {
    #[bpaf(command, command("foo"), command("bar"))]
      ^^^^ error: Multiple 'command' attributes are not allowed.
           Use 'short' and 'long' for command aliases.
    Test,
}
```

Errors point to the exact location and provide clear guidance.

## Migration from bpaf_derive

`bpaf_unsynn` is a **drop-in replacement** for `bpaf_derive`. Simply update your dependencies:

**Before:**
```toml
[dependencies]
bpaf = { version = "0.9", features = ["derive"] }
```

**After:**
```toml
[dependencies]
bpaf = "0.9"
bpaf_unsynn = "0.5"
```

And update your derive macro:

**Before:**
```rust
use bpaf::Bpaf;  // or bpaf_derive::Bpaf

#[derive(Bpaf)]
struct Args { ... }
```

**After:**
```rust
use bpaf_unsynn::Bpaf;

#[derive(Bpaf)]
struct Args { ... }
```

That's it! All your existing code will work unchanged.

## Examples

### Basic Types

```rust
use bpaf_unsynn::Bpaf;

#[derive(Debug, Clone, Bpaf)]
#[bpaf(options)]
struct Args {
    /// Verbose output
    verbose: bool,           // -> switch

    /// Output file
    output: String,          // -> argument

    /// Port number
    port: Option<u16>,       // -> optional argument

    /// Input files
    inputs: Vec<String>,     // -> collect many arguments
}
```

### Commands

```rust
use bpaf_unsynn::Bpaf;

#[derive(Debug, Clone, Bpaf)]
#[bpaf(options)]
enum Command {
    #[bpaf(command)]
    /// Initialize a new project
    Init {
        /// Project name
        name: String,
    },

    #[bpaf(command)]
    /// Build the project
    Build {
        /// Enable optimizations
        #[bpaf(short, long)]
        release: bool,
    },
}
```

### Custom Parsers

```rust
use bpaf_unsynn::Bpaf;

fn parse_size(s: String) -> Result<usize, String> {
    s.parse().map_err(|e| format!("Invalid size: {}", e))
}

#[derive(Debug, Clone, Bpaf)]
#[bpaf(options)]
struct Args {
    /// Buffer size in bytes
    #[bpaf(argument("SIZE"), parse(parse_size))]
    size: usize,
}
```

### Fallbacks and Guards

```rust
use bpaf_unsynn::Bpaf;

#[derive(Debug, Clone, Bpaf)]
#[bpaf(options)]
struct Args {
    /// Number of threads (default: 4)
    #[bpaf(argument("N"), fallback(4))]
    threads: usize,

    /// Timeout in seconds
    #[bpaf(
        argument("SECS"),
        guard(check_timeout, "Timeout must be between 1 and 3600")
    )]
    timeout: u32,
}

fn check_timeout(t: &u32) -> bool {
    *t >= 1 && *t <= 3600
}
```

## Testing

`bpaf_unsynn` has **comprehensive test coverage**:

- **233 tests** across 20 test files
- **31 compile-fail UI tests** for error messages
- **100% attribute coverage** - every attribute tested
- **Edge case testing** - complex generics, nested types, etc.

Run tests:
```bash
cargo test
```

Run compile-fail tests:
```bash
cargo test --test compile_fail
```

## Documentation

- [bpaf documentation](https://docs.rs/bpaf) - Main parser documentation
- [API documentation](https://docs.rs/bpaf_unsynn) - bpaf_unsynn API reference
- [Examples](tests/) - Comprehensive test examples

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

Licensed under either of

 * Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Authors

- Luca Barbato <lu_zero@gentoo.org>

## Acknowledgments

- Built on top of [bpaf](https://github.com/pacak/bpaf)
- Uses [unsynn](https://crates.io/crates/unsynn) for lightweight parsing
- Alternate implementation of `bpaf_derive`, a good deal of logic shared with it
