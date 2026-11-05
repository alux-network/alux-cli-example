use crate::{Note, Refusal};

/// Holds the notes of every store, each store named.
pub trait NotebookAlg {
    /// Reads the notes `store` holds, in the order they were written; a store never written holds
    /// none.
    ///
    /// # Errors
    ///
    /// Answers [`Refusal::Store`] when the store cannot be read.
    fn notebook_read(&self, store: &str) -> Result<Vec<Note>, Refusal>;

    /// Replaces the notes `store` holds with `notes`.
    ///
    /// # Errors
    ///
    /// Answers [`Refusal::Store`] when the store cannot be written.
    fn notebook_write(&self, store: &str, notes: &[Note]) -> Result<(), Refusal>;
}
