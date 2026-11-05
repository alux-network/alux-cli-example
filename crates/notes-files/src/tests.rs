use super::*;
use notes::{Note, NotebookAlg};
use notes_conformance::NotesLawsExt;
use std::env;
use std::fs;
use std::process;

/// Notes kept in files satisfy every law.
#[test]
fn satisfies_the_laws() {
    let dir = env::temp_dir().join(format!("notes-files-{}", process::id()));
    fs::create_dir_all(&dir).expect("a temporary directory");
    let laws = NoteFiles.laws(|name| dir.join(name).display().to_string());
    fs::remove_dir_all(&dir).expect("the temporary directory is removed");
    laws.expect("a law holds");
}

/// A note with a tab in its text does not fit on a line, and is refused.
#[test]
fn refuses_text_that_does_not_fit_a_line() {
    let note = Note { id: 1, tags: Vec::new(), pinned: false, text: "a\tb".to_owned() };
    assert!(NoteFiles.notebook_write("unused", &[note]).is_err());
}
