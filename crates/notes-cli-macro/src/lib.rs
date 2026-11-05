//! Declares a CLI program from its Markdown spec at compile time, the declaration
//! [`notes_cli_rust_gen::generate`] writes, without a build script.
//!
//! Both macros expand, where they are invoked, to a private module holding the declaration and a
//! `pub use` of everything it declares, `<Name>CliExt` among it. The invoking module defines the
//! operations the spec names, `Op_add_note` and `ArgNames_add_note` for `add_note`.
//!
//! ```ignore
//! // The spec in a file, relative to the crate root; editing it rebuilds the crate.
//! notes_cli_macro::markdown_cli_src!("src/cli.md");
//!
//! // The spec inline.
//! notes_cli_macro::markdown_cli!(r"
//! # notes
//! ...
//! ");
//! ```

mod expand;

use proc_macro::TokenStream;
use syn::{LitStr, parse_macro_input};

/// Declares the CLI program the Markdown file at the given path states, relative to the crate
/// root, the directory of its `Cargo.toml`.
#[proc_macro]
pub fn markdown_cli_src(input: TokenStream) -> TokenStream {
    expand::file(&parse_macro_input!(input as LitStr)).into()
}

/// Declares the CLI program the given Markdown states.
#[proc_macro]
pub fn markdown_cli(input: TokenStream) -> TokenStream {
    expand::inline(&parse_macro_input!(input as LitStr)).into()
}
