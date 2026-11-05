//! Runs the tool over Markdown CLI specs.

use alux_cli::CliProgramBuilder;
use alux_cli_direct::CliDirect;
use notes_cli_md_tool::{NotesCliMdToolCliExt, Tool};

fn main() {
    CliDirect::new(Tool).named("notes-cli-md-tool").run_main(CliProgramBuilder.notes_cli_md_tool_cli::<Tool>())
}
