use crate::{Note, NotebookAlg, Refusal};
use alux_ext::ext;

/// The notes operations.
#[ext(name = NotesExt, defunc)]
pub impl<This> This
where
    This: NotebookAlg,
{
    /// Adds a note to the store.
    fn add_note(&self, store: String, text: String, tags: Vec<String>, pinned: bool) -> Result<u64, Refusal> {
        if text.trim().is_empty() {
            return Err(Refusal::Empty);
        }
        let mut notes = self.notebook_read(&store)?;
        let id = notes.iter().map(|note| note.id).max().unwrap_or(0) + 1;
        notes.push(Note { id, tags, pinned, text });
        self.notebook_write(&store, &notes)?;
        Ok(id)
    }

    /// Lists the notes in the store, oldest first.
    fn list_notes(&self, store: String, tag: Option<String>, limit: usize) -> Result<Vec<String>, Refusal> {
        let notes = self.notebook_read(&store)?;
        let tagged = |note: &&Note| tag.as_ref().is_none_or(|tag| note.tags.contains(tag));
        Ok(notes.iter().filter(tagged).take(limit).map(Note::to_string).collect())
    }

    /// Removes a note from the store.
    fn remove_note(&self, store: String, id: u64) -> Result<(), Refusal> {
        let mut notes = self.notebook_read(&store)?;
        let before = notes.len();
        notes.retain(|note| note.id != id);
        if notes.len() == before {
            return Err(Refusal::Missing(id));
        }
        self.notebook_write(&store, &notes)
    }
}
