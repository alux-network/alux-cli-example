//! The notes app prints the Markdown its command line is stated in.

use notes_cli_markdown::CliDocsAlg;
use notes_cli_md_direct::App;

/// The command line folds back to `cli.md`.
#[test]
fn prints_the_markdown_spec() {
    assert_eq!(App.cli_docs(), include_str!("../../../notes-cli-md/src/cli.md"));
}
