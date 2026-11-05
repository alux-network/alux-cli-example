//! Generates the Rust declaration of a CLI program from its [`CliSpec`](notes_cli_describe::CliSpec): the same
//! `#[ext(defunc(via = cli))]` table a person writes, so a program stated in Markdown compiles to
//! the program a Rust declaration states.
//!
//! `notes-cli-macro` expands it in place, from a Markdown file or inline
//! Markdown.

mod generate;

pub use generate::*;
