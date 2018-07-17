# cargo-download-deps

This is a [Cargo](http://doc.crates.io) subcommand which
transitively downloads a project's dependencies.

## Installation

Currently this can be installed with:

```
$ cargo install --git https://github.com/robbym/cargo-download-deps.git
```

## Example Usage
```

$ cargo download-deps --download-path=/path/to/dest --config=/path/to/Cargo.toml

```

## Requirements

The Cargo.toml must have at least:
```toml
[package]
name = "dummy"
version = "0.0.0"
```

And an emptry ./src/lib.rs.
