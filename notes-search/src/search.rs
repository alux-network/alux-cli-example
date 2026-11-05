use alux_ext::ext;
use notes::{Note, NotebookAlg, Refusal};

/// The search operations.
#[ext(name = SearchExt, defunc)]
pub impl<This> This
where
    This: NotebookAlg,
{
    /// Finds the notes holding every word, pinned first.
    fn search_notes(&self, store: String, words: Vec<String>, limit: usize) -> Result<Vec<String>, Refusal> {
        let mut notes = self.notebook_read(&store)?;
        let words: Vec<String> = words.iter().map(|word| word.to_lowercase()).collect();
        notes.retain(|note| {
            let text = note.text.to_lowercase();
            words.iter().all(|word| text.contains(word.as_str()))
        });
        notes.sort_by_key(|note| (!note.pinned, note.id));
        Ok(notes.iter().take(limit).map(Note::to_string).collect())
    }
}
