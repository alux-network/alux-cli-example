//! A tool over Markdown CLI specs whose own command line is the Markdown spec
//! [`cli.md`](cli.md), compiled by the generator it exposes.

mod tool;

pub use tool::*;

use notes_cli_macro::markdown_cli_src;

markdown_cli_src!("src/cli.md");
