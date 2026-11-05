use notes::{Note, NotebookAlg, Refusal};
use std::fmt::Display;
use std::fs;
use std::io::ErrorKind;

/// Notes kept one per line in the file each store names.
pub struct NoteFiles;

/// Reads a store's failure as a refusal naming the store.
fn refused(store: &str, error: impl Display) -> Refusal {
    Refusal::Store(format!("{store}: {error}"))
}

/// Reads one line of a store file as a note.
fn read_line(line: &str) -> Option<Note> {
    let mut fields = line.splitn(4, '\t');
    let id = fields.next()?.parse().ok()?;
    let pinned = fields.next()? == "*";
    let tags = fields.next()?.split(' ').filter(|tag| !tag.is_empty()).map(str::to_owned).collect();
    let text = fields.next()?.to_owned();
    Some(Note { id, tags, pinned, text })
}

/// Writes a note as one line of a store file.
fn write_line(note: &Note) -> String {
    let pin = if note.pinned { "*" } else { "" };
    format!("{}\t{pin}\t{}\t{}\n", note.id, note.tags.join(" "), note.text)
}

impl NotebookAlg for NoteFiles {
    fn notebook_read(&self, store: &str) -> Result<Vec<Note>, Refusal> {
        let text = match fs::read_to_string(store) {
            Ok(text) => text,
            Err(error) if error.kind() == ErrorKind::NotFound => return Ok(Vec::new()),
            Err(error) => return Err(refused(store, error)),
        };
        text.lines()
            .enumerate()
            .map(|(index, line)| {
                read_line(line).ok_or_else(|| refused(store, format_args!("line {} is no note", index + 1)))
            })
            .collect()
    }

    fn notebook_write(&self, store: &str, notes: &[Note]) -> Result<(), Refusal> {
        let unwritable = |text: &str| text.contains(['\t', '\n', '\r']);
        if let Some(note) = notes
            .iter()
            .find(|note| unwritable(&note.text) || note.tags.iter().any(|tag| unwritable(tag) || tag.contains(' ')))
        {
            return Err(refused(store, format_args!("note {} does not fit on one line", note.id)));
        }
        fs::write(store, notes.iter().map(write_line).collect::<String>()).map_err(|error| refused(store, error))
    }
}
