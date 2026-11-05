use alux_cli::CliProgramBuilder;
use notes::{Note, NotebookAlg, Refusal};
use notes_cli_describe::Describe;
use notes_cli_markdown::{CliDocsAlg, render};
use notes_cli_md::NotesCliExt;
use notes_files::NoteFiles;

/// The notes app: notes kept in files, and its own spec.
pub struct App;

impl NotebookAlg for App {
    fn notebook_read(&self, store: &str) -> Result<Vec<Note>, Refusal> {
        NoteFiles.notebook_read(store)
    }

    fn notebook_write(&self, store: &str, notes: &[Note]) -> Result<(), Refusal> {
        NoteFiles.notebook_write(store, notes)
    }
}

impl CliDocsAlg for App {
    fn cli_docs(&self) -> String {
        let spec = Describe::spec("notes", CliProgramBuilder.notes_cli::<App>());
        render(&spec.expect("the notes command line states a spec"))
    }
}
