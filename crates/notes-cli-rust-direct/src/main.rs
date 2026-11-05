//! Runs the notes app over the command line declared in Rust.

use alux_cli::CliProgramBuilder;
use alux_cli_direct::CliDirect;
use notes_cli_rust::NotesCliExt;
use notes_cli_rust_direct::App;

fn main() {
    CliDirect::new(App).named("notes").run_main(CliProgramBuilder.notes_cli::<App>())
}
