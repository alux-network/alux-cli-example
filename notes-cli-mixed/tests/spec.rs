//! The merged command line states the one notes spec.

use alux_cli::CliProgramBuilder;
use notes_cli_describe::Describe;
use notes_cli_markdown::render;
use notes_cli_mixed::NotesCliExt;
use notes_cli_rust_direct::App;

/// Markdown and Rust merged fold back to `cli.md`.
#[test]
fn folds_back_to_the_markdown_spec() {
    let spec = Describe::spec("notes", CliProgramBuilder.notes_cli::<App>()).expect("a spec");
    assert_eq!(render(&spec), include_str!("../../notes-cli-md/src/cli.md"));
}
