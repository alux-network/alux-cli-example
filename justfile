# List the recipes.
default:
    @just --list

# Check formatting without rewriting any file.
fmt:
    cargo fmt --all -- --check

# Compile every crate, target, and test with all features enabled.
build:
    cargo build --workspace --all-features --all-targets

# Run every test; doc tests run separately because Nextest does not support them. Arguments go to
# both runs, as to `cargo test`: `just test -p notes-cli-mixed` or `just test round_trip`.
# `--workspace` is dropped when a package is selected, since it would select every package again.
test *args:
    cargo nextest run {{ if args =~ '(^|\s)(-p|--package)(\s|=|$)' { "" } else { "--workspace" } }} --all-features --no-fail-fast {{ args }}
    cargo test {{ if args =~ '(^|\s)(-p|--package)(\s|=|$)' { "" } else { "--workspace" } }} --all-features --doc {{ args }}

# Lint every target, treating warnings as errors.
clippy:
    cargo clippy --workspace --all-features --all-targets -- -D warnings

# Build the docs, denying rustdoc warnings such as broken intra-doc links.
doc:
    RUSTDOCFLAGS="-D warnings" cargo doc --workspace --all-features --no-deps

# Check that each crate packages cleanly; arguments go to `cargo package`: `just package --allow-dirty`.
package *args:
    cargo package --list --workspace {{ args }} > /dev/null

# Print the Rust declaration a Markdown spec compiles to.
rust markdown:
    cargo run -q -p notes-cli-md-tool -- rust {{ markdown }}

# Print the Markdown spec of a binary's own command line: notes-cli-rust-direct, notes-cli-md-direct, notes-cli-md-tool.
docs binary:
    cargo run -q -p {{ binary }} -- docs

# Run the whole gate, in the order CI runs it.
ci: fmt build clippy doc test package

# Remove build artifacts.
clean:
    cargo clean
