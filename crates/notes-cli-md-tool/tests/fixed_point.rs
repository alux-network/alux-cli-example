//! The tool folds back to its own Markdown and checks every spec in the workspace.

use notes_cli_markdown::CliDocsAlg;
use notes_cli_md_tool::{SpecToolExt, Tool};

/// The tool's command line folds back to the Markdown it is compiled from.
#[test]
fn folds_back_to_its_own_markdown() {
    assert_eq!(Tool.cli_docs(), include_str!("../src/cli.md"));
}

/// Every spec in the workspace reads back exactly as written.
#[test]
fn checks_the_workspace_specs() {
    for markdown in [
        concat!(env!("CARGO_MANIFEST_DIR"), "/src/cli.md"),
        concat!(env!("CARGO_MANIFEST_DIR"), "/../../notes-cli-md/src/cli.md"),
    ] {
        Tool.check(markdown.to_owned()).expect("the spec reads back as written");
    }
}
