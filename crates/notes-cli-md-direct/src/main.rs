//! Runs the notes app over the command line stated in Markdown.

use alux_cli::CliProgramBuilder;
use alux_cli_direct::CliDirect;
use notes_cli_md::NotesCliExt;
use notes_cli_md_direct::App;

fn main() {
    CliDirect::new(App).named("notes").run_main(CliProgramBuilder.notes_cli::<App>())
}
